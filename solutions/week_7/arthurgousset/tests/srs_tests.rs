use week_7_arthurgousset::{points::*, srs::*};
use num_bigint::BigInt;

#[test]
fn test_srs_generation() {
    let srs = setup_srs();
    let s = BigInt::from(2);  // Known secret from setup
    let subgroup_order = BigInt::from(17);

    // Check length
    assert_eq!(srs.len(), 9);

    // Check G1 elements (first 7 elements)
    if let SrsPoint::G1(g1) = &srs[0] {
        // Check each G1 element is s^i * G1
        for i in 1..=6 {
            if let SrsPoint::G1(point) = &srs[i] {
                let s_power = s.modpow(&BigInt::from(i), &subgroup_order);
                let expected = g1.scale(&s_power);
                assert_eq!(point, &expected);
            } else {
                panic!("Expected G1 point at position {}", i);
            }
        }
    }

    // Check G2 elements (last 2 elements)
    if let SrsPoint::G2(g2) = &srs[7] {
        if let SrsPoint::G2(point) = &srs[8] {
            let expected = g2.scale(&s);
            assert_eq!(point, &expected);
        } else {
            panic!("Expected G2 point at position 8");
        }
    }
} 