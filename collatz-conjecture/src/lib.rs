pub fn collatz(n: u64) -> Option<u64> {
    let mut res = n;
    let mut counter: u64 = 0;

    if n == 1 {
        Some(0)
    } else if res == 0 {
        None
    } else {
        while res != 1 {
            if res % 2 == 0 {
                res = res / 2;
            } else {
                res  = (3 * res) + 1;
            }

            counter += 1;
        }
        
        Some(counter)
    }
}
