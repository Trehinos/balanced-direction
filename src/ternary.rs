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
    pub fn to_ternary_pair(self) -> (Digit, Digit) {
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
    pub fn from_ternary_pair(a: Digit, b: Digit) -> Self {
        Self::from_vector(a.to_i8(), b.to_i8())
    }
}
