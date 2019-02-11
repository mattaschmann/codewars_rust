// https://www.codewars.com/kata/58712dfa5c538b6fc7000569/train/rust

fn count_red_beads(n: u32) -> u32 {
    if n < 2 { return 0 }

    (n * 2) - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_red_beads() {
        assert_eq!(count_red_beads(0), 0);
        assert_eq!(count_red_beads(1), 0);
        assert_eq!(count_red_beads(2), 2);
        assert_eq!(count_red_beads(3), 4);
    }
}
