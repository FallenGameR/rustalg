# 003 p173 - Three Sum Notes

## Rayon docs

- [Overview](https://smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-parallelism-in-rust/)
- [readme](https://github.com/rayon-rs/rayon/blob/main/README.md)
- [FAQ](https://github.com/rayon-rs/rayon/blob/main/FAQ.md)
- [Rayon main](https://docs.rs/rayon/latest/rayon/)
- [Iter](https://docs.rs/rayon/latest/rayon/iter/index.html)
- [Join](https://docs.rs/rayon/latest/rayon/fn.join.html)
- [Scope](https://docs.rs/rayon/latest/rayon/fn.scope.html)

## Measurement

```ps1
cargo run --release --example ch1_p173-three-sum examples\data\rand\1K_int.txt
cargo build --release --example ch1_p173-three-sum
hyperfine.exe --warmup 1 --export-markdown examples\data\ch1_p173-three-sum\result_single_thread.md --parameter-list SIZE 1,2,4,8,16,32 ".\target\release\examples\ch1_p173-three-sum.exe examples\data\ch1_p173-three-sum\{SIZE}Kints.txt"
```

## Analysis

```log
  .\target\release\examples\ch1_p173-three-sum.exe examples\data\ch1_p173-three-sum\1Kints.txt ran
    5.02 ± 0.13 times faster than .\target\release\examples\ch1_p173-three-sum.exe examples\data\ch1_p173-three-sum\2Kints.txt
   35.79 ± 0.71 times faster than .\target\release\examples\ch1_p173-three-sum.exe examples\data\ch1_p173-three-sum\4Kints.txt
  279.94 ± 5.74 times faster than .\target\release\examples\ch1_p173-three-sum.exe examples\data\ch1_p173-three-sum\8Kints.txt
 2234.14 ± 48.41 times faster than .\target\release\examples\ch1_p173-three-sum.exe examples\data\ch1_p173-three-sum\16Kints.txt
18151.01 ± 549.46 times faster than .\target\release\examples\ch1_p173-three-sum.exe examples\data\ch1_p173-three-sum\32Kints.txt
```

- 1k -  1
- x2 - x5
- x2 - x7
- x2 - x8
- x2 - x8
- x2 - x8

```log
8 = 2 * 2 * 2

this is O(n^3) algorithm

16k -> 144s
T = a * n^3
a = T / n^3 = 144 / 16^3 = 144 / 4096 = 0.035

32k -> 0.035 * 32^3 = 0.035 * 32768 = 1146s prediction, 1169s actual, quite accurate
```

```ps1
cargo run --release --example ch1_p173-three-sum examples\data\rand\1K_int.txt
cargo build --release --example ch1_p173-three-sum
hyperfine.exe ".\target\release\examples\ch1_p173-three-sum.exe .\examples\data\rand\8K_int.txt"
```

```log
for_impl - 7s
ranges_impl - 38s
tuples_impl - 7s - same as for_impl, good cache utilization
tuples_vec_par_filter_impl - out of memory if we collect all indexes
partitions_impl - 18s - perf hit due to worse cache utilization
partitions_par_impl - 2s - parralel version of partitions_impl
ranges_arc_impl - 38s - same as ranges_impl, but with atomic counter, for_each is slow for some reason
atomic_par_impl - 6s - parralel version of ranges_arc_impl, worse then partitions_par_impl since it uses a shared memory variable, bad for caches
atomic_par_par_impl - 6s - rayon is smart about parralelism
atomic_par_par_par_impl - 14s - lots of everhead due to parralelism
atomic_par_trailing_impl - >1min - very inefficient to make small operations parralel
```
