use diom_client::models::{KvDeleteIn, KvGetIn, KvSetIn};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use rand::{
    Rng,
    distr::{Alphanumeric, SampleString},
};

fn bench_kv(c: &mut Criterion) {
    let ctx = diom_benchmarks::setup_server_simple();
    let client = ctx.client;

    let mut rng = rand::rng();
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let test_key = "test-key".to_string();
    let mut test_val = vec![0u8; 256];
    rng.fill(&mut test_val[..]);

    c.bench_function("kv_set_fixed", |b| {
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

    c.bench_function("kv_set_random_small", |b| {
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

    c.bench_function("kv_set_random_medium_value", |b| {
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

    c.bench_function("kv_set_random_large_value", |b| {
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

    c.bench_function("kv_get", |b| {
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

    c.bench_function("kv_set_delete", |b| {
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

criterion_group!(kv, bench_kv);
criterion_main!(kv);
