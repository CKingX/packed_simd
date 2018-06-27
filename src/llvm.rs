//! LLVM's simd platform intrinsics
#![allow(dead_code)] // FIXME: remove this allow

extern "platform-intrinsic" {
    crate fn simd_eq<T, U>(x: T, y: T) -> U;
    crate fn simd_ne<T, U>(x: T, y: T) -> U;
    crate fn simd_lt<T, U>(x: T, y: T) -> U;
    crate fn simd_le<T, U>(x: T, y: T) -> U;
    crate fn simd_gt<T, U>(x: T, y: T) -> U;
    crate fn simd_ge<T, U>(x: T, y: T) -> U;

    pub fn simd_shuffle2<T, U>(x: T, y: T, idx: [u32; 2]) -> U;
    pub fn simd_shuffle4<T, U>(x: T, y: T, idx: [u32; 4]) -> U;
    pub fn simd_shuffle8<T, U>(x: T, y: T, idx: [u32; 8]) -> U;
    pub fn simd_shuffle16<T, U>(x: T, y: T, idx: [u32; 16]) -> U;
    pub fn simd_shuffle32<T, U>(x: T, y: T, idx: [u32; 32]) -> U;

    crate fn simd_insert<T, U>(x: T, idx: u32, val: U) -> T;
    crate fn simd_extract<T, U>(x: T, idx: u32) -> U;

    crate fn simd_cast<T, U>(x: T) -> U;

    crate fn simd_add<T>(x: T, y: T) -> T;
    crate fn simd_sub<T>(x: T, y: T) -> T;
    crate fn simd_mul<T>(x: T, y: T) -> T;
    crate fn simd_div<T>(x: T, y: T) -> T;
    crate fn simd_rem<T>(x: T, y: T) -> T;
    crate fn simd_shl<T>(x: T, y: T) -> T;
    crate fn simd_shr<T>(x: T, y: T) -> T;
    crate fn simd_and<T>(x: T, y: T) -> T;
    crate fn simd_or<T>(x: T, y: T) -> T;
    crate fn simd_xor<T>(x: T, y: T) -> T;

    crate fn simd_reduce_add_unordered<T, U>(x: T) -> U;
    crate fn simd_reduce_mul_unordered<T, U>(x: T) -> U;
    crate fn simd_reduce_add_ordered<T, U>(x: T, acc: U) -> U;
    crate fn simd_reduce_mul_ordered<T, U>(x: T, acc: U) -> U;
    crate fn simd_reduce_min<T, U>(x: T) -> U;
    crate fn simd_reduce_max<T, U>(x: T) -> U;
    crate fn simd_reduce_min_nanless<T, U>(x: T) -> U;
    crate fn simd_reduce_max_nanless<T, U>(x: T) -> U;
    crate fn simd_reduce_and<T, U>(x: T) -> U;
    crate fn simd_reduce_or<T, U>(x: T) -> U;
    crate fn simd_reduce_xor<T, U>(x: T) -> U;
    crate fn simd_reduce_all<T>(x: T) -> bool;
    crate fn simd_reduce_any<T>(x: T) -> bool;

    crate fn simd_select<M, T>(m: M, a: T, b: T) -> T;

    crate fn simd_fmin<T>(a: T, b: T) -> T;
    // FIXME: https://github.com/rust-lang-nursery/stdsimd/issues/416
    // crate fn simd_fmax<T>(a: T, b: T) -> T;

    crate fn simd_fsqrt<T>(a: T) -> T;
    crate fn simd_fma<T>(a: T, b: T, c: T) -> T;
}
