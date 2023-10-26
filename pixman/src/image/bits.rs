use std::{ffi::c_int, marker::PhantomData, mem::MaybeUninit};

use crate::{
    ffi, Box32, Color, CreateFailed, Edge, Fixed, FormatCode, ImageRef, Operation, OperationFailed,
    Rectangle16, Region16, Trap, Trapezoid, Triangle,
};

pub struct Image<'bits, 'alpha, const WRITABLE: bool> {
    image: ImageRef,
    _bits: PhantomData<&'bits ()>,
    _alpha: PhantomData<&'alpha ()>,
}

impl<'bits, 'alpha, const WRITABLE: bool> Image<'bits, 'alpha, WRITABLE> {
    pub unsafe fn from_ptr(ptr: *mut ffi::pixman_image_t) -> Self {
        Self {
            image: unsafe { ImageRef::from_ptr(ptr) },
            _bits: PhantomData,
            _alpha: PhantomData,
        }
    }
}

impl<'bits, 'alpha, const WRITABLE: bool> std::ops::Deref for Image<'bits, 'alpha, WRITABLE> {
    type Target = ImageRef;

    fn deref(&self) -> &Self::Target {
        &self.image
    }
}

impl<'bits, 'alpha, const WRITABLE: bool> Image<'bits, 'alpha, WRITABLE> {
    pub fn width(&self) -> usize {
        unsafe { ffi::pixman_image_get_width(self.as_ptr()) as usize }
    }

    pub fn height(&self) -> usize {
        unsafe { ffi::pixman_image_get_height(self.as_ptr()) as usize }
    }

    pub fn stride(&self) -> usize {
        unsafe { ffi::pixman_image_get_stride(self.as_ptr()) as usize }
    }

    pub fn depth(&self) -> usize {
        unsafe { ffi::pixman_image_get_depth(self.as_ptr()) as usize }
    }

    pub fn format(&self) -> Option<FormatCode> {
        let format = unsafe { ffi::pixman_image_get_format(self.as_ptr()) };
        FormatCode::try_from(format).ok()
    }

    /// Access the underlying pixel data
    ///
    /// Returns `None` in case the image has no underlying pixel data, e.g. solid/gradient/...
    pub fn data(&self) -> Option<&[u8]> {
        let height = self.height();
        let stride = self.stride();
        let ptr = unsafe { ffi::pixman_image_get_data(self.as_ptr()) };

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
}

impl<'bits, 'a, const W: bool> Image<'bits, 'a, W> {
    pub fn set_alpha_map<'alpha: 'a, const WRITABLE: bool>(
        self,
        alpha_map: &'alpha Image<'_, 'static, WRITABLE>,
        x: i16,
        y: i16,
    ) -> Image<'bits, 'alpha, W> {
        unsafe {
            ffi::pixman_image_set_alpha_map(self.as_ptr(), alpha_map.as_ptr(), x, y);
        }
        Image {
            image: self.image,
            _bits: self._bits,
            _alpha: PhantomData,
        }
    }

    pub fn clear_alpha_map(self) -> Image<'bits, 'static, W> {
        unsafe {
            ffi::pixman_image_set_alpha_map(self.as_ptr(), std::ptr::null_mut(), 0, 0);
        }
        Image {
            image: self.image,
            _bits: self._bits,
            _alpha: PhantomData,
        }
    }
}

impl<'bits> Image<'bits, 'static, false> {
    pub fn from_slice(
        format: FormatCode,
        width: usize,
        height: usize,
        bits: &'bits [u32],
        rowstride_bytes: usize,
    ) -> Result<Self, CreateFailed> {
        unsafe { Self::from_raw(format, width, height, bits.as_ptr(), rowstride_bytes) }
    }

    pub unsafe fn from_raw(
        format: FormatCode,
        width: usize,
        height: usize,
        bits: *const u32,
        rowstride_bytes: usize,
    ) -> Result<Self, CreateFailed> {
        let ptr = unsafe {
            ffi::pixman_image_create_bits_no_clear(
                format.into(),
                width as c_int,
                height as c_int,
                bits as *mut _,
                rowstride_bytes as c_int,
            )
        };

        if ptr.is_null() {
            Err(CreateFailed)
        } else {
            Ok(unsafe { Self::from_ptr(ptr) })
        }
    }
}

impl Image<'static, 'static, true> {
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
            Ok(unsafe { Self::from_ptr(ptr) })
        }
    }
}

impl<'bits> Image<'bits, 'static, true> {
    pub fn from_slice_mut(
        format: FormatCode,
        width: usize,
        height: usize,
        bits: &'bits mut [u32],
        rowstride_bytes: usize,
        clear: bool,
    ) -> Result<Self, CreateFailed> {
        unsafe {
            Self::from_raw_mut(
                format,
                width,
                height,
                bits.as_mut_ptr(),
                rowstride_bytes,
                clear,
            )
        }
    }

    pub unsafe fn from_raw_mut(
        format: FormatCode,
        width: usize,
        height: usize,
        bits: *mut u32,
        rowstride_bytes: usize,
        clear: bool,
    ) -> Result<Self, CreateFailed> {
        let ptr = if clear {
            unsafe {
                ffi::pixman_image_create_bits(
                    format.into(),
                    width as c_int,
                    height as c_int,
                    bits,
                    rowstride_bytes as c_int,
                )
            }
        } else {
            unsafe {
                ffi::pixman_image_create_bits_no_clear(
                    format.into(),
                    width as c_int,
                    height as c_int,
                    bits,
                    rowstride_bytes as c_int,
                )
            }
        };

        if ptr.is_null() {
            Err(CreateFailed)
        } else {
            Ok(unsafe { Self::from_ptr(ptr) })
        }
    }
}

impl<'bits, 'alpha> Image<'bits, 'alpha, true> {
    pub fn fill_boxes(
        &self,
        op: Operation,
        color: impl Into<Color>,
        boxes: &[Box32],
    ) -> Result<(), OperationFailed> {
        let color = color.into();
        let res = unsafe {
            ffi::pixman_image_fill_boxes(
                op.into(),
                self.as_ptr(),
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
        &self,
        op: Operation,
        color: impl Into<Color>,
        rects: &[Rectangle16],
    ) -> Result<(), OperationFailed> {
        let color = color.into();
        let res = unsafe {
            ffi::pixman_image_fill_rectangles(
                op.into(),
                self.as_ptr(),
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

    #[allow(clippy::too_many_arguments)]
    pub fn composite(
        &self,
        operation: Operation,
        src: &ImageRef,
        mask: Option<&ImageRef>,
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
            mask.as_ptr()
        } else {
            std::ptr::null_mut()
        };
        unsafe {
            ffi::pixman_image_composite(
                operation.into(),
                src.as_ptr(),
                mask_ptr,
                self.as_ptr(),
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

    #[allow(clippy::too_many_arguments)]
    pub fn composite32(
        &self,
        operation: Operation,
        src: &ImageRef,
        mask: Option<&ImageRef>,
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
            mask.as_ptr()
        } else {
            std::ptr::null_mut()
        };
        unsafe {
            ffi::pixman_image_composite32(
                operation.into(),
                src.as_ptr(),
                mask_ptr,
                self.as_ptr(),
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

    #[allow(clippy::too_many_arguments)]
    pub fn composite_triangles(
        &self,
        operation: Operation,
        src: &ImageRef,
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
                src.as_ptr(),
                self.as_ptr(),
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

    #[allow(clippy::too_many_arguments)]
    pub fn composite_trapezoids(
        &self,
        operation: Operation,
        src: &ImageRef,
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
                src.as_ptr(),
                self.as_ptr(),
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

    pub fn add_traps(&self, x_off: i16, y_off: i16, traps: &[Trap]) {
        unsafe {
            ffi::pixman_add_traps(
                self.as_ptr(),
                x_off,
                y_off,
                traps.len() as i32,
                traps.as_ptr() as *const _,
            );
        }
    }

    pub fn add_trapezoids(&self, x_off: i16, y_off: i32, traps: &[Trapezoid]) {
        unsafe {
            ffi::pixman_add_trapezoids(
                self.as_ptr(),
                x_off,
                y_off,
                traps.len() as i32,
                traps.as_ptr() as *const _,
            );
        }
    }

    pub fn add_triangles(&self, x_off: i32, y_off: i32, tris: &[Triangle]) {
        unsafe {
            ffi::pixman_add_triangles(
                self.as_ptr(),
                x_off,
                y_off,
                tris.len() as i32,
                tris.as_ptr() as *const _,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_composite_region(
        &self,
        src: &ImageRef,
        mask: Option<&ImageRef>,
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
            mask.as_ptr()
        } else {
            std::ptr::null_mut()
        };

        let mut region = MaybeUninit::uninit();
        let res = unsafe {
            ffi::pixman_compute_composite_region(
                region.as_mut_ptr(),
                src.as_ptr(),
                mask_ptr,
                self.as_ptr(),
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

    pub fn rasterize_edges(&self, l: Edge, r: Edge, t: impl Into<Fixed>, b: impl Into<Fixed>) {
        unsafe {
            ffi::pixman_rasterize_edges(
                self.as_ptr(),
                l.as_ptr() as *mut _,
                r.as_ptr() as *mut _,
                t.into().into_raw(),
                b.into().into_raw(),
            )
        }
    }

    pub fn rasterize_trapezoid(&self, trap: Trapezoid, x_off: i32, y_off: i32) {
        unsafe { ffi::pixman_rasterize_trapezoid(self.as_ptr(), trap.as_ptr(), x_off, y_off) }
    }

    /// Access the underlying pixel data
    ///
    /// Returns `None` in case the image has no underlying pixel data, e.g. solid/gradient/...
    pub fn data_mut(&self) -> Option<&mut [u8]> {
        let height = self.height();
        let stride = self.stride();
        let ptr = unsafe { ffi::pixman_image_get_data(self.as_ptr()) };

        if ptr.is_null() {
            None
        } else {
            unsafe {
                Some(std::slice::from_raw_parts_mut(
                    ptr as *mut u8,
                    stride * height,
                ))
            }
        }
    }
}
