use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut tmp: HashSet<_> = HashSet::new();
    let mut sum = 0;

    for x in factors.into_iter() {
        tmp = tmp.union(&find_multiple(x, &limit)).cloned().collect();
    }

    for x in tmp.iter() {
        sum += x;
    }

    sum
}

fn find_multiple(number: &u32, until: &u32) -> HashSet<u32> {
    let mut multiples: HashSet<u32> = HashSet::new();
    
    if number > &0 {
        for x in 1..*until {
            if x % number == 0 {
                multiples.insert(x);
            }
        }
    }

    multiples
}
