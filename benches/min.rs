use criterion::{criterion_group, criterion_main, Criterion};

use simd_benches::min::*;

fn close(l: f32, r: f32) {
    assert!((l - r).abs() < l.abs() * 0.001, "{} {}", l, r);
}

fn add_benchmark(c: &mut Criterion) {
    let name = "";
    (10..=20).step_by(2).for_each(|log2_size| {
        let size = 2usize.pow(log2_size);
        let array = (0..size)
            .map(|x| {
                let a = (x as f32) / (size as f32);
                std::f32::consts::PI * a * a - std::f32::consts::PI * a
            })
            .collect::<Vec<_>>();
        let result = naive_min(&array);

        c.bench_function(&format!("core_simd_min{} 2^{} f32", name, log2_size), |b| {
            b.iter(|| close(core_simd_min(&array), result))
        });
        c.bench_function(
            &format!("packed_simd_min{} 2^{} f32", name, log2_size),
            |b| b.iter(|| close(packed_simd_min(&array), result)),
        );
        c.bench_function(&format!("nonsimd_min{} 2^{} f32", name, log2_size), |b| {
            b.iter(|| close(nonsimd_min(&array), result))
        });
        c.bench_function(&format!("naive_min{} 2^{} f32", name, log2_size), |b| {
            b.iter(|| close(naive_min(&array), result))
        });
    });
}

criterion_group!(benches, add_benchmark);
criterion_main!(benches);
