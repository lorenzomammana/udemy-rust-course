// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use image_processing::*;

fn single_command(args: &mut Vec<String>, infile: String, outfile: String) -> Vec<String> {
    let subcommand = args.remove(0);

    match subcommand.as_str() {
        // EXAMPLE FOR CONVERSION OPERATIONS
        "blur" => {
            if args.len() < 1 {
                print_usage_and_exit();
            }
            let amount: f32 = args.remove(0).parse().expect("Failed to parse a float number");
            // **OPTION**
            // Improve the blur implementation -- see the blur() function below
            blur(infile, outfile, amount);
        }

        // **OPTION**
        // Brighten -- see the brighten() function below
        "brighten" => {
            if args.len() < 1 {
                print_usage_and_exit();
            }

            let amount: i32 = args.remove(0).parse().expect("Failed to parse an integer number");
        
            brighten(infile, outfile, amount);
        }
        // **OPTION**
        // Crop -- see the crop() function below
        "crop" => {
            if args.len() < 4 {
                print_usage_and_exit();
            }

            let x: u32 = args.remove(0).parse().expect("Failed to parse an integer number");
            let y: u32 = args.remove(0).parse().expect("Failed to parse an integer number");
            let width: u32 = args.remove(0).parse().expect("Failed to parse an integer number");
            let height: u32 = args.remove(0).parse().expect("Failed to parse an integer number");

            crop(infile, outfile, (x, y, width, height));
        }
        // **OPTION**
        // Rotate -- see the rotate() function below
        "rotate" => {
            if args.len() < 1 {
                print_usage_and_exit();
            }

            let rotate_degree: String = args.remove(0);

            rotate(infile, outfile, rotate_degree);
        }
        // **OPTION**
        // Invert -- see the invert() function below
        "invert" => {
            invert(infile, outfile);
        }
        // **OPTION**
        // Grayscale -- see the grayscale() function below
        "grayscale" => {
            grayscale(infile, outfile);
        }
        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" => {
            fractal(outfile);
        }

        // **OPTION**
        // Generate -- see the generate() function below -- this should be sort of like "fractal()"!
        "generate" => {
            if args.len() < 3 {
                print_usage_and_exit();
            }
            let red = args.remove(0).parse().expect("Failed to parse an integer number");
            let green = args.remove(0).parse().expect("Failed to parse an integer number");
            let blue = args.remove(0).parse().expect("Failed to parse an integer number");

            generate(outfile, red, green, blue);
        }
        // For everything else...
        _ => {
            print_usage_and_exit();
        }
    }

    return args.to_vec();
}

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/

    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.len() < 2 {
        print_usage_and_exit();
    }

    let mut infile: String = args.remove(0);
    let outfile: String  = args.remove(0);

    // Stack commands one after another
    while args.len() > 0 {
        single_command(&mut args, infile.clone(), outfile.clone());

        // After processing set infile value equal to outfile
        infile = outfile.clone();
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("cargo run --release infile outfile [commands]");
    println!("blur amount");
    println!("brighten amount");
    println!("crop x y width height");
    println!("rotate rotate_degree[90, 180, 270]");
    println!("invert");
    println!("grayscale");
    println!("fractal");
    println!("generate red green blue");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}



// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
