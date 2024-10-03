use num_bigint::{BigUint, RandBigInt};
use num_traits::{One, Zero};
use rand::thread_rng;

// Public key for ElGamal encryption
pub struct PublicKey {
    pub p: BigUint, // Large prime number
    pub g: BigUint, // Generator of the multiplicative group modulo p
    pub h: BigUint, // h = g^x mod p, where x is the private key
}

// Private key for ElGamal encryption
pub struct PrivateKey {
    pub x: BigUint, // Secret exponent
}

// Generate ElGamal key pair
pub fn generate_keys() -> (PublicKey, PrivateKey) {
    let mut rng = thread_rng();

    // Use a known large prime p
    // This is a 2048-bit MODP Group from RFC 3526
    let p = BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B139B22514A08798E3404DDEF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7EDEE386BFB5A899FA5AE9F24117C4B1FE649286651ECE45B3DC2007CB8A163BF0598DA48361C55D39A69163FA8FD24CF5F83655D23DCA3AD961C62F356208552BB9ED529077096966D670C354E4ABC9804F1746C08CA18217C32905E462E36CE3BE39E772C180E86039B2783A2EC07A28FB5C55DF06F4C52C9DE2BCBF6955817183995497CEA956AE515D2261898FA051015728E5A8AAAC42DAD33170D04507A33A85521ABDF1CBA64ECFB850458DBEF0A8AEA71575D060C7DB3970F85A6E1E4C7ABF5AE8CDB0933D71E8C94E04A25619DCEE3D2261AD2EE6BF12FFA06D98A0864D87602733EC86A64521F2B18177B200CBBE117577A615D6C770988C0BAD946E208E24FA074E5AB3143DB5BFCE0FD108E4B82D120A93AD2CAFFFFFFFFFFFFFFFF", 16).unwrap();

    // Choose a generator g of the multiplicative group modulo p
    // For this prime, 2 is a valid generator
    let g = BigUint::from(2u32);

    // Generate private key x
    let x = rng.gen_biguint_below(&(&p - 1u32));

    // Compute h = g^x mod p
    let h = g.modpow(&x, &p);

    (PublicKey { p, g, h }, PrivateKey { x })
}

// ElGamal encryption
pub fn encrypt(message: &BigUint, public_key: &PublicKey) -> (BigUint, BigUint) {
    let mut rng = thread_rng();
    let r = rng.gen_biguint_below(&(&public_key.p - 1u32));

    // Compute c1 = g^r mod p
    let c1 = public_key.g.modpow(&r, &public_key.p);

    // Compute c2 = m * h^r mod p
    let c2 = (message * public_key.h.modpow(&r, &public_key.p)) % &public_key.p;

    (c1, c2)
}

// ElGamal decryption
pub fn decrypt(
    ciphertext: &(BigUint, BigUint),
    private_key: &PrivateKey,
    public_key: &PublicKey,
) -> BigUint {
    let (c1, c2) = ciphertext;

    // Compute s = c1^x mod p
    let s = c1.modpow(&private_key.x, &public_key.p);

    // Compute m = c2 * s^(-1) mod p
    let s_inv = s.modpow(&(&public_key.p - 2u32), &public_key.p); // Fermat's little theorem for modular inverse
    (c2 * s_inv) % &public_key.p
}

// Structures for OT protocol
#[derive(Debug)]
pub struct EncryptedMessage(BigUint, BigUint);
#[derive(Debug)]
pub struct ReceiverRequest(BigUint);
#[derive(Debug)]
pub struct SenderData(EncryptedMessage, EncryptedMessage);

// Sender prepares two messages for OT
pub fn prepare_ot_messages(
    m0: &BigUint,
    m1: &BigUint,
    public_key: &PublicKey,
) -> (EncryptedMessage, EncryptedMessage) {
    let e0 = encrypt(m0, public_key);
    let e1 = encrypt(m1, public_key);
    (EncryptedMessage(e0.0, e0.1), EncryptedMessage(e1.0, e1.1))
}

// Receiver chooses which message to receive
pub fn choose_message(choice_bit: bool, public_key: &PublicKey) -> ReceiverRequest {
    let mut rng = thread_rng();
    let k = rng.gen_biguint_below(&(&public_key.p - 1u32));

    // Compute g^k mod p
    let gk = public_key.g.modpow(&k, &public_key.p);

    // If choice_bit is true, multiply by h
    let pk = if choice_bit {
        (gk * &public_key.h) % &public_key.p
    } else {
        gk
    };

    ReceiverRequest(pk)
}

// Sender executes OT based on receiver's request
pub fn execute_ot(
    sender_data: &SenderData,
    receiver_request: &ReceiverRequest,
    public_key: &PublicKey,
) -> EncryptedMessage {
    let SenderData(EncryptedMessage(c0, d0), EncryptedMessage(c1, d1)) = sender_data;
    let ReceiverRequest(pk) = receiver_request;

    // Reencrypt the messages
    let e0 = (c0 * pk.modpow(&BigUint::one(), &public_key.p)) % &public_key.p;
    let e1 = (c1
        * (&public_key.h
            * pk.modpow(&BigUint::one(), &public_key.p)
                .modpow(&BigUint::from(public_key.p.clone() - 2u32), &public_key.p)))
        % &public_key.p;

    EncryptedMessage(e0, e1)
}

// Example usage
fn main() {
    // Generate ElGamal key pair
    let (public_key, private_key) = generate_keys();

    println!("Public key p (prime): {}", public_key.p);
    println!("Public key g (generator): {}", public_key.g);
    println!("Public key h (g^x mod p): {}", public_key.h);
    println!("Private key x: {}", private_key.x);

    // Sender's messages
    let m0 = BigUint::from(42u32); // Arbitrary number 42
    let m1 = BigUint::from(73u32); // Arbitrary number 73
    println!("First message (m0): {}", m0);
    println!("Second message (m1): {}", m1);

    // Sender prepares OT messages
    let sender_data = prepare_ot_messages(&m0, &m1, &public_key);
    println!("Sender data (c0, d0): {:?}", sender_data.0);
    println!("Sender data (c1, d1): {:?}", sender_data.1);

    // Receiver chooses a message (e.g., the second one)
    let choice_bit = true;
    println!(
        "Choice bit: {} (false = first message, true = second message)",
        choice_bit
    );
    let receiver_request = choose_message(choice_bit, &public_key);
    println!("Receiver request: {:?}", receiver_request);

    // Sender executes OT
    let sender_data = SenderData(sender_data.0, sender_data.1);
    println!("Sender data: {:?}", sender_data);
    let encrypted_result = execute_ot(&sender_data, &receiver_request, &public_key);
    println!("Encrypted result: {:?}", encrypted_result);

    // Receiver decrypts the result
    let decrypted_result = decrypt(
        &(encrypted_result.0, encrypted_result.1),
        &private_key,
        &public_key,
    );

    println!("Decrypted result: {}", decrypted_result);
    println!("Expected result: {}", if choice_bit { m1 } else { m0 });
}
