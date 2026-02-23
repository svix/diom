use coyote_benchmarks::{BenchmarkContext, setup_cluster, setup_single_server};
use coyote_client::{
    CoyoteClient,
    models::{AckMsgRangeIn, AppendToStreamIn, CreateStreamIn, FetchFromStreamIn, MsgIn},
};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::Measurement,
};
use rand::{Rng, SeedableRng, rngs::StdRng};
use tokio::runtime::Runtime;

fn bench_append<'a, M: Measurement>(
    rt: &Runtime,
    ctx: BenchmarkContext,
    group: &mut BenchmarkGroup<'a, M>,
) {
    let client = ctx.client;

    group.sample_size(60);
    group.measurement_time(std::time::Duration::from_secs(10));

    let mut rng = StdRng::seed_from_u64(0);
    let test_stream = "bench-stream".to_string();

    rt.block_on(async {
        std::hint::black_box(
            client
                .stream()
                .create(CreateStreamIn::new(test_stream.clone()), None),
        )
        .await
        .unwrap();
    });

    group.bench_function("stream_append_single_small", |b| {
        b.iter_batched(
            || {
                let mut payload = vec![0u8; 256];
                rng.fill(&mut payload[..]);
                MsgIn::new(payload)
            },
            |msg| {
                rt.block_on(async {
                    std::hint::black_box(
                        client
                            .stream()
                            .append(AppendToStreamIn::new(vec![msg], test_stream.clone()), None),
                    )
                    .await
                    .unwrap();
                })
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("stream_append_single_medium", |b| {
        b.iter_batched(
            || {
                let mut payload = vec![0u8; 2048];
                rng.fill(&mut payload[..]);
                MsgIn::new(payload)
            },
            |msg| {
                rt.block_on(async {
                    std::hint::black_box(
                        client
                            .stream()
                            .append(AppendToStreamIn::new(vec![msg], test_stream.clone()), None),
                    )
                    .await
                    .unwrap();
                })
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("stream_append_single_large", |b| {
        b.iter_batched(
            || {
                let mut payload = vec![0u8; 4096];
                rng.fill(&mut payload[..]);
                MsgIn::new(payload)
            },
            |msg| {
                rt.block_on(async {
                    std::hint::black_box(
                        client
                            .stream()
                            .append(AppendToStreamIn::new(vec![msg], test_stream.clone()), None),
                    )
                    .await
                    .unwrap();
                })
            },
            BatchSize::SmallInput,
        )
    });
}

fn create_stream(rt: &Runtime, client: CoyoteClient, stream: String) {
    rt.block_on(async {
        client
            .stream()
            .create(CreateStreamIn::new(stream.clone()), None)
            .await
            .unwrap();
    });
}

fn populate_stream(rt: &Runtime, client: CoyoteClient, stream: String, mut n: usize) {
    let mut rng = StdRng::seed_from_u64(0);
    rt.block_on(async {
        while n > 0 {
            let batch_size = if n > 1000 {
                n -= 1000;
                1000
            } else {
                let batch_size = n;
                n = 0;
                batch_size
            };
            let msgs: Vec<MsgIn> = (0..batch_size)
                .map(|_| {
                    let mut payload = vec![0u8; 256];
                    rng.fill(&mut payload[..]);
                    MsgIn::new(payload)
                })
                .collect();
            client
                .stream()
                .append(AppendToStreamIn::new(msgs, stream.clone()), None)
                .await
                .unwrap();
        }
    });
}

fn fetch_ack<'a, M: Measurement>(
    rt: &Runtime,
    ctx: BenchmarkContext,
    group: &mut BenchmarkGroup<'a, M>,
    stream_size: usize,
    batch_size: u16,
    bench_name: &str,
) {
    let client = ctx.client;

    group.sample_size(60);
    group.measurement_time(std::time::Duration::from_secs(10));

    let stream = "bench-stream1".to_string();

    create_stream(rt, client.clone(), stream.clone());
    populate_stream(rt, client.clone(), stream.clone(), stream_size);

    let cg = "bench-cg-1".to_string();

    group.bench_function(bench_name, |b| {
        b.iter_batched(
            || (stream.clone(), stream.clone(), cg.clone(), cg.to_string()),
            |(stream_cloned1, stream_cloned2, cg_cloned1, cg_cloned2)| {
                rt.block_on(async {
                    let msgs = std::hint::black_box(client.stream().fetch(
                        FetchFromStreamIn::new(batch_size, cg_cloned1, stream_cloned1, 30),
                        None,
                    ))
                    .await
                    .unwrap();

                    let first = msgs.msgs.first().unwrap();
                    let last = msgs.msgs.last().unwrap();
                    let mut ack = AckMsgRangeIn::new(cg_cloned2, last.id, stream_cloned2);
                    ack.min_msg_id = Some(first.id);
                    std::hint::black_box(client.stream().ack_range(ack, None))
                        .await
                        .unwrap();
                })
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_stream_single(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let mut group = c.benchmark_group("stream_single_server");

    let ctx = setup_single_server();
    bench_append(&rt, ctx, &mut group);

    let ctx = setup_cluster(3);
    fetch_ack(&rt, ctx, &mut group, 10000, 1, "batch_size_1");

    let ctx = setup_cluster(3);
    fetch_ack(&rt, ctx, &mut group, 100000, 1, "batch_size_100");
}

fn bench_stream_cluster(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let mut group = c.benchmark_group("stream_cluster");
    let ctx = setup_cluster(3);
    bench_append(&rt, ctx, &mut group);

    let ctx = setup_cluster(3);
    fetch_ack(&rt, ctx, &mut group, 10000, 1, "fetch_ack_batch_size_1");

    let ctx = setup_cluster(3);
    fetch_ack(&rt, ctx, &mut group, 100000, 1, "fetch_ack_batch_size_100");
}

criterion_group!(stream, bench_stream_single, bench_stream_cluster);
criterion_main!(stream);
