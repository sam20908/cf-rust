pub fn first_true<F: Fn(i64) -> bool>(mut lo: i64, mut hi: i64, f: F) -> i64 {
    hi += 1;
    while lo < hi {
        let mid = lo + ((hi - lo) >> 1);
        if f(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

pub fn last_true<F: Fn(i64) -> bool>(mut lo: i64, mut hi: i64, f: F) -> i64 {
    lo -= 1;
    while lo < hi {
        let mid = lo + ((hi - lo + 1) >> 1);
        if f(mid) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo
}
