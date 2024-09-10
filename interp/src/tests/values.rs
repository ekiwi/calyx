#[cfg(test)]
mod val_test {
    use crate::values::BitVecValue;
    #[test]
    fn basic_print_test() {
        let v1 = BitVecValue::from(12, 5);
        println!("12 with bit width 5: {}", v1);
        assert_eq!(v1.as_u64(), 12);
    }
    #[test]
    fn basic_print_test2() {
        let v1 = BitVecValue::from(33, 6);
        println!("33 with bit width 6: {}", v1);
        assert_eq!(v1.as_u64(), 33);
    }
    #[test]
    fn too_few_bits() {
        let v_16_4 = BitVecValue::from(16, 4);
        println!("16 with bit width 4: {}", v_16_4);
        assert_eq!(v_16_4.as_u64(), 0);
        let v_31_4 = BitVecValue::from(31, 4);
        println!("31 with bit width 4: {}", v_31_4);
        let v_15_4 = BitVecValue::from(15, 4);
        println!("15 with bit width 4: {}", v_15_4);
        assert_eq!(v_31_4.as_u64(), v_15_4.as_u64());
    }

    #[test]
    fn ext() {
        let v_15_4 = BitVecValue::from(15, 4);
        assert_eq!(v_15_4.as_u64(), v_15_4.ext(8).as_u64());
    }
}

#[cfg(test)]
mod unsigned_fixed_point_tests {
    use crate::values::BitVecValue;
    use fraction::Fraction;

    #[test]
    fn test_zero() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 0, /*width=*/ 4)
                .to_unsigned_fixed_point(/*fractional_width=*/ 2),
            Fraction::new(0u32, 1u32)
        );
    }
    #[test]
    fn test_zero_fractional_width() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 0b1110, /*width=*/ 4)
                .to_unsigned_fixed_point(/*fractional_width=*/ 0),
            Fraction::new(14u32, 1u32)
        );
    }
    #[test]
    fn test_high_bits_set() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 0b1110, /*width=*/ 4)
                .to_unsigned_fixed_point(/*fractional_width=*/ 2),
            Fraction::new(7u32, 2u32)
        );
    }
    #[test]
    fn test_middle_bits_set() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 0b0110, /*width=*/ 4)
                .to_unsigned_fixed_point(/*fractional_width=*/ 2),
            Fraction::new(3u32, 2u32)
        );
    }
    #[test]
    fn test_low_bits_set() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 0b0111, /*width=*/ 4)
                .to_unsigned_fixed_point(/*fractional_width=*/ 2),
            Fraction::new(7u32, 4u32)
        );
    }
    #[test]
    fn test_low_high_bits_set() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 0b1001, /*width=*/ 4)
                .to_unsigned_fixed_point(/*fractional_width=*/ 2),
            Fraction::new(9u32, 4u32)
        );
    }
    #[test]
    fn test_32bit_fractional_value() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 1u32, /*width=*/ 32)
                .to_unsigned_fixed_point(/*fractional_width=*/ 31),
            Fraction::new(1u32, 2147483648u32)
        );
    }
    #[test]
    fn test_64bit_fractional_value() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 1u64, /*width=*/ 64)
                .to_unsigned_fixed_point(/*fractional_width=*/ 63),
            Fraction::new(1u64, 9223372036854775808u64)
        );
    }
    #[test]
    fn test_alternating_ones() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 0b10101, /*width=*/ 5)
                .to_unsigned_fixed_point(/*fractional_width=*/ 3),
            Fraction::new(21u32, 8u32)
        );
    }
    #[test]
    fn test_all_ones() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 0b111, /*width=*/ 3)
                .to_unsigned_fixed_point(/*fractional_width=*/ 1),
            Fraction::new(7u32, 2u32)
        );
    }
}

#[cfg(test)]
mod signed_fixed_point_tests {
    use crate::values::BitVecValue;
    use fraction::Fraction;

    #[test]
    fn test_zero() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 0, /*width=*/ 4)
                .to_signed_fixed_point(/*fractional_width=*/ 2),
            Fraction::new(0u32, 1u32)
        );
    }
    #[test]
    fn test_zero_fractional_width() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 0b1110, /*width=*/ 4)
                .to_signed_fixed_point(/*fractional_width=*/ 0),
            -Fraction::new(2u32, 1u32)
        );
    }
    #[test]
    fn test_high_bits_set() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 0b1110, /*width=*/ 4)
                .to_signed_fixed_point(/*fractional_width=*/ 2),
            -Fraction::new(1u32, 2u32)
        );
    }
    #[test]
    fn test_middle_bits_set() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 0b0110, /*width=*/ 4)
                .to_signed_fixed_point(/*fractional_width=*/ 2),
            Fraction::new(3u32, 2u32)
        );
    }
    #[test]
    fn test_low_bits_set() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 0b0111, /*width=*/ 4)
                .to_signed_fixed_point(/*fractional_width=*/ 2),
            Fraction::new(7u32, 4u32)
        );
    }
    #[test]
    fn test_mixed_bits_set() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 0b10110101, /*width=*/ 8)
                .to_signed_fixed_point(/*fractional_width=*/ 3),
            -Fraction::new(75u32, 8u32)
        );
    }
    #[test]
    fn test_mixed_bits_set2() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 0b10100011, /*width=*/ 8)
                .to_signed_fixed_point(/*fractional_width=*/ 4),
            -Fraction::new(93u32, 16u32)
        );
    }
    #[test]
    fn test_mixed_bits_set3() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 0b11111101, /*width=*/ 8)
                .to_signed_fixed_point(/*fractional_width=*/ 4),
            -Fraction::new(3u32, 16u32)
        );
    }
    #[test]
    fn test_low_high_bits_set() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 0b1001, /*width=*/ 4)
                .to_signed_fixed_point(/*fractional_width=*/ 2),
            -Fraction::new(7u32, 4u32)
        );
    }
    #[test]
    fn test_single_bit_set() {
        assert_eq!(
            BitVecValue::from(
                /*value=*/ 0b10000000000000000000000000000000u32,
                /*width=*/ 32,
            )
            .to_signed_fixed_point(/*fractional_width=*/ 31),
            -Fraction::new(1u32, 1u32)
        );
    }
    #[test]
    fn test_small_negative_value() {
        assert_eq!(
            BitVecValue::from(
                /*value=*/ 0b10000000000000000000000000000001u32,
                /*width=*/ 32,
            )
            .to_signed_fixed_point(/*fractional_width=*/ 31),
            -Fraction::new(2147483647u32, 2147483648u32)
        );
    }
    #[test]
    fn test_alternating_ones() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 0b10101, /*width=*/ 5)
                .to_signed_fixed_point(/*fractional_width=*/ 3),
            -Fraction::new(11u32, 8u32)
        );
    }
    #[test]
    fn test_all_ones() {
        assert_eq!(
            BitVecValue::from(/*value=*/ 0b111, /*width=*/ 3)
                .to_signed_fixed_point(/*fractional_width=*/ 1),
            -Fraction::new(1u32, 2u32)
        );
    }
}

#[cfg(test)]
mod property_tests {
    use crate::values::BitVecValue;
    use num_bigint::{BigInt, BigUint};
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]


        #[test]
        fn u8_round_trip(input: u8) {
            prop_assert_eq!(input as u64, BitVecValue::from(input, 8).as_u64())
        }

        #[test]
        fn u16_round_trip(input: u16) {
            prop_assert_eq!(input as u64, BitVecValue::from(input, 16).as_u64())
        }

        #[test]
        fn u32_round_trip(input: u32) {
            prop_assert_eq!(input as u64, BitVecValue::from(input, 32).as_u64())
        }

        #[test]
        fn u64_round_trip(input: u64) {
            prop_assert_eq!(input, BitVecValue::from(input, 64).as_u64())
        }

        #[test]
        fn u128_round_trip(input: u128) {
            prop_assert_eq!(input, BitVecValue::from(input, 128).as_u128())
        }

        #[test]
        fn i8_round_trip(input: i8) {
            prop_assert_eq!(input as i64, BitVecValue::from(input, 8).as_i64())
        }

        #[test]
        fn i16_round_trip(input: i16) {
            prop_assert_eq!(input as i64, BitVecValue::from(input, 16).as_i64())
        }

        #[test]
        fn i32_round_trip(input: i32) {
            prop_assert_eq!(input as i64, BitVecValue::from(input, 32).as_i64())
        }

        #[test]
        fn i64_round_trip(input: i64) {
            prop_assert_eq!(input, BitVecValue::from(input, 64).as_i64())
        }

        #[test]
        fn i128_round_trip(input: i128) {
            prop_assert_eq!(input, BitVecValue::from(input, 128).as_i128())
        }

        #[test]
        fn i128_to_ibig(input: i128) {
            let val = BitVecValue::from(input, 128);
            prop_assert_eq!(val.to_big_int(), input.into())
        }

        #[test]
        fn u128_to_ubig(input: u128) {
            let val = BitVecValue::from(input, 128);
            prop_assert_eq!(val.to_big_uint(), input.into())
        }


        #[test]
        fn u32_to_ubig(input: u32) {
            let val = BitVecValue::from(input, 32);
            prop_assert_eq!(val.to_big_uint(), input.into())
        }

        #[test]
        fn i32_to_ibig(input: i32) {
            let val = BitVecValue::from(input, 32);
            prop_assert_eq!(val.to_big_int(), input.into())
        }
          #[test]
        fn ibig_to_ibig_neg(input in -350_i32..350_i32) {
            dbg!(input);
            let val = BitVecValue::from(BigInt::from(input), 32);
            prop_assert_eq!(val.to_big_int(), BigInt::from(input))
        }

        #[test]
        fn ubig_roundtrip(input: u128, mul: u128) {
            let in_big: BigUint = input.into();
            let mul_big: BigUint = mul.into();
            let target: BigUint = in_big * mul_big;
            let val = BitVecValue::from(target.clone(), target.bits());
            prop_assert_eq!(val.to_big_uint(), target)
        }

        #[test]
        fn ibig_roundtrip(input: i128, mul: i128) {
            let in_big: BigInt = input.into();
            let mul_big: BigInt = mul.into();
            let target: BigInt = in_big * mul_big;
            let val = BitVecValue::from(target.clone(), target.magnitude().bits() + 1);
            println!("{}", val);
            prop_assert_eq!(val.to_big_int(), target)
        }
    }
}
