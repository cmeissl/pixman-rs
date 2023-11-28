use pixman_sys as ffi;

use crate::fixed::Fixed;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Point(ffi::pixman_point_fixed_t);

impl Point {
    #[inline]
    pub fn new(x: impl Into<Fixed>, y: impl Into<Fixed>) -> Self {
        Self(ffi::pixman_point_fixed {
            x: x.into().into_raw(),
            y: y.into().into_raw(),
        })
    }

    #[inline]
    pub fn x(&self) -> Fixed {
        Fixed::from_raw(self.0.x)
    }

    #[inline]
    pub fn y(&self) -> Fixed {
        Fixed::from_raw(self.0.y)
    }

    #[inline]
    pub(crate) fn as_ptr(&self) -> *const ffi::pixman_point_fixed_t {
        &self.0 as *const ffi::pixman_point_fixed_t
    }
}

impl From<ffi::pixman_point_fixed_t> for Point {
    #[inline]
    fn from(value: ffi::pixman_point_fixed_t) -> Self {
        Self(value)
    }
}

impl From<Point> for ffi::pixman_point_fixed_t {
    #[inline]
    fn from(value: Point) -> Self {
        value.0
    }
}

impl<X, Y> From<(X, Y)> for Point
where
    X: Into<Fixed>,
    Y: Into<Fixed>,
{
    fn from((x, y): (X, Y)) -> Self {
        Self::new(x, y)
    }
}
