use coyote::models::{
    MsgIn, MsgNamespaceCreateIn, MsgPublishIn, MsgStreamCommitIn, MsgStreamReceiveIn,
};
use coyote_benchmarks::{BenchmarkContext, setup_cluster, setup_single_server};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::Measurement,
};
use rand::{Rng, SeedableRng, rngs::StdRng};

fn bench_stream<'a, M: Measurement>(ctx: BenchmarkContext, group: &mut BenchmarkGroup<'a, M>) {
    let client = ctx.client;

    group.sample_size(60);

    let mut rng = StdRng::seed_from_u64(0);
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let mut make_msg_batch = |size: usize| {
        (0..size)
            .map(|_| {
                let mut val = vec![0u8; 256];
                rng.fill(&mut val[..]);
                MsgIn::new(val)
            })
            .collect::<Vec<_>>()
    };

    let ns_name = "bench-stream";
    let topic = format!("{ns_name}:bench-topic");

    rt.block_on(async {
        client
            .msgs()
            .namespace()
            .create(ns_name.to_owned(), MsgNamespaceCreateIn::new())
            .await
            .unwrap();
    });

    // Benchmark the `publish` operation with large batches.
    group.bench_function("publish_batch_100", |b| {
        b.iter_batched(
            || make_msg_batch(100),
            |msgs| {
                rt.block_on(async {
                    std::hint::black_box(
                        client
                            .msgs()
                            .publish(topic.clone(), MsgPublishIn::new(msgs)),
                    )
                    .await
                    .unwrap();
                })
            },
            BatchSize::SmallInput,
        )
    });

    // Seed 100k messages for the receive+commit benchmark.
    rt.block_on(async {
        for _ in 0..100 {
            client
                .msgs()
                .publish(topic.clone(), MsgPublishIn::new(make_msg_batch(1000)))
                .await
                .unwrap();
        }
    });

    let consumer_group = "bench-consumer".to_owned();

    // Benchmark `receive` + `commit` done serially against a topic with many messages.
    group.bench_function("receive_commit_batch_100", |b| {
        b.iter(|| {
            rt.block_on(async {
                let out = std::hint::black_box(
                    client
                        .msgs()
                        .stream()
                        .receive(
                            topic.clone(),
                            consumer_group.clone(),
                            MsgStreamReceiveIn::new().with_batch_size(100),
                        )
                        .await
                        .unwrap(),
                );
                let Some(last) = out.msgs.last() else {
                    return;
                };
                std::hint::black_box(
                    client
                        .msgs()
                        .stream()
                        .commit(
                            last.topic.clone(),
                            consumer_group.clone(),
                            MsgStreamCommitIn::new(last.offset),
                        )
                        .await
                        .unwrap(),
                );
            })
        })
    });
}

fn bench_stream_single(c: &mut Criterion) {
    let ctx = setup_single_server();
    let mut group = c.benchmark_group("stream_single_server");
    bench_stream(ctx, &mut group);
}

fn bench_stream_cluster(c: &mut Criterion) {
    let ctx = setup_cluster(3);
    let mut group = c.benchmark_group("stream_cluster");
    bench_stream(ctx, &mut group);
}

criterion_group!(stream, bench_stream_single, bench_stream_cluster);
criterion_main!(stream);
