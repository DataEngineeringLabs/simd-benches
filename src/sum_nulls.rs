use std::convert::TryInto;

use core_simd::f32x16;
use core_simd::mask32x16;
use packed_simd::f32x16 as p_f32x16;
use packed_simd::m32x16 as p_m32x16;

const LANES: usize = 16;

pub fn core_simd_sum(values: &[f32], mask: &[bool]) -> f32 {
    let chunks = values.chunks_exact(LANES);
    let mask_chunks = mask.chunks_exact(LANES);
    let remainder = chunks.remainder();
    let mask_remainder = mask_chunks.remainder();

    let sum = chunks
        .zip(mask_chunks)
        .fold(f32x16::default(), |acc, (chunk, chunk_mask)| {
            let chunk: [f32; 16] = chunk.try_into().unwrap();
            let chunk = f32x16::from_array(chunk);

            let mask: [bool; 16] = chunk_mask.try_into().unwrap();
            let mask = mask32x16::from_array(mask);

            acc + mask.select(chunk, Default::default())
        });

    let remainder = naive_sum(remainder, mask_remainder);

    let mut reduced = 0.0f32;
    for i in 0..LANES {
        reduced += sum[i];
    }
    reduced + remainder
}

pub fn packed_simd_sum(values: &[f32], mask: &[bool]) -> f32 {
    let chunks = values.chunks_exact(LANES);
    let mask_chunks = mask.chunks_exact(LANES);
    let remainder = chunks.remainder();
    let mask_remainder = mask_chunks.remainder();

    let sum = chunks
        .zip(mask_chunks)
        .fold(p_f32x16::default(), |acc, (chunk, chunk_mask)| {
            let chunk: [f32; 16] = chunk.try_into().unwrap();
            let chunk = p_f32x16::from_slice_unaligned(&chunk);

            let mask: [bool; 16] = chunk_mask.try_into().unwrap();
            let mask = p_m32x16::new(
                mask[0],
                mask[1],
                mask[2],
                mask[3],
                mask[4],
                mask[4 + 1],
                mask[4 + 2],
                mask[4 + 3],
                mask[8],
                mask[8 + 1],
                mask[8 + 2],
                mask[8 + 3],
                mask[12],
                mask[8 + 4 + 1],
                mask[8 + 4 + 2],
                mask[8 + 4 + 3],
            );

            acc + mask.select(chunk, Default::default())
        });

    let remainder = naive_sum(remainder, mask_remainder);

    sum.sum() + remainder
}

pub fn nonsimd_sum(values: &[f32], mask: &[bool]) -> f32 {
    let chunks = values.chunks_exact(LANES);
    let mask_chunks = mask.chunks_exact(LANES);
    let remainder = chunks.remainder();
    let mask_remainder = mask_chunks.remainder();

    let sum = chunks
        .zip(mask_chunks)
        .fold([0.0f32; LANES], |mut acc, (chunk, chunk_mask)| {
            let chunk: [f32; LANES] = chunk.try_into().unwrap();
            let mask: [bool; LANES] = chunk_mask.try_into().unwrap();
            for i in 0..LANES {
                acc[i] += if mask[i] { chunk[i] } else { 0.0f32 }
            }
            acc
        });

    let remainder = naive_sum(remainder, mask_remainder);

    let mut reduced = 0.0f32;
    (0..LANES).for_each(|i| {
        reduced += sum[i];
    });
    reduced + remainder
}

pub fn naive_sum(values: &[f32], mask: &[bool]) -> f32 {
    values
        .iter()
        .zip(mask.iter())
        .map(|(x, m)| if *m { *x } else { 0.0f32 })
        .sum()
}
