use criterion::{Criterion, criterion_group, criterion_main, black_box};

criterion_group!(benches, my_main);
criterion_main!(benches);

fn my_main(c: &mut Criterion) {
    black_box(c);
}
