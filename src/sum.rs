use std::convert::TryInto;

use core_simd::f32x16;
use packed_simd::f32x16 as p_f32x16;

const LANES: usize = 16;

pub fn packed_simd_sum(values: &[f32]) -> f32 {
    let chunks = values.chunks_exact(LANES);
    let remainder = chunks.remainder();

    let sum = chunks.fold(p_f32x16::default(), |acc, chunk| {
        let chunk: [f32; 16] = chunk.try_into().unwrap();
        let chunk: p_f32x16 = p_f32x16::from_slice_unaligned(&chunk);

        acc + chunk
    });

    let remainder: f32 = remainder.iter().copied().sum();

    sum.sum() + remainder
}

pub fn core_simd_sum(values: &[f32]) -> f32 {
    let chunks = values.chunks_exact(LANES);
    let remainder = chunks.remainder();

    let sum = chunks.fold(f32x16::default(), |acc, chunk| {
        let chunk: [f32; 16] = chunk.try_into().unwrap();
        let chunk: f32x16 = f32x16::from_array(chunk);

        acc + chunk
    });

    let remainder: f32 = remainder.iter().copied().sum();

    let mut reduced = 0.0f32;
    for i in 0..LANES {
        reduced += sum[i];
    }
    reduced + remainder
}

pub fn nonsimd_sum(values: &[f32]) -> f32 {
    let chunks = values.chunks_exact(LANES);
    let remainder = chunks.remainder();

    let sum = chunks.fold([0.0f32; LANES], |mut acc, chunk| {
        let chunk: [f32; LANES] = chunk.try_into().unwrap();
        for i in 0..LANES {
            acc[i] += chunk[i];
        }
        acc
    });

    let remainder: f32 = remainder.iter().copied().sum();

    let mut reduced = 0.0f32;
    (0..LANES).for_each(|i| {
        reduced += sum[i];
    });
    reduced + remainder
}

pub fn naive_sum(values: &[f32]) -> f32 {
    values.iter().sum()
}
