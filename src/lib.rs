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
#![no_std]
extern crate alloc;

use alloc::vec::Vec;
use core::ops::{Add, Mul, Neg, Not, Sub};

/// Represents a position within a 3x3 grid, with each variant corresponding to a specific point.
///
/// The `Balance` enum is used to model a balanced ternary direction or position
/// within a 2D grid. Each variant represents one of the nine possible positions
/// in the grid, where the center (`Balance::Center`) is `(0, 0)` and the
/// surrounding positions are offsets from this central point.
///
/// # Variants
///
/// - `TopLeft`: The position at `(-1, -1)`
/// - `Top`: The position at `(0, -1)`
/// - `TopRight`: The position at `(1, -1)`
/// - `Left`: The position at `(-1, 0)`
/// - `Center`: The central position `(0, 0)`
/// - `Right`: The position at `(1, 0)`
/// - `BottomLeft`: The position at `(-1, 1)`
/// - `Bottom`: The position at `(0, 1)`
/// - `BottomRight`: The position at `(1, 1)`
///
/// # Examples
///
/// ```
/// use balanced_direction::Balance;
///
/// let position = Balance::TopLeft;
/// assert_eq!(position.to_vector(), (-1, -1));
///
/// let center = Balance::Center;
/// assert_eq!(center.to_vector(), (0, 0));
/// ```
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Balance {
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl Balance {
    /// Converts the current `Balance` variant into a 2D vector `(i8, i8)` representing its coordinates.
    ///
    /// # Returns
    ///
    /// A tuple `(i8, i8)` representing the position in the 3x3 grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let position = Balance::TopLeft;
    /// assert_eq!(position.to_vector(), (-1, -1));
    ///
    /// let center = Balance::Center;
    /// assert_eq!(center.to_vector(), (0, 0));
    /// ```
    pub fn to_vector(self) -> (i8, i8) {
        match self {
            Balance::TopLeft => (-1, -1),
            Balance::Top => (0, -1),
            Balance::TopRight => (1, -1),
            Balance::Left => (-1, 0),
            Balance::Center => (0, 0),
            Balance::Right => (1, 0),
            Balance::BottomLeft => (-1, 1),
            Balance::Bottom => (0, 1),
            Balance::BottomRight => (1, 1),
        }
    }

    /// Converts a pair of integers `(a, b)` into the corresponding `Balance` variant.
    ///
    /// # Parameters
    ///
    /// - `a`: The x-coordinate in the 2D grid, expected to be in the range `-1..=1`.
    /// - `b`: The y-coordinate in the 2D grid, expected to be in the range `-1..=1`.
    ///
    /// # Returns
    ///
    /// The `Balance` variant that corresponds to the provided `(a, b)` coordinates.
    ///
    /// # Panics
    ///
    /// Panics if the provided `(a, b)` pair does not correspond to a valid `Balance` variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let balance = Balance::from_vector(-1, -1);
    /// assert_eq!(balance, Balance::TopLeft);
    ///
    /// let balance = Balance::from_vector(0, 1);
    /// assert_eq!(balance, Balance::Bottom);
    /// ```
    pub fn from_vector(a: i8, b: i8) -> Self {
        match (a, b) {
            (-1, -1) => Balance::TopLeft,
            (0, -1) => Balance::Top,
            (1, -1) => Balance::TopRight,
            (-1, 0) => Balance::Left,
            (0, 0) => Balance::Center,
            (1, 0) => Balance::Right,
            (-1, 1) => Balance::BottomLeft,
            (0, 1) => Balance::Bottom,
            (1, 1) => Balance::BottomRight,
            _ => panic!("Invalid vector"),
        }
    }

    /// Moves the current position upwards in the 3x3 grid while staying within bounds.
    ///
    /// # Returns
    ///
    /// The `Balance` variant representing the position directly above the current one.
    /// If the current position is at the top edge, the result will stay at the edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let balance = Balance::Center;
    /// assert_eq!(balance.up(), Balance::Top);
    ///
    /// let balance = Balance::Top;
    /// assert_eq!(balance.up(), Balance::Top);
    /// ```
    pub fn up(self) -> Self {
        let (x, y) = self.to_vector();
        Self::from_vector(x, (y - 1).clamp(-1, 1))
    }

    /// Moves the current position downwards in the 3x3 grid while staying within bounds.
    ///
    /// # Returns
    ///
    /// The `Balance` variant representing the position directly below the current one.
    /// If the current position is at the bottom edge, the result will stay at the edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let balance = Balance::Center;
    /// assert_eq!(balance.down(), Balance::Bottom);
    ///
    /// let balance = Balance::Bottom;
    /// assert_eq!(balance.down(), Balance::Bottom);
    /// ```
    pub fn down(self) -> Self {
        let (x, y) = self.to_vector();
        Self::from_vector(x, (y + 1).clamp(-1, 1))
    }

    /// Moves the current position to the left in the 3x3 grid while staying within bounds.
    ///
    /// # Returns
    ///
    /// The `Balance` variant representing the position directly to the left of the current one.
    /// If the current position is at the left edge, the result will stay at the edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let balance = Balance::Center;
    /// assert_eq!(balance.left(), Balance::Left);
    ///
    /// let balance = Balance::Left;
    /// assert_eq!(balance.left(), Balance::Left);
    /// ```
    pub fn left(self) -> Self {
        let (x, y) = self.to_vector();
        Self::from_vector((x - 1).clamp(-1, 1), y)
    }

    /// Moves the current position to the right in the 3x3 grid while staying within bounds.
    ///
    /// # Returns
    ///
    /// The `Balance` variant representing the position directly to the right of the current one.
    /// If the current position is at the right edge, the result will stay at the edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let balance = Balance::Center;
    /// assert_eq!(balance.right(), Balance::Right);
    ///
    /// let balance = Balance::Right;
    /// assert_eq!(balance.right(), Balance::Right);
    /// ```
    pub fn right(self) -> Self {
        let (x, y) = self.to_vector();
        Self::from_vector((x + 1).clamp(-1, 1), y)
    }

    /// Flips the current position horizontally in the 3x3 grid.
    ///
    /// # Returns
    ///
    /// The `Balance` variant that is mirrored horizontally across
    /// the vertical axis. For example, flipping `Balance::Left` results
    /// in `Balance::Right`, and vice-versa. Positions on the vertical axis
    /// (like `Balance::Center` or `Balance::Top`) remain unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let balance = Balance::Left;
    /// assert_eq!(balance.flip_h(), Balance::Right);
    ///
    /// let balance = Balance::Center;
    /// assert_eq!(balance.flip_h(), Balance::Center);
    /// ```
    pub fn flip_h(self) -> Self {
        let (x, y) = self.to_vector();
        Self::from_vector(-x, y)
    }

    /// Flips the current position vertically in the 3x3 grid.
    ///
    /// # Returns
    ///
    /// The `Balance` variant that is mirrored vertically across
    /// the horizontal axis. For example, flipping `Balance::Top`
    /// results in `Balance::Bottom`, and vice-versa. Positions on the horizontal axis
    /// (like `Balance::Center` or `Balance::Left`) remain unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let balance = Balance::Top;
    /// assert_eq!(balance.flip_v(), Balance::Bottom);
    ///
    /// let balance = Balance::Center;
    /// assert_eq!(balance.flip_v(), Balance::Center);
    /// ```
    pub fn flip_v(self) -> Self {
        let (x, y) = self.to_vector();
        Self::from_vector(x, -y)
    }

    /// Rotates the current position 90 degrees counterclockwise in the 3x3 grid.
    ///
    /// # Returns
    ///
    /// The `Balance` variant representing the position after a 90-degree counterclockwise
    /// rotation around the center. For example, rotating `Balance::Right` counterclockwise
    /// will result in `Balance::Top`, and `Balance::Top` will result in `Balance::Left`.
    /// The center position (`Balance::Center`) remains unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let balance = Balance::Right;
    /// assert_eq!(balance.rotate_left(), Balance::Top);
    ///
    /// let balance = Balance::Center;
    /// assert_eq!(balance.rotate_left(), Balance::Center);
    ///
    /// let balance = Balance::Top;
    /// assert_eq!(balance.rotate_left(), Balance::Left);
    /// ```
    pub fn rotate_left(self) -> Self {
        let (x, y) = self.to_vector();
        Self::from_vector(y, -x)
    }

    /// Rotates the current position 90 degrees clockwise in the 3x3 grid.
    ///
    /// # Returns
    ///
    /// The `Balance` variant representing the position after a 90-degree clockwise
    /// rotation around the center. For example, rotating `Balance::Top` clockwise
    /// results in `Balance::Right`, and `Balance::Right` will result in `Balance::Bottom`.
    /// The center position (`Balance::Center`) remains unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let balance = Balance::Top;
    /// assert_eq!(balance.rotate_right(), Balance::Right);
    ///
    /// let balance = Balance::Center;
    /// assert_eq!(balance.rotate_right(), Balance::Center);
    ///
    /// let balance = Balance::Right;
    /// assert_eq!(balance.rotate_right(), Balance::Bottom);
    /// ```
    pub fn rotate_right(self) -> Self {
        let (x, y) = self.to_vector();
        Self::from_vector(-y, x)
    }
}

impl Not for Balance {
    type Output = Self;

    fn not(self) -> Self::Output {
        let (x, y) = self.to_vector();
        Self::from_vector(y, x)
    }
}

impl Neg for Balance {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let (x, y) = self.to_vector();
        Balance::from_vector(-x, -y)
    }
}

impl Add for Balance {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (x1, y1) = self.to_vector();
        let (x2, y2) = rhs.to_vector();
        Balance::from_vector((x1 + x2).clamp(-1, 1), (y1 + y2).clamp(-1, 1))
    }
}

impl Mul for Balance {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let (x1, y1) = self.to_vector();
        let (x2, y2) = rhs.to_vector();
        Balance::from_vector(x1 * x2, y1 * y2)
    }
}

impl Sub for Balance {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let (x1, y1) = self.to_vector();
        let (x2, y2) = rhs.to_vector();
        Self::from_vector((x1 - x2).clamp(-1, 1), (y1 - y2).clamp(-1, 1))
    }
}

#[cfg(feature = "ternary")]
mod ternary {
    use super::Balance;
    use balanced_ternary::Digit;

    impl Balance {
        /// Converts the `Balance` position into a pair of ternary digits.
        ///
        /// # Returns
        ///
        /// A tuple containing two `Digit` values. The first element represents
        /// the x-coordinate and the second represents the y-coordinate of the `Balance`
        /// position in the ternary numeral system.
        ///
        /// The `Digit` values can range from `Neg` (-1), `Zero` (0), to `Pos` (1),
        /// matching the 3x3 balanced grid's coordinate representation.
        ///
        /// # Examples
        ///
        /// ```
        /// use balanced_direction::Balance;
        /// use balanced_ternary::Digit;
        ///
        /// let balance = Balance::Top;
        /// assert_eq!(balance.to_ternary_pair(), (Digit::Zero, Digit::Neg));
        ///
        /// let balance = Balance::Right;
        /// assert_eq!(balance.to_ternary_pair(), (Digit::Pos, Digit::Zero));
        /// ```
        pub fn to_ternary_pair(self) -> (Digit, Digit) {
            let (x, y) = self.to_vector();
            (Digit::from_i8(x), Digit::from_i8(y))
        }

        /// Creates a `Balance` instance from a pair of ternary digits.
        ///
        /// # Arguments
        ///
        /// * `a` - A `Digit` representing the x-coordinate in the ternary numeral system.
        /// * `b` - A `Digit` representing the y-coordinate in the ternary numeral system.
        ///
        /// # Returns
        ///
        /// A new `Balance` instance corresponding to the provided ternary coordinates.
        ///
        /// The values of `a` and `b` should be valid ternary digits within the range of:
        /// - `Neg` (-1), `Zero` (0), and `Pos` (1).
        ///
        /// This allows for mapping coordinates within the 3x3 grid system used by the `Balance` enum, ensuring
        /// that any valid pair of ternary digits maps directly to a specific `Balance` position.
        ///
        /// # Examples
        ///
        /// ```
        /// use balanced_direction::Balance;
        /// use balanced_ternary::Digit;
        ///
        /// let balance = Balance::from_ternary_pair(Digit::Zero, Digit::Neg);
        /// assert_eq!(balance, Balance::Top);
        ///
        /// let balance = Balance::from_ternary_pair(Digit::Pos, Digit::Zero);
        /// assert_eq!(balance, Balance::Right);
        /// ```
        pub fn from_ternary_pair(a: Digit, b: Digit) -> Self {
            Self::from_vector(a.to_i8(), b.to_i8())
        }
    }
}

/// Represents a sequence of movements in a 3x3 balanced grid, where each movement
/// is represented by a `Balance` value indicating direction or position.
///
/// The `Path` struct provides various utilities to manipulate and analyze the sequence
/// of movements, including iteration, transformation, normalization, and reversal.
///
/// # Examples
///
/// Creating a new `Path`:
/// ```
/// use balanced_direction::{Balance, Path};
///
/// let movements = vec![Balance::Top, Balance::Right, Balance::Bottom];
/// let path = Path::new(movements);
/// assert_eq!(path.len(), 3);
/// ```
///
/// Normalizing a `Path`:
/// ```
/// use balanced_direction::{Balance, Path};
///
/// let movements = vec![Balance::Top, Balance::Top];
/// let path = Path::new(movements).normalized();
/// assert_eq!(path.to_vector(), (0, -2));
/// ```
///
/// Reversing a `Path`:
/// ```
/// use balanced_direction::{Balance, Path};
///
/// let movements = vec![Balance::Top, Balance::Right];
/// let path = Path::new(movements).reversed();
/// assert_eq!(path.to_vector(), (1, -1));
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Path {
    raw: Vec<Balance>,
}

impl Path {
    /// Creates a new `Path` from a vector of movements.
    ///
    /// Each movement in the vector represents a step in a 3x3 grid,
    /// where each step is a `Balance` value indicating direction or position.
    ///
    /// # Arguments
    ///
    /// * `movements` - A `Vec` of `Balance` values representing the sequence of movements.
    ///
    /// # Returns
    ///
    /// A new `Path` instance containing the provided sequence of movements.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::{Balance, Path};
    ///
    /// let movements = vec![Balance::Top, Balance::Right, Balance::Bottom];
    /// let path = Path::new(movements);
    /// assert_eq!(path.len(), 3);
    /// ```
    pub fn new(movements: Vec<Balance>) -> Self {
        Self { raw: movements }
    }

    /// Returns the number of movements in the `Path`.
    ///
    /// # Returns
    ///
    /// An integer representing the number of elements in the `Path`.
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    /// Checks whether the `Path` is empty.
    ///
    /// # Returns
    ///
    /// `true` if the `Path` contains no movements, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }

    /// Retrieves the `Balance` at the specified index in the `Path`.
    ///
    /// # Arguments
    ///
    /// * `index` - The position of the `Balance` in the `Path` to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `Balance` if the index is valid, or `None` otherwise.
    pub fn get(&self, index: usize) -> Option<&Balance> {
        self.raw.get(index)
    }

    /// Returns an iterator over immutable references to the `Balance` values in the `Path`.
    ///
    /// The iterator allows traversing the sequence of movements without modifying it.
    pub fn iter(&self) -> impl Iterator<Item = &Balance> {
        self.raw.iter()
    }

    /// Returns an iterator over mutable references to the `Balance` values in the `Path`.
    ///
    /// The iterator allows traversing the sequence of movements modifying it.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Balance> {
        self.raw.iter_mut()
    }

    /// Appends a new movement to the end of the `Path`.
    ///
    /// # Arguments
    ///
    /// * `movement` - The `Balance` value to be added to the `Path`.
    pub fn push(&mut self, movement: Balance) {
        self.raw.push(movement);
    }

    /// Removes the last movement from the `Path`, if any, and returns it.
    ///
    /// # Returns
    ///
    /// An `Option` containing the `Balance` value that was removed, or `None` if the `Path` is empty.
    pub fn pop(&mut self) -> Option<Balance> {
        self.raw.pop()
    }

    /// Clears all movements from the `Path`, leaving it empty.
    pub fn clear(&mut self) {
        self.raw.clear();
    }

    /// Converts the sequence of movements in the `Path` to a vector representation.
    ///
    /// Each `Balance` value in the `Path` contributes a two-dimensional `(i8, i8)` vector,
    /// which represents its direction or position in the grid. The resulting vector
    /// is the cumulative sum of all movements in the sequence.
    ///
    /// # Returns
    ///
    /// A tuple `(i8, i8)` where:
    /// - The first element is the cumulative movement along the x-axis.
    /// - The second element is the cumulative movement along the y-axis.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::{Balance, Path};
    ///
    /// let movements = vec![Balance::Top, Balance::Right, Balance::Top];
    /// let path = Path::new(movements);
    /// let vector = path.to_vector();
    /// assert_eq!(vector, (1, -2)); // 1 step right, 2 steps up
    /// ```
    pub fn to_vector(&self) -> (i8, i8) {
        let mut x = 0;
        let mut y = 0;
        for movement in self.raw.iter() {
            let (a, b) = movement.to_vector();
            x += a;
            y += b;
        }
        (x, y)
    }

    /// Converts a vector representation `(x, y)` into a `Path`.
    ///
    /// This function takes two integers, `x` and `y`, representing cumulative movements along
    /// the x-axis and y-axis, respectively, in a 2D grid. It decomposes these movements into
    /// individual steps represented as a sequence of `Balance` values, which are stored in a `Path`.
    ///
    /// Movements are calculated progressively by reducing the values of `x` and `y` by their sign
    /// in each step until both reach 0. Each step corresponds to a direction as determined
    /// by `Balance::from_vector`.
    ///
    /// # Arguments
    ///
    /// * `x` - An `i8` representing the movement along the x-axis.
    /// * `y` - An `i8` representing the movement along the y-axis.
    ///
    /// # Returns
    ///
    /// A `Path` instance containing a sequence of movements that achieve the given `x` and `y` displacements.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::{Balance, Path};
    ///
    /// let path = Path::from_vector(2, -1);
    /// assert_eq!(path.to_vector(), (2, -1));
    /// ```
    pub fn from_vector(x: i8, y: i8) -> Self {
        let mut movements = Vec::new();
        let mut x = x;
        let mut y = y;
        while x != 0 || y != 0 {
            let (a, b) = (x.signum(), y.signum());
            x -= a;
            y -= b;
            movements.push(Balance::from_vector(a, b));
        }
        Self { raw: movements }
    }

    /// Returns a normalized `Path`.
    ///
    /// The normalized `Path` is constructed by converting the sequence of movements
    /// in the current `Path` into their cumulative vector representation using `to_vector`
    /// and then converting this vector back into a `Path` using `from_vector`.
    ///
    /// This effectively removes redundant steps in the `Path` that cancel each other out,
    /// resulting in a minimal representation of the net movement.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::{Balance, Path};
    ///
    /// let movements = vec![Balance::Top, Balance::Bottom, Balance::Right, Balance::Right];
    /// let path = Path::new(movements);
    /// let normalized_path = path.normalized();
    /// assert_eq!(normalized_path.to_vector(), (2, 0)); // Two steps right
    /// ```
    pub fn normalized(&self) -> Self {
        let (x, y) = self.to_vector();
        Self::from_vector(x, y)
    }

    /// Reverses the sequence of movements in the `Path`.
    ///
    /// The reversed `Path` will have its movements ordered in the opposite direction
    /// compared to the original `Path`. The order of the movements is inverted,
    /// but the movements themselves remain unchanged.
    ///
    /// # Returns
    ///
    /// A new `Path` instance containing the reversed sequence of movements.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::{Balance, Path};
    ///
    /// let movements = vec![Balance::Top, Balance::Right, Balance::Left];
    /// let path = Path::new(movements);
    /// let reversed_path = path.reversed();
    /// assert_eq!(path.to_vector(), (0, -1));
    /// assert_eq!(reversed_path.to_vector(), (0, -1));
    /// ```
    pub fn reversed(&self) -> Self {
        let mut movements = Vec::new();
        for movement in self.raw.iter().rev() {
            movements.push(*movement);
        }
        Self { raw: movements }
    }

    /// Applies a function `f` to each `Balance` in the `Path` and returns a new `Path` containing the results.
    ///
    /// This method iterates over all movements in the `Path`, applies the function `f` to each movement,
    /// and collects the resulting `Balance` values into a new `Path`.
    ///
    /// # Arguments
    ///
    /// * `f` - A function or closure of type `Fn(Balance) -> Balance` that takes a `Balance` as input
    ///         and returns a transformed `Balance`.
    ///
    /// # Returns
    ///
    /// A new `Path` where each `Balance` is the result of applying `f` to the corresponding
    /// `Balance` in the original `Path`.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_direction::{Balance, Path};
    ///
    /// let movements = vec![Balance::Top, Balance::Right, Balance::Left];
    /// let path = Path::new(movements);
    /// let transformed_path = path.each(Balance::up);
    /// assert_eq!(
    ///     transformed_path.to_vector(),
    ///     (0, -3)
    /// );
    /// ```
    pub fn each(&self, f: impl Fn(Balance) -> Balance) -> Self {
        let mut movements = Vec::with_capacity(self.raw.len());
        for movement in self.raw.iter() {
            movements.push(f(*movement));
        }
        Self { raw: movements }
    }

    /// Applies a function `f`, which takes two arguments of type `Balance`,
    /// and returns a transformed `Balance`, to each `Balance` in the current `Path`,
    /// using the provided `other` value as the second argument.
    ///
    /// This method iterates over all movements in the `Path` and applies the function `f`
    /// to each movement and the `other` value. The results of the function application
    /// are collected into a new `Path`.
    ///
    /// # Arguments
    ///
    /// * `f` - A function or closure of type `Fn(Balance, Balance) -> Balance` that
    ///         takes two `Balance` arguments and returns a transformed `Balance`.
    /// * `other` - A `Balance` value that is passed as the second argument to the function `f`.
    ///
    /// # Returns
    ///
    /// A new `Path` instance where each `Balance` is the result of applying `f`
    /// to the corresponding `Balance` in the original `Path` and the `other` value.
    ///
    /// # Example
    ///
    /// ```
    /// use std::ops::Add;
    /// use balanced_direction::{Balance, Path};
    ///
    /// let movements = vec![Balance::Left, Balance::TopLeft];
    /// let path = Path::new(movements);
    /// let modified_path = path.each_with(Balance::add, Balance::Right);
    /// assert_eq!(modified_path.to_vector(), (0, -1));
    /// ```
    pub fn each_with(&self, f: impl Fn(Balance, Balance) -> Balance, other: Balance) -> Self {
        let mut movements = Vec::with_capacity(self.raw.len());
        for movement in self.raw.iter() {
            movements.push(f(*movement, other));
        }
        Self { raw: movements }
    }

    /// Applies a function `f` to corresponding pairs of `Balance` values from the current `Path`
    /// and the `other` `Path`, and returns a new `Path` containing the results.
    ///
    /// This method iterates over the paired elements of the current `Path` and the provided `other`
    /// `Path`, applies the function `f` to each pair, and collects the results into a new `Path`.
    ///
    /// # Arguments
    ///
    /// * `f` - A function or closure of type `Fn(Balance, Balance) -> Balance` that takes two
    ///         `Balance` arguments (one from each `Path`) and returns a transformed `Balance`.
    /// * `other` - A reference to another `Path` whose `Balance` values will be paired with those of
    ///             the current `Path`.
    ///
    /// # Returns
    ///
    /// A new `Path` where each `Balance` is the result of applying `f` to corresponding pairs of
    /// `Balance` values from the current `Path` and the `other` `Path`.
    ///
    /// # Panics
    ///
    /// Panics if the lengths of the two `Path`s are not equal, as the method expects both `Path`s
    /// to contain the same number of movements.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ops::Add;
    /// use balanced_direction::{Balance, Path};
    ///
    /// let path1 = Path::new(vec![Balance::Top, Balance::Right]);
    /// let path2 = Path::new(vec![Balance::Bottom, Balance::Left]);
    ///
    /// let result = path1.each_zip(Balance::add, &path2);
    /// assert_eq!(result.to_vector(), (0, 0));
    /// ```
    pub fn each_zip(&self, f: impl Fn(Balance, Balance) -> Balance, other: &Self) -> Self {
        let mut movements = Vec::with_capacity(self.raw.len());
        for (a, b) in self.raw.iter().zip(other.raw.iter()) {
            movements.push(f(*a, *b));
        }
        Self { raw: movements }
    }
}
