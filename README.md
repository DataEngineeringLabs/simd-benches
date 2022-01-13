# Benchmark Rust explicit simd

This repository contains benchmarks for common vertical and horizontal operations that
leverage SIMD, comparing different implementations of the same algorithms
in them using `packed_simd2`, `core_simd` and Rust arrays.

Things implemented:

* sum of values
* sum of nullable values where nulls are represented as `Vec<bool>`
* sum of nullable values where nulls are represented as `Bitmap`
* min of values

Algorithms implemented:

* `core_simd`: vertical sum over lanes with a reduce at the end using `core_simd`
* `packed_simd`: vertical sum over lanes with a reduce at the end using `packed_simd`
* `nonsimd`: vertical sum over lanes with a reduce at the end using Rust arrays
* `naive`: sum using rust iterators

## Bench results on native

Command: 

```
RUSTFLAGS="-C target-cpu=native" cargo bench -- "2\^20"
```

### Sum of values

```
core_simd_sum 2^20 f32     [156.96 us 158.06 us 159.40 us]
packed_simd_sum 2^20 f32   [184.17 us 184.47 us 184.85 us]
nonsimd_sum 2^20 f32       [175.05 us 176.26 us 177.95 us]
naive_sum 2^20 f32         [1.6636 ms 1.6700 ms 1.6778 ms]
```

### Sum of nullable values (`Vec<bool>`)

```
core_simd_sum null 2^20 f32   [2.3610 ms 2.3713 ms 2.3831 ms]
packed_simd_sum null 2^20 f32 [1.5737 ms 1.5869 ms 1.6022 ms]
nonsimd_sum null 2^20 f32     [1.8009 ms 1.8133 ms 1.8276 ms]
naive_sum null 2^20 f32       [1.6418 ms 1.6520 ms 1.6660 ms]
```

### Sum of nullable values (`Bitmap`)

```
core_simd_sum bitmap 2^20 f32  [174.24 us 175.10 us 176.21 us]
nonsimd_sum bitmap 2^20 f32    [541.78 us 545.16 us 549.09 us]
naive_sum bitmap 2^20 f32      [1.6740 ms 1.6922 ms 1.7149 ms]
```

## Bench results on default

Command: 

```
cargo bench -- "2\^20"
```

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

### Min of values

```
core_simd_min 2^20 f32     [286.86 us 289.22 us 292.03 us]
packed_simd_min 2^20 f32   [230.50 us 234.12 us 238.86 us]
nonsimd_min 2^20 f32       [245.75 us 249.19 us 254.00 us]
naive_min 2^20 f32         [2.8560 ms 2.8721 ms 2.8885 ms]
```

### Conditions

```
$ lscpu
Architecture:        x86_64
CPU op-mode(s):      32-bit, 64-bit
Byte Order:          Little Endian
CPU(s):              4
On-line CPU(s) list: 0-3
Thread(s) per core:  2
Core(s) per socket:  2
Socket(s):           1
NUMA node(s):        1
Vendor ID:           GenuineIntel
CPU family:          6
Model:               85
Model name:          Intel(R) Xeon(R) Platinum 8171M CPU @ 2.60GHz
Stepping:            4
CPU MHz:             2095.077
BogoMIPS:            4190.15
Virtualization:      VT-x
Hypervisor vendor:   Microsoft
Virtualization type: full
L1d cache:           32K
L1i cache:           32K
L2 cache:            1024K
L3 cache:            36608K
NUMA node0 CPU(s):   0-3
Flags:               fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ss ht syscall nx pdpe1gb rdtscp lm constant_tsc rep_good nopl xtopology cpuid pni pclmulqdq vmx ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm abm 3dnowprefetch invpcid_single pti tpr_shadow vnmi ept vpid fsgsbase bmi1 hle avx2 smep bmi2 erms invpcid rtm mpx avx512f avx512dq rdseed adx smap clflushopt avx512cd avx512bw avx512vl xsaveopt xsavec xsaves md_clear
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
