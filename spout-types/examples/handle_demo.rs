//! Example demonstrating handle usage

use spout_types::{Handle, Profile};

fn main() {
    // Create a new profile
    let profile = Profile::with_image(
        "Alice".to_string(),
        "https://example.com/avatar.jpg".to_string(),
    );

    // Create a new handle with the profile
    let handle = Handle::new(profile);

    println!("Created handle: {}", handle);
    println!("Handle ID: {}", handle.id());
    println!("Public key: {:?}", handle.public_key());
    println!("Can sign: {}", handle.can_sign());

    // Sign some data
    let message = b"Hello, Spout Social!";
    let signature = handle.sign(message).expect("Failed to sign message");

    println!("Signed message: {:?}", String::from_utf8_lossy(message));
    println!("Signature: {:?}", signature);

    // Verify the signature
    match handle.verify(message, &signature) {
        Ok(()) => println!("✓ Signature verified successfully!"),
        Err(e) => println!("✗ Signature verification failed: {}", e),
    }

    // Create a verification-only handle (no private key)
    let verify_handle =
        Handle::from_public_key(handle.public_key().clone(), handle.profile().clone());

    println!("\nCreated verification handle: {}", verify_handle);
    println!("Can sign: {}", verify_handle.can_sign());

    // Verify with the verification-only handle
    match verify_handle.verify(message, &signature) {
        Ok(()) => println!("✓ Verification handle can verify signatures!"),
        Err(e) => println!("✗ Verification failed: {}", e),
    }

    // Show profile information
    println!("\nProfile information:");
    println!("Name: {}", handle.profile().name());
    println!("Image: {:?}", handle.profile().image());
}
