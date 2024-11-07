use super::field::*;
use super::extension::*;
use super::curve::*;

pub struct SRS<const P: usize> {
    pub g1_elements: Vec<ECPoint>,
    pub g2_elements: Vec<ECPointExtended>
}

impl<const P: usize> SRS<P> {
    //n being the number of gates
    pub fn setup(n: usize) -> Self {
        let size = n + 2 as usize;
        let g1 = ECPoint { x: BaseFieldElement::new(1), y: BaseFieldElement::new(2), infinity: false };
        let g2 = ECPointExtended { 
            x: BaseFieldExtension::new([BaseFieldElement::new(36), BaseFieldElement::new(0)]),
            y: BaseFieldExtension::new([BaseFieldElement::new(0), BaseFieldElement::new(31)]),
            infinity: false 
        };

        let mut g1_elements = Vec::with_capacity(size);
        let mut g2_elements = Vec::with_capacity(size);

        let tau = 2 as usize;
        let mut current_tau = tau;
        let mut current_g1 = g1;

        for _ in 0..size+1 {
            g1_elements.push(current_g1);
            current_g1 = g1 * current_tau;
            current_tau = current_tau * tau;
        }

        g2_elements.push(g2);
        g2_elements.push(g2.double());

        Self {
            g1_elements,
            g2_elements
        }
    }
}