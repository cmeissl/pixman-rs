use crate::ffi;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FVector(ffi::pixman_f_vector_t);

impl FVector {
    #[inline]
    pub fn new(v: [f64; 3]) -> Self {
        Self(ffi::pixman_f_vector_t {
            v: [v[0], v[1], v[2]],
        })
    }

    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut ffi::pixman_f_vector_t {
        &mut self.0 as *mut ffi::pixman_f_vector_t
    }
}

impl From<[f64; 3]> for FVector {
    #[inline]
    fn from(value: [f64; 3]) -> Self {
        Self::new(value)
    }
}

impl From<ffi::pixman_f_vector_t> for FVector {
    #[inline]
    fn from(value: ffi::pixman_f_vector_t) -> Self {
        Self(value)
    }
}

impl From<FVector> for ffi::pixman_f_vector_t {
    #[inline]
    fn from(value: FVector) -> Self {
        value.0
    }
}
