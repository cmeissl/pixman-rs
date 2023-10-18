use std::{marker::PhantomData, mem::MaybeUninit, os::raw::c_int};

use pixman_sys as ffi;

use crate::{
    format::FormatCode, operation::Operation, repeat::Repeat, Box32, Color, Dither, Edge, Filter,
    Fixed, GradientStop, OperationFailed, Point, Rectangle16, Region16, Region32, Transform, Trap,
    Trapezoid, Triangle,
};

pub struct Image<'bits> {
    ptr: *mut ffi::pixman_image_t,
    _phantom: PhantomData<&'bits ()>,
}

#[derive(Debug)]
pub struct CreateFailed;

impl Image<'static> {
    pub fn new(
        format: FormatCode,
        width: usize,
        height: usize,
        clear: bool,
    ) -> Result<Self, CreateFailed> {
        let ptr = if clear {
            unsafe {
                ffi::pixman_image_create_bits(
                    format.into(),
                    width as c_int,
                    height as c_int,
                    std::ptr::null_mut(),
                    0,
                )
            }
        } else {
            unsafe {
                ffi::pixman_image_create_bits_no_clear(
                    format.into(),
                    width as c_int,
                    height as c_int,
                    std::ptr::null_mut(),
                    0,
                )
            }
        };

        if ptr.is_null() {
            Err(CreateFailed)
        } else {
            Ok(Self {
                ptr,
                _phantom: PhantomData,
            })
        }
    }

    pub fn solid_fill(color: impl Into<Color>) -> Result<Self, CreateFailed> {
        let color = color.into();
        let ptr = unsafe { ffi::pixman_image_create_solid_fill(color.as_ptr()) };

        if ptr.is_null() {
            Err(CreateFailed)
        } else {
            Ok(Self {
                ptr,
                _phantom: PhantomData,
            })
        }
    }

    pub fn linear_gradient(
        p1: impl Into<Point>,
        p2: impl Into<Point>,
        stops: &[GradientStop],
    ) -> Result<Self, CreateFailed> {
        let p1: Point = p1.into();
        let p2: Point = p2.into();
        let ptr = unsafe {
            ffi::pixman_image_create_linear_gradient(
                p1.as_ptr(),
                p2.as_ptr(),
                stops.as_ptr() as *const _,
                stops.len() as c_int,
            )
        };
        if ptr.is_null() {
            Err(CreateFailed)
        } else {
            Ok(Self {
                ptr,
                _phantom: PhantomData,
            })
        }
    }

    pub fn conical_gradient(
        center: impl Into<Point>,
        angle: impl Into<Fixed>,
        stops: &[GradientStop],
    ) -> Result<Self, CreateFailed> {
        let center = center.into();
        let angle = angle.into();
        let ptr = unsafe {
            ffi::pixman_image_create_conical_gradient(
                center.as_ptr(),
                angle.into_raw(),
                stops.as_ptr() as *const _,
                stops.len() as c_int,
            )
        };
        if ptr.is_null() {
            Err(CreateFailed)
        } else {
            Ok(Self {
                ptr,
                _phantom: PhantomData,
            })
        }
    }

    pub fn radial_gradient(
        inner: impl Into<Point>,
        outer: impl Into<Point>,
        inner_radius: impl Into<Fixed>,
        outer_radius: impl Into<Fixed>,
        stops: &[GradientStop],
    ) -> Result<Self, CreateFailed> {
        let inner = inner.into();
        let outer = outer.into();
        let inner_radius = inner_radius.into();
        let other_radius = outer_radius.into();
        let ptr = unsafe {
            ffi::pixman_image_create_radial_gradient(
                inner.as_ptr(),
                outer.as_ptr(),
                inner_radius.into_raw(),
                other_radius.into_raw(),
                stops.as_ptr() as *const _,
                stops.len() as c_int,
            )
        };
        if ptr.is_null() {
            Err(CreateFailed)
        } else {
            Ok(Self {
                ptr,
                _phantom: PhantomData,
            })
        }
    }
}

impl<'bits> Image<'bits> {
    pub fn from_bits(
        format: FormatCode,
        width: usize,
        height: usize,
        bits: &'bits mut [u32],
        rowstride_bytes: usize,
        clear: bool,
    ) -> Result<Self, CreateFailed> {
        let ptr = if clear {
            unsafe {
                ffi::pixman_image_create_bits(
                    format.into(),
                    width as c_int,
                    height as c_int,
                    bits.as_mut_ptr(),
                    rowstride_bytes as c_int,
                )
            }
        } else {
            unsafe {
                ffi::pixman_image_create_bits_no_clear(
                    format.into(),
                    width as c_int,
                    height as c_int,
                    bits.as_mut_ptr(),
                    rowstride_bytes as c_int,
                )
            }
        };

        if ptr.is_null() {
            Err(CreateFailed)
        } else {
            Ok(Self {
                ptr,
                _phantom: PhantomData,
            })
        }
    }

    pub fn width(&self) -> usize {
        unsafe { ffi::pixman_image_get_width(self.ptr) as usize }
    }

    pub fn height(&self) -> usize {
        unsafe { ffi::pixman_image_get_height(self.ptr) as usize }
    }

    pub fn stride(&self) -> usize {
        unsafe { ffi::pixman_image_get_stride(self.ptr) as usize }
    }

    pub fn depth(&self) -> usize {
        unsafe { ffi::pixman_image_get_depth(self.ptr) as usize }
    }

    pub fn format(&self) -> Option<FormatCode> {
        let format = unsafe { ffi::pixman_image_get_format(self.ptr) };
        FormatCode::try_from(format).ok()
    }

    pub fn component_alpha(&self) -> bool {
        unsafe { ffi::pixman_image_get_component_alpha(self.ptr) == 1 }
    }

    pub fn set_repeat(&mut self, repeat: Repeat) {
        unsafe {
            ffi::pixman_image_set_repeat(self.ptr, repeat.into());
        }
    }

    pub fn set_transform(
        &mut self,
        transform: impl Into<Transform>,
    ) -> Result<(), OperationFailed> {
        let transform = transform.into();
        let res = unsafe { ffi::pixman_image_set_transform(self.ptr, transform.as_ptr()) };
        if res == 1 {
            Ok(())
        } else {
            Err(OperationFailed)
        }
    }

    pub fn set_alpha_map<'alpha: 'bits>(
        &mut self,
        alpha_map: Option<&Image<'alpha>>,
        x: i16,
        y: i16,
    ) {
        let alpha_map = if let Some(alpha_map) = alpha_map {
            alpha_map.ptr
        } else {
            std::ptr::null_mut()
        };
        unsafe {
            ffi::pixman_image_set_alpha_map(self.ptr, alpha_map, x, y);
        }
    }

    pub fn set_clip_region(&mut self, region: Option<Region16>) -> Result<(), OperationFailed> {
        let region = if let Some(region) = region {
            region.as_ptr()
        } else {
            std::ptr::null()
        };
        let res = unsafe {
            // FIXME: pixman_image_set_clip_region takes a *const region, but bindgen generates *mut region
            ffi::pixman_image_set_clip_region(self.ptr, region as *mut _)
        };
        if res == 1 {
            Ok(())
        } else {
            Err(OperationFailed)
        }
    }

    pub fn set_clip_region32(&mut self, region: Option<Region32>) -> Result<(), OperationFailed> {
        let region = if let Some(region) = region {
            region.as_ptr()
        } else {
            std::ptr::null()
        };
        let res = unsafe {
            // FIXME: pixman_image_set_clip_region32 takes a *const region, but bindgen generates *mut region
            ffi::pixman_image_set_clip_region32(self.ptr, region as *mut _)
        };
        if res == 1 {
            Ok(())
        } else {
            Err(OperationFailed)
        }
    }

    pub fn set_component_alpha(&mut self, component_alpha: bool) {
        let component_alpha = if component_alpha { 1 } else { 0 };
        unsafe { ffi::pixman_image_set_component_alpha(self.ptr, component_alpha) }
    }

    pub fn set_dither(&mut self, dither: Dither) {
        unsafe {
            ffi::pixman_image_set_dither(self.ptr, dither.into());
        }
    }

    pub fn set_dither_offset(&mut self, offset_x: c_int, offset_y: c_int) {
        unsafe { ffi::pixman_image_set_dither_offset(self.ptr, offset_x, offset_y) }
    }

    pub fn set_filter(
        &mut self,
        filter: Filter,
        filter_params: &[Fixed],
    ) -> Result<(), OperationFailed> {
        let res = unsafe {
            ffi::pixman_image_set_filter(
                self.ptr,
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

    pub fn set_has_client_clip(&mut self, client_clip: bool) {
        let client_clip = if client_clip { 1 } else { 0 };
        unsafe {
            ffi::pixman_image_set_has_client_clip(self.ptr, client_clip);
        }
    }

    // TODO: pixman_image_set_indexed
    //pub fn set_indexd(&mut self)

    pub fn set_source_clipping(&mut self, source_clipping: bool) {
        let source_clipping = if source_clipping { 1 } else { 0 };
        unsafe {
            ffi::pixman_image_set_source_clipping(self.ptr, source_clipping);
        }
    }

    pub fn fill_boxes(
        &mut self,
        op: Operation,
        color: impl Into<Color>,
        boxes: &[Box32],
    ) -> Result<(), OperationFailed> {
        let color = color.into();
        let res = unsafe {
            ffi::pixman_image_fill_boxes(
                op.into(),
                self.ptr,
                color.as_ptr(),
                boxes.len() as c_int,
                boxes.as_ptr(),
            )
        };
        if res == 1 {
            Ok(())
        } else {
            Err(OperationFailed)
        }
    }

    pub fn fill_rectangles(
        &mut self,
        op: Operation,
        color: impl Into<Color>,
        rects: &[Rectangle16],
    ) -> Result<(), OperationFailed> {
        let color = color.into();
        let res = unsafe {
            ffi::pixman_image_fill_rectangles(
                op.into(),
                self.ptr,
                color.as_ptr(),
                rects.len() as c_int,
                rects.as_ptr(),
            )
        };
        if res == 1 {
            Ok(())
        } else {
            Err(OperationFailed)
        }
    }

    pub fn composite(
        &mut self,
        operation: Operation,
        src: &Image<'_>,
        mask: Option<&Image<'_>>,
        src_x: i16,
        src_y: i16,
        mask_x: i16,
        mask_y: i16,
        dest_x: i16,
        dest_y: i16,
        width: u16,
        height: u16,
    ) {
        let mask_ptr = if let Some(mask) = mask {
            mask.ptr
        } else {
            std::ptr::null_mut()
        };
        unsafe {
            ffi::pixman_image_composite(
                operation.into(),
                src.ptr,
                mask_ptr,
                self.ptr,
                src_x,
                src_y,
                mask_x,
                mask_y,
                dest_x,
                dest_y,
                width,
                height,
            )
        }
    }

    pub fn composite32(
        &mut self,
        operation: Operation,
        src: &Image<'_>,
        mask: Option<&Image<'_>>,
        src_x: i32,
        src_y: i32,
        mask_x: i32,
        mask_y: i32,
        dest_x: i32,
        dest_y: i32,
        width: i32,
        height: i32,
    ) {
        let mask_ptr = if let Some(mask) = mask {
            mask.ptr
        } else {
            std::ptr::null_mut()
        };
        unsafe {
            ffi::pixman_image_composite32(
                operation.into(),
                src.ptr,
                mask_ptr,
                self.ptr,
                src_x,
                src_y,
                mask_x,
                mask_y,
                dest_x,
                dest_y,
                width,
                height,
            )
        }
    }

    pub fn composite_triangles(
        &mut self,
        operation: Operation,
        src: &Image<'_>,
        mask_format: FormatCode,
        x_src: isize,
        y_src: isize,
        x_dst: isize,
        y_dst: isize,
        tris: &[Triangle],
    ) {
        unsafe {
            ffi::pixman_composite_triangles(
                operation.into(),
                src.ptr,
                self.ptr,
                mask_format.into(),
                x_src as i32,
                y_src as i32,
                x_dst as i32,
                y_dst as i32,
                tris.len() as i32,
                tris.as_ptr() as *const _,
            );
        }
    }

    pub fn composite_trapezoids(
        &mut self,
        operation: Operation,
        src: &Image<'_>,
        mask_format: FormatCode,
        x_src: isize,
        y_src: isize,
        x_dst: isize,
        y_dst: isize,
        traps: &[Trapezoid],
    ) {
        unsafe {
            ffi::pixman_composite_trapezoids(
                operation.into(),
                src.ptr,
                self.ptr,
                mask_format.into(),
                x_src as i32,
                y_src as i32,
                x_dst as i32,
                y_dst as i32,
                traps.len() as i32,
                traps.as_ptr() as *const _,
            );
        }
    }

    pub fn add_traps(&mut self, x_off: i16, y_off: i16, traps: &[Trap]) {
        unsafe {
            ffi::pixman_add_traps(
                self.ptr,
                x_off,
                y_off,
                traps.len() as i32,
                traps.as_ptr() as *const _,
            );
        }
    }

    pub fn add_trapezoids(&mut self, x_off: i16, y_off: i32, traps: &[Trapezoid]) {
        unsafe {
            ffi::pixman_add_trapezoids(
                self.ptr,
                x_off,
                y_off,
                traps.len() as i32,
                traps.as_ptr() as *const _,
            );
        }
    }

    pub fn add_triangles(&mut self, x_off: i32, y_off: i32, tris: &[Triangle]) {
        unsafe {
            ffi::pixman_add_triangles(
                self.ptr,
                x_off,
                y_off,
                tris.len() as i32,
                tris.as_ptr() as *const _,
            );
        }
    }

    pub fn compute_composite_region(
        &self,
        src: &Image<'_>,
        mask: Option<&Image<'_>>,
        src_x: i16,
        src_y: i16,
        mask_x: i16,
        mask_y: i16,
        dest_x: i16,
        dest_y: i16,
        width: u16,
        height: u16,
    ) -> Option<Region16> {
        let mask_ptr = if let Some(mask) = mask {
            mask.ptr
        } else {
            std::ptr::null_mut()
        };

        let mut region = MaybeUninit::uninit();
        let res = unsafe {
            ffi::pixman_compute_composite_region(
                region.as_mut_ptr(),
                src.ptr,
                mask_ptr,
                self.ptr,
                src_x,
                src_y,
                mask_x,
                mask_y,
                dest_x,
                dest_y,
                width,
                height,
            )
        };
        if res == 1 {
            Some(Region16::from(unsafe { region.assume_init() }))
        } else {
            None
        }
    }

    pub fn rasterize_edges(&mut self, l: Edge, r: Edge, t: impl Into<Fixed>, b: impl Into<Fixed>) {
        unsafe {
            ffi::pixman_rasterize_edges(
                self.ptr,
                l.as_ptr() as *mut _,
                r.as_ptr() as *mut _,
                t.into().into_raw(),
                b.into().into_raw(),
            )
        }
    }

    pub fn rasterize_trapezoid(&mut self, trap: Trapezoid, x_off: i32, y_off: i32) {
        unsafe { ffi::pixman_rasterize_trapezoid(self.ptr, trap.as_ptr(), x_off, y_off) }
    }

    // TODO: pixman_composite_glyphs
    //pub fn composite_glyphs()

    // TODO: pixman_composite_glyphs
    //pub fn composite_glyphs_no_mask()

    /// Access the underlying pixel data
    ///
    /// Returns `None` in case the image has no underlying pixel data, e.g. solid/gradient/...
    pub fn data(&self) -> Option<&[u8]> {
        let height = self.height();
        let stride = self.stride();
        let ptr = unsafe { ffi::pixman_image_get_data(self.ptr) };

        if ptr.is_null() {
            None
        } else {
            unsafe {
                Some(std::slice::from_raw_parts(
                    ptr as *const u8,
                    stride * height,
                ))
            }
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ffi::pixman_image_t {
        self.ptr
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut ffi::pixman_image_t {
        self.ptr
    }
}

impl<'bits> Drop for Image<'bits> {
    fn drop(&mut self) {
        unsafe {
            ffi::pixman_image_unref(self.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{FormatCode, Image, Operation, Repeat};

    #[test]
    fn create() {
        let _ = Image::new(FormatCode::A8R8G8B8, 800, 600, false).unwrap();
    }

    #[test]
    fn from_bits() {
        let mut bits: Vec<u32> = Vec::new();
        let _ = Image::from_bits(FormatCode::A8R8G8B8, 800, 600, &mut bits, 0, true).unwrap();
    }

    #[test]
    fn solid() {
        let _ = Image::solid_fill([0xffff, 0xffff, 0xffff, 0xffff]).unwrap();
    }

    #[test]
    fn composite() {
        let mut dst = Image::new(FormatCode::A8R8G8B8, 800, 600, true).unwrap();
        let mut solid = Image::solid_fill([0xffff, 0xffff, 0xffff, 0xffff]).unwrap();
        solid.set_repeat(Repeat::Normal);
        dst.composite(Operation::Over, &solid, None, 0, 0, 0, 0, 0, 0, 50, 50);
        let _ = dst.data().unwrap();
    }
}
