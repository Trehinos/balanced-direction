use crate::Balance;
use balanced_ternary::Digit;
use core::ops::{BitAnd, BitOr, BitXor};

impl BitAnd for Balance {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let (x1, y1) = self.to_ternary_pair();
        let (x2, y2) = rhs.to_ternary_pair();
        Balance::from_ternary_pair(x1 & x2, y1 & y2)
    }
}

impl BitOr for Balance {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let (x1, y1) = self.to_ternary_pair();
        let (x2, y2) = rhs.to_ternary_pair();
        Balance::from_ternary_pair(x1 | x2, y1 | y2)
    }
}

impl BitXor for Balance {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let (x1, y1) = self.to_ternary_pair();
        let (x2, y2) = rhs.to_ternary_pair();
        Balance::from_ternary_pair(x1 ^ x2, y1 ^ y2)
    }
}

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
    pub const fn to_ternary_pair(self) -> (Digit, Digit) {
        (Digit::from_i8(self.x()), Digit::from_i8(self.y()))
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
    pub const fn from_ternary_pair(a: Digit, b: Digit) -> Self {
        Self::from_vector(a.to_i8(), b.to_i8())
    }

    /// (logic) Checks if the current logical state is `Balance::BottomRight`.
    pub const fn is_true(self) -> bool {
        matches!(self, Balance::BottomRight)
    }

    /// (logic) Checks if the current logical state includes `Balance::Bottom` or `Balance::Right`.
    pub const fn has_true(self) -> bool {
        self.x() == 1 || self.y() == 1
    }

    /// (logic) Checks if the current logical state is contradictory, representing opposing truths (`TopRight` or `BottomLeft`).
    pub const fn is_contradictory(self) -> bool {
        matches!(self, Balance::TopRight | Balance::BottomLeft)
    }

    /// (logic) Checks whether the current logical state has no certain value but is not contradictory.
    pub const fn has_unknown(self) -> bool {
        // = self.is_orthogonal().
        self.x() == 0 || self.y() == 0
    }

    /// (logic) Checks whether the current logical state is uncertain in terms of logical balance.
    pub const fn is_uncertain(self) -> bool {
        !self.is_certain()
    }

    /// (logic) Returns whether the current logical state represents a certain state in logical balance (one of `is_true()` or `is_false()` is true).
    pub const fn is_certain(self) -> bool {
        matches!(self, Balance::BottomRight | Balance::TopLeft)
    }

    /// (logic) Determines whether the current logical state includes the `Balance::Top` or `Balance::Left` variant.
    pub const fn has_false(self) -> bool {
        self.x() == -1 || self.y() == -1
    }

    /// (logic) Checks if the current logical state is `Balance::TopLeft`.
    pub const fn is_false(self) -> bool {
        matches!(self, Balance::TopLeft)
    }

    /// Converts the `Balance` logical state into a boolean representation.
    ///
    /// # Returns
    ///
    /// A `bool` value that represents the certainty of the `Balance` logical state.
    /// - Returns `true` if the position is logically `true` (e.g., `Balance::BottomRight`).
    /// - Returns `false` if the position is logically `false` (e.g., `Balance::TopLeft`).
    ///
    /// # Panics
    ///
    /// Panics if the `Balance` logical state is uncertain. Uncertain states include cases
    /// where the position cannot be determined or does not logically map to `true` or `false`.
    ///
    /// > Use [Balance::is_certain] to check certainty of the `Balance` logical state.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let balance = Balance::BottomRight;
    /// assert_eq!(balance.to_bool(), true);
    ///
    /// let balance = Balance::TopLeft;
    /// assert_eq!(balance.to_bool(), false);
    ///
    /// let balance = Balance::Center;
    /// // This will panic because `Balance::Center` is an uncertain logical state.
    /// // balance.to_bool();
    /// ```
    pub const fn to_bool(self) -> bool {
        self.is_true() || {
            if self.is_false() {
                false
            } else {
                panic!("Cannot convert an uncertain Balance to a boolean value.")
            }
        }
    }

    /// Converts the x-coordinate of the `Balance` position into a boolean representation.
    ///
    /// # Returns
    ///
    /// A `bool` value representing the logical state of the x-coordinate:
    /// - Returns `true` if the x-coordinate is logically `true` (1).
    /// - Returns `false` if the x-coordinate is logically `false` (-1).
    ///
    /// # Panics
    ///
    /// Panics if the x-coordinate is unknown (i.e., `0`), as it cannot be converted
    /// into a boolean value.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let balance = Balance::Right;
    /// assert_eq!(balance.x_to_bool(), true);
    ///
    /// let balance = Balance::Left;
    /// assert_eq!(balance.x_to_bool(), false);
    ///
    /// let balance = Balance::Center;
    /// // This will panic because the x-coordinate is unknown.
    /// // balance.x_to_bool();
    /// ```
    pub const fn x_to_bool(self) -> bool {
        self.x() == 1 || {
            if self.x() == -1 {
                false
            } else {
                panic!("Cannot convert an unknown-x Balance to a boolean value.")
            }
        }
    }

    /// Converts the y-coordinate of the `Balance` position into a boolean representation.
    ///
    /// # Returns
    ///
    /// A `bool` value representing the logical state of the y-coordinate:
    /// - Returns `true` if the y-coordinate is logically `true` (1).
    /// - Returns `false` if the y-coordinate is logically `false` (-1).
    ///
    /// # Panics
    ///
    /// Panics if the y-coordinate is unknown (i.e., `0`), as it cannot be converted
    /// into a boolean value.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let balance = Balance::Top;
    /// assert_eq!(balance.y_to_bool(), false);
    ///
    /// let balance = Balance::Bottom;
    /// assert_eq!(balance.y_to_bool(), true);
    ///
    /// let balance = Balance::Center;
    /// // This will panic because the y-coordinate is unknown.
    /// // balance.y_to_bool();
    /// ```
    pub const fn y_to_bool(self) -> bool {
        self.y() == 1 || {
            if self.y() == -1 {
                false
            } else {
                panic!("Cannot convert an unknown-y Balance to a boolean value.")
            }
        }
    }

    /// Applies [Digit::possibly] on `x` and `y`.
    pub const fn possibly(self) -> Self {
        let (x, y) = self.to_ternary_pair();
        Self::from_ternary_pair(x.possibly(), y.possibly())
    }
    /// Applies [Digit::necessary] on `x` and `y`.
    pub const fn necessary(self) -> Self {
        let (x, y) = self.to_ternary_pair();
        Self::from_ternary_pair(x.necessary(), y.necessary())
    }
    /// Applies [Digit::contingently] on `x` and `y`.
    pub const fn contingently(self) -> Self {
        let (x, y) = self.to_ternary_pair();
        Self::from_ternary_pair(x.contingently(), y.contingently())
    }
    /// Applies [Digit::absolute_positive] on `x` and `y`.
    pub const fn absolute_positive(self) -> Self {
        let (x, y) = self.to_ternary_pair();
        Self::from_ternary_pair(x.absolute_positive(), y.absolute_positive())
    }
    /// Applies [Digit::positive] on `x` and `y`.
    pub const fn positive(self) -> Self {
        let (x, y) = self.to_ternary_pair();
        Self::from_ternary_pair(x.positive(), y.positive())
    }
    /// Applies [Digit::not_negative] on `x` and `y`.
    pub const fn not_negative(self) -> Self {
        let (x, y) = self.to_ternary_pair();
        Self::from_ternary_pair(x.not_negative(), y.not_negative())
    }
    /// Applies [Digit::not_positive] on `x` and `y`.
    pub const fn not_positive(self) -> Self {
        let (x, y) = self.to_ternary_pair();
        Self::from_ternary_pair(x.not_positive(), y.not_positive())
    }
    /// Applies [Digit::negative] on `x` and `y`.
    pub const fn negative(self) -> Self {
        let (x, y) = self.to_ternary_pair();
        Self::from_ternary_pair(x.negative(), y.negative())
    }
    /// Applies [Digit::absolute_negative] on `x` and `y`.
    pub const fn absolute_negative(self) -> Self {
        let (x, y) = self.to_ternary_pair();
        Self::from_ternary_pair(x.absolute_negative(), y.absolute_negative())
    }
    /// Applies [Digit::ht_not] on `x` and `y`.
    pub const fn ht_not(self) -> Self {
        let (x, y) = self.to_ternary_pair();
        Self::from_ternary_pair(x.ht_not(), y.ht_not())
    }
    /// Applies [Digit::post] on `x` and `y`.
    pub const fn post(self) -> Self {
        let (x, y) = self.to_ternary_pair();
        Self::from_ternary_pair(x.post(), y.post())
    }
    /// Applies [Digit::pre] on `x` and `y`.
    pub const fn pre(self) -> Self {
        let (x, y) = self.to_ternary_pair();
        Self::from_ternary_pair(x.pre(), y.pre())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let balances = [
            Balance::TopLeft,
            Balance::Top,
            Balance::TopRight,
            Balance::Left,
            Balance::Center,
            Balance::Right,
            Balance::BottomLeft,
            Balance::Bottom,
            Balance::BottomRight,
        ];
        for balance in balances.iter() {
            let result = balance.pre();
            println!("{:?} -> {:?}", balance, result);
        }
    }
}