//! 3.11 Usar un crate externo (rand)
//!
//! La dependencia ya está en conceptos/Cargo.toml. En un proyecto
//! nuevo se agrega con `cargo add rand`.

fn main() {
    // Número aleatorio entre 1 y 6 (inclusive).
    let dado: u32 = rand::random_range(1..=6);
    println!("Salió: {dado}");
}
