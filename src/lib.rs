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
}