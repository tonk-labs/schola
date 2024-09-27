
/**
 * Note: This code is an educational implementation provided by goblin. Parts were generated entirely by o1. 
 * This is not secure.
 * It is intended for educational purposes only and should not be used in production.
 */

/// AES S-box for SubBytes step
const S_BOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

/// AES Inverse S-box for InvSubBytes step
const INV_S_BOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

/// Round constants for key expansion
const RCON: [u8; 11] = [
    0x00, // Rcon[0] is not used
    0x01, 0x02, 0x04, 0x08,
    0x10, 0x20, 0x40, 0x80,
    0x1B, 0x36,
];

const BLOCK_SIZE: usize = 16;
const NUM_ROUNDS: usize = 10;

fn inv_sub_bytes(state: &mut [u8]) {
    for i in 0..BLOCK_SIZE {
        state[i] = INV_S_BOX[state[i] as usize];
    }
}

fn sub_bytes(state: &mut [u8]) {
    for i in 0..BLOCK_SIZE {
        state[i] = S_BOX[state[i] as usize];
    }
}

fn mix_columns(state: &mut [u8]) {
    for i in 0..4 {
        let col = &mut state[i * 4..(i + 1) * 4];
        let temp = col.to_vec();

        col[0] = gmul(temp[0], 2) ^ gmul(temp[1], 3) ^ temp[2] ^ temp[3];
        col[1] = temp[0] ^ gmul(temp[1], 2) ^ gmul(temp[2], 3) ^ temp[3];
        col[2] = temp[0] ^ temp[1] ^ gmul(temp[2], 2) ^ gmul(temp[3], 3);
        col[3] = gmul(temp[0], 3) ^ temp[1] ^ temp[2] ^ gmul(temp[3], 2);
    }
}

fn inv_mix_columns(state: &mut [u8]) {
    for i in 0..4 {
        let col = &mut state[i * 4..(i + 1) * 4];
        let temp = col.to_vec();

        col[0] = gmul(temp[0], 14) ^ gmul(temp[1], 11) ^ gmul(temp[2], 13) ^ gmul(temp[3], 9);
        col[1] = gmul(temp[0], 9) ^ gmul(temp[1], 14) ^ gmul(temp[2], 11) ^ gmul(temp[3], 13);
        col[2] = gmul(temp[0], 13) ^ gmul(temp[1], 9) ^ gmul(temp[2], 14) ^ gmul(temp[3], 11);
        col[3] = gmul(temp[0], 11) ^ gmul(temp[1], 13) ^ gmul(temp[2], 9) ^ gmul(temp[3], 14);
    }
}

fn gmul(a: u8, b: u8) -> u8 {
    let mut p = 0u8;
    let mut a = a;
    let mut b = b;
    for _ in 0..8 {
        if b & 1 != 0 {
            p ^= a;
        }
        let high_bit = a & 0x80;
        a <<= 1;
        if high_bit != 0 {
            a ^= 0x1B; // AES modulo polynomial
        }
        b >>= 1;
    }
    p
}

fn inv_shift_rows(state: &mut [u8]) {
    let mut temp = [0u8; BLOCK_SIZE];
    temp.copy_from_slice(state);

    // Row 0: no shift
    // Row 1: shift right by 1
    state[1] = temp[13];
    state[5] = temp[1];
    state[9] = temp[5];
    state[13] = temp[9];

    // Row 2: shift right by 2
    state[2] = temp[10];
    state[6] = temp[14];
    state[10] = temp[2];
    state[14] = temp[6];

    // Row 3: shift right by 3 (or left by 1)
    state[3] = temp[7];
    state[7] = temp[11];
    state[11] = temp[15];
    state[15] = temp[3];
}

fn shift_rows(state: &mut [u8]) {
    let mut temp = [0u8; BLOCK_SIZE];
    temp.copy_from_slice(state);

    // Row 0: no shift
    // Row 1: shift left by 1
    state[1] = temp[5];
    state[5] = temp[9];
    state[9] = temp[13];
    state[13] = temp[1];

    // Row 2: shift left by 2
    state[2] = temp[10];
    state[6] = temp[14];
    state[10] = temp[2];
    state[14] = temp[6];

    // Row 3: shift left by 3 (or right by 1)
    state[3] = temp[15];
    state[7] = temp[3];
    state[11] = temp[7];
    state[15] = temp[11];
}

fn key_expansion(key: &[u8]) -> Vec<u8> {
    let key_words = 4; // For AES-128
    let total_words = (NUM_ROUNDS + 1) * key_words;
    let mut w = vec![0u8; total_words * 4];

    // Copy the initial key into the first key_words of w
    w[..16].copy_from_slice(key);

    for i in key_words..total_words {
        let mut temp = [
            w[(i - 1) * 4],
            w[(i - 1) * 4 + 1],
            w[(i - 1) * 4 + 2],
            w[(i - 1) * 4 + 3],
        ];

        if i % key_words == 0 {
            // Rotate word
            temp = [temp[1], temp[2], temp[3], temp[0]];
            // Substitute bytes using S-box
            for j in 0..4 {
                temp[j] = S_BOX[temp[j] as usize];
            }
            // XOR with round constant
            temp[0] ^= RCON[i / key_words];
        }

        // XOR with the word key_words positions earlier
        for j in 0..4 {
            w[i * 4 + j] = w[(i - key_words) * 4 + j] ^ temp[j];
        }
    }

    w
}

fn add_round_key(state: &mut [u8], round_key: &[u8]) {
    for i in 0..16 {
        state[i] ^= round_key[i];
    }
}

fn aes_inv_round(state: &mut [u8], round_key: &[u8]) {
    inv_sub_bytes(state);
    inv_shift_rows(state);
    add_round_key(state, round_key);
    inv_mix_columns(state);
}

fn aes_round(state: &mut [u8], round_key: &[u8]) {
    sub_bytes(state);
    shift_rows(state);
    mix_columns(state);
    add_round_key(state, round_key);
}

fn encrypt_block(block: &[u8], key_schedule: &[u8]) -> Vec<u8> {
    let mut state = [0u8; 16];
    state.copy_from_slice(block);

    add_round_key(&mut state, &key_schedule[..BLOCK_SIZE]);

    for round in 1..NUM_ROUNDS {
        aes_round(&mut state, &key_schedule[round * BLOCK_SIZE..(round + 1) * BLOCK_SIZE]);
    }

    sub_bytes(&mut state);
    shift_rows(&mut state);
    add_round_key(&mut state, &key_schedule[NUM_ROUNDS * BLOCK_SIZE..(NUM_ROUNDS + 1) * BLOCK_SIZE]);

    state.to_vec()
}


fn decrypt_block(block: &[u8], key_schedule: &[u8]) -> Vec<u8> {
    let mut state = [0u8; 16];
    state.copy_from_slice(block);

    add_round_key(&mut state, &key_schedule[NUM_ROUNDS * BLOCK_SIZE..(NUM_ROUNDS + 1) * BLOCK_SIZE]);

    for round in (1..NUM_ROUNDS).rev() {
        aes_inv_round(&mut state, &key_schedule[round * BLOCK_SIZE..(round + 1) * BLOCK_SIZE]);
    }

    inv_shift_rows(&mut state);
    inv_sub_bytes(&mut state);
    add_round_key(&mut state, &key_schedule[..BLOCK_SIZE]);

    state.to_vec()
}

pub struct AES {
    pub key: [u8; 16],
    expanded_key: Vec<u8>,
}

impl AES {
    pub fn new(key: [u8; 16]) -> Self {
        let expanded_key = key_expansion(&key);
        AES { key, expanded_key }
    }
}

pub trait Encryptable {
    fn encrypt(&self, plaintext: &[u8]) -> Vec<u8>;
    fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8>;
}

use rand::Rng;

// Add this struct to represent the CTR mode
pub struct AES_CTR {
    aes: AES,
    nonce: [u8; 8],
}

impl AES_CTR {
    pub fn new(key: [u8; 16]) -> Self {
        let mut rng = rand::thread_rng();
        let nonce: [u8; 8] = rng.gen();
        let aes = AES::new(key);
        AES_CTR { aes, nonce }
    }
}

impl Encryptable for AES_CTR {
    fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        let mut ciphertext = Vec::with_capacity(plaintext.len());
        let mut counter = 0u64;

        for chunk in plaintext.chunks(BLOCK_SIZE) {
            let mut counter_block = [0u8; BLOCK_SIZE];
            counter_block[..8].copy_from_slice(&self.nonce);
            counter_block[8..].copy_from_slice(&counter.to_be_bytes());

            let keystream = encrypt_block(&counter_block, &self.aes.expanded_key);
            
            for (i, &byte) in chunk.iter().enumerate() {
                ciphertext.push(byte ^ keystream[i]);
            }

            counter += 1;
        }

        // Prepend the nonce to the ciphertext
        let mut result = Vec::with_capacity(self.nonce.len() + ciphertext.len());
        result.extend_from_slice(&self.nonce);
        result.extend_from_slice(&ciphertext);
        result
    }

    fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
        if ciphertext.len() <= 8 {
            return Vec::new(); // Not enough data to decrypt
        }

        let nonce = &ciphertext[..8];
        let ciphertext = &ciphertext[8..];
        let mut plaintext = Vec::with_capacity(ciphertext.len());
        let mut counter = 0u64;

        for chunk in ciphertext.chunks(BLOCK_SIZE) {
            let mut counter_block = [0u8; BLOCK_SIZE];
            counter_block[..8].copy_from_slice(nonce);
            counter_block[8..].copy_from_slice(&counter.to_be_bytes());

            let keystream = encrypt_block(&counter_block, &self.aes.expanded_key);
            
            for (i, &byte) in chunk.iter().enumerate() {
                plaintext.push(byte ^ keystream[i]);
            }

            counter += 1;
        }

        plaintext
    }
}
