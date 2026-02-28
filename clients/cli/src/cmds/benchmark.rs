use std::{sync::Arc, time::Duration};

use anyhow::Result;
use clap::Args;
use comfy_table::{Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL_CONDENSED};
use coyote_client::{
    CoyoteClient,
    models::{KvGetIn, KvSetIn},
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

// ── Statistics ────────────────────────────────────────────────────────────────

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

fn new_bar(prefix: impl Into<String>, iterations: u64) -> ProgressBar {
    let pb = ProgressBar::new(iterations);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {prefix:.bold} [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_prefix(prefix.into());
    pb
}

// ── Entry point ───────────────────────────────────────────────────────────────

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
                    let module = TomModuleKv::setup(concurrency, iterations);
                    module.bench(Arc::clone(&client), &mut all_stats).await?;
                }
                BenchmarkModule::Cache => {
                    eprintln!("[cache]");
                }
                BenchmarkModule::Msgs => {
                    eprintln!("[msgs]");
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

// ── kv ────────────────────────────────────────────────────────────────────────

#[derive(Clone)]
struct TomBenchKvSet {
    keys: Arc<Vec<String>>,
    // FIXME: remove from here.
    iterations: u64,
}

impl TomBenchKvSet {
    fn name(&self) -> &'static str {
        "kv.set"
    }

    fn setup(rng: &mut StdRng, iterations: u64) -> Self {
        Self {
            keys: Arc::new(
                (0..iterations)
                    .map(|_| Alphanumeric.sample_string(rng, 16))
                    .collect(),
            ),
            iterations,
        }
    }

    async fn run(&self, client: &CoyoteClient, rng: &mut StdRng, i: u64) -> Result<Duration> {
        let key = self.keys.get(i as usize).unwrap();
        let mut value = vec![0u8; 256];
        rng.fill(&mut value[..]);

        // Start of real code
        let t = quanta::Instant::now();
        client.kv().set(KvSetIn::new(key.clone(), value)).await?;
        Ok(t.elapsed())
    }

    /// Runs the benchmark of one shard (shard is the unit of concurrency)
    async fn bench_shard(
        self,
        client: Arc<CoyoteClient>,
        shard_id: u64,
        pb: ProgressBar,
    ) -> Result<BenchResult> {
        let mut hist = BenchHistogram::new(3)?;
        let mut total_time = Duration::from_secs(0);
        let mut rng = StdRng::seed_from_u64(shard_id);

        for i in 0..self.iterations {
            let t = self.run(&client, &mut rng, i).await?;
            hist.record(t.as_micros() as u64)?;
            total_time += t;
            pb.set_position(i);
        }
        Ok(BenchResult { hist, total_time })
    }
}

struct BenchResult {
    hist: BenchHistogram,
    total_time: Duration,
}

struct TomModuleKv {
    concurrency: u64,
    iterations: u64,
}

impl TomModuleKv {
    fn setup(concurrency: u64, iterations: u64) -> Self {
        Self {
            concurrency,
            iterations,
        }
    }

    /// Runs the full benchmark for the module
    async fn bench(&self, client: Arc<CoyoteClient>, all_stats: &mut Vec<Stats>) -> Result<()> {
        let kv_set = ToBeDeleted::setup(self.concurrency, self.iterations);
        kv_set.bench(Arc::clone(&client), all_stats).await?;
        let kv_get = ToBeDeleted::setup(self.concurrency, self.iterations);
        kv_get.bench(Arc::clone(&client), all_stats).await?;
        Ok(())
    }
}

struct ToBeDeleted {
    instances: Arc<Vec<TomBenchKvSet>>,
    concurrency: u64,
    iterations: u64,
}

impl ToBeDeleted {
    fn name(&self) -> &'static str {
        "kv.set"
    }

    fn setup(concurrency: u64, iterations: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(0);
        Self {
            instances: Arc::new(
                (0..concurrency)
                    .map(|_| TomBenchKvSet::setup(&mut rng, iterations))
                    .collect(),
            ),
            concurrency,
            iterations,
        }
    }

    fn get_test(&self, index: u64) -> TomBenchKvSet {
        self.instances.get(index as usize).unwrap().clone()
    }

    /// Runs the full benchmark for the module
    async fn bench(&self, client: Arc<CoyoteClient>, all_stats: &mut Vec<Stats>) -> Result<()> {
        let iterations = self.iterations;
        let concurrency = self.concurrency;

        let pb = new_bar(self.name().to_string(), iterations);
        let handles = (0..concurrency).map(|shard_id| {
            let client = Arc::clone(&client);
            let pb = pb.clone();
            let test = self.get_test(shard_id);
            let keys = test.keys.clone();
            test.bench_shard(client, shard_id, pb)
        });
        pb.finish();

        let mut combined = BenchHistogram::new(3).unwrap();
        let mut total_time_ms = 0;
        let joined_handles = try_join_all(handles).await?.into_iter();
        for res in joined_handles {
            combined.add(res.hist)?;
            total_time_ms += res.total_time.as_millis() as u64;
        }
        // Get the average time per run
        total_time_ms = total_time_ms / concurrency;

        all_stats.push(hist_compute_stats(
            self.name(),
            combined,
            total_time_ms,
            iterations * concurrency,
        ));

        Ok(())
    }
}



