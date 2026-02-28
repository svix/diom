use coyote_benchmarks::{BenchmarkContext, setup_cluster, setup_single_server};
use coyote_client::models::{CreateNamespaceIn, MsgIn, PublishIn, StreamCommitIn, StreamReceiveIn};
use criterion::{
    BatchSize, BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::Measurement,
};
use rand::{Rng, SeedableRng, rngs::StdRng};

fn bench_msgs<'a, M: Measurement>(ctx: BenchmarkContext, group: &mut BenchmarkGroup<'a, M>) {
    let client = ctx.client;

    group.sample_size(60);

    let mut rng = StdRng::seed_from_u64(0);
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let make_test_namespace = |ns: &str| {
        rt.block_on(async {
            client
                .msgs()
                .namespace()
                .create(CreateNamespaceIn::new(ns.to_owned()))
                .await
                .unwrap();
        });
    };

    let mut make_msg_batch = |size: usize| {
        (0..size)
            .map(|_| {
                let mut val = vec![0u8; 256];
                rng.fill(&mut val[..]);
                MsgIn::new(val)
            })
            .collect::<Vec<_>>()
    };

    // Benchmark the `publish` operation.
    // I really just care about large batches here. (The assumption being, smaller batches will trivially be cheaper to write.)

    {
        let ns_name = "bench-publish";
        let topic = format!("{ns_name}:bench-topic");
        make_test_namespace(ns_name);

        group.bench_function("msgs_publish_batch_100", |b| {
            b.iter_batched(
                || make_msg_batch(100),
                |msgs| {
                    rt.block_on(async {
                        std::hint::black_box(
                            client.msgs().publish(PublishIn::new(msgs, topic.clone())),
                        )
                        .await
                        .unwrap();
                    })
                },
                BatchSize::SmallInput,
            )
        });
    }

    // Benchmark the `receive` + `commit` operation when done serially.
    // We're benchmarking this against a topic with a LOT of messages, to try to force
    // any issues with long msgs streams to show up.
    {
        let ns_name = "bench-publish";
        let topic = format!("{ns_name}:bench-topic");
        make_test_namespace(ns_name);

        // Start with 100_000 messages
        rt.block_on(async {
            for _ in 0..100 {
                client
                    .msgs()
                    .publish(PublishIn::new(make_msg_batch(1000), topic.clone()))
                    .await
                    .unwrap();
            }
        });

        let consumer_group = "bench-consumer".to_owned();

        group.bench_function("msgs_stream_receive_commit_batch_100", |b| {
            b.iter(|| {
                rt.block_on(async {
                    let mut input = StreamReceiveIn::new(consumer_group.clone(), topic.clone());
                    input.batch_size = Some(100);
                    let out =
                        std::hint::black_box(client.msgs().stream().receive(input).await.unwrap());
                    let last = out.msgs.last().unwrap();
                    // Response topic already includes namespace, pass directly to commit
                    std::hint::black_box(
                        client
                            .msgs()
                            .stream()
                            .commit(StreamCommitIn::new(
                                consumer_group.clone(),
                                last.offset,
                                last.topic.clone(),
                            ))
                            .await
                            .unwrap(),
                    );
                })
            })
        });
    }
}

fn bench_msgs_single(c: &mut Criterion) {
    let ctx = setup_single_server();
    let mut group = c.benchmark_group("msgs_single_server");
    bench_msgs(ctx, &mut group);
}

fn bench_msgs_cluster(c: &mut Criterion) {
    let ctx = setup_cluster(3);
    let mut group = c.benchmark_group("msgs_cluster");
    bench_msgs(ctx, &mut group);
}

criterion_group!(msgs, bench_msgs_single, bench_msgs_cluster);
criterion_main!(msgs);
