//! This module provides an enum `Balance` that models a position within a 3x3 grid,
//! along with several utility methods to manipulate these positions.
//!
//! The `Balance` enum is intended for scenarios where balanced ternary logic or
//! grid-based movement is required. It represents nine possible positions in a
//! 3x3 grid, with the central point being `(0, 0)` and other points positioned
//! as offsets relative to this center.
//!
//! ## Main Features
//!
//! - `Balance` enum variants represent specific positions in the grid, such as
//!   `TopLeft`, `Center`, or `BottomRight`.
//! - Methods to convert between `Balance` variants and their 2D vector representations.
//! - Utility methods to move a position in the grid (e.g., `up`, `down`, `left`, `right`).
//!
//! ## Usage Examples
//!
//! Basic usage of `Balance` to convert between variants and vectors:
//!
//! ```rust
//! use balanced_direction::Balance;
//!
//! let position = Balance::TopLeft;
//! assert_eq!(position.to_vector(), (-1, -1));
//!
//! let center = Balance::Center;
//! assert_eq!(center.to_vector(), (0, 0));
//!
//! let balance = Balance::from_vector(-1, -1);
//! assert_eq!(balance, Balance::TopLeft);
//! ```
//!
//! Moving positions in the grid:
//!
//! ```rust
//! use balanced_direction::Balance;
//!
//! let balance = Balance::Center;
//! assert_eq!(balance.up(), Balance::Top);
//! assert_eq!(balance.down(), Balance::Bottom);
//! assert_eq!(balance.left(), Balance::Left);
//! assert_eq!(balance.right(), Balance::Right);
//! ```
#![cfg_attr(not(test), no_std)]
extern crate alloc;

mod balance;
mod conversions;
mod operations;

#[cfg(feature = "ternary")]
mod ternary;

mod path;

pub use balance::Balance;
pub use path::Path;

#[cfg(test)]
mod tests {
    use core::fmt::Debug;
    
    pub fn assert_eq_array<T: PartialEq + Debug>(a: [T; 9], b: [T; 9]) {
        for i in 0..9 {
            assert_eq!(a[i], b[i]);
        }
    }

    mod readme_examples {
        use alloc::vec;
        use crate::{Balance, Path};
        use balanced_ternary::Digit;

        #[test]
        fn example_usage() {
            let position = Balance::TopLeft;
            assert_eq!(position.to_vector(), (-1, -1));

            let moved = position.right();
            assert_eq!(moved, Balance::Top);

            let rotated = moved.rotate_left();
            assert_eq!(rotated, Balance::Left);
            assert_eq!(rotated.to_angle(), Balance::WEST);
        }

        #[test]
        fn path_example() {
            let movements = vec![Balance::Top, Balance::Right, Balance::Bottom];
            let path = Path::new(movements);

            assert_eq!(path.len(), 3);
            assert_eq!(path.to_vector(), (1, 0)); // Cumulative movement: right by 1

            let movements = vec![Balance::Top, Balance::Top, Balance::Top, Balance::Bottom];
            let path = Path::new(movements);
            assert_eq!(path.to_vector(), (0, -2)); // Cumulative movement: top by 2
            let normalized_path = path.normalized();
            assert_eq!(normalized_path.to_vector(), (0, -2));
            assert_eq!(normalized_path.len(), 2);
        }

        #[test]
        fn ternary_example() {
            let position = Balance::Top;
            let ternary_pair = position.to_ternary_pair();
            assert_eq!(ternary_pair, (Digit::Zero, Digit::Neg)); // Top is represented as (0, -1)

            let recreated_position = Balance::from_ternary_pair(Digit::Zero, Digit::Neg);
            assert_eq!(recreated_position, Balance::Top);
            let possibly = recreated_position.possibly();
            assert_eq!(possibly, Balance::TopRight);
        }

    }
}