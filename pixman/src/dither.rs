use pixman_sys as ffi;

#[derive(Debug, Clone, Copy)]
pub enum Dither {
    None,
    Fast,
    Good,
    Best,
    OrderedBayer8,
    OrderedBlueNoise64,
}

#[derive(Debug)]
pub struct UnknownDither(ffi::pixman_dither_t);

impl TryFrom<ffi::pixman_dither_t> for Dither {
    type Error = UnknownDither;

    fn try_from(value: ffi::pixman_dither_t) -> Result<Self, Self::Error> {
        let repeat = match value {
            ffi::pixman_dither_t_PIXMAN_DITHER_NONE => Dither::None,
            ffi::pixman_dither_t_PIXMAN_DITHER_FAST => Dither::Fast,
            ffi::pixman_dither_t_PIXMAN_DITHER_GOOD => Dither::Good,
            ffi::pixman_dither_t_PIXMAN_DITHER_BEST => Dither::Best,
            ffi::pixman_dither_t_PIXMAN_DITHER_ORDERED_BAYER_8 => Dither::OrderedBayer8,
            ffi::pixman_dither_t_PIXMAN_DITHER_ORDERED_BLUE_NOISE_64 => Dither::OrderedBlueNoise64,
            _ => return Err(UnknownDither(value)),
        };
        Ok(repeat)
    }
}

impl From<Dither> for ffi::pixman_dither_t {
    fn from(value: Dither) -> Self {
        match value {
            Dither::None => ffi::pixman_dither_t_PIXMAN_DITHER_NONE,
            Dither::Fast => ffi::pixman_dither_t_PIXMAN_DITHER_FAST,
            Dither::Good => ffi::pixman_dither_t_PIXMAN_DITHER_GOOD,
            Dither::Best => ffi::pixman_dither_t_PIXMAN_DITHER_BEST,
            Dither::OrderedBayer8 => ffi::pixman_dither_t_PIXMAN_DITHER_ORDERED_BAYER_8,
            Dither::OrderedBlueNoise64 => ffi::pixman_dither_t_PIXMAN_DITHER_ORDERED_BLUE_NOISE_64,
        }
    }
}
