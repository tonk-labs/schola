use goblinoats::rsa::*;
fn main() {
    let key = RsaPrivateKey::new(2048);
    println!("{:?}", key);
}
