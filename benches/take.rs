use criterion::{criterion_group, criterion_main, Criterion};

use simd_benches::take::*;

fn close(l: &[f32], r: &[f32]) {
    for (l, r) in l.iter().zip(r.iter()) {
        assert!((l - r).abs() < l * 0.001);
    }
}

fn add_benchmark(c: &mut Criterion) {
    let name = "";
    (10..=20).step_by(2).for_each(|log2_size| {
        let size = 2usize.pow(log2_size);
        let array = (0..size).map(|x| 1.0 + x as f32).collect::<Vec<_>>();
        let indices = (0..size).collect::<Vec<_>>();
        // check that they are equal...
        close(
            &core_simd_take(&array, &indices),
            &naive_take(&array, &indices),
        );

        c.bench_function(
            &format!("core_simd_take{} 2^{} f32", name, log2_size),
            |b| b.iter(|| core_simd_take(&array, &indices)),
        );
        c.bench_function(&format!("naive_take{} 2^{} f32", name, log2_size), |b| {
            b.iter(|| naive_take(&array, &indices))
        });
    });
}

criterion_group!(benches, add_benchmark);
criterion_main!(benches);
