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
    /// `TopLeft`: The position at `(-1, -1)`
    TopLeft,
    /// `Top`: The position at `(0, -1)`
    Top,
    /// `TopRight`: The position at `(1, -1)`
    TopRight,
    /// `Left`: The position at `(-1, 0)`
    Left,
    /// `Center`: The central position `(0, 0)`
    Center,
    /// `Right`: The position at `(1, 0)`
    Right,
    /// `BottomLeft`: The position at `(-1, 1)`
    BottomLeft,
    /// `Bottom`: The position at `(0, 1)`
    Bottom,
    /// `BottomRight`: The position at `(1, 1)`
    BottomRight,
}

impl Balance {
    /// Returns the x-coordinate of the current `Balance` position in the 3x3 grid.
    ///
    /// # Returns
    ///
    /// An `i8` value representing the x-coordinate of the position.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let position = Balance::Right;
    /// assert_eq!(position.x(), 1);
    ///
    /// let position = Balance::Center;
    /// assert_eq!(position.x(), 0);
    /// ```
    pub const fn x(self) -> i8 {
        if self.has_left() {
            -1
        } else if self.has_right() {
            1
        } else {
            0
        }
    }

    /// Returns the y-coordinate of the current `Balance` position in the 3x3 grid.
    ///
    /// # Returns
    ///
    /// An `i8` value representing the y-coordinate of the position.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let position = Balance::Bottom;
    /// assert_eq!(position.y(), 1);
    ///
    /// let position = Balance::Center;
    /// assert_eq!(position.y(), 0);
    /// ```
    pub const fn y(self) -> i8 {
        if self.has_top() {
            -1
        } else if self.has_bottom() {
            1
        } else {
            0
        }
    }

    /// (spatial) Checks if the current position has the `Balance::Top` variant or any variant
    /// that includes the top row in the 3x3 grid.
    pub const fn has_top(self) -> bool {
        matches!(self, Balance::Top | Balance::TopLeft | Balance::TopRight)
    }

    /// (spatial) Checks if the current position has the `Balance::Bottom` variant or any variant
    /// that includes the bottom row in the 3x3 grid.
    pub const fn has_bottom(self) -> bool {
        matches!(
            self,
            Balance::Bottom | Balance::BottomLeft | Balance::BottomRight
        )
    }

    /// (spatial) Checks if the current position has the `Balance::Left` variant or any variant
    /// that includes the left row in the 3x3 grid.
    pub const fn has_left(self) -> bool {
        matches!(self, Balance::Left | Balance::TopLeft | Balance::BottomLeft)
    }

    /// (spatial) Checks if the current position has the `Balance::Right` variant or any variant
    /// that includes the right column in the 3x3 grid.
    pub const fn has_right(self) -> bool {
        matches!(
            self,
            Balance::Right | Balance::TopRight | Balance::BottomRight
        )
    }

    /// (spatial) Checks if the current position includes the center or any direct neighbor
    /// (top, bottom, left, or right) in the 3x3 grid.
    pub const fn is_orthogonal(self) -> bool {
        matches!(
            self,
            Balance::Center | Balance::Top | Balance::Bottom | Balance::Left | Balance::Right
        )
    }

    /// (spatial) Checks if the current position includes the center or any indirect neighbor
    /// (corners) in the 3x3 grid.
    pub const fn is_diagonal(self) -> bool {
        matches!(
            self,
            Balance::Center
                | Balance::TopLeft
                | Balance::TopRight
                | Balance::BottomLeft
                | Balance::BottomRight
        )
    }

    /// (spatial) Determines whether the current position is one of the edge positions
    /// (top, bottom, left, or right) in the 3x3 grid.
    pub const fn is_edge(self) -> bool {
        matches!(
            self,
            Balance::Top | Balance::Bottom | Balance::Left | Balance::Right
        )
    }

    /// (spatial) Checks if the current position is one of the corner positions
    /// (top-left, top-right, bottom-left, or bottom-right) in the 3x3 grid.
    pub const fn is_corner(self) -> bool {
        matches!(
            self,
            Balance::TopLeft | Balance::TopRight | Balance::BottomLeft | Balance::BottomRight
        )
    }


    /// Converts the current `Balance` position into a symbol representation.
    ///
    /// # Returns
    ///
    /// A `&'static str` that visually represents the position using an emoji.
    /// Each position in the 3x3 grid is mapped to a unique symbol:
    ///
    /// - `TopLeft`: "↖️"
    /// - `Top`: "⬆️"
    /// - `TopRight`: "↗️"
    /// - `Left`: "⬅️"
    /// - `Center`: "⏺️"
    /// - `Right`: "➡️"
    /// - `BottomLeft`: "↙️"
    /// - `Bottom`: "⬇️"
    /// - `BottomRight`: "↘️"
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let position = Balance::Top;
    /// assert_eq!(position.to_symbol(), "⬆️");
    ///
    /// let position = Balance::Center;
    /// assert_eq!(position.to_symbol(), "⏺️");
    /// ```
    pub const fn to_symbol(self) -> &'static str {
        match self {
            Balance::TopLeft => "↖️",
            Balance::Top => "⬆️",
            Balance::TopRight => "↗️",
            Balance::Left => "⬅️",
            Balance::Center => "⏺️",
            Balance::Right => "➡️",
            Balance::BottomLeft => "↙️",
            Balance::Bottom => "⬇️",
            Balance::BottomRight => "↘️",
        }
    }
}
