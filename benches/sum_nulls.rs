use criterion::{criterion_group, criterion_main, Criterion};

use simd_benches::sum_nulls::*;

fn close(l: f32, r: f32) {
    assert!((l - r).abs() < l * 0.0001);
}

fn add_benchmark(c: &mut Criterion) {
    let name = " null";
    (10..=20).step_by(2).for_each(|log2_size| {
        let size = 2usize.pow(log2_size);
        let array = (0..size)
            .map(|x| std::f32::consts::PI * x as f32 * x as f32 - std::f32::consts::PI * x as f32)
            .collect::<Vec<_>>();

        let mask = (0..size).map(|x| x % 123 == 0).collect::<Vec<_>>();

        let result = naive_sum(&array, &mask);

        c.bench_function(&format!("core_simd_sum{} 2^{} f32", name, log2_size), |b| {
            b.iter(|| close(core_simd_sum(&array, &mask), result))
        });
        c.bench_function(
            &format!("packed_simd_sum{} 2^{} f32", name, log2_size),
            |b| b.iter(|| close(packed_simd_sum(&array, &mask), result)),
        );
        c.bench_function(&format!("nonsimd_sum{} 2^{} f32", name, log2_size), |b| {
            b.iter(|| close(nonsimd_sum(&array, &mask), result))
        });
        c.bench_function(&format!("naive_sum{} 2^{} f32", name, log2_size), |b| {
            b.iter(|| close(naive_sum(&array, &mask), result))
        });
    });
}

criterion_group!(benches, add_benchmark);
criterion_main!(benches);
