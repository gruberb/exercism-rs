pub fn nth(n: u32) -> u32 {
    let mut prime_numbers = Vec::with_capacity(n as usize);
    let mut i = 1u32;
    
    while prime_numbers.len() <= n as usize {
        i+=1;
        if prime_numbers.iter().any(|&prime| i % prime == 0) {
            continue;
        }
        
        prime_numbers.push(i);
    }
    
    prime_numbers.pop().unwrap()
}
