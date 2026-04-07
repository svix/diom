use std::time::Duration;

use coyote::models::{MsgIn, MsgNamespaceCreateIn, MsgPublishIn, MsgQueueAckIn, MsgQueueReceiveIn};
use coyote_benchmarks::{BenchmarkContext, setup_cluster, setup_single_server};
use criterion::{
    BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::Measurement,
};
use rand::{Rng, SeedableRng, rngs::StdRng};

fn bench_queue<'a, M: Measurement>(ctx: BenchmarkContext, group: &mut BenchmarkGroup<'a, M>) {
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

    // Seed a topic with 100k messages for the receive+ack benchmark.
    {
        let ns_name = "bench-queue-ack";
        let topic = format!("{ns_name}:bench-topic");

        rt.block_on(async {
            client
                .msgs()
                .namespace()
                .create(ns_name.to_owned(), MsgNamespaceCreateIn::new())
                .await
                .unwrap();
            for _ in 0..100 {
                client
                    .msgs()
                    .publish(topic.clone(), MsgPublishIn::new(make_msg_batch(1000)))
                    .await
                    .unwrap();
            }
        });

        // Benchmark queue `receive` + `ack` done serially.
        group.bench_function("receive_ack_batch_100", |b| {
            b.iter(|| {
                rt.block_on(async {
                    let out = std::hint::black_box(
                        client
                            .msgs()
                            .queue()
                            .receive(
                                topic.clone(),
                                "bench-cg".to_owned(),
                                MsgQueueReceiveIn::new().with_batch_size(100u16),
                            )
                            .await
                            .unwrap(),
                    );
                    if out.msgs.is_empty() {
                        return;
                    }
                    let msg_ids: Vec<String> = out.msgs.into_iter().map(|m| m.msg_id).collect();
                    std::hint::black_box(
                        client
                            .msgs()
                            .queue()
                            .ack(
                                topic.clone(),
                                "bench-cg".to_owned(),
                                MsgQueueAckIn::new(msg_ids),
                            )
                            .await
                            .unwrap(),
                    );
                })
            })
        });
    }

    // Benchmark queue `receive` in isolation (no ack). Uses a short lease so messages
    // cycle back and the benchmark can run indefinitely against a fixed message set.
    {
        let ns_name = "bench-queue-receive";
        let topic = format!("{ns_name}:bench-topic");

        rt.block_on(async {
            client
                .msgs()
                .namespace()
                .create(ns_name.to_owned(), MsgNamespaceCreateIn::new())
                .await
                .unwrap();
            for _ in 0..100 {
                client
                    .msgs()
                    .publish(topic.clone(), MsgPublishIn::new(make_msg_batch(1000)))
                    .await
                    .unwrap();
            }
        });

        group.bench_function("receive_only_batch_100", |b| {
            b.iter(|| {
                rt.block_on(async {
                    std::hint::black_box(
                        client
                            .msgs()
                            .queue()
                            .receive(
                                topic.clone(),
                                "bench-cg".to_owned(),
                                MsgQueueReceiveIn::new()
                                    .with_batch_size(100u16)
                                    .with_lease_duration(Duration::from_millis(100)),
                            )
                            .await
                            .unwrap(),
                    );
                })
            })
        });
    }
}

fn bench_queue_single(c: &mut Criterion) {
    let ctx = setup_single_server();
    let mut group = c.benchmark_group("queue_single_server");
    bench_queue(ctx, &mut group);
}

fn bench_queue_cluster(c: &mut Criterion) {
    let ctx = setup_cluster(3);
    let mut group = c.benchmark_group("queue_cluster");
    bench_queue(ctx, &mut group);
}

criterion_group!(queue, bench_queue_single, bench_queue_cluster);
criterion_main!(queue);
