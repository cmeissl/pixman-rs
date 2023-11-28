use pixman::{Fixed, FormatCode, GradientStop, Image, Operation, Point, Repeat, Transform};

const WIDTH: usize = 400;
const HEIGHT: usize = 200;

pub fn main() {
    let stops = [
        GradientStop::new(0, [0x0000, 0x0000, 0x0000, 0x0000]),
        GradientStop::new(1, [0xffff, 0x0000, 0x1111, 0xffff]),
    ];

    let p1 = Point::from((0f64, 0f64));
    let p2 = Point::from((WIDTH as f64, 0f64));

    let transform = Transform::new([
        [Fixed::ONE, Fixed::ZERO, Fixed::ZERO],
        [Fixed::ZERO, Fixed::ONE, Fixed::ZERO],
        [Fixed::ZERO, Fixed::ZERO, Fixed::ONE],
    ]);

    let mut alpha = [0x4f00004fu32; WIDTH * HEIGHT]; /* pale blue */
    let mut alpha_img = Image::from_bits(
        FormatCode::A8R8G8B8,
        WIDTH,
        HEIGHT,
        &mut alpha,
        WIDTH * 4,
        false,
    )
    .unwrap();

    let mut dest = [0xffffff00u32; WIDTH * HEIGHT]; /* yellow */
    let mut dest_img = Image::from_bits(
        FormatCode::A8R8G8B8,
        WIDTH,
        HEIGHT,
        &mut dest,
        WIDTH * 4,
        false,
    )
    .unwrap();

    let mut src = [0xffff0000; WIDTH * HEIGHT];
    let mut src_img = Image::from_bits(
        FormatCode::A8R8G8B8,
        WIDTH,
        HEIGHT,
        &mut src,
        WIDTH * 4,
        false,
    )
    .unwrap();

    let mut grad_img = Image::linear_gradient(p1, p2, &stops).unwrap();
    grad_img.set_transform(transform).unwrap();
    grad_img.set_repeat(Repeat::Pad);

    alpha_img.composite(
        Operation::Over,
        &grad_img,
        None,
        0,
        0,
        0,
        0,
        0,
        0,
        (10 * WIDTH) as u16,
        HEIGHT as u16,
    );

    src_img.set_alpha_map(Some(&alpha_img), 10, 10);

    dest_img.composite(
        Operation::Over,
        &src_img,
        None,
        0,
        0,
        0,
        0,
        0,
        0,
        (10 * WIDTH) as u16,
        HEIGHT as u16,
    );

    let mut out_img = Image::new(
        FormatCode::A8B8G8R8,
        dest_img.width(),
        dest_img.height(),
        false,
    )
    .unwrap();
    out_img.composite(
        Operation::Src,
        &dest_img,
        None,
        0,
        0,
        0,
        0,
        0,
        0,
        dest_img.width() as u16,
        dest_img.height() as u16,
    );

    let out_data = out_img.data().unwrap();
    let image_buffer = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(
        out_img.width() as u32,
        out_img.height() as u32,
        out_data,
    )
    .unwrap();
    image_buffer
        .save_with_format("out.png", image::ImageFormat::Png)
        .unwrap();
}
