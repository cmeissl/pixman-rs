use pixman_sys as ffi;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Fixed(ffi::pixman_fixed_t);

impl Fixed {
    pub const ONE: Fixed = Fixed::from_int(1);
    pub const ZERO: Fixed = Fixed::from_raw(0);

    #[inline]
    pub const fn from_raw(value: ffi::pixman_fixed_t) -> Self {
        Self(value)
    }

    #[inline]
    #[doc(alias = "pixman_int_to_fixed")]
    pub const fn from_int(value: i32) -> Self {
        Self((value << 16) as ffi::pixman_fixed_t)
    }

    #[inline]
    #[doc(alias = "pixman_double_to_fixed")]
    pub fn from_double(value: f64) -> Self {
        Self((value * 65536.0) as ffi::pixman_fixed_t)
    }

    #[inline]
    #[doc(alias = "pixman_fixed_to_int")]
    pub const fn to_int(self) -> i32 {
        self.0 >> 16
    }

    #[inline]
    pub fn into_raw(self) -> ffi::pixman_fixed_t {
        self.0
    }
}

impl std::ops::Add for Fixed {
    type Output = Fixed;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl From<i32> for Fixed {
    #[inline]
    fn from(value: i32) -> Self {
        Self::from_int(value)
    }
}

impl From<f32> for Fixed {
    #[inline]
    fn from(value: f32) -> Self {
        Self::from_double(value as f64)
    }
}

impl From<f64> for Fixed {
    #[inline]
    fn from(value: f64) -> Self {
        Self::from_double(value)
    }
}
