use crate::{ffi, Point};

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Line(ffi::pixman_line_fixed_t);

impl Line {
    #[inline]
    pub fn new(p1: impl Into<Point>, p2: impl Into<Point>) -> Self {
        Self(ffi::pixman_line_fixed_t {
            p1: p1.into().into(),
            p2: p2.into().into(),
        })
    }

    #[inline]
    pub fn p1(&self) -> Point {
        Point::from(self.0.p1)
    }

    #[inline]
    pub fn p2(&self) -> Point {
        Point::from(self.0.p2)
    }

    #[inline]
    pub(crate) fn as_ptr(&self) -> *const ffi::pixman_line_fixed_t {
        &self.0
    }
}

impl From<ffi::pixman_line_fixed_t> for Line {
    #[inline]
    fn from(value: ffi::pixman_line_fixed_t) -> Self {
        Self(value)
    }
}

impl From<Line> for ffi::pixman_line_fixed_t {
    #[inline]
    fn from(value: Line) -> Self {
        value.0
    }
}

impl<P: Into<Point> + Copy> From<[P; 2]> for Line {
    #[inline]
    fn from(value: [P; 2]) -> Self {
        Self::new(value[0], value[1])
    }
}
