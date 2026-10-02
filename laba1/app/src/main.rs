use core_lib::{BigInt, add, cmp, convert_from_bigint_to_hex, convert_from_hex_to_bigint, mul};
use std::io;
use std::time::Instant;

fn main() {
    println!("Please enter A:");
    let mut input_a = String::new();
    io::stdin()
        .read_line(&mut input_a)
        .expect("Failed to read line");

    println!("Please enter B:");
    let mut input_b = String::new();
    io::stdin()
        .read_line(&mut input_b)
        .expect("Failed to read line");

    println!("Please enter C:");
    let mut input_c = String::new();
    io::stdin()
        .read_line(&mut input_c)
        .expect("Failed to read line");

    let a = convert_from_hex_to_bigint(input_a.trim());
    let b = convert_from_hex_to_bigint(input_b.trim());
    let c = convert_from_hex_to_bigint(input_c.trim());

    println!("\n--- Test 1: (a + b) * c == a * c + b * c ---");
    let a_plus_b = add(a, b);
    let left1 = mul(&a_plus_b, &c);

    let ac = mul(&a, &c);
    let bc = mul(&b, &c);
    let right1 = add(ac, bc);

    if cmp(&left1, &right1) == 0 {
        println!("Test 1 OK");
    } else {
        println!("Test 1 Failed");
        println!("Left: {}", convert_from_bigint_to_hex(left1));
        println!("Right: {}", convert_from_bigint_to_hex(right1));
    }

    println!("\n--- Test 2: n * a == a + a + ... + a (n = 100) ---");

    let n = convert_from_hex_to_bigint("64");
    let left2 = mul(&n, &a);

    let mut right2 = BigInt::zero();
    for _ in 0..100 {
        right2 = add(right2, a);
    }

    if cmp(&left2, &right2) == 0 {
        println!("Test 2 OK");
    } else {
        println!("Test 2 Failed");
    }

    println!("\n--- Time measurement (average over 1000 operations) ---");
    let iterations = 1000;

    let start_add = Instant::now();
    for _ in 0..iterations {
        let _res = add(a, b);
    }
    let duration_add = start_add.elapsed();
    println!("Addition: {:?}", duration_add / iterations);

    let start_mul = Instant::now();
    for _ in 0..iterations {
        let _res = mul(&a, &b);
    }
    let duration_mul = start_mul.elapsed();
    println!("Multiplication: {:?}", duration_mul / iterations);
}