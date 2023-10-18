use pixman_sys as ffi;

use crate::{Color, Fixed};

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct GradientStop(ffi::pixman_gradient_stop_t);

impl GradientStop {
    #[inline]
    pub fn new(x: impl Into<Fixed>, color: impl Into<Color>) -> Self {
        Self(ffi::pixman_gradient_stop {
            x: x.into().into_raw(),
            color: color.into().into(),
        })
    }

    #[inline]
    pub fn x(&self) -> Fixed {
        Fixed::from_raw(self.0.x)
    }

    #[inline]
    pub fn color(&self) -> Color {
        Color::from(self.0.color)
    }
}

impl From<ffi::pixman_gradient_stop_t> for GradientStop {
    #[inline]
    fn from(value: ffi::pixman_gradient_stop_t) -> Self {
        Self(value)
    }
}

impl From<GradientStop> for ffi::pixman_gradient_stop_t {
    #[inline]
    fn from(value: GradientStop) -> Self {
        value.0
    }
}