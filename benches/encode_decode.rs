use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[cfg(feature = "alloc")]
fn bench_encode(c: &mut Criterion) {
    use oi4_dnp_encoding::encode;
    let small = "Hello World!";
    let medium = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";
    let large = small.repeat(512);
    c.bench_function("encode_small", |b| b.iter(|| black_box(encode(small))));
    c.bench_function("encode_medium", |b| b.iter(|| black_box(encode(medium))));
    c.bench_function("encode_large", |b| b.iter(|| black_box(encode(&large))));
}

#[cfg(feature = "alloc")]
fn bench_decode(c: &mut Criterion) {
    use oi4_dnp_encoding::{decode, encode};
    let src = "Hello World!".repeat(256);
    let enc = encode(&src);
    c.bench_function("decode_large", |b| {
        b.iter(|| black_box(decode(&enc).unwrap()))
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    #[cfg(feature = "alloc")]
    {
        bench_encode(c);
        bench_decode(c);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
