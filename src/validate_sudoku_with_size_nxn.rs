// https://www.codewars.com/kata/validate-sudoku-with-size-nxn

use std::collections::{ HashMap, HashSet };

struct Sudoku {
  data: Vec<Vec<u32>>,
}

impl Sudoku {
  fn is_valid(&self) -> bool {
    println!("data: {:?}", self.data);
    let size = self.data[0].len();
    if size == 0 { return false }

    let mut cols = HashMap::new();

    let valid = self.data.iter().all(|v| {
      if v.len() != size { return false }

      let mut row = HashSet::new();

      v.iter().enumerate().all(|(i, n)| {
        if n > &(size as u32) || n == &0 { return false }
        let col = cols.entry(i).or_insert(HashSet::new());

        if row.contains(n) || col.contains(n) {
          return false;
        }

        row.insert(n);
        col.insert(n);

        true
      })
    });

    if valid && cols[&0].len() != size { return false }
    valid
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
        vec![2, 3, 4, 5, 6, 7, 8, 9, 1],
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

    let bad_sudoku_4 = Sudoku {
      data: vec![
        vec![2]
      ],
    };

    let bad_sudoku_5 = Sudoku {
      data: vec![
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
      ],
    };

    let bad_sudoku_6 = Sudoku {
      data: vec![ vec![] ],
    };

    let bad_sudoku_7 = Sudoku {
      data: vec![
        vec![0]
      ],
    };

    let bad_sudoku_8 = Sudoku {
      data: vec![
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![2, 3, 1, 5, 6, 4, 8, 9, 7],
        vec![3, 1, 2, 6, 4, 5, 9, 7, 8],
        vec![4, 5, 6, 7, 8, 9, 1, 2, 3],
        vec![5, 6, 4, 8, 9, 7, 2, 3, 1],
        vec![6, 4, 5, 9, 7, 8, 3, 1, 2],
        vec![7, 8, 9, 1, 2, 3, 4, 5, 6],
        vec![8, 9, 7, 2, 3, 1, 5, 6, 4],
        vec![9, 7, 8, 3, 1, 2, 6, 4, 5]
      ],
    };

    assert!(!bad_sudoku_1.is_valid());
    assert!(!bad_sudoku_2.is_valid());
    assert!(!bad_sudoku_3.is_valid());
    assert!(!bad_sudoku_4.is_valid());
    assert!(!bad_sudoku_5.is_valid());
    assert!(!bad_sudoku_6.is_valid());
    assert!(!bad_sudoku_7.is_valid());
    assert!(!bad_sudoku_8.is_valid());
  }
}
