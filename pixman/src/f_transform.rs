use std::mem::MaybeUninit;

use crate::{ffi, Box16, FVector, Transform};

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FTransform(ffi::pixman_f_transform_t);

impl FTransform {
    #[inline]
    pub fn new(m: [[f64; 3]; 3]) -> Self {
        let m = [
            [m[0][0], m[0][1], m[0][2]],
            [m[1][0], m[1][1], m[1][2]],
            [m[2][0], m[2][1], m[2][2]],
        ];
        Self(ffi::pixman_f_transform_t { m })
    }

    pub fn transform_bounds(&self, mut b: Box16) -> Option<Box16> {
        let res = unsafe { ffi::pixman_f_transform_bounds(self.as_ptr(), &mut b) };
        if res == 1 {
            Some(b)
        } else {
            None
        }
    }

    #[inline]
    pub fn identity() -> Self {
        let mut transform = MaybeUninit::uninit();
        unsafe {
            ffi::pixman_f_transform_init_identity(transform.as_mut_ptr());
        }
        Self(unsafe { transform.assume_init() })
    }

    #[inline]
    #[doc(alias = "pixman_f_transform_init_rotate")]
    pub fn from_rotation(cos: f64, sin: f64) -> Self {
        let mut transform = MaybeUninit::uninit();
        unsafe {
            ffi::pixman_f_transform_init_rotate(transform.as_mut_ptr(), cos, sin);
        }
        Self(unsafe { transform.assume_init() })
    }

    #[inline]
    #[doc(alias = "pixman_f_transform_init_scale")]
    pub fn from_scale(sx: f64, sy: f64) -> Self {
        let mut transform = MaybeUninit::uninit();
        unsafe {
            ffi::pixman_f_transform_init_scale(transform.as_mut_ptr(), sx, sy);
        }
        Self(unsafe { transform.assume_init() })
    }

    #[inline]
    #[doc(alias = "pixman_f_transform_init_translate")]
    pub fn from_translation(tx: f64, ty: f64) -> Self {
        let mut transform = MaybeUninit::uninit();
        unsafe {
            ffi::pixman_f_transform_init_translate(transform.as_mut_ptr(), tx, ty);
        }
        Self(unsafe { transform.assume_init() })
    }

    pub fn invert(&self) -> Option<Self> {
        let mut transform = MaybeUninit::uninit();
        let res = unsafe { ffi::pixman_f_transform_invert(transform.as_mut_ptr(), self.as_ptr()) };

        if res == 1 {
            Some(Self(unsafe { transform.assume_init() }))
        } else {
            None
        }
    }

    pub fn multiply(&self, other: &FTransform) -> FTransform {
        let mut transform = MaybeUninit::uninit();
        unsafe {
            ffi::pixman_f_transform_multiply(transform.as_mut_ptr(), self.as_ptr(), other.as_ptr());
        };
        Self(unsafe { transform.assume_init() })
    }

    pub fn transform_point(&self, mut vector: FVector) -> Option<FVector> {
        let res = unsafe { ffi::pixman_f_transform_point(self.as_ptr(), vector.as_mut_ptr()) };
        if res == 1 {
            Some(vector)
        } else {
            None
        }
    }

    pub fn transform_point_3d(&self, mut vector: FVector) -> FVector {
        unsafe { ffi::pixman_f_transform_point_3d(self.as_ptr(), vector.as_mut_ptr()) };
        vector
    }

    pub fn rotate(mut self, c: f64, s: f64, reverse: bool) -> Option<Self> {
        let c = c;
        let s = s;
        let res = if reverse {
            unsafe { ffi::pixman_f_transform_rotate(std::ptr::null_mut(), self.as_mut_ptr(), c, s) }
        } else {
            unsafe { ffi::pixman_f_transform_rotate(self.as_mut_ptr(), std::ptr::null_mut(), c, s) }
        };

        if res == 1 {
            Some(self)
        } else {
            None
        }
    }

    pub fn scale(mut self, sx: f64, sy: f64, reverse: bool) -> Option<Self> {
        let sx = sx;
        let sy = sy;
        let res = if reverse {
            unsafe {
                ffi::pixman_f_transform_scale(std::ptr::null_mut(), self.as_mut_ptr(), sx, sy)
            }
        } else {
            unsafe {
                ffi::pixman_f_transform_scale(self.as_mut_ptr(), std::ptr::null_mut(), sx, sy)
            }
        };

        if res == 1 {
            Some(self)
        } else {
            None
        }
    }

    pub fn translate(mut self, tx: f64, ty: f64, reverse: bool) -> Option<Self> {
        let tx = tx;
        let ty = ty;
        let res = if reverse {
            unsafe {
                ffi::pixman_f_transform_translate(std::ptr::null_mut(), self.as_mut_ptr(), tx, ty)
            }
        } else {
            unsafe {
                ffi::pixman_f_transform_translate(self.as_mut_ptr(), std::ptr::null_mut(), tx, ty)
            }
        };

        if res == 1 {
            Some(self)
        } else {
            None
        }
    }

    #[inline]
    pub fn m(&self) -> [[f64; 3]; 3] {
        self.0.m
    }

    #[inline]
    pub(crate) fn as_ptr(&self) -> *const ffi::pixman_f_transform_t {
        &self.0 as *const ffi::pixman_f_transform_t
    }

    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut ffi::pixman_f_transform_t {
        &mut self.0 as *mut ffi::pixman_f_transform_t
    }
}

impl From<ffi::pixman_f_transform_t> for FTransform {
    #[inline]
    fn from(value: ffi::pixman_f_transform_t) -> Self {
        Self(value)
    }
}

impl From<FTransform> for ffi::pixman_f_transform_t {
    #[inline]
    fn from(value: FTransform) -> Self {
        value.0
    }
}

impl From<[[f64; 3]; 3]> for FTransform {
    #[inline]
    fn from(value: [[f64; 3]; 3]) -> Self {
        FTransform::new(value)
    }
}

impl From<Transform> for FTransform {
    fn from(value: Transform) -> Self {
        let mut transform = MaybeUninit::uninit();
        unsafe {
            ffi::pixman_f_transform_from_pixman_transform(transform.as_mut_ptr(), value.as_ptr());
        }
        Self(unsafe { transform.assume_init() })
    }
}
