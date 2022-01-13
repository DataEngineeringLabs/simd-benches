use std::convert::TryInto;

use core_simd::f32x16;
use packed_simd::f32x16 as p_f32x16;

const LANES: usize = 16;

fn min_iter<I: Iterator<Item = f32>>(i: I) -> f32 {
    i.fold(f32::MAX, |a, b| a.min(b))
}

pub fn packed_simd_min(values: &[f32]) -> f32 {
    let chunks = values.chunks_exact(LANES);
    let remainder = chunks.remainder();

    let min = chunks.fold(p_f32x16::default(), |acc, chunk| {
        let chunk: [f32; 16] = chunk.try_into().unwrap();
        let chunk: p_f32x16 = p_f32x16::from_slice_unaligned(&chunk);

        acc.min(chunk)
    });

    let remainder: f32 = min_iter(remainder.iter().copied());

    min.min_element().min(remainder)
}

pub fn core_simd_min(values: &[f32]) -> f32 {
    let chunks = values.chunks_exact(LANES);
    let remainder = chunks.remainder();

    let acc = f32x16::splat(f32::MAX);
    let min = chunks.fold(acc, |acc, chunk| {
        let chunk: [f32; 16] = chunk.try_into().unwrap();
        let chunk: f32x16 = f32x16::from_array(chunk);
        acc.min(chunk)
    });

    let remainder: f32 = min_iter(remainder.iter().copied());

    min.horizontal_min().min(remainder)
}

pub fn nonsimd_min(values: &[f32]) -> f32 {
    let chunks = values.chunks_exact(LANES);
    let remainder = chunks.remainder();

    let sum = chunks.fold([f32::MAX; LANES], |mut acc, chunk| {
        let chunk: [f32; LANES] = chunk.try_into().unwrap();
        for i in 0..LANES {
            acc[i] = chunk[i].min(acc[i]);
        }
        acc
    });

    let remainder: f32 = min_iter(remainder.iter().copied());

    let mut reduced = f32::MAX;
    (0..LANES).for_each(|i| {
        reduced = reduced.min(sum[i]);
    });
    reduced.min(remainder)
}

pub fn naive_min(values: &[f32]) -> f32 {
    min_iter(values.iter().copied())
}
