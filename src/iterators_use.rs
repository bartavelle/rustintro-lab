// All the exercices in this module must be performed WITHOUT loops

use std::collections::HashMap;

pub fn isum(l: &[u64]) -> u64 {
    todo!()
}

// hint: use map
pub fn increment_all(l: &[u64]) -> Vec<u64> {
    todo!()
}

pub fn all_even(l: &[u64]) -> bool {
    todo!()
}

// hint: use the range syntax, x..=y and collect
pub fn fizzbuzz(len: usize) -> Vec<String> {
    todo!()
}

pub struct Move {
    dx: i64,
    dy: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn displace(self, m: &Move) -> Self {
        todo!()
    }
}

// hint: use fold
pub fn position(start: Point, moves: &[Move]) -> Point {
    todo!()
}

// hint: use flat_map
pub fn cartesian_product<A: Clone, B: Clone>(l: Vec<A>, r: Vec<B>) -> Vec<(A, B)> {
    todo!()
}

pub fn make_map<I1, I2, A>(v: I1) -> HashMap<Point, A>
where
    I1: IntoIterator<Item = I2>,
    I2: IntoIterator<Item = A>,
{
    todo!()
}

pub fn traverse_result<F, A, B, E>(f: &F, l: &[A]) -> Result<Vec<B>, E>
where
    F: Fn(&A) -> Result<B, E>,
{
    todo!()
}

#[cfg(test)]
mod test {
    #[test]
    fn isum() {
        assert_eq!(super::isum(&[4, 8, 12]), 24)
    }

    #[test]
    fn increment_all() {
        assert_eq!(super::increment_all(&[4, 8, 12]), [5, 9, 13])
    }

    #[test]
    fn all_even() {
        assert!(super::all_even(&[4, 8]));
        assert!(!super::all_even(&[3, 5]));
        assert!(!super::all_even(&[4, 5]));
        assert!(!super::all_even(&[5, 8]));
        assert!(super::all_even(&[]));
    }

    #[test]
    fn fizzbuzz() {
        assert_eq!(
            super::fizzbuzz(30),
            [
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz", "16", "17", "Fizz", "19", "Buzz", "Fizz", "22", "23",
                "Fizz", "Buzz", "26", "Fizz", "28", "29", "FizzBuzz"
            ]
        )
    }

    #[test]
    fn position() {
        use super::{Move, Point};

        let p = super::position(
            Point { x: 3, y: 5 },
            &[
                Move { dx: 1, dy: 1 },
                Move { dx: -3, dy: 8 },
                Move { dx: 7, dy: 1 },
            ],
        );
        assert_eq!(p, Point { x: 8, y: 15 })
    }

    #[test]
    fn cartesian_product() {
        let l1 = vec!["A", "B", "C"];
        let l2: Vec<usize> = vec![1, 2, 3, 4];
        let mut expected = Vec::new();
        for a in l1.iter() {
            for b in l2.iter() {
                expected.push((*a, *b))
            }
        }
        assert_eq!(super::cartesian_product(l1, l2), expected)
    }

    #[test]
    fn make_map() {
        use super::Point;
        let mp = ["  AB ".chars(), "  XX ".chars(), "A   B".chars()];
        let r = super::make_map(mp);
        assert_eq!(
            r,
            std::collections::HashMap::from([
                (Point { x: 2, y: 4 }, 'B'),
                (Point { x: 2, y: 3 }, ' '),
                (Point { x: 2, y: 1 }, ' '),
                (Point { x: 0, y: 1 }, ' '),
                (Point { x: 1, y: 2 }, 'X'),
                (Point { x: 1, y: 4 }, ' '),
                (Point { x: 0, y: 4 }, ' '),
                (Point { x: 0, y: 3 }, 'B'),
                (Point { x: 1, y: 1 }, ' '),
                (Point { x: 0, y: 2 }, 'A'),
                (Point { x: 1, y: 0 }, ' '),
                (Point { x: 2, y: 0 }, 'A'),
                (Point { x: 1, y: 3 }, 'X'),
                (Point { x: 2, y: 2 }, ' '),
                (Point { x: 0, y: 0 }, ' ')
            ])
        )
    }
}
