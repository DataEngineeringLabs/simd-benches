use criterion::{criterion_group, criterion_main, Criterion};

use simd_benches::bitmap_ops;
use simd_benches::take::*;

fn close(l: &[f32], r: &[f32]) {
    for (l, r) in l.iter().zip(r.iter()) {
        assert!((l - r).abs() < l * 0.001 || (l.abs() < 0.000001 && r.abs() < 0.000001));
    }
}

fn add_benchmark(c: &mut Criterion) {
    let name = "";
    (10..=20).step_by(2).for_each(|log2_size| {
        let size = 2usize.pow(log2_size);
        let array = (0..size).map(|x| 1.0 + x as f32).collect::<Vec<_>>();
        let mut mask = vec![0u8; size / 8];
        // 10% nulls
        (0..size).for_each(|x| bitmap_ops::set_bit(&mut mask, x, (1 + x) % 10 != 0));
        let mask = (mask, size);
        let indices = (0..size).collect::<Vec<_>>();
        // check that they are equal...
        close(
            &core_simd_take_nulls(&array, &indices, &mask),
            &naive_take_nulls(&array, &indices, &mask),
        );

        c.bench_function(
            &format!("core_simd_take_nulls{} 2^{} f32", name, log2_size),
            |b| b.iter(|| core_simd_take_nulls(&array, &indices, &mask)),
        );
        c.bench_function(
            &format!("naive_take_nulls{} 2^{} f32", name, log2_size),
            |b| b.iter(|| naive_take_nulls(&array, &indices, &mask)),
        );
    });
}

criterion_group!(benches, add_benchmark);
criterion_main!(benches);
