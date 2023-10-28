use std::os::raw::c_int;

use pixman_sys as ffi;

use crate::{
    repeat::Repeat, Dither, Filter, Fixed, OperationFailed, Region16, Region32, Transform,
};

mod bits;
mod conical_gradient;
mod linear_gradient;
mod radial_gradient;
mod solid;

pub use bits::*;
pub use conical_gradient::ConicalGradient;
pub use linear_gradient::LinearGradient;
pub use radial_gradient::RadialGradient;
pub use solid::Solid;

#[derive(Debug)]
pub struct ImageRef(*mut ffi::pixman_image_t);

impl ImageRef {
    pub fn set_repeat(&self, repeat: Repeat) {
        unsafe {
            ffi::pixman_image_set_repeat(self.0, repeat.into());
        }
    }

    pub fn set_transform(&self, transform: impl Into<Transform>) -> Result<(), OperationFailed> {
        let transform = transform.into();
        let res = unsafe { ffi::pixman_image_set_transform(self.0, transform.as_ptr()) };
        if res == 1 {
            Ok(())
        } else {
            Err(OperationFailed)
        }
    }

    pub fn set_clip_region(&self, region: Option<Region16>) -> Result<(), OperationFailed> {
        let region = if let Some(region) = region {
            region.as_ptr()
        } else {
            std::ptr::null()
        };
        let res = unsafe {
            // FIXME: pixman_image_set_clip_region takes a *const region, but bindgen generates *mut region
            ffi::pixman_image_set_clip_region(self.0, region as *mut _)
        };
        if res == 1 {
            Ok(())
        } else {
            Err(OperationFailed)
        }
    }

    pub fn set_clip_region32(&self, region: Option<Region32>) -> Result<(), OperationFailed> {
        let region = if let Some(region) = region {
            region.as_ptr()
        } else {
            std::ptr::null()
        };
        let res = unsafe {
            // FIXME: pixman_image_set_clip_region32 takes a *const region, but bindgen generates *mut region
            ffi::pixman_image_set_clip_region32(self.0, region as *mut _)
        };
        if res == 1 {
            Ok(())
        } else {
            Err(OperationFailed)
        }
    }

    pub fn set_dither(&self, dither: Dither) {
        unsafe {
            ffi::pixman_image_set_dither(self.0, dither.into());
        }
    }

    pub fn set_dither_offset(&self, offset_x: c_int, offset_y: c_int) {
        unsafe { ffi::pixman_image_set_dither_offset(self.0, offset_x, offset_y) }
    }

    pub fn set_filter(
        &self,
        filter: Filter,
        filter_params: &[Fixed],
    ) -> Result<(), OperationFailed> {
        let res = unsafe {
            ffi::pixman_image_set_filter(
                self.0,
                filter.into(),
                filter_params.as_ptr() as *const _,
                filter_params.len() as i32,
            )
        };
        if res == 1 {
            Ok(())
        } else {
            Err(OperationFailed)
        }
    }

    pub fn set_has_client_clip(&self, client_clip: bool) {
        let client_clip = if client_clip { 1 } else { 0 };
        unsafe {
            ffi::pixman_image_set_has_client_clip(self.0, client_clip);
        }
    }

    // TODO: pixman_image_set_indexed
    //pub fn set_indexed(&mut self)

    pub fn set_source_clipping(&self, source_clipping: bool) {
        let source_clipping = if source_clipping { 1 } else { 0 };
        unsafe {
            ffi::pixman_image_set_source_clipping(self.0, source_clipping);
        }
    }

    pub fn component_alpha(&self) -> bool {
        unsafe { ffi::pixman_image_get_component_alpha(self.0) == 1 }
    }

    pub fn set_component_alpha(&self, component_alpha: bool) {
        let component_alpha = if component_alpha { 1 } else { 0 };
        unsafe { ffi::pixman_image_set_component_alpha(self.0, component_alpha) }
    }
}

impl ImageRef {
    pub unsafe fn from_ptr(ptr: *mut ffi::pixman_image_t) -> Self {
        assert!(!ptr.is_null());
        ImageRef(ptr)
    }

    pub fn as_ptr(&self) -> *mut ffi::pixman_image_t {
        self.0
    }
}

impl Drop for ImageRef {
    fn drop(&mut self) {
        unsafe {
            ffi::pixman_image_unref(self.0);
        }
    }
}

macro_rules! image_type {
    ($name:ident) => {
        #[derive(Debug)]
        pub struct $name<'alpha> {
            image: $crate::ImageRef,
            _phantom: std::marker::PhantomData<&'alpha ()>,
        }

        impl<'a> $name<'a> {
            pub fn set_alpha_map<'alpha: 'a, const WRITABLE: bool>(
                self,
                alpha_map: &'alpha crate::Image<'_, 'static, WRITABLE>,
                x: i16,
                y: i16,
            ) -> $name<'alpha> {
                unsafe {
                    $crate::ffi::pixman_image_set_alpha_map(
                        self.as_ptr(),
                        alpha_map.as_ptr(),
                        x,
                        y,
                    );
                }
                $name {
                    image: self.image,
                    _phantom: std::marker::PhantomData,
                }
            }

            pub fn clear_alpha_map(self) -> $name<'static> {
                unsafe {
                    $crate::ffi::pixman_image_set_alpha_map(
                        self.as_ptr(),
                        std::ptr::null_mut(),
                        0,
                        0,
                    );
                }
                $name {
                    image: self.image,
                    _phantom: std::marker::PhantomData,
                }
            }
        }

        impl<'alpha> $name<'alpha> {
            pub unsafe fn from_ptr(ptr: *mut ffi::pixman_image_t) -> Self {
                Self {
                    image: $crate::ImageRef::from_ptr(ptr),
                    _phantom: std::marker::PhantomData,
                }
            }
        }

        impl<'alpha> std::ops::Deref for $name<'alpha> {
            type Target = $crate::ImageRef;

            fn deref(&self) -> &Self::Target {
                &self.image
            }
        }
    };
}

pub(crate) use image_type;

#[derive(Debug)]
pub struct CreateFailed;