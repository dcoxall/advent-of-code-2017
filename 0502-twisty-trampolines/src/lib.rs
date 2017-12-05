#[cfg(test)]
mod tests {
    use super::steps;

    #[test]
    fn it_works() {
        let mut jumps = vec![0, 3, 0, 1, -3];
        assert_eq!(10, steps(&mut jumps));
    }
}

pub fn steps(jumps: &mut Vec<i32>) -> u32 {
    let mut current_jump: usize = 0;
    let mut step: u32 = 0;
    loop {
        // Advanced to next jump
        if let Some(n) = jumps.get_mut(current_jump) {
            step += 1;
            // ensure we dont fall off bottom of jump list
            current_jump = ((current_jump as i32) + *n).max(0) as usize;

            // modify the jump
            if *n >= 3 {
                *n -= 1;
            } else {
                *n += 1;
            }

            continue
        }
        break // we reached the end
    };
    step
}
