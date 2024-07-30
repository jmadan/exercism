pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut divisors = 2..;
    while n > 1 {
        let divisor = divisors.next().unwrap();
        while n % divisor == 0 {
            n /= divisor;
            factors.push(divisor);
        }
    }
    factors
}
