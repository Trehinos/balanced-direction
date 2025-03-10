use crate::Balance;

impl Balance {
    pub const EAST: f64 = 0.0;
    pub const NORTH_EAST: f64 = 45.0;
    pub const NORTH: f64 = 90.0;
    pub const NORTH_WEST: f64 = 135.0;
    pub const WEST: f64 = 180.0;
    pub const SOUTH_EAST: f64 = -45.0;
    pub const SOUTH: f64 = -90.0;
    pub const SOUTH_WEST: f64 = -135.0;

    /// Returns a unique integer value associated with each `Balance` variant.
    ///
    /// This mapping assigns a unique value to each position in the 3x3 grid,
    /// which could be useful for serialization, indexing, or logical calculations
    /// that depend on the position.
    ///
    /// # Returns
    ///
    /// An `i8` integer representing the `Balance` variant.
    ///
    /// # Mapping
    ///
    /// - `Balance::TopLeft` => `-4`
    /// - `Balance::Top` => `-3`
    /// - `Balance::TopRight` => `-2`
    /// - `Balance::Left` => `-1`
    /// - `Balance::Center` => `0`
    /// - `Balance::Right` => `1`
    /// - `Balance::BottomLeft` => `2`
    /// - `Balance::Bottom` => `3`
    /// - `Balance::BottomRight` => `4`
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let position = Balance::Center;
    /// assert_eq!(position.to_value(), 0);
    ///
    /// let position = Balance::TopRight;
    /// assert_eq!(position.to_value(), -2);
    /// ```
    pub const fn to_value(self) -> i8 {
        match self {
            Balance::TopLeft => -4,
            Balance::Top => -3,
            Balance::TopRight => -2,
            Balance::Left => -1,
            Balance::Center => 0,
            Balance::Right => 1,
            Balance::BottomLeft => 2,
            Balance::Bottom => 3,
            Balance::BottomRight => 4,
        }
    }

    /// Constructs a `Balance` variant from a given `i8` value.
    ///
    /// This method maps an integer value to a specific `Balance` variant.
    /// Values outside the valid range will cause a panic.
    ///
    /// # Arguments
    ///
    /// - `value` - An `i8` integer value corresponding to a `Balance` variant.
    ///
    /// # Returns
    ///
    /// A `Balance` instance mapped from the provided integer.
    ///
    /// # Panics
    ///
    /// This function will panic if the input value is not in the range `-4..=4`.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let position = Balance::from_value(-4);
    /// assert_eq!(position, Balance::TopLeft);
    ///
    /// let position = Balance::from_value(0);
    /// assert_eq!(position, Balance::Center);
    ///
    /// // This will panic:
    /// // let invalid = Balance::from_value(5);
    /// ```
    pub const fn from_value(value: i8) -> Self {
        match value {
            -4 => Balance::TopLeft,
            -3 => Balance::Top,
            -2 => Balance::TopRight,
            -1 => Balance::Left,
            0 => Balance::Center,
            1 => Balance::Right,
            2 => Balance::BottomLeft,
            3 => Balance::Bottom,
            4 => Balance::BottomRight,
            _ => panic!("Invalid value"),
        }
    }

    /// Calculates the scalar magnitude squared for the vector representation
    /// of the current `Balance` position within the grid.
    ///
    /// The scalar magnitude squared is defined as `x^2 + y^2`, where `(x, y)`
    /// are the coordinates of the position.
    ///
    /// # Returns
    ///
    /// An `i8` value representing the scalar magnitude squared of the position.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let position = Balance::TopLeft;
    /// assert_eq!(position.to_scalar(), 2);
    ///
    /// let center = Balance::Center;
    /// assert_eq!(center.to_scalar(), 0);
    /// ```
    pub const fn to_scalar(self) -> i8 {
        let (x, y) = self.to_vector();
        x * x + y * y
    }

    /// Calculates the Euclidean (or absolute) magnitude of the vector representation
    /// of the current `Balance` position.
    ///
    /// The magnitude is defined as the square root of the scalar magnitude squared (`√(x² + y²)`),
    /// where `(x, y)` are the coordinates of the position.
    ///
    /// # Returns
    ///
    /// A `f64` value representing the Euclidean magnitude of the position with low precision (but fast) calculus.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let position = Balance::TopLeft;
    /// assert_eq!(position.to_magnitude(), 2.0f64.sqrt());
    ///
    /// let center = Balance::Center;
    /// assert_eq!(center.to_magnitude(), 0.0);
    /// ```
    pub const fn to_magnitude(self) -> f64 {
        if self.is_corner() {
            core::f64::consts::SQRT_2
        } else if self.is_edge() {
            1.0
        } else {
            0.0
        }
    }

    /// Converts the current `Balance` position into its corresponding
    /// angle in degrees in a Cartesian coordinate system.
    ///
    /// The angle is returned in the range `[-180.0, 180.0]` degrees.
    ///
    /// # Returns
    ///
    /// A `f64` value representing the angle in degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let position = Balance::Top;
    /// assert_eq!(position.to_angle(), 90.0);
    /// ```
    pub const fn to_angle(self) -> f64 {
        match self {
            Balance::TopLeft => Self::NORTH_WEST,
            Balance::Top => Self::NORTH,
            Balance::TopRight => Self::NORTH_EAST,
            Balance::Left => Self::WEST,
            Balance::Right => Self::EAST,
            Balance::BottomLeft => Self::SOUTH_WEST,
            Balance::Bottom => Self::SOUTH,
            Balance::BottomRight => Self::SOUTH_EAST,
            _ => panic!("Invalid value: cannot convert Balance::Center to an angle."),
        }
    }

    /// Constructs a `Balance` enum variant based on the given angle in degrees.
    ///
    /// # Parameters
    ///
    /// - `angle`: A `f64` value representing the angle in degrees.
    ///
    /// # Returns
    ///
    /// A `Balance` enum variant corresponding to the direction indicated by the angle.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_direction::Balance;
    ///
    /// let balance = Balance::from_angle(45.0);
    /// assert_eq!(balance, Balance::TopRight);
    ///
    /// let balance = Balance::from_angle(-135.0);
    /// assert_eq!(balance, Balance::BottomLeft);
    ///
    /// let balance = Balance::from_angle(270.0);
    /// assert_eq!(balance, Balance::Bottom);
    /// ```
    pub const fn from_angle(angle: f64) -> Self {
        let mut angle = angle % 360.0;
        if angle > 180.0 {
            angle = -(360.0 - angle);
        }
        match angle {
            Self::EAST => Balance::Right,
            Self::NORTH_EAST => Balance::TopRight,
            Self::NORTH => Balance::Top,
            Self::NORTH_WEST => Balance::TopLeft,
            Self::WEST => Balance::Left,
            Self::SOUTH_WEST => Balance::BottomLeft,
            Self::SOUTH => Balance::Bottom,
            Self::SOUTH_EAST => Balance::BottomRight,
            _ => panic!("Invalid angle. Cannot construct a Balance from an approximate angle."),
        }
    }

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
    pub const fn to_vector(self) -> (i8, i8) {
        (self.x(), self.y())
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
    pub const fn from_vector(a: i8, b: i8) -> Self {
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
}
