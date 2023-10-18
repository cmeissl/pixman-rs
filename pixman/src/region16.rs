use std::{
    mem::MaybeUninit,
    os::raw::{c_int, c_uint},
};

use pixman_sys as ffi;

use crate::Box16;

#[derive(Debug)]
pub struct Region16(ffi::pixman_region16_t);

impl Region16 {
    pub fn init_rect(x: i16, y: i16, width: u16, height: u16) -> Self {
        let mut region = MaybeUninit::uninit();
        unsafe {
            ffi::pixman_region_init_rect(
                region.as_mut_ptr(),
                x as c_int,
                y as c_int,
                width as c_uint,
                height as c_uint,
            );
        }
        Self(unsafe { region.assume_init() })
    }

    pub fn init_rects(boxes: &[Box16]) -> Self {
        let mut region = MaybeUninit::uninit();
        unsafe {
            ffi::pixman_region_init_rects(region.as_mut_ptr(), boxes.as_ptr(), boxes.len() as i32);
        }
        Self(unsafe { region.assume_init() })
    }

    pub fn init_with_extents(extents: &[Box16]) -> Self {
        let mut region = MaybeUninit::uninit();
        unsafe { ffi::pixman_region_init_with_extents(region.as_mut_ptr(), extents.as_ptr()) }
        Self(unsafe { region.assume_init() })
    }

    pub fn intersect(&self, other: &Region16) -> Region16 {
        let mut region = Region16::default();
        unsafe {
            ffi::pixman_region_intersect(&mut region.0, &self.0, &other.0);
        }
        region
    }

    pub fn intersect_rect(&self, x: i16, y: i16, width: u16, height: u16) -> Self {
        let mut dest = Region16::default();
        unsafe {
            ffi::pixman_region_intersect_rect(
                &mut dest.0,
                &self.0,
                x as c_int,
                y as c_int,
                width as c_uint,
                height as c_uint,
            );
        }
        dest
    }

    // Take a region and a box and return a region that is everything
    // in the box but not in the region. The careful reader will note
    // that this is the same as subtracting the region from the box...
    pub fn inverse(&self, bbox: Box16) -> Self {
        let mut region = Region16::default();
        unsafe {
            ffi::pixman_region_inverse(&mut region.0, &self.0, &bbox);
        }
        region
    }

    pub fn n_rects(&self) -> usize {
        unsafe { ffi::pixman_region_n_rects(&self.0) as usize }
    }

    pub fn is_non_empty(&self) -> bool {
        unsafe { ffi::pixman_region_not_empty(&self.0) == 1 }
    }

    pub fn rectangles(&self) -> &[Box16] {
        let mut n_rects = 0;

        let rectangles = unsafe {
            let rects = ffi::pixman_region_rectangles(&self.0, &mut n_rects);
            std::slice::from_raw_parts(rects, n_rects as usize)
        };

        rectangles
    }

    pub fn reset(&mut self, box_: Box16) {
        unsafe {
            ffi::pixman_region_reset(&mut self.0, &box_);
        }
    }

    pub fn selfcheck(&mut self) -> bool {
        unsafe { ffi::pixman_region_selfcheck(&mut self.0) == 1 }
    }

    // Subtract reg_s from reg_m and leave the result in reg_d.
    // S stands for subtrahend, M for minuend and D for difference.
    pub fn subtract(&self, other: &Region16) -> Self {
        let mut region = Region16::default();
        unsafe {
            ffi::pixman_region_subtract(&mut region.0, &self.0, &other.0);
        }
        region
    }

    pub fn translate(&mut self, x: i16, y: i16) {
        unsafe { ffi::pixman_region_translate(&mut self.0, x as c_int, y as c_int) };
    }

    pub fn union(&self, other: &Region16) -> Self {
        let mut region = Region16::default();
        unsafe {
            ffi::pixman_region_union(&mut region.0, &self.0, &other.0);
        }
        region
    }

    pub fn union_rect(&self, x: i16, y: i16, width: u16, height: u16) -> Self {
        let mut region = Region16::default();
        unsafe {
            ffi::pixman_region_union_rect(
                &mut region.0,
                &self.0,
                x as c_int,
                y as c_int,
                width as c_uint,
                height as c_uint,
            );
        }
        region
    }

    pub fn clear(&mut self) {
        unsafe {
            ffi::pixman_region_clear(&mut self.0);
        }
    }

    pub fn contains_point(&self, x: i16, y: i16) -> Option<Box16> {
        let mut r#box = MaybeUninit::uninit();
        let contains_point = unsafe {
            ffi::pixman_region_contains_point(&self.0, x as c_int, y as c_int, r#box.as_mut_ptr())
                == 1
        };

        if contains_point {
            Some(unsafe { r#box.assume_init() })
        } else {
            None
        }
    }

    pub fn contains_rectangle(&self, rect: Box16) -> Option<usize> {
        let overlap = unsafe { ffi::pixman_region_contains_rectangle(&self.0, &rect) };

        if overlap > 0 {
            Some(overlap as usize)
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn as_ptr(&self) -> *const ffi::pixman_region16_t {
        &self.0
    }
}

impl Default for Region16 {
    #[inline]
    fn default() -> Self {
        let mut region = MaybeUninit::uninit();
        unsafe {
            ffi::pixman_region_init(region.as_mut_ptr());
        }
        Self(unsafe { region.assume_init() })
    }
}

impl Clone for Region16 {
    #[inline]
    fn clone(&self) -> Self {
        let mut region = Region16::default();
        unsafe {
            ffi::pixman_region_copy(&mut region.0, &self.0);
        }
        region
    }
}

impl PartialEq for Region16 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::pixman_region_equal(&self.0, &other.0) == 1 }
    }
}
impl Eq for Region16 {}

impl Drop for Region16 {
    fn drop(&mut self) {
        unsafe {
            ffi::pixman_region_fini(&mut self.0);
        }
    }
}

impl From<ffi::pixman_region16_t> for Region16 {
    #[inline]
    fn from(value: ffi::pixman_region16_t) -> Self {
        Self(value)
    }
}

impl From<Region16> for ffi::pixman_region16_t {
    #[inline]
    fn from(value: Region16) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::Region16;
    use crate::Box16;

    #[test]
    fn init() {
        let region = Region16::default();
        assert!(!region.is_non_empty());
    }

    #[test]
    fn init_rect() {
        let region = Region16::init_rect(-100, -200, 300, 400);
        assert!(region.is_non_empty());
    }

    #[test]
    fn init_rects() {
        let region = Region16::init_rects(&[Box16 {
            x1: 0,
            x2: 100,
            y1: 0,
            y2: 100,
        }]);
        assert!(region.is_non_empty());
    }

    #[test]
    fn init_with_extents() {
        let region = Region16::init_with_extents(&[Box16 {
            x1: 0,
            x2: 100,
            y1: 0,
            y2: 100,
        }]);
        assert!(region.is_non_empty());
    }

    #[test]
    fn intersect() {
        let reg1 = Region16::init_rect(0, 0, 100, 100);
        let reg2 = Region16::init_rect(50, 50, 100, 100);
        let _new_reg = reg1.intersect(&reg2);
    }

    #[test]
    fn intersect_rect() {
        let reg1 = Region16::init_rect(0, 0, 100, 100);
        let _new_reg = reg1.intersect_rect(50, 50, 100, 100);
    }

    #[test]
    fn inverse() {
        let region = Region16::init_rect(0, 0, 50, 50);
        let _inverse = region.inverse(Box16 {
            x1: 0,
            x2: 100,
            y1: 0,
            y2: 100,
        });
    }

    #[test]
    fn rectangles() {
        assert!(Region16::default().rectangles().is_empty());
        let region = Region16::init_rect(0, 0, 50, 50);
        let inverse = region.inverse(Box16 {
            x1: 0,
            x2: 100,
            y1: 0,
            y2: 100,
        });
        assert!(!inverse.rectangles().is_empty());
    }

    #[test]
    fn subtract() {
        let reg1 = Region16::init_rect(0, 0, 100, 100);
        let reg2 = Region16::init_rect(50, 50, 100, 100);
        let _new_reg = reg1.subtract(&reg2);
    }
}
