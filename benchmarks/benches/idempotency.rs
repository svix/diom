use criterion::{
    BatchSize, BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::Measurement,
};
use diom_benchmarks::{BenchmarkContext, setup_cluster, setup_single_server};
use rand::{
    Rng, SeedableRng,
    distr::{Alphanumeric, SampleString},
    rngs::StdRng,
};
use reqwest::StatusCode;
use serde_json::json;

fn bench_idempotency<'a, M: Measurement>(ctx: BenchmarkContext, group: &mut BenchmarkGroup<'a, M>) {
    let client = ctx.test_client;

    group.sample_size(60);
    group.measurement_time(std::time::Duration::from_secs(10));

    let mut rng = StdRng::seed_from_u64(0);
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    group.bench_function("idempotency_start_abort", |b| {
        b.iter_batched(
            || Alphanumeric.sample_string(&mut rng, 16),
            |key| {
                rt.block_on(async {
                    client
                        .post("v1.idempotency.start")
                        .json(json!({
                            "key": &key,
                            "lock_period_ms": 60
                        }))
                        .await
                        .unwrap()
                        .expect(StatusCode::OK);

                    client
                        .post("v1.idempotency.abort")
                        .json(json!({ "key": &key }))
                        .await
                        .unwrap()
                        .expect(StatusCode::OK);
                })
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("idempotency_start_complete", |b| {
        b.iter_batched(
            || Alphanumeric.sample_string(&mut rng, 16),
            |key| {
                rt.block_on(async {
                    client
                        .post("v1.idempotency.start")
                        .json(json!({
                            "key": &key,
                            "lock_period_ms": 60
                        }))
                        .await
                        .unwrap()
                        .expect(StatusCode::OK);

                    client
                        .post("v1.idempotency.complete")
                        .json(json!({
                            "key": &key,
                            "response": "ok".as_bytes(),
                            "ttl_ms": 60
                        }))
                        .await
                        .unwrap()
                        .expect(StatusCode::OK);
                })
            },
            BatchSize::SmallInput,
        )
    });

    let initialize_idempotency = |key: &str, payload: &[u8]| {
        rt.block_on(async {
            client
                .post("v1.idempotency.start")
                .json(json!({
                    "key": key,
                    "lock_period_ms": 60
                }))
                .await
                .unwrap();
        });

        rt.block_on(async {
            client
                .post("v1.idempotency.complete")
                .json(json!({
                    "key": key,
                    "response": payload,
                    "ttk_ms": 60
                }))
                .await
                .unwrap();
        });
    };

    let test_key = Alphanumeric.sample_string(&mut rng, 16);
    let mut payload = vec![0u8; 256];
    rng.fill(&mut payload[..]);

    initialize_idempotency(&test_key, &payload);

    group.bench_function("idempotency_small_payload", |b| {
        b.iter(|| {
            rt.block_on(async {
                client
                    .post("v1.idempotency.start")
                    .json(json!({
                        "key": &test_key,
                        "lock_period_ms": 60
                    }))
                    .await
                    .unwrap()
                    .expect(StatusCode::OK);
            })
        })
    });

    let test_key = Alphanumeric.sample_string(&mut rng, 16);
    let mut payload = vec![0u8; 1024];
    rng.fill(&mut payload[..]);
    initialize_idempotency(&test_key, &payload);

    group.bench_function("idempotency_medium_payload", |b| {
        b.iter(|| {
            rt.block_on(async {
                client
                    .post("v1.idempotency.start")
                    .json(json!({
                        "key": &test_key,
                        "lock_period_ms": 60
                    }))
                    .await
                    .unwrap()
                    .expect(StatusCode::OK);
            })
        })
    });

    let test_key = Alphanumeric.sample_string(&mut rng, 16);
    let mut payload = vec![0u8; 4096];
    rng.fill(&mut payload[..]);
    initialize_idempotency(&test_key, &payload);

    group.bench_function("idempotency_large_payload", |b| {
        b.iter(|| {
            rt.block_on(async {
                client
                    .post("v1.idempotency.start")
                    .json(json!({
                        "key": &test_key,
                        "lock_period_ms": 60
                    }))
                    .await
                    .unwrap()
                    .expect(StatusCode::OK);
            })
        })
    });
}

fn bench_idempotency_single(c: &mut Criterion) {
    let ctx = setup_single_server();
    let mut group = c.benchmark_group("idempotency_single_server");
    bench_idempotency(ctx, &mut group);
}

fn bench_idempotency_cluster(c: &mut Criterion) {
    let ctx = setup_cluster(3);
    let mut group = c.benchmark_group("idempotency_cluster");
    bench_idempotency(ctx, &mut group);
}

criterion_group!(
    idempotency,
    bench_idempotency_single,
    bench_idempotency_cluster
);
criterion_main!(idempotency);
