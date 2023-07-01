pub fn linear_search(haystack: &[i32], needle: i32) -> bool {
    for i in 0..haystack.len() {
        if haystack[i] == needle {
            return true;
        }
    }

    return false;
}
