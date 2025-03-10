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

    /// (logic) Checks if the current logical state is `Balance::BottomRight` (True, True).
    pub const fn is_true(self) -> bool {
        matches!(self, Balance::BottomRight)
    }

    /// (logic) Checks if the current logical state includes `Balance::Bottom` or `Balance::Right`.
    ///
    /// * [Balance::TopRight] (True, False)
    /// * [Balance::Right] (True, Unknown)
    /// * [Balance::BottomLeft] (False, True)
    /// * [Balance::Bottom] (Unknown, True)
    /// * [Balance::BottomRight] (True, True)
    pub const fn has_true(self) -> bool {
        self.x() == 1 || self.y() == 1
    }

    /// (logic) Checks if the current logical state is contradictory, representing opposing truths (`TopRight` or `BottomLeft`).
    ///
    /// * [Balance::TopRight] (True, False)
    /// * [Balance::BottomLeft] (False, True)
    pub const fn is_contradictory(self) -> bool {
        matches!(self, Balance::TopRight | Balance::BottomLeft)
    }

    /// (logic) Checks whether the current logical state has no certain value but is not contradictory.
    ///
    /// * [Balance::Top] (Unknown, False)
    /// * [Balance::Left] (False, Unknown)
    /// * [Balance::Center] (Unknown, Unknown)
    /// * [Balance::Right] (True, Unknown)
    /// * [Balance::Bottom] (Unknown, True)
    pub const fn has_unknown(self) -> bool {
        // = self.is_orthogonal().
        self.x() == 0 || self.y() == 0
    }

    /// (logic) Checks whether the current logical state is uncertain in terms of logical balance.
    ///
    /// Equals:
    /// * `is_contradictory() || has_unknown()`,
    /// * or `!is_certain()`.
    ///
    /// These cases return `true`:
    /// * [Balance::TopRight] (True, False)
    /// * [Balance::BottomLeft] (False, True)
    /// * [Balance::Top] (Unknown, False)
    /// * [Balance::Left] (False, Unknown)
    /// * [Balance::Center] (Unknown, Unknown)
    /// * [Balance::Right] (True, Unknown)
    /// * [Balance::Bottom] (Unknown, True)
    pub const fn is_uncertain(self) -> bool {
        !self.is_certain()
    }

    /// (logic) Returns whether the current logical state represents a certain state in logical balance (one of `is_true()` or `is_false()` is true).
    ///
    /// * [Balance::TopLeft] (False, False)
    /// * [Balance::BottomRight] (True, True)
    pub const fn is_certain(self) -> bool {
        matches!(self, Balance::BottomRight | Balance::TopLeft)
    }

    /// (logic) Determines whether the current logical state includes the `Balance::Top` or `Balance::Left` variant.
    ///
    /// * [Balance::TopLeft] (False, False)
    /// * [Balance::Top] (Unknown, False)
    /// * [Balance::TopRight] (True, False)
    /// * [Balance::Left] (False, Unknown)
    /// * [Balance::BottomLeft] (False, True)
    pub const fn has_false(self) -> bool {
        self.x() == -1 || self.y() == -1
    }

    /// (logic) Checks if the current logical state is `Balance::TopLeft` (False, False).
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
    /// Applies [Digit::k3_imply] on `x` and `y`.
    pub const fn k3_imply(self, other: Self) -> Self {
        let (x1, y1) = self.to_ternary_pair();
        let (x2, y2) = other.to_ternary_pair();
        Self::from_ternary_pair(x1.k3_imply(x2), y1.k3_imply(y2))
    }
    /// Applies [Digit::k3_equiv] on `x` and `y`.
    ///
    /// Equivalent to `self * other`.
    pub const fn k3_equiv(self, other: Self) -> Self {
        let (x1, y1) = self.to_ternary_pair();
        let (x2, y2) = other.to_ternary_pair();
        Self::from_ternary_pair(x1.k3_equiv(x2), y1.k3_equiv(y2))
    }
    /// Applies [Digit::ht_imply] on `x` and `y`.
    pub const fn ht_imply(self, other: Self) -> Self {
        let (x1, y1) = self.to_ternary_pair();
        let (x2, y2) = other.to_ternary_pair();
        Self::from_ternary_pair(x1.ht_imply(x2), y1.ht_imply(y2))
    }

    /// Applies a transformation function `x` and another on `y` coordinates.
    ///
    /// This function allows you to provide two different transformation functions,
    /// one for `x` (`op_x`) and another for `y` (`op_y`). These functions
    /// process the coordinates independently, and the resulting transformed
    /// `x` and `y` coordinates are combined into a new `Balance`.
    ///
    /// # Parameters
    ///
    /// - `op_x`: A function that transforms the `x` coordinate.
    /// - `op_y`: A function that transforms the `y` coordinate.
    ///
    /// # Returns
    ///
    /// A new `Balance` object with the transformed `x` and `y` coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_ternary::Digit;
    /// use balanced_direction::Balance;
    ///
    /// let balance = Balance::Center;
    /// let transformed = balance.apply(Digit::not_negative, Digit::not_positive);
    /// assert_eq!(transformed, Balance::TopRight);
    /// ```
    pub fn apply<FX, FY>(self, op_x: FX, op_y: FY) -> Self
    where
        FX: Fn(Digit) -> Digit,
        FY: Fn(Digit) -> Digit,
    {
        let (x, y) = self.to_ternary_pair();
        Self::from_ternary_pair(op_x(x), op_y(y))
    }

    /// Applies two transformation functions - one for `x` and one for `y` - on a `Balance` instance along with another `Balance`.
    ///
    /// This function takes two coordinate transformation functions (`op_x` and `op_y`) as well as another `Balance` instance (`other`).
    /// It applies `op_x` to the `x` coordinates of both balances, and `op_y` to the `y` coordinates of both balances, combining the results into a new `Balance`.
    ///
    /// # Parameters
    ///
    /// - `op_x`: A function that transforms the `x` coordinates of both balances.
    /// - `op_y`: A function that transforms the `y` coordinates of both balances.
    /// - `other`: The second `Balance` object to use for coordinate transformations.
    ///
    /// # Returns
    ///
    /// A new `Balance` object with transformed `x` and `y` based on the provided functions and the two input balances.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_ternary::Digit;
    /// use balanced_direction::Balance;
    ///
    /// let balance1 = Balance::TopRight;
    /// let balance2 = Balance::BottomLeft;
    ///
    /// let result = balance1.apply_with(
    ///     |x1, x2| x1.k3_equiv(x2),
    ///     |y1, y2| y1.k3_imply(y2),
    ///     balance2
    /// );
    ///
    /// assert_eq!(result, Balance::BottomLeft);
    /// ```
    pub fn apply_with<FX, FY>(self, op_x: FX, op_y: FY, other: Self) -> Self
    where
        FX: Fn(Digit, Digit) -> Digit,
        FY: Fn(Digit, Digit) -> Digit,
    {
        let (x1, y1) = self.to_ternary_pair();
        let (x2, y2) = other.to_ternary_pair();
        Self::from_ternary_pair(op_x(x1, x2), op_y(y1, y2))
    }
    /// Applies the given transformation on both `x` and `y`.
    /// 
    /// See [Balance::apply].
    pub fn apply_both<F>(self, op: F) -> Self
    where
        F: Fn(Digit) -> Digit + Clone,
    {
        self.apply(op.clone(), op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_eq_array;

    const BALANCES: [Balance; 9] = [
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

    #[test]
    fn test_possibly() {
        let results = BALANCES.map(Balance::possibly);
        assert_eq_array(
            results,
            [
                Balance::TopLeft,
                Balance::TopRight,
                Balance::TopRight,
                Balance::BottomLeft,
                Balance::BottomRight,
                Balance::BottomRight,
                Balance::BottomLeft,
                Balance::BottomRight,
                Balance::BottomRight,
            ],
        );
    }

    #[test]
    fn test_necessary() {
        let results = BALANCES.map(Balance::necessary);
        assert_eq_array(
            results,
            [
                Balance::TopLeft,
                Balance::TopLeft,
                Balance::TopRight,
                Balance::TopLeft,
                Balance::TopLeft,
                Balance::TopRight,
                Balance::BottomLeft,
                Balance::BottomLeft,
                Balance::BottomRight,
            ],
        );
    }

    #[test]
    fn test_contingently() {
        let results = BALANCES.map(Balance::contingently);
        assert_eq_array(
            results,
            [
                Balance::TopLeft,
                Balance::TopRight,
                Balance::TopLeft,
                Balance::BottomLeft,
                Balance::BottomRight,
                Balance::BottomLeft,
                Balance::TopLeft,
                Balance::TopRight,
                Balance::TopLeft,
            ],
        );
    }

    #[test]
    fn test_absolute_positive() {
        assert_eq_array(
            BALANCES.map(Balance::absolute_positive),
            [
                Balance::BottomRight,
                Balance::Bottom,
                Balance::BottomRight,
                Balance::Right,
                Balance::Center,
                Balance::Right,
                Balance::BottomRight,
                Balance::Bottom,
                Balance::BottomRight,
            ],
        );
    }

    #[test]
    fn test_positive() {
        assert_eq_array(
            BALANCES.map(Balance::positive),
            [
                Balance::Center,
                Balance::Center,
                Balance::Right,
                Balance::Center,
                Balance::Center,
                Balance::Right,
                Balance::Bottom,
                Balance::Bottom,
                Balance::BottomRight,
            ],
        );
    }

    #[test]
    fn test_not_negative() {
        assert_eq_array(
            BALANCES.map(Balance::not_negative),
            [
                Balance::Center,
                Balance::Right,
                Balance::Right,
                Balance::Bottom,
                Balance::BottomRight,
                Balance::BottomRight,
                Balance::Bottom,
                Balance::BottomRight,
                Balance::BottomRight,
            ],
        );
    }

    #[test]
    fn test_not_positive() {
        assert_eq_array(
            BALANCES.map(Balance::not_positive),
            [
                Balance::TopLeft,
                Balance::TopLeft,
                Balance::Top,
                Balance::TopLeft,
                Balance::TopLeft,
                Balance::Top,
                Balance::Left,
                Balance::Left,
                Balance::Center,
            ],
        );
    }

    #[test]
    fn test_negative() {
        assert_eq_array(
            BALANCES.map(Balance::negative),
            [
                Balance::TopLeft,
                Balance::Top,
                Balance::Top,
                Balance::Left,
                Balance::Center,
                Balance::Center,
                Balance::Left,
                Balance::Center,
                Balance::Center,
            ],
        );
    }

    #[test]
    fn test_absolute_negative() {
        assert_eq_array(
            BALANCES.map(Balance::absolute_negative),
            [
                Balance::TopLeft,
                Balance::Top,
                Balance::TopLeft,
                Balance::Left,
                Balance::Center,
                Balance::Left,
                Balance::TopLeft,
                Balance::Top,
                Balance::TopLeft,
            ],
        );
    }

    #[test]
    fn test_ht_not() {
        assert_eq_array(
            BALANCES.map(Balance::ht_not),
            [
                Balance::BottomRight,
                Balance::BottomLeft,
                Balance::BottomLeft,
                Balance::TopRight,
                Balance::TopLeft,
                Balance::TopLeft,
                Balance::TopRight,
                Balance::TopLeft,
                Balance::TopLeft,
            ],
        );
    }

    #[test]
    fn test_post() {
        assert_eq_array(
            BALANCES.map(Balance::post),
            [
                Balance::Center,
                Balance::Right,
                Balance::Left,
                Balance::Bottom,
                Balance::BottomRight,
                Balance::BottomLeft,
                Balance::Top,
                Balance::TopRight,
                Balance::TopLeft,
            ],
        );
    }

    #[test]
    fn test_pre() {
        assert_eq_array(
            BALANCES.map(Balance::pre),
            [
                Balance::BottomRight,
                Balance::BottomLeft,
                Balance::Bottom,
                Balance::TopRight,
                Balance::TopLeft,
                Balance::Top,
                Balance::Right,
                Balance::Left,
                Balance::Center,
            ],
        );
    }
}
