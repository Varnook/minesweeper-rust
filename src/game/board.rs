use crate::errors::{MinesweeperError, Result};
use rand::seq::SliceRandom;

pub struct Board<const X: usize, const Y: usize> {
        tiles: [[i32; Y]; X]
}

impl<const X: usize, const Y: usize> Board<X, Y> {
        pub fn new(mine_amount) -> Self {
                let reduced_mine_amount = if mine_amount > X * Y {
                        X * Y
                } else {
                        mine_amount
                };

                let mut b = Board {
                        tiles: [[0; Y]; X],
                };


        }

        fn convert_to_1d(x_coordinate: i32, y_coordinate: i32) -> i32 {
                x_coordinate * Y + y_coordinate
        }

        fn convert_from_1d(point: i32) -> (i32, i32) {
                (point / Y, point % Y)
        }
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn new_board() {
                let b = Board::<1,2>::new();
        }
}
