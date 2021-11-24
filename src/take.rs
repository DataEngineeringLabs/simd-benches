use core_simd::*;

pub fn naive_take(values: &[f32], indices: &[usize]) -> Vec<f32> {
    indices.iter().map(|i| values[*i]).collect()
}

pub fn core_simd_take(values: &[f32], indices: &[usize]) -> Vec<f32> {
    let chunks = indices.chunks_exact(8);
    // todo handle remainder

    let mut result = vec![0.0; indices.len()]; // todo: maybeUninit
    let result_chunks = result.chunks_exact_mut(8);
    chunks.zip(result_chunks).for_each(|(chunk, r_chunk)| {
        let idxs: [usize; 8] = chunk.try_into().unwrap();
        let idxs: usizex8 = usizex8::from_array(idxs);

        let r = Simd::gather_or_default(&values, idxs);
        let r: [f32; 8] = r.to_array();

        let r_chunk: &mut [f32; 8] = r_chunk.try_into().unwrap();
        *r_chunk = r;
    });

    result
}
