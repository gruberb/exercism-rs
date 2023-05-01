pub fn private_key(p: u64) -> u64 {
    p/2
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(p, g, a)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(p, b_pub, a)
}

fn modular_exponentiation(modulo: u64, base: u64, exp: u64) -> u64 {
    let mut exponent = exp;
    let mut base = base;
    let mut result = 1;

    while exponent > 0 {
        // bitwise op
        if exponent & 1 != 0 {
            result = (result * base) % modulo;
        }
        // shifting op - shifts everything by one bit
        exponent >>= 1;
        base = (base * base) % modulo;
    }
    result
}