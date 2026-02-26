use diom_benchmarks::{BenchmarkContext, setup_cluster, setup_single_server};
use diom_client::models::{CacheDeleteIn, CacheGetIn, CacheSetIn};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::Measurement,
};
use rand::{
    Rng, SeedableRng,
    distr::{Alphanumeric, SampleString},
    rngs::StdRng,
};

fn bench_cache<'a, M: Measurement>(ctx: BenchmarkContext, group: &mut BenchmarkGroup<'a, M>) {
    let client = ctx.client;

    group.sample_size(60);

    let mut rng = StdRng::seed_from_u64(0);
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let test_key = "test-key".to_string();
    let mut test_val = vec![0u8; 256];
    rng.fill(&mut test_val[..]);

    group.bench_function("cache_set_fixed_high_ttl", |b| {
        b.iter_batched(
            || (test_key.clone(), test_val.clone()),
            |(key, val)| {
                rt.block_on(async {
                    std::hint::black_box(client.cache().set(CacheSetIn::new(key, 60_000, val)))
                        .await
                        .unwrap();
                })
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("cache_set_fixed_low_ttl", |b| {
        b.iter_batched(
            || (test_key.clone(), test_val.clone()),
            |(key, val)| {
                rt.block_on(async {
                    std::hint::black_box(client.cache().set(CacheSetIn::new(key, 1, val)))
                        .await
                        .unwrap();
                })
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("cache_set_random_small", |b| {
        b.iter_batched(
            || {
                let k = Alphanumeric.sample_string(&mut rng, 16);
                let mut v = vec![0u8; 256];
                rng.fill(&mut v[..]);
                (k, v)
            },
            |(k, v)| {
                rt.block_on(async {
                    std::hint::black_box(client.cache().set(CacheSetIn::new(k, 60_000, v)))
                        .await
                        .unwrap();
                })
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("cache_set_random_medium_value", |b| {
        b.iter_batched(
            || {
                let k = Alphanumeric.sample_string(&mut rng, 16);
                let mut v = vec![0u8; 2048];
                rng.fill(&mut v[..]);
                (k, v)
            },
            |(k, v)| {
                rt.block_on(async {
                    std::hint::black_box(client.cache().set(CacheSetIn::new(k, 60_000, v)))
                        .await
                        .unwrap();
                })
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("cache_set_random_large_value", |b| {
        b.iter_batched(
            || {
                let k = Alphanumeric.sample_string(&mut rng, 16);
                let mut v = vec![0u8; 4096];
                rng.fill(&mut v[..]);
                (k, v)
            },
            |(k, v)| {
                rt.block_on(async {
                    std::hint::black_box(client.cache().set(CacheSetIn::new(k, 60_000, v)))
                        .await
                        .unwrap();
                })
            },
            criterion::BatchSize::SmallInput,
        )
    });

    // Make sure we have a key to test repeated gets
    rt.block_on(async {
        std::hint::black_box(client.cache().set(CacheSetIn::new(
            test_key.clone(),
            60_000,
            test_val.clone(),
        )))
        .await
        .unwrap();
    });

    group.bench_function("cache_get", |b| {
        b.iter_batched(
            || test_key.clone(),
            |key| {
                rt.block_on(async {
                    std::hint::black_box(client.cache().get(CacheGetIn::new(key)))
                        .await
                        .unwrap();
                })
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("cache_set_delete", |b| {
        b.iter_batched(
            || (test_key.clone(), test_key.clone(), test_val.clone()),
            |(key1, key2, val)| {
                rt.block_on(async {
                    std::hint::black_box(client.cache().set(CacheSetIn::new(key1, 60_000, val)))
                        .await
                        .unwrap();
                    std::hint::black_box(client.cache().delete(CacheDeleteIn::new(key2)))
                        .await
                        .unwrap();
                })
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_cache_single(c: &mut Criterion) {
    let ctx = setup_single_server();
    let mut group = c.benchmark_group("cache_single_server");
    bench_cache(ctx, &mut group);
}

fn bench_cache_cluster(c: &mut Criterion) {
    let ctx = setup_cluster(3);
    let mut group = c.benchmark_group("cache_cluster");
    bench_cache(ctx, &mut group);
}

criterion_group!(cache, bench_cache_single, bench_cache_cluster);
criterion_main!(cache);
