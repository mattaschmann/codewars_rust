const DICE: [i32; 6] = [1, 2, 3, 4, 5, 6];

fn rolldice_sum_prob(sum:i32, dice_amount:i32) -> f64 {
  let mut combs: i32 = 0;

  recurse_sum_dice(0, sum, dice_amount - 1, &mut combs);

  combs as f64 / 6_f64.powf(dice_amount.into())
}
fn recurse_sum_dice(sum: i32, target: i32, dice_left: i32, combs: &mut i32) {

  for n in DICE.iter() {
    if dice_left == 0 {
      if n + sum == target {
        *combs += 1;
      }
    } else {
      recurse_sum_dice(n + sum, target, dice_left - 1, combs);
    }

  }
}

#[test]
fn test_recurese_sum_dice() {
  let mut combs: i32 = 0;
  recurse_sum_dice(1, 2, 0, &mut combs);
  assert_eq!(combs, 1);

  combs = 0;
  recurse_sum_dice(0, 8, 1, &mut combs);
  assert_eq!(combs, 5);
}

#[test]
fn test_rolldice_sum_prob() {
  assert_eq!(rolldice_sum_prob(11, 2), 1_f64/18_f64);
  assert_eq!(rolldice_sum_prob(8, 2), 5_f64/36_f64);
  assert_eq!(rolldice_sum_prob(8, 3), 21_f64/6_f64.powf(3.0));
}
