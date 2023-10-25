use pixman_sys as ffi;

#[derive(Debug, Clone, Copy)]
pub enum FormatCode {
    RgbaFloat,
    RgbFloat,
    A8R8G8B8,
    X8R8G8B8,
    A8B8G8R8,
    X8B8G8R8,
    B8G8R8A8,
    B8G8R8X8,
    R8G8B8A8,
    R8G8B8X8,
    X14R6G6B6,
    X2R10G10B10,
    A2R10G10B10,
    X2B10G10R10,
    A2B10G10R10,
    A8R8G8B8sRGB,
    R8G8B8,
    B8G8R8,
    R5G6B5,
    B5G6R5,
    A1R5G5B5,
    X1R5G5B5,
    A1B5G5R5,
    X1B5G5R5,
    A4R4G4B4,
    X4R4G4B4,
    A4B4G4R4,
    X4B4G4R4,
    A8,
    R3G3B2,
    B2G3R3,
    A2R2G2B2,
    A2B2G2R2,
    C8,
    G8,
    X4A4,
    X4C4,
    X4G4,
    A4,
    R1G2B1,
    B1G2R1,
    A1R1G1B1,
    A1B1G1R1,
    C4,
    G4,
    A1,
    G1,
    YUY2,
    YV12,
}

impl FormatCode {
    pub fn bpp(code: Self) -> u32 {
        let val = Into::<ffi::pixman_format_code_t>::into(code);
        let ofs = 24;
        let num = 8;
        ((val >> (ofs)) & ((1 << (num)) - 1)) << ((val >> 22) & 3)
    }
}

#[derive(Debug)]
pub struct UnknownFormat(ffi::pixman_format_code_t);

impl TryFrom<ffi::pixman_format_code_t> for FormatCode {
    type Error = UnknownFormat;

    fn try_from(value: ffi::pixman_format_code_t) -> Result<Self, Self::Error> {
        let format = match value {
            ffi::pixman_format_code_t_PIXMAN_rgba_float => FormatCode::RgbaFloat,
            ffi::pixman_format_code_t_PIXMAN_rgb_float => FormatCode::RgbFloat,
            ffi::pixman_format_code_t_PIXMAN_a8r8g8b8 => FormatCode::A8R8G8B8,
            ffi::pixman_format_code_t_PIXMAN_x8r8g8b8 => FormatCode::X8R8G8B8,
            ffi::pixman_format_code_t_PIXMAN_a8b8g8r8 => FormatCode::A8B8G8R8,
            ffi::pixman_format_code_t_PIXMAN_x8b8g8r8 => FormatCode::X8B8G8R8,
            ffi::pixman_format_code_t_PIXMAN_b8g8r8a8 => FormatCode::B8G8R8A8,
            ffi::pixman_format_code_t_PIXMAN_b8g8r8x8 => FormatCode::B8G8R8X8,
            ffi::pixman_format_code_t_PIXMAN_r8g8b8a8 => FormatCode::R8G8B8A8,
            ffi::pixman_format_code_t_PIXMAN_r8g8b8x8 => FormatCode::R8G8B8X8,
            ffi::pixman_format_code_t_PIXMAN_x14r6g6b6 => FormatCode::X14R6G6B6,
            ffi::pixman_format_code_t_PIXMAN_x2r10g10b10 => FormatCode::X2R10G10B10,
            ffi::pixman_format_code_t_PIXMAN_a2r10g10b10 => FormatCode::A2R10G10B10,
            ffi::pixman_format_code_t_PIXMAN_x2b10g10r10 => FormatCode::X2B10G10R10,
            ffi::pixman_format_code_t_PIXMAN_a2b10g10r10 => FormatCode::A2B10G10R10,
            ffi::pixman_format_code_t_PIXMAN_a8r8g8b8_sRGB => FormatCode::A8R8G8B8sRGB,
            ffi::pixman_format_code_t_PIXMAN_r8g8b8 => FormatCode::R8G8B8,
            ffi::pixman_format_code_t_PIXMAN_b8g8r8 => FormatCode::B8G8R8,
            ffi::pixman_format_code_t_PIXMAN_r5g6b5 => FormatCode::R5G6B5,
            ffi::pixman_format_code_t_PIXMAN_b5g6r5 => FormatCode::B5G6R5,
            ffi::pixman_format_code_t_PIXMAN_a1r5g5b5 => FormatCode::A1R5G5B5,
            ffi::pixman_format_code_t_PIXMAN_x1r5g5b5 => FormatCode::X1R5G5B5,
            ffi::pixman_format_code_t_PIXMAN_a1b5g5r5 => FormatCode::A1B5G5R5,
            ffi::pixman_format_code_t_PIXMAN_x1b5g5r5 => FormatCode::X1B5G5R5,
            ffi::pixman_format_code_t_PIXMAN_a4r4g4b4 => FormatCode::A4R4G4B4,
            ffi::pixman_format_code_t_PIXMAN_x4r4g4b4 => FormatCode::X4R4G4B4,
            ffi::pixman_format_code_t_PIXMAN_a4b4g4r4 => FormatCode::A4B4G4R4,
            ffi::pixman_format_code_t_PIXMAN_x4b4g4r4 => FormatCode::X4B4G4R4,
            ffi::pixman_format_code_t_PIXMAN_a8 => FormatCode::A8,
            ffi::pixman_format_code_t_PIXMAN_r3g3b2 => FormatCode::R3G3B2,
            ffi::pixman_format_code_t_PIXMAN_b2g3r3 => FormatCode::B2G3R3,
            ffi::pixman_format_code_t_PIXMAN_a2r2g2b2 => FormatCode::A2R2G2B2,
            ffi::pixman_format_code_t_PIXMAN_a2b2g2r2 => FormatCode::A2B2G2R2,
            ffi::pixman_format_code_t_PIXMAN_c8 => FormatCode::C8,
            ffi::pixman_format_code_t_PIXMAN_g8 => FormatCode::G8,
            ffi::pixman_format_code_t_PIXMAN_x4a4 => FormatCode::X4A4,
            // NOTE: These format codes are identical to c8 and g8, respectively
            // ffi::pixman_format_code_t_PIXMAN_x4c4 => FormatCode::X4C4,
            // ffi::pixman_format_code_t_PIXMAN_x4g4 => FormatCode::X4G4,
            ffi::pixman_format_code_t_PIXMAN_a4 => FormatCode::A4,
            ffi::pixman_format_code_t_PIXMAN_r1g2b1 => FormatCode::R1G2B1,
            ffi::pixman_format_code_t_PIXMAN_b1g2r1 => FormatCode::B1G2R1,
            ffi::pixman_format_code_t_PIXMAN_a1r1g1b1 => FormatCode::A1R1G1B1,
            ffi::pixman_format_code_t_PIXMAN_a1b1g1r1 => FormatCode::A1B1G1R1,
            ffi::pixman_format_code_t_PIXMAN_c4 => FormatCode::C4,
            ffi::pixman_format_code_t_PIXMAN_g4 => FormatCode::G4,
            ffi::pixman_format_code_t_PIXMAN_a1 => FormatCode::A1,
            ffi::pixman_format_code_t_PIXMAN_g1 => FormatCode::G1,
            ffi::pixman_format_code_t_PIXMAN_yuy2 => FormatCode::YUY2,
            ffi::pixman_format_code_t_PIXMAN_yv12 => FormatCode::YV12,
            _ => return Err(UnknownFormat(value)),
        };
        Ok(format)
    }
}

impl From<FormatCode> for ffi::pixman_format_code_t {
    fn from(value: FormatCode) -> Self {
        match value {
            FormatCode::RgbaFloat => ffi::pixman_format_code_t_PIXMAN_rgba_float,
            FormatCode::RgbFloat => ffi::pixman_format_code_t_PIXMAN_rgb_float,
            FormatCode::A8R8G8B8 => ffi::pixman_format_code_t_PIXMAN_a8r8g8b8,
            FormatCode::X8R8G8B8 => ffi::pixman_format_code_t_PIXMAN_x8r8g8b8,
            FormatCode::A8B8G8R8 => ffi::pixman_format_code_t_PIXMAN_a8b8g8r8,
            FormatCode::X8B8G8R8 => ffi::pixman_format_code_t_PIXMAN_x8b8g8r8,
            FormatCode::B8G8R8A8 => ffi::pixman_format_code_t_PIXMAN_b8g8r8a8,
            FormatCode::B8G8R8X8 => ffi::pixman_format_code_t_PIXMAN_b8g8r8x8,
            FormatCode::R8G8B8A8 => ffi::pixman_format_code_t_PIXMAN_r8g8b8a8,
            FormatCode::R8G8B8X8 => ffi::pixman_format_code_t_PIXMAN_r8g8b8x8,
            FormatCode::X14R6G6B6 => ffi::pixman_format_code_t_PIXMAN_x14r6g6b6,
            FormatCode::X2R10G10B10 => ffi::pixman_format_code_t_PIXMAN_x2r10g10b10,
            FormatCode::A2R10G10B10 => ffi::pixman_format_code_t_PIXMAN_a2r10g10b10,
            FormatCode::X2B10G10R10 => ffi::pixman_format_code_t_PIXMAN_x2b10g10r10,
            FormatCode::A2B10G10R10 => ffi::pixman_format_code_t_PIXMAN_a2b10g10r10,
            FormatCode::A8R8G8B8sRGB => ffi::pixman_format_code_t_PIXMAN_a8r8g8b8_sRGB,
            FormatCode::R8G8B8 => ffi::pixman_format_code_t_PIXMAN_r8g8b8,
            FormatCode::B8G8R8 => ffi::pixman_format_code_t_PIXMAN_b8g8r8,
            FormatCode::R5G6B5 => ffi::pixman_format_code_t_PIXMAN_r5g6b5,
            FormatCode::B5G6R5 => ffi::pixman_format_code_t_PIXMAN_b5g6r5,
            FormatCode::A1R5G5B5 => ffi::pixman_format_code_t_PIXMAN_a1r5g5b5,
            FormatCode::X1R5G5B5 => ffi::pixman_format_code_t_PIXMAN_x1r5g5b5,
            FormatCode::A1B5G5R5 => ffi::pixman_format_code_t_PIXMAN_a1b5g5r5,
            FormatCode::X1B5G5R5 => ffi::pixman_format_code_t_PIXMAN_x1b5g5r5,
            FormatCode::A4R4G4B4 => ffi::pixman_format_code_t_PIXMAN_a4r4g4b4,
            FormatCode::X4R4G4B4 => ffi::pixman_format_code_t_PIXMAN_x4r4g4b4,
            FormatCode::A4B4G4R4 => ffi::pixman_format_code_t_PIXMAN_a4b4g4r4,
            FormatCode::X4B4G4R4 => ffi::pixman_format_code_t_PIXMAN_x4b4g4r4,
            FormatCode::A8 => ffi::pixman_format_code_t_PIXMAN_a8,
            FormatCode::R3G3B2 => ffi::pixman_format_code_t_PIXMAN_r3g3b2,
            FormatCode::B2G3R3 => ffi::pixman_format_code_t_PIXMAN_b2g3r3,
            FormatCode::A2R2G2B2 => ffi::pixman_format_code_t_PIXMAN_a2r2g2b2,
            FormatCode::A2B2G2R2 => ffi::pixman_format_code_t_PIXMAN_a2b2g2r2,
            FormatCode::C8 => ffi::pixman_format_code_t_PIXMAN_c8,
            FormatCode::G8 => ffi::pixman_format_code_t_PIXMAN_g8,
            FormatCode::X4A4 => ffi::pixman_format_code_t_PIXMAN_x4a4,
            FormatCode::X4C4 => ffi::pixman_format_code_t_PIXMAN_x4c4,
            FormatCode::X4G4 => ffi::pixman_format_code_t_PIXMAN_x4g4,
            FormatCode::A4 => ffi::pixman_format_code_t_PIXMAN_a4,
            FormatCode::R1G2B1 => ffi::pixman_format_code_t_PIXMAN_r1g2b1,
            FormatCode::B1G2R1 => ffi::pixman_format_code_t_PIXMAN_b1g2r1,
            FormatCode::A1R1G1B1 => ffi::pixman_format_code_t_PIXMAN_a1r1g1b1,
            FormatCode::A1B1G1R1 => ffi::pixman_format_code_t_PIXMAN_a1b1g1r1,
            FormatCode::C4 => ffi::pixman_format_code_t_PIXMAN_c4,
            FormatCode::G4 => ffi::pixman_format_code_t_PIXMAN_g4,
            FormatCode::A1 => ffi::pixman_format_code_t_PIXMAN_a1,
            FormatCode::G1 => ffi::pixman_format_code_t_PIXMAN_g1,
            FormatCode::YUY2 => ffi::pixman_format_code_t_PIXMAN_yuy2,
            FormatCode::YV12 => ffi::pixman_format_code_t_PIXMAN_yv12,
        }
    }
}

#[cfg(feature = "drm-fourcc")]
#[derive(Debug)]
pub struct UnsupportedDrmFourcc;

#[cfg(feature = "drm-fourcc")]
impl TryFrom<drm_fourcc::DrmFourcc> for FormatCode {
    type Error = UnsupportedDrmFourcc;

    fn try_from(value: drm_fourcc::DrmFourcc) -> Result<Self, Self::Error> {
        use drm_fourcc::DrmFourcc;

        let format = match value {
            #[cfg(target_endian = "little")]
            DrmFourcc::Rgb565 => FormatCode::R5G6B5,

            #[cfg(target_endian = "little")]
            DrmFourcc::Xrgb8888 => FormatCode::X8R8G8B8,
            #[cfg(target_endian = "big")]
            DrmFourcc::Xrgb8888 => FormatCode::B8G8R8X8,

            #[cfg(target_endian = "little")]
            DrmFourcc::Argb8888 => FormatCode::A8R8G8B8,
            #[cfg(target_endian = "big")]
            DrmFourcc::Argb8888 => FormatCode::B8G8R8A8,

            #[cfg(target_endian = "little")]
            DrmFourcc::Xbgr8888 => FormatCode::X8B8G8R8,
            #[cfg(target_endian = "big")]
            DrmFourcc::Xbgr8888 => FormatCode::R8G8B8X8,

            #[cfg(target_endian = "little")]
            DrmFourcc::Abgr8888 => FormatCode::A8B8G8R8,
            #[cfg(target_endian = "big")]
            DrmFourcc::Abgr8888 => FormatCode::R8G8B8A8,

            #[cfg(target_endian = "little")]
            DrmFourcc::Rgbx8888 => FormatCode::R8G8B8X8,
            #[cfg(target_endian = "big")]
            DrmFourcc::Rgbx8888 => FormatCode::X8B8G8R8,

            #[cfg(target_endian = "little")]
            DrmFourcc::Rgba8888 => FormatCode::R8G8B8A8,
            #[cfg(target_endian = "big")]
            DrmFourcc::Rgba8888 => FormatCode::A8B8G8R8,

            #[cfg(target_endian = "little")]
            DrmFourcc::Bgrx8888 => FormatCode::B8G8R8X8,
            #[cfg(target_endian = "big")]
            DrmFourcc::Bgrx8888 => FormatCode::X8R8G8B8,

            #[cfg(target_endian = "little")]
            DrmFourcc::Bgra8888 => FormatCode::B8G8R8A8,
            #[cfg(target_endian = "big")]
            DrmFourcc::Bgra8888 => FormatCode::A8R8G8B8,

            #[cfg(target_endian = "little")]
            DrmFourcc::Xrgb2101010 => FormatCode::X2R10G10B10,

            #[cfg(target_endian = "little")]
            DrmFourcc::Argb2101010 => FormatCode::A2R10G10B10,

            #[cfg(target_endian = "little")]
            DrmFourcc::Xbgr2101010 => FormatCode::X2B10G10R10,

            #[cfg(target_endian = "little")]
            DrmFourcc::Abgr2101010 => FormatCode::A2B10G10R10,
            _ => return Err(UnsupportedDrmFourcc),
        };
        Ok(format)
    }
}

#[cfg(feature = "drm-fourcc")]
impl TryFrom<FormatCode> for drm_fourcc::DrmFourcc {
    type Error = UnsupportedDrmFourcc;

    fn try_from(value: FormatCode) -> Result<Self, Self::Error> {
        use drm_fourcc::DrmFourcc;

        let format = match value {
            #[cfg(target_endian = "little")]
            FormatCode::R5G6B5 => DrmFourcc::Rgb565,

            #[cfg(target_endian = "little")]
            FormatCode::X8R8G8B8 => DrmFourcc::Xrgb8888,
            #[cfg(target_endian = "big")]
            FormatCode::B8G8R8X8 => DrmFourcc::Xrgb8888,

            #[cfg(target_endian = "little")]
            FormatCode::A8R8G8B8 => DrmFourcc::Argb8888,
            #[cfg(target_endian = "big")]
            FormatCode::B8G8R8A8 => DrmFourcc::Argb8888,

            #[cfg(target_endian = "little")]
            FormatCode::X8B8G8R8 => DrmFourcc::Xbgr8888,
            #[cfg(target_endian = "big")]
            FormatCode::R8G8B8X8 => DrmFourcc::Xbgr8888,

            #[cfg(target_endian = "little")]
            FormatCode::A8B8G8R8 => DrmFourcc::Abgr8888,
            #[cfg(target_endian = "big")]
            FormatCode::R8G8B8A8 => DrmFourcc::Abgr8888,

            #[cfg(target_endian = "little")]
            FormatCode::R8G8B8X8 => DrmFourcc::Rgbx8888,
            #[cfg(target_endian = "big")]
            FormatCode::X8B8G8R8 => DrmFourcc::Rgbx8888,

            #[cfg(target_endian = "little")]
            FormatCode::R8G8B8A8 => DrmFourcc::Rgba8888,
            #[cfg(target_endian = "big")]
            FormatCode::A8B8G8R8 => DrmFourcc::Rgba8888,

            #[cfg(target_endian = "little")]
            FormatCode::B8G8R8X8 => DrmFourcc::Bgrx8888,
            #[cfg(target_endian = "big")]
            FormatCode::X8R8G8B8 => DrmFourcc::Bgrx8888,

            #[cfg(target_endian = "little")]
            FormatCode::B8G8R8A8 => DrmFourcc::Bgra8888,
            #[cfg(target_endian = "big")]
            FormatCode::A8R8G8B8 => DrmFourcc::Bgra8888,

            #[cfg(target_endian = "little")]
            FormatCode::X2R10G10B10 => DrmFourcc::Xrgb2101010,

            #[cfg(target_endian = "little")]
            FormatCode::A2R10G10B10 => DrmFourcc::Argb2101010,

            #[cfg(target_endian = "little")]
            FormatCode::X2B10G10R10 => DrmFourcc::Xbgr2101010,

            #[cfg(target_endian = "little")]
            FormatCode::A2B10G10R10 => DrmFourcc::Abgr2101010,
            _ => return Err(UnsupportedDrmFourcc),
        };
        Ok(format)
    }
}
