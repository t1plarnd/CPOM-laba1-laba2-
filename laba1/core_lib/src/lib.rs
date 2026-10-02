const BITS: usize = 2048;
const WORDS: usize = BITS / 32;

#[derive(Debug, Clone, Copy)]
pub struct BigInt {
    pub words: [u32; WORDS],
}

impl BigInt {
    pub fn zero() -> Self {
        BigInt { words: [0; WORDS] }
    }

    pub fn one() -> Self {
        let mut ans = BigInt { words: [0; WORDS] };
        ans.words[0] = 1;
        ans
    }
}

pub fn add(a: BigInt, b: BigInt) -> BigInt {
    let mut ans = BigInt { words: [0; WORDS] };
    let mut carry: u64 = 0;

    for i in 0..WORDS {
        let temp = (a.words[i] as u64) + (b.words[i] as u64) + carry;
        ans.words[i] = (temp & 0xFFFFFFFF) as u32;

        carry = temp >> 32;
    }

    ans
}

pub fn convert_from_hex_to_bigint(s: &str) -> BigInt {
    let mut res = BigInt { words: [0; WORDS] };

    let mut current_str = s.to_string();
    let mut word_idx = 0;

    while !current_str.is_empty() && word_idx < WORDS {
        let len = current_str.len();

        let split_pos = if len >= 8 { len - 8 } else { 0 };

        let chunk = current_str.split_off(split_pos);

        res.words[word_idx] = u32::from_str_radix(&chunk, 16).unwrap();

        word_idx += 1;
    }

    res
}

pub fn convert_from_bigint_to_hex(b: BigInt) -> String {
    let mut res = String::new();

    for i in (0..WORDS).rev() {
        let word = b.words[i];
        let chunk = format!("{:08X}", word);
        res.push_str(&chunk);
    }

    let trimmed = res.trim_start_matches('0');

    if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn sub(a: BigInt, b: BigInt) -> BigInt {
    let mut ans: BigInt = BigInt { words: [0; WORDS] };
    let mut borrow: i64 = 0;

    for i in 0..WORDS {
        let temp: i64 = (a.words[i] as i64) - (b.words[i] as i64) - borrow;

        if temp >= 0 {
            ans.words[i] = temp as u32;
            borrow = 0;
        } else {
            ans.words[i] = (0x1_0000_0000_i64 + temp) as u32;
            borrow = 1;
        }
    }

    ans
}

pub fn cmp(a: &BigInt, b: &BigInt) -> i32 {
    for i in (0..WORDS).rev() {
        if a.words[i] > b.words[i] {
            return 1;
        } else if a.words[i] < b.words[i] {
            return -1;
        }
    }
    0
}

pub fn mul_one_digit(a: &BigInt, b: u32) -> BigInt {
    let mut ans = BigInt::zero();
    let mut carry: u64 = 0;
    let b_64 = b as u64;

    for i in 0..WORDS {
        let temp: u64 = (a.words[i] as u64) * b_64 + carry;
        
        ans.words[i] = (temp & 0xFFFFFFFF) as u32;
        
        carry = temp >> 32;
    }
    
    ans
}

pub fn shift_left(a: &BigInt, shift: usize) -> BigInt {
    let mut ans = BigInt::zero();
    
    if shift >= BITS {
        return ans;
    }

    let word_shift = shift / 32; 
    let bit_shift = shift % 32;  

    if bit_shift == 0 {
        for i in word_shift..WORDS {
            ans.words[i] = a.words[i - word_shift];
        }
    } else {
        let mut carry: u32 = 0;
        for i in word_shift..WORDS {
            let temp = (a.words[i - word_shift] as u64) << bit_shift;
            
            ans.words[i] = (temp as u32) | carry;
            
            carry = (temp >> 32) as u32;
        }
    }

    ans
}

pub fn mul(a: &BigInt, b: &BigInt) -> BigInt {
    let mut c = BigInt::zero();

    for i in 0..WORDS {
        if b.words[i] == 0 {
            continue;
        }

        let temp = mul_one_digit(a, b.words[i]);

        let shifted_temp = shift_left(&temp, i * 32);

        c = add(c, shifted_temp);
    }

    c
}

pub fn square(a: &BigInt) -> BigInt {
    mul(a, a)
}

pub fn bit_length(a: &BigInt) -> usize {
    for i in (0..WORDS).rev() {
        if a.words[i] != 0 {
            let bit_len_in_word = 32 - a.words[i].leading_zeros() as usize;
            return i * 32 + bit_len_in_word;
        }
    }
    0
}

pub fn div_mod(a: &BigInt, b: &BigInt) -> (BigInt, BigInt) {
    let mut q = BigInt::zero();
    let mut r = *a;

    let k = bit_length(b);
    if k == 0 {
        panic!("Division by zero!");
    }

    while cmp(&r, b) >= 0 {
        let mut t = bit_length(&r);
        
        let mut shift_amount = t - k;
        let mut c = shift_left(b, shift_amount);

        if cmp(&r, &c) < 0 {
            t -= 1;
            shift_amount = t - k;
            c = shift_left(b, shift_amount);
        }

        r = sub(r, c);

        let word_idx = shift_amount / 32;
        let bit_idx = shift_amount % 32;
        q.words[word_idx] |= 1 << bit_idx;
    }

    (q, r)
}

pub fn power(a: &BigInt, b: &BigInt) -> BigInt {
    let mut c = BigInt::one();
    let mut current_a = *a;
    
    let m = bit_length(b);
    
    for i in 0..m {
        let word_idx = i / 32;
        let bit_idx = i % 32;
        let is_bit_set = (b.words[word_idx] & (1 << bit_idx)) != 0;
        
        if is_bit_set {
            c = mul(&c, &current_a);
        }
        
        if i < m - 1 {
            current_a = square(&current_a);
        }
    }
    
    c
}