use aes::Aes128;
use aes::cipher::{
    BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

fn main() {
    // Example key and plaintext
    let key = b"0123456789abcdef"; // 16-byte key for AES-128
    let plaintext = b"Hello, AES world!"; // 17-byte plaintext

    // Create AES-128 cipher instance
    let cipher = Aes128::new(GenericArray::from_slice(key));

    // Pad the plaintext to a multiple of 16 bytes (AES block size)
    let padded_len = (plaintext.len() + 15) / 16 * 16;
    let mut padded_plaintext = vec![0u8; padded_len];
    padded_plaintext[..plaintext.len()].copy_from_slice(plaintext);

    // Encrypt
    let mut blocks: Vec<GenericArray<u8, _>> = padded_plaintext
        .chunks(16)
        .map(GenericArray::clone_from_slice)
        .collect();
    for block in &mut blocks {
        cipher.encrypt_block(block);
    }
    println!("Encrypted: {:?}", blocks);

    // Decrypt
    for block in &mut blocks {
        cipher.decrypt_block(block);
    }
    let decrypted: Vec<u8> = blocks.into_iter().flatten().collect();
    println!("Decrypted: {:?}", &decrypted[..plaintext.len()]);
    
    // Remove padding and convert to string
    let decrypted_text = String::from_utf8_lossy(&decrypted[..plaintext.len()]);
    println!("Decrypted text: {}", decrypted_text);
}
