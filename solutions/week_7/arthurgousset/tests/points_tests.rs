use week_7_arthurgousset::points::*;
use week_7_arthurgousset::field::FieldElement;
use num_bigint::BigInt;
use num_traits::{One, Zero};

#[test]
fn test_g1_point_operations() {
    let g1 = Point::new(BigInt::from(1), BigInt::from(2));
    
    // Test doubling
    let doubled = g1.double();
    assert_eq!(doubled, Point::new(BigInt::from(68), BigInt::from(74)));

    // Test inversion
    let inverted = g1.invert();
    assert_eq!(inverted, Point::new(BigInt::from(1), BigInt::from(99)));

    // Test addition
    let added = g1.add(&doubled);
    assert_eq!(added, Point::new(BigInt::from(26), BigInt::from(45)));

    // Test scalar multiplication
    assert_eq!(g1.scale(&BigInt::from(2)), Point::new(BigInt::from(68), BigInt::from(74)));
    assert_eq!(g1.scale(&BigInt::from(3)), Point::new(BigInt::from(26), BigInt::from(45)));
}

#[test]
fn test_g1_subgroup() {
    let g1 = Point::new(BigInt::from(1), BigInt::from(2));
    let expected_points = vec![
        Point::new(BigInt::from(1), BigInt::from(2)),     // 1
        Point::new(BigInt::from(68), BigInt::from(74)),   // 2
        Point::new(BigInt::from(26), BigInt::from(45)),   // 3
        Point::new(BigInt::from(65), BigInt::from(98)),   // 4
        Point::new(BigInt::from(12), BigInt::from(32)),   // 5
        Point::new(BigInt::from(32), BigInt::from(42)),   // 6
        Point::new(BigInt::from(91), BigInt::from(35)),   // 7
        Point::new(BigInt::from(18), BigInt::from(49)),   // 8
        Point::new(BigInt::from(18), BigInt::from(52)),   // 9
        Point::new(BigInt::from(91), BigInt::from(66)),   // 10
        Point::new(BigInt::from(32), BigInt::from(59)),   // 11
        Point::new(BigInt::from(12), BigInt::from(69)),   // 12
        Point::new(BigInt::from(65), BigInt::from(3)),    // 13
        Point::new(BigInt::from(26), BigInt::from(56)),   // 14
        Point::new(BigInt::from(68), BigInt::from(27)),   // 15
        Point::new(BigInt::from(1), BigInt::from(99)),    // 16
    ];

    let mut current = g1.clone();
    assert_eq!(current, expected_points[0]);
    
    for i in 1..16 {
        current = current.add(&g1);
        assert_eq!(current, expected_points[i]);
    }
}

#[test]
fn test_g2_operations() {
    let g2 = G2Point::new(
        FieldElement::new(BigInt::from(36), BigInt::zero()),
        FieldElement::new(BigInt::zero(), BigInt::from(31)),
    );

    let s = BigInt::from(2);
    let scaled_g2 = g2.scale(&s);
    
    assert_eq!(
        scaled_g2,
        G2Point::new(
            FieldElement::new(BigInt::from(90), BigInt::zero()),
            FieldElement::new(BigInt::zero(), BigInt::from(82)),
        )
    );
} 