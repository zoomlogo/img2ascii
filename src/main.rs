use std::env;
use image::GenericImageView;
use image::io::Reader;

fn main() {
    let args: Vec<String> = env::args().collect();

    let brightness_string = ".,-~:;=!*#$@";

    /* error if args is less than 2 */
    if args.len() < 2 {
        panic!("not enough arguments, need 1");
    }

    let file = &args[1];
    let img = Reader::open(file).unwrap().decode().unwrap();
    let (width, _) = img.dimensions();

    let mut i = 0;
    for pixel in img.pixels() {
        let argb = pixel.2.0;
        let brightness = (argb[1] as u32
            + argb[2] as u32
            + argb[3] as u32) / 3;
        let index = brightness as usize / 23;
        print!("{}", brightness_string.as_bytes()[index] as char);
        if i % width == 0 {
            println!();
        }
        i += 1;
    }
}
