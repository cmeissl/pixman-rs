#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use std::os::raw::c_int;

pub use pixman_sys as ffi;

pub type Box16 = ffi::pixman_box16_t;
pub type Box32 = ffi::pixman_box32_t;
pub type Rectangle16 = ffi::pixman_rectangle16_t;
pub type Rectangle32 = ffi::pixman_rectangle32_t;

mod color;
mod dither;
mod edge;
mod f_transform;
mod f_vector;
mod filter;
mod fixed;
mod format;
mod gradient_stop;
mod image;
mod line;
mod operation;
mod point;
mod region16;
mod region32;
mod repeat;
mod span;
mod transform;
mod trap;
mod trapezoid;
mod triangle;
mod vector;

pub use color::*;
pub use dither::*;
pub use edge::*;
pub use f_transform::*;
pub use f_vector::*;
pub use filter::*;
pub use fixed::*;
pub use format::*;
pub use gradient_stop::*;
pub use image::*;
pub use line::*;
pub use operation::*;
pub use point::*;
pub use region16::*;
pub use region32::*;
pub use repeat::*;
pub use span::*;
pub use transform::*;
pub use trap::*;
pub use trapezoid::*;
pub use triangle::*;
pub use vector::*;

#[derive(Debug)]
pub struct OperationFailed;

#[allow(clippy::too_many_arguments)]
pub fn blit(
    src_bits: &[u32],
    dst_bits: &mut [u32],
    src_stride: isize,
    dst_stride: isize,
    src_bpp: isize,
    dst_bpp: isize,
    src_x: isize,
    src_y: isize,
    dest_x: isize,
    dest_y: isize,
    width: isize,
    height: isize,
) -> Result<(), OperationFailed> {
    let res = unsafe {
        ffi::pixman_blt(
            src_bits.as_ptr() as *mut _,
            dst_bits.as_mut_ptr(),
            src_stride as c_int,
            dst_stride as c_int,
            src_bpp as c_int,
            dst_bpp as c_int,
            src_x as c_int,
            src_y as c_int,
            dest_x as c_int,
            dest_y as c_int,
            width as c_int,
            height as c_int,
        )
    };
    if res == 1 {
        Ok(())
    } else {
        Err(OperationFailed)
    }
}

#[allow(clippy::too_many_arguments)]
pub fn fill(
    bits: &mut [u32],
    stride: u32,
    bpp: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    filler: u32,
) -> Result<(), OperationFailed> {
    let res = unsafe {
        ffi::pixman_fill(
            bits.as_mut_ptr(),
            stride as c_int,
            bpp as c_int,
            x as c_int,
            y as c_int,
            width as c_int,
            height as c_int,
            filler,
        )
    };
    if res == 1 {
        Ok(())
    } else {
        Err(OperationFailed)
    }
}

// TODO: pixman_filter_create_separable_convolution
// TODO: pixman_format_supported_destination
// TODO: pixman_format_supported_source
// TODO: pixman_glyph_cache_create
// TODO: pixman_glyph_cache_destroy
// TODO: pixman_glyph_cache_freeze
// TODO: pixman_glyph_cache_insert
// TODO: pixman_glyph_cache_lookup
// TODO: pixman_glyph_cache_remove
// TODO: pixman_glyph_cache_thaw
// TODO: pixman_glyph_get_extents
// TODO: pixman_glyph_get_mask_format

/// Compute the smallest value greater than or equal to y which is on a
/// grid row.
pub fn sample_ceil_y(y: impl Into<Fixed>, bpp: i32) -> Fixed {
    Fixed::from_raw(unsafe { ffi::pixman_sample_ceil_y(y.into().into_raw(), bpp) })
}

/// Compute the largest value strictly less than y which is on a
/// grid row.
pub fn sample_floor_y(y: impl Into<Fixed>, bpp: i32) -> Fixed {
    Fixed::from_raw(unsafe { ffi::pixman_sample_floor_y(y.into().into_raw(), bpp) })
}
