//! Implementation of the general laser point type used throughout the crate.

use std::hash::{Hash, Hasher};

/// A position in 2D space represented by x and y coordinates.
pub type Position = [f32; 2];

/// Red, green and blue channels of a single colour.
pub type Rgb = [f32; 3];

/// The point type used within the laser frame stream API.
///
/// The point represents the location to which the scanner should point and the colour that the
/// scanner should be at this point.
///
/// If two consecutive points have two different colours, the `color` values will be linearly
/// interpolated.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    /// The position of the point. `-1` represents the minimum value along the axis and `1`
    /// represents the maximum.
    pub position: Position,
    /// The color of the point.
    pub color: Rgb,
    /// The minimum number of extra times this point should be drawn.
    ///
    /// `0` is the default used for drawing sequences of smooth line segments.
    ///
    /// Values greater than `0` are useful for accenting individual points.
    pub weight: u32,
}

/// The **Point** type used for describing raw laser streams.
///
/// The point represents the location to which the scanner should point and the colour that the
/// scanner should be at this point.
///
/// If two consecutive points have two different colours, the `color` values will be linearly
/// interpolated.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RawPoint {
    /// The position of the point. `-1` represents the minimum value along the axis and `1`
    /// represents the maximum.
    pub position: Position,
    /// The color of the point.
    pub color: Rgb,
}

impl Point {
    /// The default weight for points used to draw lines.
    pub const DEFAULT_LINE_POINT_WEIGHT: u32 = 0;

    /// Create a **Point** at the given position with the given colour with a default weight.
    pub fn new(position: Position, color: Rgb) -> Self {
        Point::with_weight(position, color, Self::DEFAULT_LINE_POINT_WEIGHT)
    }

    /// The same as `Point::new` but allows for specifying the weight of the point.
    pub fn with_weight(position: Position, color: Rgb, weight: u32) -> Self {
        Point {
            position,
            color,
            weight,
        }
    }

    /// Create a blank point at `[0, 0]`.
    pub fn centered_blank() -> Self {
        Point::new([0.0, 0.0], [0.0, 0.0, 0.0])
    }

    /// Returns a point with the same position as `self` but with a black (blank) color.
    pub fn blanked(&self) -> Self {
        let mut blanked = *self;
        blanked.color = [0.0, 0.0, 0.0];
        blanked
    }

    /// Whether or not the point is blank.
    ///
    /// A point is considered blank if the colour is black.
    pub fn is_blank(&self) -> bool {
        color_is_blank(self.color)
    }

    /// Converts to a single raw point with the same position and color.
    pub fn to_raw(&self) -> RawPoint {
        RawPoint::new(self.position, self.color)
    }

    /// Converts to `weight` number of raw points with the same position and color.
    pub fn to_raw_weighted(&self) -> impl Iterator<Item = RawPoint> {
        let Point {
            position,
            color,
            weight,
        } = *self;
        (0..weight).map(move |_| RawPoint::new(position, color))
    }
}

impl RawPoint {
    /// Create a **Point** at the given position with the given colour.
    pub fn new(position: Position, color: Rgb) -> Self {
        RawPoint { position, color }
    }

    /// Convert to a point compatible with a laser *frame* stream with the given weight.
    pub fn with_weight(&self, weight: u32) -> Point {
        Point::with_weight(self.position, self.color, weight)
    }

    /// Create a blank point at `[0, 0]`.
    pub fn centered_blank() -> Self {
        RawPoint::new([0.0, 0.0], [0.0, 0.0, 0.0])
    }

    /// Returns a point with the same position as `self` but with a black (blank) color.
    pub fn blanked(&self) -> Self {
        let mut blanked = *self;
        blanked.color = [0.0, 0.0, 0.0];
        blanked
    }

    /// Whether or not the point is blank.
    ///
    /// A point is considered blank if the colour is black.
    pub fn is_blank(&self) -> bool {
        color_is_blank(self.color)
    }
}

impl lasy::Lerp for RawPoint {
    type Scalar = f32;
    fn lerp(&self, other: &Self, amt: f32) -> Self {
        RawPoint::new(
            self.position.lerp(&other.position, amt),
            self.color.lerp(&other.color, amt),
        )
    }
}

impl lasy::IsBlank for Point {
    fn is_blank(&self) -> bool {
        color_is_blank(self.color)
    }
}

impl lasy::Position for Point {
    fn position(&self) -> [f32; 2] {
        self.position
    }
}

impl lasy::Weight for Point {
    fn weight(&self) -> u32 {
        self.weight
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        // A hashable version of a `Point`, used for removing point duplicates during euler graph
        // generation.
        #[derive(Eq, Hash, PartialEq)]
        struct HashPoint {
            pos: [i32; 2],
            rgb: [u32; 3],
        }

        impl From<Point> for HashPoint {
            fn from(p: Point) -> Self {
                let [px, py] = p.position;
                let [pr, pg, pb] = p.color;
                let x = (px * std::i16::MAX as f32) as i32;
                let y = (py * std::i16::MAX as f32) as i32;
                let r = (pr * std::u16::MAX as f32) as u32;
                let g = (pg * std::u16::MAX as f32) as u32;
                let b = (pb * std::u16::MAX as f32) as u32;
                let pos = [x, y];
                let rgb = [r, g, b];
                HashPoint { pos, rgb }
            }
        }

        HashPoint::from(*self).hash(hasher);
    }
}

impl lasy::Blanked for RawPoint {
    fn blanked(&self) -> Self {
        RawPoint::blanked(self)
    }
}

impl From<Point> for RawPoint {
    fn from(p: Point) -> Self {
        p.to_raw()
    }
}

/// Whether or not the given point is blank (black).
pub fn color_is_blank([r, g, b]: Rgb) -> bool {
    r == 0.0 && g == 0.0 && b == 0.0
}
