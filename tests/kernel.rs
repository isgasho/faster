// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(test,stdsimd)]

extern crate faster;

#[cfg(test)]
mod tests {
    use faster::*;

    macro_rules! kernel_definite {
        ($name:ident, $native_type:ty, $simd_type:ident) => (

            /// Tests a number of simple kernel computations with integer values.
            #[test]
            fn $name() {
                for n in 0 .. 16 {

                    let vec_of_1 = vec![1 as $native_type; n];
                    let vec_of_3 = vec![3 as $native_type; n];
                    let mut out_vec = vec![0 as $native_type; n];

                    // Should produce n times (3 - 1) * (3 - 1) == n * 4 for each element
                    let sum: $native_type = ((&vec_of_3[..]).simd_iter($simd_type(0)), (&vec_of_1[..]).simd_iter($simd_type(0))).zip()
                        .simd_map(|(a, b)| (a - b) * (a - b))
                        .scalar_fill(&mut out_vec)
                        .iter()
                        .sum();

                    assert_eq!(sum, (n * 4) as $native_type);

                    // Same as above, but this time we reduce with simd_reduce
                    let sum: $native_type = ((&vec_of_3[..]).simd_iter($simd_type(0)), (&vec_of_1[..]).simd_iter($simd_type(0))).zip()
                        .simd_map(|(a, b)| (a - b) * (a - b))
                        .simd_reduce($simd_type(0), |a, v| a + v)
                        .sum();

                    assert_eq!(sum, (n * 4) as $native_type);
                }
            }
        )
    }

    kernel_definite!(kernel_i64, i64, i64s);
    kernel_definite!(kernel_i32, i32, i32s);
    kernel_definite!(kernel_i16, i16, i16s);
    kernel_definite!(kernel_i8, i8, i8s);

    kernel_definite!(kernel_u64, u64, u64s);
    kernel_definite!(kernel_u32, u32, u32s);
    kernel_definite!(kernel_u16, u16, u16s);
    kernel_definite!(kernel_u8, u8, u8s);

    macro_rules! kernel_relative {
        ($name:ident, $native_type:ty, $simd_type:ident) => (

            /// Tests a number of simple kernel computations with float values.
            #[test]
            fn $name() {
                for n in 0 .. 16 {
                    let vec_of_1 = vec![1 as $native_type; n];
                    let vec_of_3 = vec![3 as $native_type; n];

                    // Should produce n times (1 - 3) * (1 - 3) == n * 4 for each element
                    let sum_scalar: $native_type = vec_of_1.iter()
                        .zip(vec_of_3.iter())
                        .map(|(a, b)| (a - b) * (a - b))
                        .sum();

                    // Same as above, but this time we reduce with simd_reduce
                    let sum_simd: $native_type = (vec_of_1.simd_iter($simd_type(0.0 as $native_type)),
                                                  vec_of_3.simd_iter($simd_type(0.0 as $native_type)))
                        .zip()
                        .simd_map(|(a, b)| (a - b) * (a - b))
                        .simd_reduce($simd_type(0.0 as $native_type), |a, v| a + v)
                        .sum();

                    // Ensure both ways produce the same result
                    assert_eq!(sum_scalar, sum_simd);

                    // Make sure the result is equal to our target within a certain limit.
                    assert!((sum_simd - (n * 4) as $native_type).abs() < 0.0001);
                }
            }
        )
    }

    kernel_relative!(kernel_f32, f32, f32s);
    kernel_relative!(kernel_f64, f64, f64s);
}
