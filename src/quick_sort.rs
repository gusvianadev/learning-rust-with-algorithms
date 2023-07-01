fn partition(arr: &mut [i32], lo: i32, hi: i32) -> i32 {
    let pivot = arr[hi as usize];

    let mut idx = lo - 1;

    for i in lo..hi {
        if arr[i as usize] <= pivot {
            idx += 1;
            let tmp = arr[i as usize];
            arr[i as usize] = arr[idx as usize];
            arr[idx as usize] = tmp;
        }
    }

    idx += 1;
    arr[hi as usize] = arr[idx as usize];
    arr[idx as usize] = pivot;

    idx
}

fn qs(arr: &mut [i32], lo: i32, hi: i32) {
    if lo >= hi {
        return;
    }

    let pivot_idx = partition(arr, lo, hi);
    qs(arr, lo, pivot_idx - 1);
    qs(arr, pivot_idx + 1, hi);
}

pub fn quick_sort(arr: &mut [i32]) {
    qs(arr, 0, arr.len() as i32 - 1);
}

#[cfg(test)]
mod tests {
    use std::path;

    use super::*;

    #[test]
    fn t_quick_sort() {
        let mut arr = [8, 2, 4, 7, 1, 3, 9, 6, 5];

        quick_sort(&mut arr);
        assert_eq!(&mut arr, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
