pub fn binary_search(haystack: &[i32], needle: i32) -> bool {
    let mut lo = 0;
    let mut hi = haystack.len();

    loop {
        if (lo >= hi) {
            break;
        }

        let half = (lo + (hi - lo) / 2) as f64;
        let m = half.floor() as usize;
        let v = haystack[m];

        if v == needle {
            return true;
        } else if v > needle {
            hi = m
        } else {
            lo = m + 1
        }
    }

    return false;
}
