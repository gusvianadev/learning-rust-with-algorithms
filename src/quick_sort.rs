fn partition(arr: &mut [i32], lo: usize, hi: usize) -> usize {
    let pivot = arr[hi];

    let mut idx = lo.checked_sub(1).unwrap_or(0);

    let mut inside = false;
    for i in lo..hi {
        dbg!(&idx);
        if arr[i] <= pivot {
            if lo == 0 {
                if inside {
                    idx += 1;
                } else {
                    inside = true;
                }
            } else {
                idx += 1;
            };

            dbg!(&idx);
            let tmp = arr[i];

            arr[i] = arr[idx];
            arr[idx] = tmp;
        }
    }

    idx += 1;
    arr[hi] = arr[idx];
    arr[idx] = pivot;

    idx
}

fn qs(arr: &mut [i32], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let pivot_idx = partition(arr, lo, hi);
    qs(arr, lo, pivot_idx - 1);
    qs(arr, pivot_idx + 1, hi);
}

pub fn quick_sort(arr: &mut [i32]) {
    qs(arr, 0, arr.len() - 1);
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
