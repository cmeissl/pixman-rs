use crate::{ffi, Fixed};

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Vector(ffi::pixman_vector_t);

impl Vector {
    #[inline]
    pub fn new<T: Into<Fixed> + Copy>(vector: [T; 3]) -> Self {
        Self(ffi::pixman_vector {
            vector: [
                vector[0].into().into_raw(),
                vector[1].into().into_raw(),
                vector[2].into().into_raw(),
            ],
        })
    }

    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut ffi::pixman_vector_t {
        &mut self.0 as *mut ffi::pixman_vector_t
    }
}

impl<T: Into<Fixed> + Copy> From<[T; 3]> for Vector {
    #[inline]
    fn from(value: [T; 3]) -> Self {
        Self::new(value)
    }
}

impl From<ffi::pixman_vector_t> for Vector {
    #[inline]
    fn from(value: ffi::pixman_vector_t) -> Self {
        Self(value)
    }
}

impl From<Vector> for ffi::pixman_vector_t {
    #[inline]
    fn from(value: Vector) -> Self {
        value.0
    }
}
