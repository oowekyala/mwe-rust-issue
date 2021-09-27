use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main, black_box};

criterion_group!(benches, reactor_main);
criterion_main!(benches);

fn reactor_main(_: &mut Criterion) {}
