pub fn mod_pow(mut a: i64, mut b: i64, m: i64) -> i64 {
    a %= m;
    let mut x = 1;
    while b > 0 {
        if (b & 1) > 0 {
            x = x * a % m;
        }
        a = a * a % m;
        b >>= 1;
    }
    x
}

pub fn ceil_div(a: i64, b: i64) -> i64 {
    let mut res = a / b;
    if b * res != a {
        res += ((a > 0) & (b > 0)) as i64;
    }
    res
}
