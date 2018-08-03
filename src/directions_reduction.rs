// https://www.codewars.com/kata/550f22f4d758534c1100025a

#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
enum Direction {
  NORTH,
  SOUTH,
  EAST,
  WEST,
}

use self::Direction::*;

fn is_opposite(a: &Direction, b: &Direction) -> bool {
  match a {
    &NORTH => b == &SOUTH,
    &SOUTH => b == &NORTH,
    &EAST  => b == &WEST,
    &WEST  => b == &EAST
  }
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
  arr.into_iter()
     .fold(Vec::new(), |mut acc: Vec<Direction>, dir| {
       let last = acc.pop();
       match last {
         Some(d) => {
           if is_opposite(&d, dir) {
             return acc;
           }
           acc.push(d)
         },
         None => { }
       }

       acc.push(dir.clone());
       acc
     })
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_opposite() {
    assert_eq!(is_opposite(&NORTH, &SOUTH), true);
    assert_eq!(is_opposite(&SOUTH, &NORTH), true);
    assert_eq!(is_opposite(&EAST, &WEST), true);
    assert_eq!(is_opposite(&WEST, &EAST), true);
  }

  #[test]
  fn returns_expected() {
    use self::Direction::*;
    let a = [NORTH, SOUTH, SOUTH, EAST, WEST, NORTH, WEST];
    assert_eq!(dir_reduc(&a), [WEST]);
    let a = [NORTH, WEST, SOUTH, EAST];
    assert_eq!(dir_reduc(&a), [NORTH, WEST, SOUTH, EAST]);
  }
}
