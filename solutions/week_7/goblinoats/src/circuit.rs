#[derive(Copy, Clone)]
pub struct Circuit {
    pub qL: [i32; 4],
    pub qR: [i32; 4],
    pub q0: [i32; 4],
    pub qM: [i32; 4],
    pub qC: [i32; 4],
    pub a: [i32; 4],
    pub b: [i32; 4],
    pub c: [i32; 4],
    pub s_1: [i32; 4],
    pub s_2: [i32; 4],
    pub s_3: [i32; 4]
}

// This expresses the constraint a^2 + b^2 = c^2
// for the values a = 3, b = 4, c = 5
pub const PLONK_CIRCUIT: Circuit = Circuit {
    // selectors
    qL: [0, 0, 0, 1],
    qR: [0, 0, 0, 1],
    q0: [-1, -1, -1, -1],
    qM: [1, 1, 1, 0],
    qC: [0, 0, 0, 0],
    
    // input wires
    a: [3, 4, 5, 9],
    b: [3, 4, 5, 16],
    c: [9, 16, 25, 25],

    // copy constraints
    // these are generated from subgroups specified in proof.rs
    s_1: [2,8,15,3],
    s_2: [1,4,16,12],
    s_3: [13,9,5,14]
};