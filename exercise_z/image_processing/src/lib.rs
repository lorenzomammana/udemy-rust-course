pub fn blur(infile: String, outfile: String, amount: f32) {
    // Here's how you open an existing image file
    let img = image::open(infile).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let img2 = img.blur(amount);
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn brighten(infile: String, outfile: String, amount: i32) {
    // See blur() for an example of how to open / save an image.
    let img = image::open(infile).expect("Failed to open INFILE.");
    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.
    let img2 = img.brighten(amount);
    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
    img2.save(outfile).expect("Failed writing OUTFILE");
}

pub fn crop(infile: String, outfile: String, (x, y, width, height): (u32, u32, u32, u32)) {
    // See blur() for an example of how to open an image.
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.
    let img2 = img.crop(x, y, width, height);
    // Challenge: parse the four values from the command-line and pass them
    // through to this function.
    img2.save(outfile).expect("Failed writing OUTFILE");
    // See blur() for an example of how to save the image.
}

pub fn rotate(infile: String, outfile: String, rotate_degree: String) {
    // See blur() for an example of how to open an image.
    let img = image::open(infile).expect("Failed to open INFILE.");
    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!
    let img2 = match rotate_degree.as_str() {
        "90" => img.rotate90(),
        "180" => img.rotate180(),
        "270" => img.rotate270(),
        _ => img
    };
    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.
    img2.save(outfile).expect("Failed writing OUTFILE");
    // See blur() for an example of how to save the image.
}

pub fn invert(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.
    img.invert();
    // See blur() for an example of how to save the image.
    img.save(outfile).expect("Failed writing OUTFILE");
}

pub fn grayscale(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    // .grayscale() takes no arguments. It returns a new image.
    img = img.grayscale();
    // See blur() for an example of how to save the image.
    img.save(outfile).expect("Failed writing OUTFILE");
}

pub fn generate(outfile: String, red: u8, green: u8, blue: u8) {
    // Create an ImageBuffer -- see fractal() for an example
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);
    // Iterate over the coordinates and pixels of the image -- see fractal() for an example
    for (x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Check whether the difference is less than zero or not
        let current_red = match ((0.3 * x as f32) as u8).checked_sub(red) {
            None => 0,
            Some(diff) => diff,
        };

        let current_green = match ((0.3 * x as f32) as u8).checked_sub(green) {
            None => 0,
            Some(diff) => diff,
        };

        let current_blue = match ((0.3 * x as f32) as u8).checked_sub(blue) {
            None => 0,
            Some(diff) => diff,
        };

        *pixel = image::Rgb([current_red, current_green, current_blue]);
    }
    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!
    imgbuf.save(outfile).unwrap();
    // See blur() for an example of how to save the image
}

// This code was adapted from https://github.com/PistonDevelopers/image
pub fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}