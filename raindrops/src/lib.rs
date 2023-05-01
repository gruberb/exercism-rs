use std::collections::HashMap;

pub fn raindrops(n: u32) -> String {
    let mut nums = HashMap::new();
    nums.insert(3, "Pling");
    nums.insert(5, "Plang");
    nums.insert(7, "Plong");

    let mut factors = Vec::new();
    
    for i in 1..n+1 {
        if n % i == 0 {
            factors.push(i);
        }
    }
    let result: String = compose_string(nums, factors);

    if result.len() == 0 {
        return n.to_string()
    }

    result
}

fn compose_string(nums: HashMap<u32, &str>, factors: Vec<u32>) -> String {
    let mut result = String::from("");

    for num in factors {
        match nums.get(&num) {
            Some(&str) => {
                result.push_str(str);   
            },
            _ => println!("Not found"),
        }
    }

    
    String::from(result)
}

