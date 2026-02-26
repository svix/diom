use std::time::Instant;

use anyhow::Result;
use clap::Args;
use coyote_client::{
    CoyoteClient,
    models::{
        Ack, AppendToStreamIn, CacheGetIn, CacheSetIn, CreateNamespaceIn, FetchFromStreamIn,
        KvGetIn, KvSetIn, MsgIn,
    },
};
use rand::{
    Rng, SeedableRng,
    distr::{Alphanumeric, SampleString},
    rngs::StdRng,
};

// TODO(238): Idempotency/Rate-limit does not currently work in SDK

#[derive(Clone, Debug, PartialEq, clap::ValueEnum)]
pub enum BenchmarkModule {
    Kv,
    Cache,
    Stream,
}

#[derive(Args)]
pub struct BenchmarkArgs {
    /// Number of operations to perform (default: 500)
    #[arg(short, long, default_value_t = 500)]
    pub count: usize,

    /// Modules to benchmark (default: all)
    #[arg(short, long, value_delimiter = ',')]
    pub modules: Vec<BenchmarkModule>,
}

impl BenchmarkArgs {
    pub async fn exec(self, client: &CoyoteClient) -> Result<()> {
        let count = self.count;

        let modules = if self.modules.is_empty() {
            vec![
                BenchmarkModule::Kv,
                BenchmarkModule::Cache,
                BenchmarkModule::Stream,
            ]
        } else {
            self.modules
        };

        println!("Running benchmark ({count} ops each)...");

        for module in &modules {
            println!();
            match module {
                BenchmarkModule::Kv => bench_kv(client, count).await?,
                BenchmarkModule::Cache => bench_cache(client, count).await?,
                BenchmarkModule::Stream => bench_stream(client, count).await?,
            }
        }

        Ok(())
    }
}

fn print_result(op: &str, count: usize, secs: f64) {
    let ops_per_sec = count as f64 / secs;
    println!("  {op:<8} {ops_per_sec:>8.0} ops/sec  ({secs:.2}s total)",);
}

async fn bench_kv(client: &CoyoteClient, count: usize) -> Result<()> {
    let mut rng = StdRng::seed_from_u64(0);
    let mut keys = vec![];
    println!("[kv]");

    eprint!("  set... ");
    let start = Instant::now();
    for _ in 0..count {
        let key = Alphanumeric.sample_string(&mut rng, 16);
        keys.push(key.clone());
        let mut value = vec![0u8; 256];
        rng.fill(&mut value[..]);
        client.kv().set(KvSetIn::new(key, value)).await?;
    }
    let set_secs = start.elapsed().as_secs_f64();
    eprintln!("done");

    eprint!("  get... ");
    let start = Instant::now();
    for key in keys {
        client.kv().get(KvGetIn::new(key)).await?;
    }
    let get_secs = start.elapsed().as_secs_f64();
    eprintln!("done");

    println!();
    print_result("set", count, set_secs);
    print_result("get", count, get_secs);

    Ok(())
}

async fn bench_cache(client: &CoyoteClient, count: usize) -> Result<()> {
    const TTL_MS: u64 = 60_000;
    let mut rng = StdRng::seed_from_u64(0);
    let mut keys = vec![];
    println!("[cache]");

    eprint!("  set... ");
    let start = Instant::now();
    for _ in 0..count {
        let key = Alphanumeric.sample_string(&mut rng, 16);
        keys.push(key.clone());
        let mut value = vec![0u8; 256];
        rng.fill(&mut value[..]);
        client
            .cache()
            .set(CacheSetIn::new(key, TTL_MS, value.clone()))
            .await?;
    }
    let set_secs = start.elapsed().as_secs_f64();
    eprintln!("done");

    eprint!("  get... ");
    let start = Instant::now();
    for key in keys {
        client.cache().get(CacheGetIn::new(key)).await?;
    }
    let get_secs = start.elapsed().as_secs_f64();
    eprintln!("done");

    println!();
    print_result("set", count, set_secs);
    print_result("get", count, get_secs);

    Ok(())
}

async fn bench_stream(client: &CoyoteClient, count: usize) -> Result<()> {
    const CONSUMER_GROUP: &str = "__bench_consumer";
    let mut rng = StdRng::seed_from_u64(0);
    let stream_name = Alphanumeric.sample_string(&mut rng, 16);
    println!("[stream]");

    client
        .msgs()
        .namespace()
        .create(CreateNamespaceIn::new(stream_name.clone()))
        .await?;

    eprint!("  append... ");
    let start = Instant::now();
    for _ in 0..count {
        let mut payload = vec![0u8; 256];
        rng.fill(&mut payload[..]);
        client
            .stream()
            .append(AppendToStreamIn::new(
                vec![MsgIn::new(payload)],
                stream_name.clone(),
            ))
            .await?;
    }
    let append_secs = start.elapsed().as_secs_f64();
    eprintln!("done");

    eprint!("  fetch/ack...  ");
    let start = Instant::now();
    let mut fetched = 0;
    while fetched < count {
        let out = client
            .stream()
            .fetch(FetchFromStreamIn::new(
                1,
                CONSUMER_GROUP.to_owned(),
                stream_name.clone(),
                30,
            ))
            .await?;
        for msg in out.msgs {
            client
                .stream()
                .ack(Ack::new(
                    CONSUMER_GROUP.to_owned(),
                    msg.id,
                    stream_name.clone(),
                ))
                .await?;
            fetched += 1;
        }
    }
    let fetch_secs = start.elapsed().as_secs_f64();
    eprintln!("done");

    println!();
    print_result("append", count, append_secs);
    print_result("fetch/ack", count, fetch_secs);

    Ok(())
}
