use std::mem::MaybeUninit;

use crate::{ffi, Box16, FTransform, Fixed, Vector};

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Transform(ffi::pixman_transform_t);

impl Transform {
    #[inline]
    pub fn new<T: Into<Fixed> + Copy>(matrix: [[T; 3]; 3]) -> Self {
        let matrix = [
            [
                matrix[0][0].into().into_raw(),
                matrix[0][1].into().into_raw(),
                matrix[0][2].into().into_raw(),
            ],
            [
                matrix[1][0].into().into_raw(),
                matrix[1][1].into().into_raw(),
                matrix[1][2].into().into_raw(),
            ],
            [
                matrix[2][0].into().into_raw(),
                matrix[2][1].into().into_raw(),
                matrix[2][2].into().into_raw(),
            ],
        ];
        Self(ffi::pixman_transform { matrix })
    }

    pub fn transform_bounds(&self, mut b: Box16) -> Option<Box16> {
        let res = unsafe { ffi::pixman_transform_bounds(self.as_ptr(), &mut b) };
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
            ffi::pixman_transform_init_identity(transform.as_mut_ptr());
        }
        Self(unsafe { transform.assume_init() })
    }

    #[inline]
    #[doc(alias = "pixman_transform_init_rotate")]
    pub fn from_rotation(cos: impl Into<Fixed>, sin: impl Into<Fixed>) -> Self {
        let mut transform = MaybeUninit::uninit();
        unsafe {
            ffi::pixman_transform_init_rotate(
                transform.as_mut_ptr(),
                cos.into().into_raw(),
                sin.into().into_raw(),
            );
        }
        Self(unsafe { transform.assume_init() })
    }

    #[inline]
    #[doc(alias = "pixman_transform_init_scale")]
    pub fn from_scale(sx: impl Into<Fixed>, sy: impl Into<Fixed>) -> Self {
        let mut transform = MaybeUninit::uninit();
        unsafe {
            ffi::pixman_transform_init_scale(
                transform.as_mut_ptr(),
                sx.into().into_raw(),
                sy.into().into_raw(),
            );
        }
        Self(unsafe { transform.assume_init() })
    }

    #[inline]
    #[doc(alias = "pixman_transform_init_translate")]
    pub fn from_translation(tx: impl Into<Fixed>, ty: impl Into<Fixed>) -> Self {
        let mut transform = MaybeUninit::uninit();
        unsafe {
            ffi::pixman_transform_init_translate(
                transform.as_mut_ptr(),
                tx.into().into_raw(),
                ty.into().into_raw(),
            );
        }
        Self(unsafe { transform.assume_init() })
    }

    pub fn invert(&self) -> Option<Self> {
        let mut transform = MaybeUninit::uninit();
        let res = unsafe { ffi::pixman_transform_invert(transform.as_mut_ptr(), self.as_ptr()) };

        if res == 1 {
            Some(Self(unsafe { transform.assume_init() }))
        } else {
            None
        }
    }

    pub fn is_identity(&self) -> bool {
        unsafe { ffi::pixman_transform_is_identity(self.as_ptr()) == 1 }
    }

    pub fn is_int_translate(&self) -> bool {
        unsafe { ffi::pixman_transform_is_int_translate(self.as_ptr()) == 1 }
    }

    pub fn is_inverse(&self, other: &Transform) -> bool {
        unsafe { ffi::pixman_transform_is_inverse(self.as_ptr(), other.as_ptr()) == 1 }
    }

    pub fn is_scale(&self) -> bool {
        unsafe { ffi::pixman_transform_is_scale(self.as_ptr()) == 1 }
    }

    pub fn multiply(&self, other: &Transform) -> Option<Transform> {
        let mut transform = MaybeUninit::uninit();
        let res = unsafe {
            ffi::pixman_transform_multiply(transform.as_mut_ptr(), self.as_ptr(), other.as_ptr())
        };

        if res == 1 {
            Some(Self(unsafe { transform.assume_init() }))
        } else {
            None
        }
    }

    pub fn transform_point(&self, mut vector: Vector) -> Option<Vector> {
        let res = unsafe { ffi::pixman_transform_point(self.as_ptr(), vector.as_mut_ptr()) };
        if res == 1 {
            Some(vector)
        } else {
            None
        }
    }

    pub fn transform_point_3d(&self, mut vector: Vector) -> Option<Vector> {
        let res = unsafe { ffi::pixman_transform_point_3d(self.as_ptr(), vector.as_mut_ptr()) };
        if res == 1 {
            Some(vector)
        } else {
            None
        }
    }

    pub fn rotate(
        mut self,
        c: impl Into<Fixed>,
        s: impl Into<Fixed>,
        reverse: bool,
    ) -> Option<Self> {
        let c = c.into().into_raw();
        let s = s.into().into_raw();
        let res = if reverse {
            unsafe { ffi::pixman_transform_rotate(std::ptr::null_mut(), self.as_mut_ptr(), c, s) }
        } else {
            unsafe { ffi::pixman_transform_rotate(self.as_mut_ptr(), std::ptr::null_mut(), c, s) }
        };

        if res == 1 {
            Some(self)
        } else {
            None
        }
    }

    pub fn scale(
        mut self,
        sx: impl Into<Fixed>,
        sy: impl Into<Fixed>,
        reverse: bool,
    ) -> Option<Self> {
        let sx = sx.into().into_raw();
        let sy = sy.into().into_raw();
        let res = if reverse {
            unsafe { ffi::pixman_transform_scale(std::ptr::null_mut(), self.as_mut_ptr(), sx, sy) }
        } else {
            unsafe { ffi::pixman_transform_scale(self.as_mut_ptr(), std::ptr::null_mut(), sx, sy) }
        };

        if res == 1 {
            Some(self)
        } else {
            None
        }
    }

    pub fn translate(
        mut self,
        tx: impl Into<Fixed>,
        ty: impl Into<Fixed>,
        reverse: bool,
    ) -> Option<Self> {
        let tx = tx.into().into_raw();
        let ty = ty.into().into_raw();
        let res = if reverse {
            unsafe {
                ffi::pixman_transform_translate(std::ptr::null_mut(), self.as_mut_ptr(), tx, ty)
            }
        } else {
            unsafe {
                ffi::pixman_transform_translate(self.as_mut_ptr(), std::ptr::null_mut(), tx, ty)
            }
        };

        if res == 1 {
            Some(self)
        } else {
            None
        }
    }

    #[inline]
    pub fn matrix(&self) -> [[Fixed; 3]; 3] {
        [
            [
                Fixed::from_raw(self.0.matrix[0][0]),
                Fixed::from_raw(self.0.matrix[0][1]),
                Fixed::from_raw(self.0.matrix[0][2]),
            ],
            [
                Fixed::from_raw(self.0.matrix[1][0]),
                Fixed::from_raw(self.0.matrix[1][1]),
                Fixed::from_raw(self.0.matrix[1][2]),
            ],
            [
                Fixed::from_raw(self.0.matrix[2][0]),
                Fixed::from_raw(self.0.matrix[2][1]),
                Fixed::from_raw(self.0.matrix[2][2]),
            ],
        ]
    }

    #[inline]
    pub(crate) fn as_ptr(&self) -> *const ffi::pixman_transform_t {
        &self.0 as *const ffi::pixman_transform_t
    }

    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut ffi::pixman_transform_t {
        &mut self.0 as *mut ffi::pixman_transform_t
    }
}

impl From<ffi::pixman_transform_t> for Transform {
    #[inline]
    fn from(value: ffi::pixman_transform_t) -> Self {
        Self(value)
    }
}

impl From<Transform> for ffi::pixman_transform_t {
    #[inline]
    fn from(value: Transform) -> Self {
        value.0
    }
}

impl<T: Into<Fixed> + Copy> From<[[T; 3]; 3]> for Transform {
    #[inline]
    fn from(value: [[T; 3]; 3]) -> Self {
        Transform::new(value)
    }
}

pub struct TransformError;
impl TryFrom<FTransform> for Transform {
    type Error = TransformError;

    fn try_from(value: FTransform) -> Result<Self, Self::Error> {
        let mut transform = MaybeUninit::uninit();
        let res = unsafe {
            ffi::pixman_transform_from_pixman_f_transform(transform.as_mut_ptr(), value.as_ptr())
        };
        if res != 1 {
            Err(TransformError)
        } else {
            Ok(Self(unsafe { transform.assume_init() }))
        }
    }
}
