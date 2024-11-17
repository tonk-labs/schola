use crate::field::FieldElement;
use crate::points::{G2Point, Point};
use num_bigint::BigInt;
use num_traits::{One, Zero};

#[derive(Debug, Clone)]
pub enum SrsPoint {
    G1(Point),
    G2(G2Point),
}

pub fn setup_srs() -> Vec<SrsPoint> {
    // Initialize G1 generator in F_101
    let g1 = Point::new(BigInt::from(1), BigInt::from(2));

    // Initialize structure reference string
    let mut srs: Vec<SrsPoint> = Vec::new();

    let nr_gates = 4;
    let subgroup_order = BigInt::from(17);

    // $1 \times G_1$
    srs.push(SrsPoint::G1(g1.clone()));

    // $S \times G_1$, $S^2 \times G_1$, $S^3 \times G_1$, $S^4 \times G_1$, $S^5 \times G_1$, $S^6 \times G_1$
    let s = BigInt::from(2); // Simplification to follow Plonk by Hand tutorial
    for i in 1..=nr_gates + 2 {
        let s_power = s.modpow(&BigInt::from(i), &subgroup_order);
        let g1_times_s_power = g1.clone().scale(&s_power);
        srs.push(SrsPoint::G1(g1_times_s_power));
    }

    // Initialize G2 generator in extension field F_{101^2}
    let g2 = G2Point::new(
        FieldElement::new(BigInt::from(36), BigInt::zero()),
        FieldElement::new(BigInt::zero(), BigInt::from(31)),
    );

    // $1 \times G_2$
    srs.push(SrsPoint::G2(g2.clone()));

    // $S \times G_2$
    let s_power = s.modpow(&BigInt::from(1), &subgroup_order);
    let g2_times_s_power = g2.clone().scale(&s_power);
    srs.push(SrsPoint::G2(g2_times_s_power));

    srs
}
