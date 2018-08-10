// https://www.codewars.com/kata/validate-sudoku-with-size-nxn

use std::collections::{ HashMap, HashSet };

struct Sudoku {
  data: Vec<Vec<u32>>,
}

impl Sudoku {
  fn is_valid(&self) -> bool {
    println!("data: {:?}", self.data);
    let size = self.data[0].len();
    let sq_size = (size as f32).sqrt();
    if size == 0 { return false }

    let mut cols = HashMap::new();
    let mut squares = HashMap::new();

    let valid = self.data.iter().enumerate().all(|(j, v)| {
      if v.len() != size { return false }

      let mut row = HashSet::new();

      v.iter().enumerate().all(|(i, n)| {
        if n > &(size as u32) || n == &0 { return false }

        // calculate which square we are in
        let sq_num = (((i + 1) as f32 / sq_size).ceil() + (j as f32 / sq_size).floor() * 3_f32) as usize;
        let col = cols.entry(i).or_insert(HashSet::new());
        let sq = squares.entry(sq_num).or_insert(HashSet::new());

        if row.contains(n) || col.contains(n) || sq.contains(n) {
          return false;
        }

        row.insert(n);
        col.insert(n);
        sq.insert(n);

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

    let good_sudoku_3 = Sudoku {
      data: vec![
        vec![17, 19, 1, 11, 15, 8, 24, 5, 16, 9, 4, 20, 22, 7, 21, 13, 6, 23, 12, 10, 2, 18, 25, 3, 14],
        vec![4, 9, 14, 13, 8, 6, 21, 18, 17, 12, 1, 2, 3, 16, 15, 24, 25, 7, 5, 19, 11, 10, 22, 23, 20],
        vec![24, 25, 7, 21, 12, 4, 1, 2, 20, 3, 13, 5, 23, 10, 11, 9, 22, 8, 18, 14, 15, 19, 16, 6, 17],
        vec![16, 3, 23, 2, 5, 19, 13, 14, 22, 10, 6, 17, 18, 24, 25, 11, 20, 15, 4, 21, 12, 1, 7, 9, 8],
        vec![20, 10, 18, 22, 6, 15, 25, 23, 11, 7, 12, 9, 8, 19, 14, 17, 1, 3, 16, 2, 4, 24, 21, 13, 5],
        vec![19, 6, 20, 5, 25, 18, 2, 16, 15, 21, 17, 8, 7, 9, 23, 4, 12, 10, 14, 1, 24, 3, 11, 22, 13],
        vec![11, 13, 3, 17, 10, 22, 20, 12, 9, 23, 25, 15, 24, 6, 5, 8, 2, 18, 19, 16, 21, 4, 1, 14, 7],
        vec![15, 24, 9, 18, 21, 10, 7, 3, 4, 5, 14, 1, 11, 2, 16, 20, 13, 17, 23, 22, 6, 25, 19, 8, 12],
        vec![14, 7, 16, 12, 2, 1, 17, 19, 6, 8, 21, 22, 4, 18, 13, 3, 24, 25, 15, 11, 10, 20, 23, 5, 9],
        vec![8, 23, 22, 1, 4, 24, 11, 25, 13, 14, 3, 12, 10, 20, 19, 5, 9, 21, 7, 6, 18, 16, 2, 17, 15],
        vec![12, 1, 5, 10, 24, 2, 3, 21, 14, 11, 15, 25, 6, 22, 17, 16, 8, 9, 13, 4, 20, 23, 18, 7, 19],
        vec![23, 21, 2, 3, 17, 13, 12, 10, 7, 4, 8, 18, 19, 5, 9, 25, 15, 1, 20, 24, 22, 14, 6, 16, 11],
        vec![18, 8, 11, 20, 14, 16, 9, 17, 25, 1, 24, 21, 12, 4, 7, 6, 19, 22, 2, 23, 13, 5, 15, 10, 3],
        vec![6, 22, 25, 19, 13, 5, 8, 20, 18, 15, 23, 3, 16, 1, 2, 21, 11, 14, 10, 7, 9, 17, 12, 4, 24],
        vec![9, 16, 4, 15, 7, 23, 6, 22, 24, 19, 10, 11, 13, 14, 20, 18, 17, 5, 3, 12, 25, 21, 8, 2, 1],
        vec![7, 2, 13, 9, 20, 17, 16, 11, 21, 22, 18, 24, 14, 23, 1, 15, 3, 6, 25, 5, 8, 12, 10, 19, 4],
        vec![22, 18, 19, 24, 16, 3, 4, 8, 12, 25, 5, 13, 17, 15, 6, 10, 7, 11, 1, 9, 14, 2, 20, 21, 23],
        vec![5, 12, 6, 4, 1, 20, 18, 15, 23, 24, 16, 10, 2, 21, 3, 22, 14, 19, 8, 13, 7, 9, 17, 11, 25],
        vec![25, 17, 8, 14, 3, 7, 10, 13, 1, 2, 20, 19, 9, 11, 22, 12, 23, 4, 21, 18, 16, 15, 5, 24, 6],
        vec![10, 15, 21, 23, 11, 14, 19, 9, 5, 6, 7, 4, 25, 12, 8, 2, 16, 24, 17, 20, 1, 13, 3, 18, 22],
        vec![3, 11, 24, 7, 18, 9, 23, 1, 8, 13, 19, 16, 21, 17, 12, 14, 10, 20, 22, 25, 5, 6, 4, 15, 2],
        vec![13, 14, 15, 25, 19, 21, 22, 4, 10, 18, 2, 6, 1, 3, 24, 23, 5, 12, 11, 8, 17, 7, 9, 20, 16],
        vec![2, 4, 17, 16, 9, 11, 15, 7, 19, 20, 22, 14, 5, 25, 10, 1, 18, 13, 6, 3, 23, 8, 24, 12, 21],
        vec![21, 20, 12, 8, 22, 25, 5, 6, 3, 16, 9, 23, 15, 13, 18, 7, 4, 2, 24, 17, 19, 11, 14, 1, 10],
        vec![1, 5, 10, 6, 23, 12, 14, 24, 2, 17, 11, 7, 20, 8, 4, 19, 21, 16, 9, 15, 3, 22, 13, 25, 18]
      ],
    };

    assert!(good_sudoku_1.is_valid());
    assert!(good_sudoku_2.is_valid());
    assert!(good_sudoku_3.is_valid());
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
