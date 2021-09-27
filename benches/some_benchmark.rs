use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main, black_box};
use std::time::Duration;

criterion_group!(benches, reactor_main);
criterion_main!(benches);

fn reactor_main(c: &mut Criterion) {
    let mut group = c.benchmark_group("savina_pong");
    for num_pongs in [1000, 10_000, 50_000, 1_000_000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_pongs),
            num_pongs,
            |b, &size| b.iter(|| black_box(size)),
        );
    }
    group.finish();
}
