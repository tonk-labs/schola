use aes::cipher::{generic_array::GenericArray, BlockCipher, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;

// AES state is represented as a 4x4 matrix of bytes
type State = [[u8; 4]; 4];

fn main() {
    println!("AES Round Functions Implementation");

    // AES-128 uses a 128-bit key (16 bytes)
    let key = [0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
    let plaintext = "Hello, World!";
    
    println!("Plaintext: {}", plaintext);
    println!("Key: {:02x?}", key);
    
    let encrypted = encrypt_string(plaintext, &key);
    println!("Encrypted: {:02x?}", encrypted);
}

// SubBytes: Substitute each byte using the AES S-box
// This provides non-linearity to the cipher and helps resist differential and linear cryptanalysis
fn sub_bytes(state: &mut State) {
    for row in state.iter_mut() {
        for byte in row.iter_mut() {
            // We're using the AES crate's implementation of the S-box for simplicity
            // In a full implementation, you'd have a 256-byte lookup table
            let mut block = GenericArray::from([0u8; 16]);
            block[0] = *byte;
            Aes128::new(&GenericArray::from([0u8; 16])).encrypt_block(&mut block);
            *byte = block[0];
        }
    }
}

// ShiftRows: Cyclically shift the rows of the state
// This provides diffusion by ensuring that the bytes from each column are spread out
fn shift_rows(state: &mut State) {
    let mut temp = [0u8; 4];
    for i in 1..4 {
        temp.copy_from_slice(&state[i]);
        for j in 0..4 {
            // Row 1 shifts by 1, row 2 by 2, row 3 by 3
            state[i][j] = temp[(j + i) % 4];
        }
    }
}

// MixColumns: Mix the columns of the state
// This provides diffusion at the byte level and ensures that after a complete round,
// changing one byte affects all bytes in the state
fn mix_columns(state: &mut State) {
    for i in 0..4 {
        let a = state[0][i];
        let b = state[1][i];
        let c = state[2][i];
        let d = state[3][i];

        // These operations are matrix multiplications in GF(2^8)
        // The coefficients (2, 3, 1, 1) form an MDS matrix
        state[0][i] = gf_mul(a, 2) ^ gf_mul(b, 3) ^ c ^ d;
        state[1][i] = a ^ gf_mul(b, 2) ^ gf_mul(c, 3) ^ d;
        state[2][i] = a ^ b ^ gf_mul(c, 2) ^ gf_mul(d, 3);
        state[3][i] = gf_mul(a, 3) ^ b ^ c ^ gf_mul(d, 2);
    }
}

// AddRoundKey: XOR the state with the round key
// This is the only step that directly uses the key and provides confusion by mixing in key material
fn add_round_key(state: &mut State, cipher: &Aes128, round: usize) {
    let mut block = GenericArray::from([0u8; 16]);
    for i in 0..4 {
        for j in 0..4 {
            block[i * 4 + j] = state[i][j];
        }
    }

    // Note: This is a simplification. In a real implementation, you'd derive the round key here.
    // Each round should use a different round key derived from the original key.
    cipher.encrypt_block(&mut block);

    for i in 0..4 {
        for j in 0..4 {
            state[i][j] ^= block[i * 4 + j];
        }
    }
}

// Helper function for Galois Field multiplication
// This is used in the MixColumns step
fn gf_mul(a: u8, b: u8) -> u8 {
    let mut p = 0;
    let mut high_bit_set;
    let mut a = a;
    let mut b = b;
    for _ in 0..8 {
        if b & 1 == 1 {
            p ^= a;
        }
        high_bit_set = a & 0x80;
        a <<= 1;
        if high_bit_set == 0x80 {
            a ^= 0x1b; // The irreducible polynomial x^8 + x^4 + x^3 + x + 1
        }
        b >>= 1;
    }
    p
}

// Helper function to print the state
fn print_state(state: &State) {
    for row in state.iter() {
        for &byte in row.iter() {
            print!("{:02x} ", byte);
        }
        println!();
    }
    println!();
}

// Function to encrypt a string using AES
fn encrypt_string(input: &str, key: &[u8; 16]) -> Vec<u8> {
    let cipher = Aes128::new(&GenericArray::from(*key));
    let mut padded_input = input.as_bytes().to_vec();
    
    // Pad the input to be a multiple of 16 bytes (128 bits, the AES block size)
    // Note: This is a simple padding scheme. PKCS#7 is more commonly used in practice.
    while padded_input.len() % 16 != 0 {
        padded_input.push(0);
    }
    
    println!("Padded input: {:02x?}", padded_input);
    
    let mut encrypted = Vec::new();
    // Process each 16-byte block
    for (block_num, chunk) in padded_input.chunks(16).enumerate() {
        println!("\nProcessing block {}", block_num + 1);
        
        let mut block = [0u8; 16];
        block.copy_from_slice(chunk);
        println!("Initial block: {:02x?}", block);
        
        // Convert block to state (4x4 matrix of bytes)
        let mut state = [[0u8; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                state[i][j] = block[i * 4 + j];
            }
        }
        
        println!("Initial state:");
        print_state(&state);
        
        // AES-128 performs 10 rounds of encryption
        for round in 0..10 {
            println!("Round {}", round + 1);
            
            // 1. SubBytes: Substitute each byte using the S-box
            sub_bytes(&mut state);
            println!("After SubBytes:");
            print_state(&state);
            
            // 2. ShiftRows: Cyclically shift the rows
            shift_rows(&mut state);
            println!("After ShiftRows:");
            print_state(&state);
            
            // 3. MixColumns: Mix the columns (skipped in the final round)
            if round < 9 {
                mix_columns(&mut state);
                println!("After MixColumns:");
                print_state(&state);
            }
            
            // 4. AddRoundKey: XOR with the round key
            add_round_key(&mut state, &cipher, round);
            println!("After AddRoundKey:");
            print_state(&state);
        }
        
        // Convert state back to block
        for i in 0..4 {
            for j in 0..4 {
                block[i * 4 + j] = state[i][j];
            }
        }
        
        println!("Encrypted block: {:02x?}", block);
        encrypted.extend_from_slice(&block);
    }
    
    encrypted
}
