pub fn factorial(n: i64) -> i64 {
    let mut total = 1;
    for m in 1..=n {
        total *= m
    }
    total
}

pub fn combinations(n: i64, mut r: i64) -> i64 {
    r = r.max(n - r);
    let mut total = 1;

    for m in r + 1..n {
        total *= m
    }
    total /= factorial(n - r);

    total
}
