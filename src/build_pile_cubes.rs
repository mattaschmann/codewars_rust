// https://www.codewars.com/kata/5592e3bd57b64d00f3000047

fn find_nb(n: u64) -> i32 {
    let mut height: u64 = 0;
    let mut remaining_vol = n;
    let mut next_vol = (height + 1).pow(3);


    while remaining_vol >= next_vol {
        remaining_vol -= next_vol;
        height += 1;
        next_vol = (height + 1).pow(3);
    }

    if remaining_vol == 0 { return height as i32 }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: u64, exp: i32) -> () {
        assert_eq!(find_nb(n), exp);
    }

    #[test]
    fn basics_find_nb() {
        testing(1, 1);
        testing(4183059834009, 2022);
        testing(24723578342962, -1);
        testing(135440716410000, 4824);
        testing(40539911473216, 3568);
        testing(26825883955641, 3218);
    }
}
