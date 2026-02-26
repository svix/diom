use coyote_benchmarks::{BenchmarkContext, setup_cluster, setup_single_server};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::Measurement,
};
use rand::{
    SeedableRng,
    distr::{Alphanumeric, SampleString},
    rngs::StdRng,
};
use reqwest::StatusCode;
use serde_json::json;

fn bench_rate_limiter<'a, M: Measurement>(
    ctx: BenchmarkContext,
    group: &mut BenchmarkGroup<'a, M>,
) {
    let client = ctx.test_client;

    group.sample_size(60);

    let mut rng = StdRng::seed_from_u64(0);
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    group.bench_function("token_bucket_large_capacity_large_refill", |b| {
        b.iter_batched(
            || Alphanumeric.sample_string(&mut rng, 16),
            |key| {
                rt.block_on(async {
                    std::hint::black_box(client.post("rate-limiter/limit").json(json!({
                        "key": key,
                        "units": 1,
                        "method": "token_bucket",
                        "config": {
                            "capacity": 1_000_000,
                            "refill_amount": 500_00,
                            "refill_interval_seconds": 1
                        }
                    })))
                    .await
                    .unwrap()
                    .expect(StatusCode::OK);
                })
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("token_bucket_large_capacity_small_refill", |b| {
        b.iter_batched(
            || Alphanumeric.sample_string(&mut rng, 16),
            |key| {
                rt.block_on(async {
                    std::hint::black_box(client.post("rate-limiter/limit").json(json!({
                        "key": key,
                        "units": 1,
                        "method": "token_bucket",
                        "config": {
                            "capacity": 1_000_000,
                            "refill_amount": 1,
                            "refill_interval_seconds": 1
                        }
                    })))
                    .await
                    .unwrap()
                    .expect(StatusCode::OK);
                })
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("token_bucket_small_capacity", |b| {
        b.iter_batched(
            || Alphanumeric.sample_string(&mut rng, 16),
            |key| {
                rt.block_on(async {
                    std::hint::black_box(client.post("rate-limiter/limit").json(json!({
                        "key": key,
                        "units": 1,
                        "method": "token_bucket",
                        "config": {
                            "capacity": 5,
                            "refill_amount": 1,
                            "refill_interval_seconds": 1
                        }
                    })))
                    .await
                    .unwrap()
                    .expect(StatusCode::OK);
                })
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("fixed_window_large_capacity", |b| {
        b.iter_batched(
            || Alphanumeric.sample_string(&mut rng, 16),
            |key| {
                rt.block_on(async {
                    std::hint::black_box(client.post("rate-limiter/limit").json(json!({
                        "key": key,
                        "units": 1,
                        "method": "fixed_window",
                        "config": {
                            "max_requests": 1_000_000,
                            "window_size": 1
                        }
                    })))
                    .await
                    .unwrap()
                    .expect(StatusCode::OK);
                })
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("fixed_window_small_capacity", |b| {
        b.iter_batched(
            || Alphanumeric.sample_string(&mut rng, 16),
            |key| {
                rt.block_on(async {
                    std::hint::black_box(client.post("rate-limiter/limit").json(json!({
                        "key": key,
                        "units": 1,
                        "method": "fixed_window",
                        "config": {
                            "max_requests": 10,
                            "window_size": 1
                        }
                    })))
                    .await
                    .unwrap()
                    .expect(StatusCode::OK);
                })
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_rate_limiter_single(c: &mut Criterion) {
    let ctx = setup_single_server();
    let mut group = c.benchmark_group("rate_limiter_single_server");
    bench_rate_limiter(ctx, &mut group);
}

fn bench_rate_limiter_cluster(c: &mut Criterion) {
    let ctx = setup_cluster(3);
    let mut group = c.benchmark_group("rate_limiter_cluster");
    bench_rate_limiter(ctx, &mut group);
}

criterion_group!(
    rate_limiter,
    bench_rate_limiter_single,
    bench_rate_limiter_cluster
);
criterion_main!(rate_limiter);
