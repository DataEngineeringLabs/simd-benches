use core_simd::*;

use super::bitmap_ops::*;

pub fn naive_take(values: &[f32], indices: &[usize]) -> Vec<f32> {
    indices.iter().map(|i| values[*i]).collect()
}

const LANES: usize = 8;
const MASK_LANES: usize = 8 / 8;

pub fn core_simd_take(values: &[f32], indices: &[usize]) -> Vec<f32> {
    let chunks = indices.chunks_exact(LANES);
    // todo handle remainder

    let mut result = vec![0.0; indices.len()]; // todo: maybeUninit
    let result_chunks = result.chunks_exact_mut(LANES);
    chunks.zip(result_chunks).for_each(|(chunk, r_chunk)| {
        let idxs: [usize; LANES] = chunk.try_into().unwrap();
        let idxs: usizex8 = usizex8::from_array(idxs);

        let r = Simd::gather_or_default(&values, idxs);
        let r: [f32; LANES] = r.to_array();

        let r_chunk: &mut [f32; LANES] = r_chunk.try_into().unwrap();
        *r_chunk = r;
    });

    result
}

type Bitmap = (Vec<u8>, usize);

pub fn naive_take_nulls(values: &[f32], indices: &[usize], mask: &Bitmap) -> Vec<f32> {
    let mask = (0..mask.1).map(|x| get_bit(&mask.0, x));

    indices
        .iter()
        .zip(mask)
        .map(|(x, m)| if m { values[*x] } else { 0.0f32 })
        .collect()
}

pub fn core_simd_take_nulls(values: &[f32], indices: &[usize], mask: &Bitmap) -> Vec<f32> {
    assert_eq!(mask.1 % 16, 0); // todo: handle remainders
    let chunks = indices.chunks_exact(LANES);
    let mask_chunks = mask.0.chunks_exact(MASK_LANES);
    //let remainder = chunks.remainder();
    //let mask_remainder = mask_chunks.remainder();

    let mut result = vec![0.0; indices.len()]; // todo: maybeUninit
    let result_chunks = result.chunks_exact_mut(LANES);
    chunks
        .zip(mask_chunks)
        .zip(result_chunks)
        .for_each(|((chunk, mask_chunk), r_chunk)| {
            let idxs: [usize; LANES] = chunk.try_into().unwrap();
            let idxs: usizex8 = usizex8::from_array(idxs);

            let mask: [u8; MASK_LANES] = mask_chunk.try_into().unwrap();
            let mask = masksizex8::from_bitmask(mask);

            let r = Simd::gather_select(&values, mask, idxs, Simd::splat(f32::default()));
            let r: [f32; LANES] = r.to_array();

            let r_chunk: &mut [f32; LANES] = r_chunk.try_into().unwrap();
            *r_chunk = r;
        });
    result
}
