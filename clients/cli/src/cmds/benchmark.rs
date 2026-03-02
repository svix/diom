use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::Result;
use clap::Args;
use comfy_table::{Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL_CONDENSED};
use coyote_client::{
    CoyoteClient,
    models::{
        CacheGetIn, CacheSetIn, KvGetIn, KvSetIn, MsgIn, MsgNamespaceCreateIn, MsgPublishIn,
        MsgStreamCommitIn, MsgStreamReceiveIn,
    },
};
use futures_util::future::try_join_all;
use glob::Pattern;
use hdrhistogram::Histogram;
use indicatif::{ProgressBar, ProgressStyle};
use rand::{
    Rng, SeedableRng,
    distr::{Alphanumeric, SampleString},
    rngs::StdRng,
};
use serde::Serialize;
use tokio::sync::Barrier;

// TODO(238): Idempotency/Rate-limit does not currently work in SDK

type BenchHistogram = Histogram<u64>;

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

    /// Batch size for msgs publish/receive operations
    #[arg(short, long, default_value_t = 1)]
    pub batch_size: u16,

    /// Glob filter: only run benchmarks whose name matches
    #[arg(long)]
    pub filter: Option<String>,
}

impl BenchmarkArgs {
    pub async fn exec(self, client: Arc<CoyoteClient>) -> Result<()> {
        let iterations = self.iterations;
        let concurrency = self.concurrency;
        let batch_size = self.batch_size;
        eprintln!(
            "Running benchmark: {iterations} iterations · {concurrency} concurrent · batch_size {batch_size}",
        );

        let filter = self
            .filter
            .as_deref()
            .map(|f| {
                if f.contains('*') {
                    Pattern::new(f)
                } else {
                    Pattern::new(&format!("{f}*"))
                }
            })
            .transpose()?;

        // Each op type's wall-clock time: rounds run sequentially, tasks within a round run concurrently.
        let mut all_stats: Vec<Stats> = Vec::new();

        let bench_cfg = Arc::new(BenchConfig {
            client: Arc::clone(&client),
            concurrency,
            iterations,
        });

        eprintln!();
        bench_kv(Arc::clone(&bench_cfg), &mut all_stats, filter.as_ref()).await?;
        bench_cache(Arc::clone(&bench_cfg), &mut all_stats, filter.as_ref()).await?;
        bench_msgs(Arc::clone(&bench_cfg), &mut all_stats, filter.as_ref()).await?;

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
    op_batch_size: u16,
    mean_us: u64,
    std_dev_us: u64,
    p50_us: u64,
    p99_us: u64,
    p999_us: u64,
    max_us: u64,
    bytes_per_sec: u64,
}

fn hist_compute_stats(
    op: impl Into<String>,
    hist: BenchHistogram,
    total_time_ms: u64,
    operations: u64,
    total_bytes: u64,
    batch_size: u16,
) -> Stats {
    Stats {
        op: op.into(),
        ops_per_sec: calculate_per_sec(operations, total_time_ms),
        op_batch_size: batch_size,
        mean_us: hist.mean() as u64,
        std_dev_us: hist.stdev() as u64,
        p50_us: hist.value_at_quantile(0.50),
        p99_us: hist.value_at_quantile(0.99),
        p999_us: hist.value_at_quantile(0.999),
        max_us: hist.max(),
        bytes_per_sec: calculate_per_sec(total_bytes, total_time_ms),
    }
}

fn calculate_per_sec(count: u64, total_time_ms: u64) -> u64 {
    (count * 1_000).checked_div(total_time_ms).unwrap_or(0)
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

fn format_bytes(n: u64) -> String {
    if n == 0 {
        return "--".to_string();
    }

    let mut n = n as f64;
    for unit in &["B", "KB", "MB", "GB", "TB", "PB"] {
        if n.abs() < 1000.0 {
            return format!("{:.2} {}", n, unit);
        }
        n /= 1000.0;
    }
    format!("{:.1} EB", n)
}

fn print_table(all_stats: &[Stats]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            "op",
            "ops/sec",
            "mean",
            "±",
            "p50",
            "p99",
            "p99.9",
            "max",
            "bytes/sec",
            "entities/sec",
        ]);
    for s in all_stats {
        table.add_row(vec![
            s.op.clone(),
            format!("{}", s.ops_per_sec),
            fmt_us(s.mean_us),
            fmt_us(s.std_dev_us),
            fmt_us(s.p50_us),
            fmt_us(s.p99_us),
            fmt_us(s.p999_us),
            fmt_us(s.max_us),
            format_bytes(s.bytes_per_sec),
            format!("{}", s.ops_per_sec * (s.op_batch_size as u64)),
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

struct BenchConfig {
    client: Arc<CoyoteClient>,
    concurrency: u64,
    iterations: u64,
}

#[derive(Clone)]
struct BenchResult {
    hist: BenchHistogram,
    total_time: Duration,
    total_bytes: u64,
    batch_size: u16,
}

impl BenchResult {
    fn new() -> Self {
        Self {
            hist: BenchHistogram::new(3).expect("can never fail"),
            total_time: Duration::from_secs(0),
            total_bytes: 0,
            batch_size: 1,
        }
    }

    fn set_batch_size(mut self, batch_size: u16) -> Self {
        self.batch_size = batch_size;
        self
    }

    fn process(&mut self, t: Duration, bytes: u64) -> Result<()> {
        self.hist.record(t.as_micros() as u64)?;
        self.total_time += t;
        self.total_bytes += bytes;
        Ok(())
    }

    fn finalize_result_stats(
        cfg: Arc<BenchConfig>,
        results: impl IntoIterator<Item = Self>,
        op: impl Into<String>,
        all_stats: &mut Vec<Stats>,
    ) -> Result<()> {
        let mut combined = BenchHistogram::new(3).unwrap();
        let mut total_time_ms = 0;
        let mut total_bytes = 0;
        let mut batch_size = 0;
        for res in results {
            combined.add(&res.hist)?;
            total_time_ms += res.total_time.as_millis() as u64;
            total_bytes += res.total_bytes;
            batch_size = res.batch_size;
        }
        // Get the average time per run
        total_time_ms /= cfg.concurrency;

        all_stats.push(hist_compute_stats(
            op,
            combined,
            total_time_ms,
            cfg.iterations * cfg.concurrency,
            total_bytes,
            batch_size,
        ));

        Ok(())
    }
}

trait BenchShard {
    fn test_name(&self) -> String;

    async fn run(
        &mut self,
        client: &CoyoteClient,
        rng: &mut StdRng,
        shard_id: u64,
        iteration: u64,
    ) -> Result<()>;

    fn finalize_result_stats(
        &self,
        cfg: Arc<BenchConfig>,
        results: impl IntoIterator<Item = Self>,
        all_stats: &mut Vec<Stats>,
    ) -> Result<()>;

    async fn bench_shard(
        mut self,
        cfg: Arc<BenchConfig>,
        barrier: Arc<Barrier>,
        pb: ProgressBar,
        shard_id: u64,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let mut rng = StdRng::seed_from_u64(0);

        barrier.wait().await;

        for iteration in 0..cfg.iterations {
            self.run(&cfg.client, &mut rng, shard_id, iteration).await?;
            pb.set_position(iteration);
        }
        Ok(self)
    }

    async fn bench_shards_concurrent(
        &self,
        cfg: Arc<BenchConfig>,
        all_stats: &mut Vec<Stats>,
        filter: Option<&Pattern>,
    ) -> Result<()>
    where
        Self: Sized + Clone,
    {
        if let Some(pat) = filter
            && !pat.matches(&self.test_name())
        {
            return Ok(());
        }

        // Sleep for 1 second before tests to give fjall time to catch up.
        tokio::time::sleep(Duration::from_secs(1)).await;

        let concurrency = cfg.concurrency;
        let iterations = cfg.iterations;
        let test_name = self.test_name();
        let pb = new_bar(test_name.to_string(), iterations);
        let barrier = Arc::new(Barrier::new(concurrency as usize));
        let handles = (0..concurrency).map(|shard_id| {
            let pb = pb.clone();
            self.clone()
                .bench_shard(Arc::clone(&cfg), Arc::clone(&barrier), pb, shard_id)
        });

        let joined_handles = try_join_all(handles).await?.into_iter();
        pb.finish();
        self.finalize_result_stats(Arc::clone(&cfg), joined_handles, all_stats)?;

        Ok(())
    }
}

fn bench_generate_key(shard_id: u64, iteration: u64) -> String {
    // Assumes shard_id is no larger than u16::MAX
    let mut rng = StdRng::seed_from_u64(iteration << (16 + shard_id));
    Alphanumeric.sample_string(&mut rng, 16)
}

// KV module

#[derive(Clone)]
struct BenchKvSet {
    bench_result: BenchResult,
}

impl BenchKvSet {
    fn new() -> Self {
        Self {
            bench_result: BenchResult::new(),
        }
    }
}

impl BenchShard for BenchKvSet {
    fn test_name(&self) -> String {
        "kv.set".to_owned()
    }

    async fn run(
        &mut self,
        client: &CoyoteClient,
        rng: &mut StdRng,
        shard_id: u64,
        iteration: u64,
    ) -> Result<()> {
        let key = bench_generate_key(shard_id, iteration);
        let mut value = vec![0u8; 2054];
        rng.fill(&mut value[..]);

        // Start of real code
        let bytes = value.len() as u64;
        let t = Instant::now();
        client.kv().set(KvSetIn::new(key.clone(), value)).await?;
        self.bench_result.process(t.elapsed(), bytes)?;
        Ok(())
    }

    fn finalize_result_stats(
        &self,
        cfg: Arc<BenchConfig>,
        results: impl IntoIterator<Item = Self>,
        all_stats: &mut Vec<Stats>,
    ) -> Result<()> {
        BenchResult::finalize_result_stats(
            cfg,
            results.into_iter().map(|x| x.bench_result),
            self.test_name(),
            all_stats,
        )
    }
}

#[derive(Clone)]
struct BenchKvGet {
    bench_result: BenchResult,
}

impl BenchKvGet {
    fn new() -> Self {
        Self {
            bench_result: BenchResult::new(),
        }
    }
}

impl BenchShard for BenchKvGet {
    fn test_name(&self) -> String {
        "kv.get".to_owned()
    }

    async fn run(
        &mut self,
        client: &CoyoteClient,
        _rng: &mut StdRng,
        shard_id: u64,
        iteration: u64,
    ) -> Result<()> {
        let key = bench_generate_key(shard_id, iteration);

        // Start of real code
        let t = Instant::now();
        let ret = client.kv().get(KvGetIn::new(key.clone())).await?;
        let bytes = ret.value.len() as u64;
        self.bench_result.process(t.elapsed(), bytes)?;
        Ok(())
    }

    fn finalize_result_stats(
        &self,
        cfg: Arc<BenchConfig>,
        results: impl IntoIterator<Item = Self>,
        all_stats: &mut Vec<Stats>,
    ) -> Result<()> {
        BenchResult::finalize_result_stats(
            cfg,
            results.into_iter().map(|x| x.bench_result),
            self.test_name(),
            all_stats,
        )
    }
}

async fn bench_kv(
    cfg: Arc<BenchConfig>,
    all_stats: &mut Vec<Stats>,
    filter: Option<&Pattern>,
) -> Result<()> {
    BenchKvSet::new()
        .bench_shards_concurrent(Arc::clone(&cfg), all_stats, filter)
        .await?;
    BenchKvGet::new()
        .bench_shards_concurrent(Arc::clone(&cfg), all_stats, filter)
        .await?;
    Ok(())
}

// Cache module

#[derive(Clone)]
struct BenchCacheSet {
    bench_result: BenchResult,
}

impl BenchCacheSet {
    fn new() -> Self {
        Self {
            bench_result: BenchResult::new(),
        }
    }
}

impl BenchShard for BenchCacheSet {
    fn test_name(&self) -> String {
        "cache.set".to_owned()
    }

    async fn run(
        &mut self,
        client: &CoyoteClient,
        rng: &mut StdRng,
        shard_id: u64,
        iteration: u64,
    ) -> Result<()> {
        let ttl_bench_ms = 300_000; // 5 minutes
        let key = bench_generate_key(shard_id, iteration);
        let mut value = vec![0u8; 2562];
        rng.fill(&mut value[..]);

        // Start of real code
        let bytes = value.len() as u64;
        let t = Instant::now();
        client
            .cache()
            .set(key, CacheSetIn::new(value, ttl_bench_ms))
            .await?;
        self.bench_result.process(t.elapsed(), bytes)?;
        Ok(())
    }

    fn finalize_result_stats(
        &self,
        cfg: Arc<BenchConfig>,
        results: impl IntoIterator<Item = Self>,
        all_stats: &mut Vec<Stats>,
    ) -> Result<()> {
        BenchResult::finalize_result_stats(
            cfg,
            results.into_iter().map(|x| x.bench_result),
            self.test_name(),
            all_stats,
        )
    }
}

#[derive(Clone)]
struct BenchCacheGet {
    bench_result: BenchResult,
}

impl BenchCacheGet {
    fn new() -> Self {
        Self {
            bench_result: BenchResult::new(),
        }
    }
}

impl BenchShard for BenchCacheGet {
    fn test_name(&self) -> String {
        "cache.get".to_owned()
    }

    async fn run(
        &mut self,
        client: &CoyoteClient,
        _rng: &mut StdRng,
        shard_id: u64,
        iteration: u64,
    ) -> Result<()> {
        let key = bench_generate_key(shard_id, iteration);

        // Start of real code
        let t = Instant::now();
        let ret = client.cache().get(key.clone(), CacheGetIn::new()).await?;
        let bytes = ret.value.len() as u64;
        self.bench_result.process(t.elapsed(), bytes)?;
        Ok(())
    }

    fn finalize_result_stats(
        &self,
        cfg: Arc<BenchConfig>,
        results: impl IntoIterator<Item = Self>,
        all_stats: &mut Vec<Stats>,
    ) -> Result<()> {
        BenchResult::finalize_result_stats(
            cfg,
            results.into_iter().map(|x| x.bench_result),
            self.test_name(),
            all_stats,
        )
    }
}

async fn bench_cache(
    cfg: Arc<BenchConfig>,
    all_stats: &mut Vec<Stats>,
    filter: Option<&Pattern>,
) -> Result<()> {
    BenchCacheSet::new()
        .bench_shards_concurrent(Arc::clone(&cfg), all_stats, filter)
        .await?;
    BenchCacheGet::new()
        .bench_shards_concurrent(Arc::clone(&cfg), all_stats, filter)
        .await?;
    Ok(())
}

// Msgs module

#[derive(Clone)]
struct BenchMsgsPublish {
    bench_result: BenchResult,
    batch_size: u16,
}

impl BenchMsgsPublish {
    fn new(batch_size: u16) -> Self {
        Self {
            bench_result: BenchResult::new().set_batch_size(batch_size),
            batch_size,
        }
    }
}

impl BenchShard for BenchMsgsPublish {
    fn test_name(&self) -> String {
        format!("msgs.publish (batch={})", self.batch_size)
    }

    async fn run(
        &mut self,
        client: &CoyoteClient,
        rng: &mut StdRng,
        shard_id: u64,
        _iteration: u64,
    ) -> Result<()> {
        let topic = format!("bench:bench/topic/{shard_id}");
        let msgs: Vec<_> = (0..self.batch_size)
            .map(|_| {
                let mut payload = vec![0u8; 2_834];
                rng.fill(&mut payload[..]);
                MsgIn::new(payload)
            })
            .collect();

        // Start of real code
        let bytes = msgs.iter().fold(0, |acc, e| acc + e.value.len()) as u64;
        let t = Instant::now();
        client
            .msgs()
            .publish(MsgPublishIn::new(topic.clone(), msgs))
            .await?;
        self.bench_result.process(t.elapsed(), bytes)?;
        Ok(())
    }

    fn finalize_result_stats(
        &self,
        cfg: Arc<BenchConfig>,
        results: impl IntoIterator<Item = Self>,
        all_stats: &mut Vec<Stats>,
    ) -> Result<()> {
        BenchResult::finalize_result_stats(
            cfg,
            results.into_iter().map(|x| x.bench_result),
            self.test_name(),
            all_stats,
        )
    }
}

#[derive(Clone)]
struct BenchMsgsStreamReceive {
    bench_result_rcv: BenchResult,
    bench_result_commit: BenchResult,
    batch_size: u16,
}

impl BenchMsgsStreamReceive {
    fn new(batch_size: u16) -> Self {
        Self {
            bench_result_rcv: BenchResult::new().set_batch_size(batch_size),
            bench_result_commit: BenchResult::new().set_batch_size(batch_size),
            batch_size,
        }
    }
}

impl BenchShard for BenchMsgsStreamReceive {
    fn test_name(&self) -> String {
        format!("msgs.receive (batch={})", self.batch_size)
    }

    async fn run(
        &mut self,
        client: &CoyoteClient,
        rng: &mut StdRng,
        shard_id: u64,
        _iteration: u64,
    ) -> Result<()> {
        let consumer_group = "consumer";
        let topic = format!("bench:bench/topic/{shard_id}");
        let mut value = vec![0u8; 256];
        rng.fill(&mut value[..]);

        // Start of real code
        let t = Instant::now();
        let mut recv = MsgStreamReceiveIn::new(consumer_group.to_owned(), topic.clone());
        recv.batch_size = Some(self.batch_size);
        let out = client.msgs().stream().receive(recv).await?;
        let rcv_bytes = out.msgs.iter().fold(0, |acc, e| acc + e.value.len()) as u64;
        self.bench_result_rcv.process(t.elapsed(), rcv_bytes)?;

        let latest_by_topic = out.msgs.into_iter().fold(HashMap::new(), |mut map, msg| {
            map.entry(msg.topic)
                .and_modify(|offset: &mut u64| *offset = (*offset).max(msg.offset))
                .or_insert(msg.offset);
            map
        });

        let t = Instant::now();
        for (topic, offset) in latest_by_topic {
            client
                .msgs()
                .stream()
                .commit(MsgStreamCommitIn::new(
                    consumer_group.to_owned(),
                    topic,
                    offset,
                ))
                .await?;
        }
        self.bench_result_commit.process(t.elapsed(), 0)?;

        Ok(())
    }

    fn finalize_result_stats(
        &self,
        cfg: Arc<BenchConfig>,
        results: impl IntoIterator<Item = Self>,
        all_stats: &mut Vec<Stats>,
    ) -> Result<()> {
        let (rcv_results, commit_results): (Vec<_>, Vec<_>) = results
            .into_iter()
            .map(|x| (x.bench_result_rcv, x.bench_result_commit))
            .unzip();
        BenchResult::finalize_result_stats(
            Arc::clone(&cfg),
            rcv_results,
            format!("{} - receive", self.test_name()),
            all_stats,
        )?;
        BenchResult::finalize_result_stats(
            Arc::clone(&cfg),
            commit_results,
            format!("{} - commit", self.test_name()),
            all_stats,
        )?;
        Ok(())
    }
}

async fn bench_msgs(
    cfg: Arc<BenchConfig>,
    all_stats: &mut Vec<Stats>,
    filter: Option<&Pattern>,
) -> Result<()> {
    let ns_name = "bench";

    cfg.client
        .msgs()
        .namespace()
        .create(MsgNamespaceCreateIn::new(ns_name.to_string()))
        .await?;

    BenchMsgsPublish::new(1)
        .bench_shards_concurrent(Arc::clone(&cfg), all_stats, filter)
        .await?;
    BenchMsgsStreamReceive::new(1)
        .bench_shards_concurrent(Arc::clone(&cfg), all_stats, filter)
        .await?;

    BenchMsgsPublish::new(10)
        .bench_shards_concurrent(Arc::clone(&cfg), all_stats, filter)
        .await?;
    BenchMsgsStreamReceive::new(10)
        .bench_shards_concurrent(Arc::clone(&cfg), all_stats, filter)
        .await?;
    Ok(())
}
