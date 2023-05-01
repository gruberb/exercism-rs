pub fn square_of_sum(n: u32) -> u32 {
    let vec: Vec<u32> = (0..n+1).collect();
    let sum: u32 = vec.iter().sum();

    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let vec: Vec<u32> = (0..n+1).collect();
    vec.iter().fold(0, |acc, x| acc + x.pow(2))
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
