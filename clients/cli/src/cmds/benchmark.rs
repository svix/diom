use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::{Duration, Instant};
use futures::future::try_join_all;

use anyhow::Result;
use clap::Args;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL_CONDENSED, Table};
use coyote_client::{
    CoyoteClient,
    models::{
        Ack, AppendToStreamIn, CacheGetIn, CacheSetIn, CreateNamespaceIn, FetchFromStreamIn,
        KvGetIn, KvSetIn, MsgIn,
    },
};
use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use hdrhistogram::Histogram;
use indicatif::{ProgressBar, ProgressStyle};
use rand::{
    Rng,
    SeedableRng,
    distr::{Alphanumeric, SampleString},
    rngs::StdRng,
};
use serde::Serialize;

// TODO(238): Idempotency/Rate-limit does not currently work in SDK

type BenchHistogram = Histogram<u64>;

#[derive(Clone, Debug, PartialEq, clap::ValueEnum)]
pub enum BenchmarkModule {
    Kv,
    Cache,
    Stream,
}

#[derive(Args)]
pub struct BenchmarkArgs {
    /// Server URL to benchmark against (overrides config)
    #[arg(value_name = "URL")]
    pub server_url: Option<String>,

    /// Duration of each round in seconds
    #[arg(long, default_value_t = 5)]
    pub duration: u64,

    /// Number of measurement rounds (samples are merged across rounds)
    #[arg(long, default_value_t = 1)]
    pub rounds: usize,

    /// Concurrent tasks per operation type
    #[arg(short = 'c', long, default_value_t = 1)]
    pub concurrency: usize,

    /// Max ops/sec rate limit per task (0 = unlimited)
    #[arg(short, long, default_value_t = 0)]
    pub rate: u32,

    /// Output results as JSON instead of a table
    #[arg(long)]
    pub json: bool,

    /// Modules to benchmark (default: all)
    #[arg(short, long, value_delimiter = ',')]
    pub modules: Vec<BenchmarkModule>,
}

// ── Statistics ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct Stats {
    op: String,
    ops_per_sec: f64,
    mean_us: u64,
    std_dev_us: u64,
    p50_us: u64,
    p99_us: u64,
    p999_us: u64,
    max_us: u64,
}

/// `wall_clock_secs` is the total elapsed time across all rounds (rounds are
/// sequential; tasks within a round are concurrent, so wall time = rounds × duration).
fn compute_stats(op: impl Into<String>, mut samples: Vec<Duration>, wall_clock_secs: f64) -> Stats {
    assert!(!samples.is_empty());
    samples.sort_unstable();

    let n = samples.len();
    let us: Vec<f64> = samples
        .iter()
        .map(|d| d.as_secs_f64() * 1_000_000.0)
        .collect();

    let mean_us = us.iter().sum::<f64>() / n as f64;
    let variance = us.iter().map(|x| (x - mean_us).powi(2)).sum::<f64>() / n as f64;

    let pct = |p: f64| -> f64 {
        let idx = ((p / 100.0) * (n - 1) as f64).round() as usize;
        us[idx.min(n - 1)]
    };

    Stats {
        op: op.into(),
        ops_per_sec: n as f64 / wall_clock_secs,
        mean_us: mean_us as u64,
        std_dev_us: variance.sqrt() as u64,
        p50_us: pct(50.0) as u64,
        p99_us: pct(99.0) as u64,
        p999_us: pct(99.9) as u64,
        max_us: us[n - 1] as u64,
    }
}

/// `wall_clock_secs` is the total elapsed time across all rounds (rounds are
/// sequential; tasks within a round are concurrent, so wall time = rounds × duration).
fn hist_compute_stats(op: impl Into<String>, hist: BenchHistogram, wall_clock_secs: f64) -> Stats {
    Stats {
        op: op.into(),
        ops_per_sec: 0.0,
        mean_us: hist.mean() as u64,
        std_dev_us: hist.stdev() as u64,
        p50_us: hist.value_at_quantile(0.50),
        p99_us: hist.value_at_quantile(0.99),
        p999_us: hist.value_at_quantile(0.999),
        max_us: hist.max(),
    }
}

// ── Formatting helpers ────────────────────────────────────────────────────────

fn fmt_us(us: u64) -> String {
    if us >= 1_000_000 {
        format!("{:.2}s", us as f64 / 1_000_000.0)
    } else if us >= 1_000 {
        format!("{:.2}ms", us as f64 / 1_000.0)
    } else {
        format!("{:.2}µs", us)
    }
}

fn print_table(all_stats: &[Stats]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            "op", "ops/sec", "mean", "±", "p50", "p99", "p99.9", "max",
        ]);
    for s in all_stats {
        table.add_row(vec![
            s.op.clone(),
            format!("{:.0}", s.ops_per_sec),
            fmt_us(s.mean_us),
            fmt_us(s.std_dev_us),
            fmt_us(s.p50_us),
            fmt_us(s.p99_us),
            fmt_us(s.p999_us),
            fmt_us(s.max_us),
        ]);
    }
    println!("{table}");
}

// ── Progress bar helpers ──────────────────────────────────────────────────────

fn new_bar(prefix: impl Into<String>, duration_secs: u64) -> ProgressBar {
    let pb = ProgressBar::new(duration_secs);
    pb.set_style(
        ProgressStyle::with_template(
            "  {prefix:.bold} [{bar:40.cyan/blue}] {elapsed_precise}",
        )
        .unwrap()
        .progress_chars("=>-"),
    );
    pb.set_prefix(prefix.into());
    pb
}

fn start_ticker(pb: ProgressBar, duration_secs: u64) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let start = std::time::Instant::now();
        loop {
            let secs = start.elapsed().as_secs();
            pb.set_position(secs.min(duration_secs));
            if secs >= duration_secs {
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
}

// ── Rate limiter helpers ──────────────────────────────────────────────────────

fn make_limiter(rate: u32) -> Option<Arc<DefaultDirectRateLimiter>> {
    match rate {
        0 => None,
        n => Some(Arc::new(RateLimiter::direct(
            Quota::per_second(NonZeroU32::new(n).unwrap()),
        ))),
    }
}

async fn maybe_wait(limiter: Option<&DefaultDirectRateLimiter>) {
    if let Some(lim) = limiter {
        lim.until_ready().await;
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

impl BenchmarkArgs {
    pub async fn exec(self, client: Arc<CoyoteClient>) -> Result<()> {
        let duration = self.duration;
        let rounds = self.rounds;
        let concurrency = self.concurrency;
        let rate = self.rate;
        let json = self.json;

        let modules = if self.modules.is_empty() {
            vec![
                BenchmarkModule::Kv,
                BenchmarkModule::Cache,
                BenchmarkModule::Stream,
            ]
        } else {
            self.modules
        };

        let rate_str = match rate {
            0 => String::new(),
            n => format!(" · rate limit {n} rps/task"),
        };
        eprintln!(
            "Running benchmark: {duration}s × {rounds} rounds · {concurrency} concurrent{rate_str}",
        );

        // Each op type's wall-clock time: rounds run sequentially, tasks within a round run concurrently.
        let wall_clock_secs = rounds as f64 * duration as f64;
        let mut all_stats: Vec<Stats> = Vec::new();

        for module in &modules {
            eprintln!();
            match module {
                BenchmarkModule::Kv => {
                    eprintln!("[kv]");
                    bench_kv(
                        Arc::clone(&client),
                        duration,
                        rounds,
                        concurrency,
                        rate,
                        wall_clock_secs,
                        &mut all_stats,
                    )
                    .await?;
                }
                BenchmarkModule::Cache => {
                    eprintln!("[cache]");
                    bench_cache(
                        Arc::clone(&client),
                        duration,
                        rounds,
                        concurrency,
                        rate,
                        wall_clock_secs,
                        &mut all_stats,
                    )
                    .await?;
                }
                BenchmarkModule::Stream => {
                    eprintln!("[stream]");
                    bench_stream(
                        Arc::clone(&client),
                        duration,
                        rounds,
                        concurrency,
                        rate,
                        wall_clock_secs,
                        &mut all_stats,
                    )
                    .await?;
                }
            }
        }

        eprintln!();

        if json {
            println!("{}", serde_json::to_string_pretty(&all_stats)?);
        } else {
            print_table(&all_stats);
        }

        Ok(())
    }
}

// ── kv ────────────────────────────────────────────────────────────────────────

trait BenchmarkTest {
    fn name(&self) -> &'static str;

    async fn run(
        &self,
        client: &CoyoteClient,
        rng: &mut StdRng,
    ) -> Result<Duration>;
}

#[derive(Clone)]
struct BenchKvSet {
}

impl BenchKvSet {
    fn new() -> Self {
        Self {}
    }
}

impl BenchmarkTest for BenchKvSet {
    fn name(&self) -> &'static str {
        "kv.set"
    }

    async fn run(
        &self,
        client: &CoyoteClient,
        rng: &mut StdRng,
    ) -> Result<Duration> {
        let key = Alphanumeric.sample_string(rng, 16);
        let mut value = vec![0u8; 256];
        rng.fill(&mut value[..]);

        // Start of real code
        let t = quanta::Instant::now();
        client.kv().set(KvSetIn::new(key.clone(), value)).await?;
        Ok(t.elapsed())
    }
}

#[derive(Clone)]
struct BenchKvGet {
}

impl BenchKvGet {
    fn new() -> Self {
        Self {}
    }
}

impl BenchmarkTest for BenchKvGet {
    fn name(&self) -> &'static str {
        "kv.get"
    }

    async fn run(
        &self,
        client: &CoyoteClient,
        rng: &mut StdRng,
    ) -> Result<Duration> {
        let key = Alphanumeric.sample_string(rng, 16);

        // Start of real code
        let t = quanta::Instant::now();
        client.kv().get(KvGetIn::new(key.clone())).await?;
        Ok(t.elapsed())
    }
}

async fn bench_kv(
    client: Arc<CoyoteClient>,
    duration_secs: u64,
    rounds: usize,
    concurrency: usize,
    rate: u32,
    wall_clock_secs: f64,
    all_stats: &mut Vec<Stats>,
) -> Result<()> {
    let test  = BenchKvSet::new();

    for round in 1..=rounds {
        // Set phase
        let test = test.clone();
        let pb = new_bar(format!("{} {round}/{rounds}", test.name()), duration_secs);
        let ticker = start_ticker(pb.clone(), duration_secs);
        let handles = (0..concurrency)
            .map(|i| {
                let client = Arc::clone(&client);
                let limiter = make_limiter(rate);
                let test = test.clone();
                tokio::spawn(async move {
                    let mut hist = BenchHistogram::new(3)?;
                    let mut rng = StdRng::seed_from_u64((i * round) as u64);
                    let deadline = Instant::now() + Duration::from_secs(duration_secs);
                    while Instant::now() < deadline {
                        maybe_wait(limiter.as_deref()).await;
                        let t = test.run(&client, &mut rng).await?;
                        hist.record(t.as_micros() as u64).unwrap();
                    }
                    Ok::<BenchHistogram, anyhow::Error>(hist)
                })
            });
        let joined_handles = try_join_all(handles)
            .await?
            .into_iter();

        let mut combined = BenchHistogram::new(3).unwrap();
        for handle in joined_handles {
            let hist = handle?;
            combined.add(hist)?;
        }
        ticker.abort();
        pb.set_position(duration_secs);
        pb.finish();
        all_stats.push(hist_compute_stats(test.name(), combined, wall_clock_secs));
    }

    Ok(())
}

// ── cache ─────────────────────────────────────────────────────────────────────

async fn bench_cache(
    client: Arc<CoyoteClient>,
    duration_secs: u64,
    rounds: usize,
    concurrency: usize,
    rate: u32,
    wall_clock_secs: f64,
    all_stats: &mut Vec<Stats>,
) -> Result<()> {
    const TTL_MS: u64 = 300_000; // 5 minutes — outlives the whole benchmark run

    let mut set_samples: Vec<Duration> = Vec::new();
    let mut get_samples: Vec<Duration> = Vec::new();

    for round in 1..=rounds {
        // Set phase
        let pb = new_bar(format!("set {round}/{rounds}"), duration_secs);
        let ticker = start_ticker(pb.clone(), duration_secs);
        let handles: Vec<_> = (0..concurrency)
            .map(|i| {
                let client = Arc::clone(&client);
                let limiter = make_limiter(rate);
                tokio::spawn(async move {
                    let mut rng = StdRng::seed_from_u64((i * round) as u64);
                    let mut keys = Vec::new();
                    let mut samples = Vec::new();
                    let deadline = Instant::now() + Duration::from_secs(duration_secs);
                    while Instant::now() < deadline {
                        maybe_wait(limiter.as_deref()).await;
                        let key = Alphanumeric.sample_string(&mut rng, 16);
                        let mut value = vec![0u8; 256];
                        rng.fill(&mut value[..]);
                        let t = quanta::Instant::now();
                        client
                            .cache()
                            .set(CacheSetIn::new(key.clone(), TTL_MS, value))
                            .await?;
                        samples.push(t.elapsed());
                        keys.push(key);
                    }
                    Ok::<(Vec<String>, Vec<Duration>), anyhow::Error>((keys, samples))
                })
            })
            .collect();
        let mut round_keys: Vec<String> = Vec::new();
        for handle in handles {
            let (keys, samples) = handle.await??;
            round_keys.extend(keys);
            set_samples.extend(samples);
        }
        ticker.abort();
        pb.set_position(duration_secs);
        pb.finish();

        // Get phase
        if !round_keys.is_empty() {
            let pb = new_bar(format!("get {round}/{rounds}"), duration_secs);
            let ticker = start_ticker(pb.clone(), duration_secs);
            let round_keys = Arc::new(round_keys);
            let handles: Vec<_> = (0..concurrency)
                .map(|task_idx| {
                    let client = Arc::clone(&client);
                    let keys = Arc::clone(&round_keys);
                    let limiter = make_limiter(rate);
                    tokio::spawn(async move {
                        let mut samples = Vec::new();
                        let deadline = Instant::now() + Duration::from_secs(duration_secs);
                        let mut idx = task_idx;
                        while Instant::now() < deadline {
                            maybe_wait(limiter.as_deref()).await;
                            let key = &keys[idx % keys.len()];
                            let t = quanta::Instant::now();
                            client.cache().get(CacheGetIn::new(key.clone())).await?;
                            samples.push(t.elapsed());
                            idx = idx.wrapping_add(concurrency.max(1));
                        }
                        Ok::<Vec<Duration>, anyhow::Error>(samples)
                    })
                })
                .collect();
            for handle in handles {
                get_samples.extend(handle.await??);
            }
            ticker.abort();
            pb.set_position(duration_secs);
            pb.finish();
        }
    }

    if !set_samples.is_empty() {
        all_stats.push(compute_stats("cache set", set_samples, wall_clock_secs));
    }
    if !get_samples.is_empty() {
        all_stats.push(compute_stats("cache get", get_samples, wall_clock_secs));
    }
    Ok(())
}

// ── stream ────────────────────────────────────────────────────────────────────

async fn bench_stream(
    client: Arc<CoyoteClient>,
    duration_secs: u64,
    rounds: usize,
    concurrency: usize,
    rate: u32,
    wall_clock_secs: f64,
    all_stats: &mut Vec<Stats>,
) -> Result<()> {
    // Each measurement round uses its own namespace so consumer groups start fresh.

    let mut append_samples: Vec<Duration> = Vec::new();
    let mut fetch_samples: Vec<Duration> = Vec::new();

    for round in 1..=rounds {
        let mut rng = StdRng::seed_from_u64(round as u64);
        let ns = format!(
            "__bench_r{}_{}", round,
            Alphanumeric.sample_string(&mut rng, 8)
        );
        client
            .msgs()
            .namespace()
            .create(CreateNamespaceIn::new(ns.clone()))
            .await?;

        // Append phase
        let pb = new_bar(format!("append    {round}/{rounds}"), duration_secs);
        let ticker = start_ticker(pb.clone(), duration_secs);
        let handles: Vec<_> = (0..concurrency)
            .map(|i| {
                let client = Arc::clone(&client);
                let ns = ns.clone();
                let limiter = make_limiter(rate);
                tokio::spawn(async move {
                    let mut rng = StdRng::seed_from_u64((i * round) as u64);
                    let mut samples = Vec::new();
                    let deadline = Instant::now() + Duration::from_secs(duration_secs);
                    while Instant::now() < deadline {
                        maybe_wait(limiter.as_deref()).await;
                        let mut payload = vec![0u8; 256];
                        rng.fill(&mut payload[..]);
                        let t = quanta::Instant::now();
                        client
                            .stream()
                            .append(AppendToStreamIn::new(
                                vec![MsgIn::new(payload)],
                                ns.clone(),
                            ))
                            .await?;
                        samples.push(t.elapsed());
                    }
                    Ok::<Vec<Duration>, anyhow::Error>(samples)
                })
            })
            .collect();
        for handle in handles {
            append_samples.extend(handle.await??);
        }
        ticker.abort();
        pb.set_position(duration_secs);
        pb.finish();

        // Fetch/ack phase: each task gets its own consumer group so they
        // independently drain all messages in the namespace.
        let pb = new_bar(format!("fetch/ack {round}/{rounds}"), duration_secs);
        let ticker = start_ticker(pb.clone(), duration_secs);
        let handles: Vec<_> = (0..concurrency)
            .map(|task_idx| {
                let client = Arc::clone(&client);
                let ns = ns.clone();
                let consumer_group = format!("__bench_consumer_{task_idx}");
                let limiter = make_limiter(rate);
                tokio::spawn(async move {
                    let mut samples = Vec::new();
                    let deadline = Instant::now() + Duration::from_secs(duration_secs);
                    while Instant::now() < deadline {
                        maybe_wait(limiter.as_deref()).await;
                        let t = quanta::Instant::now();
                        let out = client
                            .stream()
                            .fetch(FetchFromStreamIn::new(
                                1,
                                consumer_group.clone(),
                                ns.clone(),
                                30,
                            ))
                            .await?;
                        if out.msgs.is_empty() {
                            // Stream exhausted for this consumer group
                            break;
                        }
                        for msg in out.msgs {
                            client
                                .stream()
                                .ack(Ack::new(
                                    consumer_group.clone(),
                                    msg.id,
                                    ns.clone(),
                                ))
                                .await?;
                            samples.push(t.elapsed());
                        }
                    }
                    Ok::<Vec<Duration>, anyhow::Error>(samples)
                })
            })
            .collect();
        for handle in handles {
            fetch_samples.extend(handle.await??);
        }
        ticker.abort();
        pb.set_position(duration_secs);
        pb.finish();
    }

    if !append_samples.is_empty() {
        all_stats.push(compute_stats("stream append", append_samples, wall_clock_secs));
    }
    if !fetch_samples.is_empty() {
        all_stats.push(compute_stats(
            "stream fetch/ack",
            fetch_samples,
            wall_clock_secs,
        ));
    }
    Ok(())
}
