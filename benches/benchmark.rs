use criterion::{criterion_group, criterion_main, Criterion};

fn with_capacity() -> Vec<usize> {
    let mut col = Vec::with_capacity(1000);
    for i in 0..(1000*1000){
        if col.capacity() == 0 {
            col.reserve(1000);
        }
        col.push(i);
    }
    col
}

fn without_capacity() -> Vec<usize> {
    let mut col = Vec::new();
    for i in 0..(1000*1000){
        col.push(i);
    }
    col
}


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("with_capacity", |b| b.iter(|| with_capacity()));
    c.bench_function("without_capacity", |b| b.iter(|| without_capacity()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);