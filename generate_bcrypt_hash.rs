use bcrypt::{hash, DEFAULT_COST};

fn main() {
    let password = "admin";
    match hash(password, DEFAULT_COST) {
        Ok(hashed) => println!("BCrypt hash for '{}': {}", password, hashed),
        Err(err) => eprintln!("Error generating hash: {}", err),
    }
}