use std::{sync::Arc, time::Duration};

use anyhow::Result;
use clap::Args;
use comfy_table::{Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL_CONDENSED};
use coyote_client::{
    CoyoteClient,
    models::{
        CacheGetIn, CacheSetIn, CreateNamespaceIn, KvGetIn, KvSetIn, MsgIn, PublishIn,
        StreamCommitIn, StreamReceiveIn,
    },
};
use futures::future::try_join_all;
use hdrhistogram::Histogram;
use indicatif::{ProgressBar, ProgressStyle};
use rand::{
    Rng, SeedableRng,
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
    Msgs,
}

#[derive(Args)]
pub struct BenchmarkArgs {
    /// Server URL to benchmark against (overrides config)
    #[arg(value_name = "URL")]
    pub server_url: Option<String>,

    /// Number of iterations
    #[arg(long, default_value_t = 1000)]
    iterations: u64,

    /// Concurrent tasks per operation type
    #[arg(short = 'c', long, default_value_t = 4)]
    pub concurrency: u64,

    /// Output results as JSON instead of a table
    #[arg(long)]
    pub json: bool,

    /// Modules to benchmark (default: all)
    #[arg(short, long, value_delimiter = ',')]
    pub modules: Vec<BenchmarkModule>,
}

impl BenchmarkArgs {
    pub async fn exec(self, client: Arc<CoyoteClient>) -> Result<()> {
        let iterations = self.iterations;
        let concurrency = self.concurrency;

        let modules = if self.modules.is_empty() {
            vec![
                BenchmarkModule::Kv,
                BenchmarkModule::Cache,
                BenchmarkModule::Msgs,
            ]
        } else {
            self.modules
        };

        eprintln!("Running benchmark: {iterations} iterations · {concurrency} concurrent",);

        // Each op type's wall-clock time: rounds run sequentially, tasks within a round run concurrently.
        let mut all_stats: Vec<Stats> = Vec::new();

        for module in &modules {
            eprintln!();
            match module {
                BenchmarkModule::Kv => {
                    eprintln!("[kv]");
                    bench_kv(Arc::clone(&client), &mut all_stats, concurrency, iterations).await?;
                }
                BenchmarkModule::Cache => {
                    eprintln!("[cache]");
                    bench_cache(Arc::clone(&client), &mut all_stats, concurrency, iterations)
                        .await?;
                }
                BenchmarkModule::Msgs => {
                    eprintln!("[msgs]");
                    bench_msgs(Arc::clone(&client), &mut all_stats, concurrency, iterations)
                        .await?;
                }
            }
        }

        eprintln!("\n");

        if self.json {
            println!("{}", serde_json::to_string_pretty(&all_stats)?);
        } else {
            print_table(&all_stats);
        }

        Ok(())
    }
}

// Statistics

#[derive(Debug, Serialize)]
struct Stats {
    op: String,
    ops_per_sec: u64,
    mean_us: u64,
    std_dev_us: u64,
    p50_us: u64,
    p99_us: u64,
    p999_us: u64,
    max_us: u64,
}

fn hist_compute_stats(
    op: impl Into<String>,
    hist: BenchHistogram,
    total_time_ms: u64,
    operations: u64,
) -> Stats {
    Stats {
        op: op.into(),
        ops_per_sec: (operations * 1_000) / total_time_ms,
        mean_us: hist.mean() as u64,
        std_dev_us: hist.stdev() as u64,
        p50_us: hist.value_at_quantile(0.50),
        p99_us: hist.value_at_quantile(0.99),
        p999_us: hist.value_at_quantile(0.999),
        max_us: hist.max(),
    }
}

// Formatting helpers

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

fn new_bar(prefix: impl Into<String>, iterations: u64) -> ProgressBar {
    let pb = ProgressBar::new(iterations);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {prefix:20.bold} [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_prefix(prefix.into());
    pb
}

// Benchmark helpers

async fn bench_shards_concurrent(
    client: Arc<CoyoteClient>,
    test_name: &str,
    all_instances: Vec<impl BenchShard>,
    iterations: u64,
    all_stats: &mut Vec<Stats>,
) -> Result<()> {
    let concurrency = all_instances.len() as u64;

    let pb = new_bar(test_name.to_string(), iterations);
    let handles = all_instances.into_iter().enumerate().map(|(shard_id, test)| {
        let client = Arc::clone(&client);
        let pb = pb.clone();
        test.bench_shard(client, iterations, pb, shard_id as u64)
    });

    let mut combined = BenchHistogram::new(3).unwrap();
    let mut total_time_ms = 0;
    let joined_handles = try_join_all(handles).await?.into_iter();
    pb.finish();
    for res in joined_handles {
        combined.add(res.hist)?;
        total_time_ms += res.total_time.as_millis() as u64;
    }
    // Get the average time per run
    total_time_ms /= concurrency;

    all_stats.push(hist_compute_stats(
        test_name,
        combined,
        total_time_ms,
        iterations * concurrency,
    ));

    Ok(())
}

struct BenchResult {
    hist: BenchHistogram,
    total_time: Duration,
}

trait BenchShard {
    async fn run(&self, client: &CoyoteClient, rng: &mut StdRng, shard_id: u64, iteration: u64) -> Result<Duration>;

    async fn bench_shard(
        self,
        client: Arc<CoyoteClient>,
        iterations: u64,
        pb: ProgressBar,
        shard_id: u64,
    ) -> Result<BenchResult>
    where
        Self: Sized,
    {
        let mut hist = BenchHistogram::new(3)?;
        let mut total_time = Duration::from_secs(0);
        let mut rng = StdRng::seed_from_u64(0);

        for iteration in 0..iterations {
            let t = self.run(&client, &mut rng, shard_id, iteration).await?;
            hist.record(t.as_micros() as u64)?;
            total_time += t;
            pb.set_position(iteration);
        }
        Ok(BenchResult { hist, total_time })
    }
}

// KV module

#[derive(Clone)]
struct BenchKvSet {
    keys: Arc<Vec<String>>,
}

impl BenchKvSet {
    fn setup(keys: Arc<Vec<String>>) -> Self {
        Self { keys }
    }
}

impl BenchShard for BenchKvSet {
    async fn run(&self, client: &CoyoteClient, rng: &mut StdRng, _shard_id: u64, iteration: u64) -> Result<Duration> {
        let key = self.keys.get(iteration as usize).unwrap();
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
    keys: Arc<Vec<String>>,
}

impl BenchKvGet {
    fn setup(keys: Arc<Vec<String>>) -> Self {
        Self { keys }
    }
}

impl BenchShard for BenchKvGet {
    async fn run(&self, client: &CoyoteClient, rng: &mut StdRng, _shard_id: u64, iteration: u64) -> Result<Duration> {
        let key = self.keys.get(iteration as usize).unwrap();
        let mut value = vec![0u8; 256];
        rng.fill(&mut value[..]);

        // Start of real code
        let t = quanta::Instant::now();
        client.kv().get(KvGetIn::new(key.clone())).await?;
        Ok(t.elapsed())
    }
}

async fn bench_kv(
    client: Arc<CoyoteClient>,
    all_stats: &mut Vec<Stats>,
    concurrency: u64,
    iterations: u64,
) -> Result<()> {
    let mut all_kv_set: Vec<_> = Vec::with_capacity(concurrency as usize);
    let mut all_kv_get: Vec<_> = Vec::with_capacity(concurrency as usize);
    for shard_id in 0..concurrency {
        let mut rng = StdRng::seed_from_u64(shard_id);
        let keys: Arc<Vec<_>> = Arc::new(
            (0..iterations)
                .map(|_| Alphanumeric.sample_string(&mut rng, 16))
                .collect(),
        );
        all_kv_set.push(BenchKvSet::setup(keys.clone()));
        all_kv_get.push(BenchKvGet::setup(keys.clone()));
    }

    bench_shards_concurrent(client.clone(), "kv.set", all_kv_set, iterations, all_stats).await?;
    bench_shards_concurrent(client.clone(), "kv.get", all_kv_get, iterations, all_stats).await?;
    Ok(())
}

// Cache module

#[derive(Clone)]
struct BenchCacheSet {
    keys: Arc<Vec<String>>,
}

impl BenchCacheSet {
    fn setup(keys: Arc<Vec<String>>) -> Self {
        Self { keys }
    }
}

impl BenchShard for BenchCacheSet {
    async fn run(&self, client: &CoyoteClient, rng: &mut StdRng, _shard_id: u64, iteration: u64) -> Result<Duration> {
        let ttl_bench_ms = 300_000; // 5 minutes
        let key = self.keys.get(iteration as usize).unwrap();
        let mut value = vec![0u8; 256];
        rng.fill(&mut value[..]);

        // Start of real code
        let t = quanta::Instant::now();
        client
            .cache()
            .set(CacheSetIn::new(key.clone(), ttl_bench_ms, value))
            .await?;
        Ok(t.elapsed())
    }
}

#[derive(Clone)]
struct BenchCacheGet {
    keys: Arc<Vec<String>>,
}

impl BenchCacheGet {
    fn setup(keys: Arc<Vec<String>>) -> Self {
        Self { keys }
    }
}

impl BenchShard for BenchCacheGet {
    async fn run(&self, client: &CoyoteClient, rng: &mut StdRng, _shard_id: u64, iteration: u64) -> Result<Duration> {
        let key = self.keys.get(iteration as usize).unwrap();
        let mut value = vec![0u8; 256];
        rng.fill(&mut value[..]);

        // Start of real code
        let t = quanta::Instant::now();
        client.cache().get(CacheGetIn::new(key.clone())).await?;
        Ok(t.elapsed())
    }
}

async fn bench_cache(
    client: Arc<CoyoteClient>,
    all_stats: &mut Vec<Stats>,
    concurrency: u64,
    iterations: u64,
) -> Result<()> {
    let mut all_cache_set: Vec<_> = Vec::with_capacity(concurrency as usize);
    let mut all_cache_get: Vec<_> = Vec::with_capacity(concurrency as usize);
    for shard_id in 0..concurrency {
        let mut rng = StdRng::seed_from_u64(shard_id);
        let keys: Arc<Vec<_>> = Arc::new(
            (0..iterations)
                .map(|_| Alphanumeric.sample_string(&mut rng, 16))
                .collect(),
        );
        all_cache_set.push(BenchCacheSet::setup(keys.clone()));
        all_cache_get.push(BenchCacheGet::setup(keys.clone()));
    }

    bench_shards_concurrent(
        client.clone(),
        "cache.set",
        all_cache_set,
        iterations,
        all_stats,
    )
    .await?;
    bench_shards_concurrent(
        client.clone(),
        "cache.get",
        all_cache_get,
        iterations,
        all_stats,
    )
    .await?;
    Ok(())
}

// Msgs module

#[derive(Clone)]
struct BenchMsgsPublish {}

impl BenchMsgsPublish {
    fn setup() -> Self {
        Self {}
    }
}

impl BenchShard for BenchMsgsPublish {
    async fn run(&self, client: &CoyoteClient, rng: &mut StdRng, shard_id: u64, _iteration: u64) -> Result<Duration> {
        let ns_name = "bench";
        let topic = format!("bench/topic/{shard_id}");
        let mut payload = vec![0u8; 256];
        rng.fill(&mut payload[..]);

        // Start of real code
        let t = quanta::Instant::now();
        client
            .msgs()
            .publish(PublishIn::new(
                vec![MsgIn::new(payload)],
                ns_name.to_string(),
                topic.to_owned(),
            ))
            .await?;
        Ok(t.elapsed())
    }
}

#[derive(Clone)]
struct BenchMsgsStreamReceive {}

impl BenchMsgsStreamReceive {
    fn setup() -> Self {
        Self {}
    }
}

impl BenchShard for BenchMsgsStreamReceive {
    async fn run(&self, client: &CoyoteClient, rng: &mut StdRng, shard_id: u64, _iteration: u64) -> Result<Duration> {
        let consumer_group = "consumer";
        let ns_name = "bench";
        let topic = format!("bench/topic/{shard_id}");
        let mut value = vec![0u8; 256];
        rng.fill(&mut value[..]);

        // Start of real code
        let t = quanta::Instant::now();
        let mut recv = StreamReceiveIn::new(
            consumer_group.to_owned(),
            ns_name.to_owned(),
            topic.to_owned(),
        );
        recv.batch_size = Some(1);
        let out = client.msgs().stream().receive(recv).await?;
        for msg in out.msgs {
            client
                .msgs()
                .stream()
                .commit(StreamCommitIn::new(
                    consumer_group.to_owned(),
                    ns_name.to_owned(),
                    msg.offset,
                    msg.topic.clone(),
                ))
                .await?;
        }
        Ok(t.elapsed())
    }
}

async fn bench_msgs(
    client: Arc<CoyoteClient>,
    all_stats: &mut Vec<Stats>,
    concurrency: u64,
    iterations: u64,
) -> Result<()> {
    let ns_name = "bench";

    client
        .msgs()
        .namespace()
        .create(CreateNamespaceIn::new(ns_name.to_string()))
        .await?;

    let mut all_cache_set: Vec<_> = Vec::with_capacity(concurrency as usize);
    let mut all_cache_get: Vec<_> = Vec::with_capacity(concurrency as usize);
    for _shard_id in 0..concurrency {
        all_cache_set.push(BenchMsgsPublish::setup());
        all_cache_get.push(BenchMsgsStreamReceive::setup());
    }

    bench_shards_concurrent(
        client.clone(),
        "msgs.publish (batch=1)",
        all_cache_set,
        iterations,
        all_stats,
    )
    .await?;
    // bench_shards_concurrent(client.clone(), "msgs.publish (topic=500)", all_cache_set, iterations, all_stats).await?;
    bench_shards_concurrent(
        client.clone(),
        "msgs.stream.receive (batch=1)",
        all_cache_get,
        iterations,
        all_stats,
    )
    .await?;
    Ok(())
}
