use oatsplonk::field::*;
use oatsplonk::extension::*;
use oatsplonk::curve::*;
use oatsplonk::srs::*;

// elliptic curve equation y^2 = x^3 + 3
// generator = (1,2)

// // Generator point G1 = (1, 2)
// const G1_X: i64 = 1;
// const G1_Y: i64 = 2;



fn main() {
    let srs = SRS::<101>::setup(4); //4 being the # of gates we'll be using
    
    println!("Generated SRS with {} G1 elements and {} G2 elements", 
        srs.g1_elements.len(),
        srs.g2_elements.len()
    );

    println!("\nG1 elements:");
    for (i, point) in srs.g1_elements.iter().enumerate() {
        println!("G1_{}: x={}, y={}", i, point.x.value, point.y.value);
    }

    println!("\nG2 elements:");
    for (i, point) in srs.g2_elements.iter().enumerate() {
        println!("G2_{}: x=[{}, {}], y=[{}, {}]", 
            i,
            point.x.coeffs[0].value, point.x.coeffs[1].value,
            point.y.coeffs[0].value, point.y.coeffs[1].value
        );
    }
}



