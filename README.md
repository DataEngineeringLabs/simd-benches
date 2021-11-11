# Benchmark Rust explicit simd

This repository contains benchmarks for common vertical and horizontal operations that
leverage SIMD, comparing different implementations of the same algorithms
in them using `packed_simd2` and `core_simd`.

Things implemented:

* sum of values
* sum of nullable values where nulls are represented as `Vec<bool>`
* sum of nullable values where nulls are represented as `Bitmap`

Algorithms implemented:

* `core_simd`: vertical sum over lanes with a reduce at the end using `core_simd`
* `packed_simd`: vertical sum over lanes with a reduce at the end using `packed_simd`
* `nonsimd`: vertical sum over lanes with a reduce at the end using Rust arrays
* `naive`: sum using rust iterators

## Bench results on my computer

### Sum of values

```
core_simd_sum 2^20 f32     [184.95 us 185.86 us 186.97 us]
packed_simd_sum 2^20 f32   [184.97 us 186.85 us 189.59 us]
nonsimd_sum 2^20 f32       [191.35 us 192.67 us 194.46 us]
naive_sum 2^20 f32         [1.6385 ms 1.6426 ms 1.6466 ms]
```

### Sum of nullable values (`Vec<bool>`)

```
core_simd_sum null 2^20 f32   [882.21 us 889.56 us 897.74 us]
packed_simd_sum null 2^20 f32 [824.37 us 835.77 us 849.63 us]
nonsimd_sum null 2^20 f32     [695.79 us 707.87 us 721.98 us]
naive_sum null 2^20 f32       [1.6418 ms 1.6520 ms 1.6660 ms]
```

### Sum of nullable values (`Bitmap`)

```
core_simd_sum bitmap 2^20 f32  [929.95 us 936.31 us 943.64 us]
nonsimd_sum bitmap 2^20 f32    [454.78 us 462.08 us 471.82 us]
naive_sum bitmap 2^20 f32      [1.7633 ms 1.7736 ms 1.7855 ms]
```

### Conclusions so far:

* for non-null sums, it is advantageous to use SIMD
* for sums with nulls, it is not advantageous to use SIMD

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
