pub fn factors(n: u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    let mut number = n;
    let mut i = 2;
    
    if n < 2 {
        res.clear()
    }
    
    while i <= n {
        if number % i == 0 {
            res.push(i);
            number = number / i;
            continue;
        }
        
        if number as f64 / i as f64 == 1.0 {
            res.push(i);
            break;
        }

        i += 1;


    }

    res
}

