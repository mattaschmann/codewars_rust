// https://www.codewars.com/kata/validate-sudoku-with-size-nxn

use std::collections::{ HashMap, HashSet };

struct Sudoku {
  data: Vec<Vec<u32>>,
}

impl Sudoku {
  fn is_valid(&self) -> bool {
    let size = self.data[0].len();
    let mut cols = HashMap::new();

    self.data.iter().all(|v| {
      if v.len() != size { return false }

      let mut row = HashSet::new();

      v.iter().enumerate().all(|(i, n)| {
        let col = cols.entry(i).or_insert(HashSet::new());

        if row.contains(n) || col.contains(n) {
          return false;
        }

        row.insert(n);
        col.insert(n);

        true
      })
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn good_sudoku() {
    let good_sudoku_1 = Sudoku {
      data: vec![
        vec![7, 8, 4, 1, 5, 9, 3, 2, 6],
        vec![5, 3, 9, 6, 7, 2, 8, 4, 1],
        vec![6, 1, 2, 4, 3, 8, 7, 5, 9],
        vec![9, 2, 8, 7, 1, 5, 4, 6, 3],
        vec![3, 5, 7, 8, 4, 6, 1, 9, 2],
        vec![4, 6, 1, 9, 2, 3, 5, 8, 7],
        vec![8, 7, 6, 3, 9, 4, 2, 1, 5],
        vec![2, 4, 3, 5, 6, 1, 9, 7, 8],
        vec![1, 9, 5, 2, 8, 7, 6, 3, 4],
      ],
    };

    let good_sudoku_2 = Sudoku {
      data: vec![
        vec![1, 4, 2, 3],
        vec![3, 2, 4, 1],
        vec![4, 1, 3, 2],
        vec![2, 3, 1, 4],
      ],
    };
    assert!(good_sudoku_1.is_valid());
    assert!(good_sudoku_2.is_valid());
  }

  #[test]
  fn bad_sudoku() {
    let bad_sudoku_1 = Sudoku {
      data: vec![
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
      ],
    };

    let bad_sudoku_2 = Sudoku {
      data: vec![
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
        vec![1],
      ],
    };

    let bad_sudoku_3 = Sudoku {
      data: vec![
        vec![1, 2, 3, 4, 5],
        vec![2, 1, 4, 3],
        vec![3, 4, 1, 2],
        vec![4],
      ],
    };

    assert!(!bad_sudoku_1.is_valid());
    assert!(!bad_sudoku_2.is_valid());
    assert!(!bad_sudoku_3.is_valid());
  }
}
