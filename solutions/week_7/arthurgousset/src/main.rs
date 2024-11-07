use num_bigint::BigInt;
use week_7_arthurgousset::{srs::setup_srs, mod_prime};

fn main() {
    let srs = setup_srs();
    println!("\nStructured Reference String (SRS):");
    for (i, element) in srs.iter().enumerate() {
        println!("[{}]: {:?}", i, element);
    }

    // We happen to know that the 4th roots of unity in F_{17} are 1, 4, 13, 16.
    let H = vec![
        BigInt::from(1),
        BigInt::from(4),
        BigInt::from(16),
        BigInt::from(13),
    ];

    // Define cosets using K_1 = 2, and K_2 = 3
    let k1 = BigInt::from(2);
    let k2 = BigInt::from(3);
    let modulus = BigInt::from(17);

    let K1H = H
        .iter()
        .map(|h| mod_prime(&(h * &k1), &modulus))
        .collect::<Vec<BigInt>>();
    let K2H = H
        .iter()
        .map(|h| mod_prime(&(h * &k2), &modulus))
        .collect::<Vec<BigInt>>();

    println!("\nK1H: {:?}", K1H);
    println!("K2H: {:?}", K2H);
}
