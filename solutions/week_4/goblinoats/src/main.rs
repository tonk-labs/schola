use goblinoats::aes::Encryptable;
use goblinoats::aes;
use std::fs::File;
use std::io::Read;
use image::{ImageReader, ImageBuffer, Rgb, ImageFormat};
use rand::Rng;
use std::env;
use std::path::PathBuf;

fn load_image(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Get the project root directory
    let project_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    
    // Construct the full path to the image
    let full_path = project_root.join(path);
    println!("Attempting to load image from: {:?}", full_path);

    // Open the file
    let mut file = File::open(&full_path)?;
    
    // Read the file contents into a buffer
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    // Use the image crate to decode the image
    let img = ImageReader::new(std::io::Cursor::new(buffer))
        .with_guessed_format()?
        .decode()?;
    
    // Convert the image to RGB format and return as a vector of bytes
    Ok(img.to_rgb8().into_raw())
}


fn save_image(path: &str, data: &[u8], width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
    // Get the project root directory
    let project_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    
    // Construct the full path for saving the image
    let full_path = project_root.join(path);
    println!("Attempting to save image to: {:?}", full_path);

    // Create an ImageBuffer from the raw data
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, data.to_vec())
        .ok_or("Failed to create ImageBuffer")?;

    // Save the image as PNG
    img.save_with_format(full_path, ImageFormat::Png)?;

    Ok(())
}



fn main() {
    // run_reference();
    let mut rng = rand::thread_rng();
    let key: [u8; 16] = rng.gen();
    let plaintext = b"Hello, world!";
    let aes_ctr = aes::AES_CTR::new(key);
    let ciphertext = aes_ctr.encrypt(plaintext);
    let decrypted = aes_ctr.decrypt(&ciphertext);
    println!("{:?}", ciphertext);
    println!("{:?}", String::from_utf8_lossy(&decrypted));

    match load_image("data/goblin.png") {
        Ok(img) => {
            println!("Image loaded successfully, size: {} bytes", img.len());
            // Uncomment these lines when you're ready to encrypt and save
            let encrypted_img = aes_ctr.encrypt(&img);
            save_image("data/goblin-encrypted.png", &encrypted_img, 512, 512).expect("Failed to save encrypted image");
        },
        Err(e) => println!("Failed to load image: {}", e),
    }
}
