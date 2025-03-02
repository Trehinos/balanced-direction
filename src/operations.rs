use crate::Balance;
use core::ops::{Add, Mul, Neg, Not, Sub};

impl Balance {
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
    pub const fn up(self) -> Self {
        let (x, y) = self.to_vector();
        Self::from_vector(x, if y == -1 { -1 } else { y - 1 })
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
    pub const fn down(self) -> Self {
        let (x, y) = self.to_vector();
        Self::from_vector(x, if y == 1 { 1 } else { y + 1 })
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
    pub const fn left(self) -> Self {
        let (x, y) = self.to_vector();
        Self::from_vector(if x == -1 { -1 } else { x - 1 }, y)
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
    pub const fn right(self) -> Self {
        let (x, y) = self.to_vector();
        Self::from_vector(if x == 1 { 1 } else { x + 1 }, y)
    }

    /// Moves the position upwards in the 3x3 grid with wrapping behavior.
    pub const fn up_wrap(self) -> Self {
        let (x, y) = self.to_vector();
        Self::from_vector(x, if y == -1 { 1 } else { y - 1 })
    }

    /// Moves the position downwards in the 3x3 grid with wrapping behavior.
    pub const fn down_wrap(self) -> Self {
        let (x, y) = self.to_vector();
        Self::from_vector(x, if y == 1 { -1 } else { y + 1 })
    }

    /// Moves the position leftwards in the 3x3 grid with wrapping behavior.
    pub const fn left_wrap(self) -> Self {
        let (x, y) = self.to_vector();
        Self::from_vector(if x == -1 { 1 } else { x - 1 }, y)
    }

    /// Moves the position rightwards in the 3x3 grid with wrapping behavior.
    pub const fn right_wrap(self) -> Self {
        let (x, y) = self.to_vector();
        Self::from_vector(if x == 1 { -1 } else { x + 1 }, y)
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
    pub const fn flip_h(self) -> Self {
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
    pub const fn flip_v(self) -> Self {
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
    pub const fn rotate_left(self) -> Self {
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
    pub const fn rotate_right(self) -> Self {
        let (x, y) = self.to_vector();
        Self::from_vector(-y, x)
    }

    /// Centers the current position horizontally in the 3x3 grid by setting the x-coordinate to 0.
    ///
    /// # Returns
    ///
    /// A `Balance` variant where the x-coordinate is always 0, keeping the same y-coordinate.
    /// This effectively moves the current position to the vertical axis of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let balance = Balance::Left;
    /// assert_eq!(balance.center_h(), Balance::Center);
    ///
    /// let balance = Balance::BottomLeft;
    /// assert_eq!(balance.center_h(), Balance::Bottom);
    /// ```
    pub const fn center_h(self) -> Self {
        let (_, y) = self.to_vector();
        Self::from_vector(0, y)
    }

    /// Centers the current position vertically in the 3x3 grid by setting the y-coordinate to 0.
    ///
    /// # Returns
    ///
    /// A `Balance` variant where the y-coordinate is always 0, keeping the same x-coordinate.
    /// This effectively moves the current position to the horizontal axis of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let balance = Balance::Top;
    /// assert_eq!(balance.center_v(), Balance::Center);
    ///
    /// let balance = Balance::TopRight;
    /// assert_eq!(balance.center_v(), Balance::Right);
    /// ```
    pub const fn center_v(self) -> Self {
        let (x, _) = self.to_vector();
        Self::from_vector(x, 0)
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
