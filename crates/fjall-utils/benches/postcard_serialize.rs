use criterion::{Criterion, criterion_group, criterion_main};
use fjall_utils::V0Wrapper;
use postcard::ser_flavors;
use rand::{Rng, SeedableRng, rngs::StdRng};

/// Mimics KvPairRow: a struct with a large byte payload, an optional timestamp, and a version.
#[derive(serde::Serialize, serde::Deserialize)]
struct KvPairLike {
    #[serde(with = "serde_bytes")]
    value: Vec<u8>,
    expiry: Option<i64>,
    version: u64,
}

fn make_row(rng: &mut StdRng, value_size: usize) -> KvPairLike {
    let mut value = vec![0u8; value_size];
    rng.fill(&mut value[..]);
    KvPairLike {
        value,
        expiry: Some(1_700_000_000_000),
        version: 42,
    }
}

fn bench_size(c: &mut Criterion, name: &str, row: &KvPairLike) {
    let mut group = c.benchmark_group(name);

    group.bench_function("to_allocvec", |b| {
        b.iter(|| {
            let bytes: Vec<u8> = postcard::to_allocvec(&V0Wrapper::V0(row)).unwrap();
            let _slice: fjall::Slice = bytes.into();
        });
    });

    group.bench_function("two_pass_byteview", |b| {
        b.iter(|| {
            let wrapped = V0Wrapper::V0(row);
            let size =
                postcard::serialize_with_flavor(&wrapped, ser_flavors::Size::default()).unwrap();
            let mut builder = byteview::ByteView::builder(size);
            postcard::to_slice(&wrapped, &mut builder).unwrap();
            let _slice: fjall::Slice = builder.freeze().into();
        });
    });

    group.finish();
}

fn bench_postcard_serialize(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(0xBEEF);

    let row_20b = make_row(&mut rng, 20);
    let row_1kb = make_row(&mut rng, 1024);
    let row_2kb = make_row(&mut rng, 2048);
    let row_20kb = make_row(&mut rng, 20480);

    bench_size(c, "postcard_serialize_20b", &row_20b);
    bench_size(c, "postcard_serialize_1kb", &row_1kb);
    bench_size(c, "postcard_serialize_2kb", &row_2kb);
    bench_size(c, "postcard_serialize_20kb", &row_20kb);
}

criterion_group!(benches, bench_postcard_serialize);
criterion_main!(benches);
