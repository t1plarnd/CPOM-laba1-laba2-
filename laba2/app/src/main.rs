use app::{
    BigInt, add, sub, mul, square, div_mod, gcd, lcm, 
    mod_add, mod_sub, mod_mul, mod_square, mod_power, is_zero,
    convert_from_bigint_to_hex, convert_from_hex_to_bigint
};
use std::io;
use std::time::Instant;

fn main() {
    println!("Please enter a:");
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).expect("Failed to read line");

    println!("Please enter b:");
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).expect("Failed to read line");

    println!("Please enter n (modulus):");
    let mut input_n = String::new();
    io::stdin().read_line(&mut input_n).expect("Failed to read line");

    let a = convert_from_hex_to_bigint(input_a.trim());
    let b = convert_from_hex_to_bigint(input_b.trim());
    let n = convert_from_hex_to_bigint(input_n.trim());

    println!("\n--- 1. Base Operations ---");
    println!("A + B = {}", convert_from_bigint_to_hex(add(a, b)));
    println!("A - B = {}", convert_from_bigint_to_hex(sub(a, b)));
    println!("A * B = {}", convert_from_bigint_to_hex(mul(&a, &b)));
    println!("A^2   = {}", convert_from_bigint_to_hex(square(&a)));
    
    let (q, r) = div_mod(&a, &b);
    println!("A / B = {}", convert_from_bigint_to_hex(q));
    println!("A mod B = {}", convert_from_bigint_to_hex(r));

    println!("\n--- 2. GCD and LCM ---");
    println!("GCD(a, b) = {}", convert_from_bigint_to_hex(gcd(a, b)));
    println!("LCM(a, b) = {}", convert_from_bigint_to_hex(lcm(&a, &b)));

    println!("\n--- 3. Modular Arithmetic ---");
    println!("(A + B) mod n = {}", convert_from_bigint_to_hex(mod_add(&a, &b, &n)));
    println!("(A - B) mod n = {}", convert_from_bigint_to_hex(mod_sub(&a, &b, &n)));
    println!("(A * B) mod n = {}", convert_from_bigint_to_hex(mod_mul(&a, &b, &n)));
    println!("(A^2) mod n   = {}", convert_from_bigint_to_hex(mod_square(&a, &n)));

    let d_test = convert_from_hex_to_bigint("10");
    println!("(A^d) mod n   = {}", convert_from_bigint_to_hex(mod_power(&a, &d_test, &n)));

    println!("\n--- 4. Identities Control (Item B) ---");
    
    let c = convert_from_hex_to_bigint("1A2B3C4D");
    let a_plus_b = mod_add(&a, &b, &n);
    let left1 = mod_mul(&a_plus_b, &c, &n);
    let ac = mod_mul(&a, &c, &n);
    let bc = mod_mul(&b, &c, &n);
    let right1 = mod_add(&ac, &bc, &n);
    let diff1 = mod_sub(&left1, &right1, &n);
    
    if is_zero(&diff1) {
        println!("Identity 1: OK");
    } else {
        println!("Identity 1: FAILED");
    }

    let scalar = convert_from_hex_to_bigint("64");
    let left2 = mod_mul(&scalar, &a, &n);
    let mut right2 = BigInt::zero();
    for _ in 0..100 {
        right2 = mod_add(&right2, &a, &n);
    }
    let diff2 = mod_sub(&left2, &right2, &n);
    
    if is_zero(&diff2) {
        println!("Identity 2: OK");
    } else {
        println!("Identity 2: FAILED");
    }

    println!("\n--- 5. Time Measurement (Item V) ---");
    let iterations = 1000;

    let start_gcd = Instant::now();
    for _ in 0..iterations {
        let _ = gcd(a, b);
    }
    println!("GCD average time: {:?}", start_gcd.elapsed() / iterations);

    let start_mod_mul = Instant::now();
    for _ in 0..iterations {
        let _ = mod_mul(&a, &b, &n);
    }
    println!("Modular multiplication average time: {:?}", start_mod_mul.elapsed() / iterations);

    let start_mod_pow = Instant::now();
    for _ in 0..iterations {
        let _ = mod_power(&a, &d_test, &n);
    }
    println!("Modular exponentiation average time: {:?}", start_mod_pow.elapsed() / iterations);
}