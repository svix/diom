use coyote_benchmarks::{BenchmarkContext, setup_cluster, setup_single_server};
use coyote_client::models::{KvDeleteIn, KvGetIn, KvSetIn};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::Measurement,
};
use rand::{
    Rng, SeedableRng,
    distr::{Alphanumeric, SampleString},
    rngs::StdRng,
};

fn bench_kv<'a, M: Measurement>(ctx: BenchmarkContext, group: &mut BenchmarkGroup<'a, M>) {
    let client = ctx.client;

    group.measurement_time(std::time::Duration::from_secs(15));

    let mut rng = StdRng::seed_from_u64(0);
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let test_key = "test-key".to_string();
    let mut test_val = vec![0u8; 256];
    rng.fill(&mut test_val[..]);

    group.bench_function("kv_set_fixed", |b| {
        b.iter_batched(
            || (test_key.clone(), test_val.clone()),
            |(key, val)| {
                rt.block_on(async {
                    std::hint::black_box(client.kv().set(KvSetIn::new(key, val), None))
                        .await
                        .unwrap();
                })
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("kv_set_random_small", |b| {
        b.iter_batched(
            || {
                let k = Alphanumeric.sample_string(&mut rng, 16);
                let mut v = vec![0u8; 256];
                rng.fill(&mut v[..]);
                (k, v)
            },
            |(k, v)| {
                rt.block_on(async {
                    std::hint::black_box(client.kv().set(KvSetIn::new(k, v), None))
                        .await
                        .unwrap();
                })
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("kv_set_random_medium_value", |b| {
        b.iter_batched(
            || {
                let k = Alphanumeric.sample_string(&mut rng, 16);
                let mut v = vec![0u8; 2048];
                rng.fill(&mut v[..]);
                (k, v)
            },
            |(k, v)| {
                rt.block_on(async {
                    std::hint::black_box(client.kv().set(KvSetIn::new(k, v), None))
                        .await
                        .unwrap();
                })
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("kv_set_random_large_value", |b| {
        b.iter_batched(
            || {
                let k = Alphanumeric.sample_string(&mut rng, 16);
                let mut v = vec![0u8; 4096];
                rng.fill(&mut v[..]);
                (k, v)
            },
            |(k, v)| {
                rt.block_on(async {
                    std::hint::black_box(client.kv().set(KvSetIn::new(k, v), None))
                        .await
                        .unwrap();
                })
            },
            criterion::BatchSize::SmallInput,
        )
    });

    // Make sure we have a key to test repeated gets
    rt.block_on(async {
        std::hint::black_box(
            client
                .kv()
                .set(KvSetIn::new(test_key.clone(), test_val.clone()), None),
        )
        .await
        .unwrap();
    });

    group.bench_function("kv_get", |b| {
        b.iter_batched(
            || test_key.clone(),
            |key| {
                rt.block_on(async {
                    std::hint::black_box(client.kv().get(KvGetIn::new(key), None))
                        .await
                        .unwrap();
                })
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("kv_set_delete", |b| {
        b.iter_batched(
            || (test_key.clone(), test_key.clone(), test_val.clone()),
            |(key1, key2, test_val)| {
                rt.block_on(async {
                    std::hint::black_box(client.kv().set(KvSetIn::new(key1, test_val), None))
                        .await
                        .unwrap();
                    std::hint::black_box(client.kv().delete(KvDeleteIn::new(key2), None))
                        .await
                        .unwrap();
                })
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_kv_single(c: &mut Criterion) {
    let ctx = setup_single_server();
    let mut group = c.benchmark_group("kv_single_server");
    bench_kv(ctx, &mut group);
}

fn bench_kv_cluster(c: &mut Criterion) {
    let ctx = setup_cluster(3);
    let mut group = c.benchmark_group("kv_cluster");
    bench_kv(ctx, &mut group);
}

criterion_group!(kv, bench_kv_single, bench_kv_cluster);
criterion_main!(kv);
