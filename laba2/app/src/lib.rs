pub use core_lib::{BigInt, add, sub, mul, square, cmp, bit_length, div_mod, convert_from_hex_to_bigint, convert_from_bigint_to_hex};

pub fn is_zero(a: &BigInt) -> bool {
    a.words.iter().all(|&w| w == 0)
}

pub fn gcd(mut a: BigInt, mut b: BigInt) -> BigInt {
    while !is_zero(&b) {
        let (_, r) = div_mod(&a, &b);
        a = b;
        b = r;
    }
    a
}

pub fn lcm(a: &BigInt, b: &BigInt) -> BigInt {
    if is_zero(a) || is_zero(b) {
        return BigInt::zero();
    }
    let m = mul(a, b);
    let g = gcd(*a, *b);
    let (res, _) = div_mod(&m, &g);
    res
}

pub fn mod_add(a: &BigInt, b: &BigInt, n: &BigInt) -> BigInt {
    let sum = add(*a, *b);
    if cmp(&sum, n) >= 0 {
        sub(sum, *n)
    } else {
        sum
    }
}

pub fn mod_sub(a: &BigInt, b: &BigInt, n: &BigInt) -> BigInt {
    if cmp(a, b) >= 0 {
        let (_, r) = div_mod(&sub(*a, *b), n);
        r
    } else {
        let diff = sub(*b, *a);
        let (_, r) = div_mod(&diff, n);
        if is_zero(&r) {
            BigInt::zero()
        } else {
            sub(*n, r)
        }
    }
}

pub fn mod_mul(a: &BigInt, b: &BigInt, n: &BigInt) -> BigInt {
    let m = mul(a, b);
    let (_, r) = div_mod(&m, n);
    r
}

pub fn mod_square(a: &BigInt, n: &BigInt) -> BigInt {
    let s = square(a);
    let (_, r) = div_mod(&s, n);
    r
}

pub fn mod_power(a: &BigInt, d: &BigInt, n: &BigInt) -> BigInt {
    let mut c = BigInt::one();
    let mut current_a = *a;
    let m = bit_length(d);
    
    for i in 0..m {
        let word_idx = i / 32;
        let bit_idx = i % 32;
        let is_bit_set = (d.words[word_idx] & (1 << bit_idx)) != 0;
        
        if is_bit_set {
            c = mod_mul(&c, &current_a, n);
        }
        
        if i < m - 1 {
            current_a = mod_square(&current_a, n);
        }
    }
    c
}