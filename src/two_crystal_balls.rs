pub fn two_crystal_balls(breaks: &[bool]) -> i32 {
    let jump_amount = (breaks.len() as f64).sqrt() as usize;

    let mut i = jump_amount as usize;

    loop {
        if breaks[i] {
            break;
        }

        i += jump_amount;
    }

    let mut current_floor = i - jump_amount;

    for floor in current_floor..breaks.len() {
        if breaks[floor] {
            return floor as i32;
        }

        current_floor += 1;
    }

    return -1;
}
