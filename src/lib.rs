use crossterm::ExecutableCommand;
use image;
use noise::{self, NoiseFn};
use viuer;

pub fn print_sequence<I>(images: I, config: &viuer::Config) -> ()
where
    I: IntoIterator<Item = image::DynamicImage>,
{
    for image in images.into_iter() {
        let (_printed_width, printed_height) = viuer::print(&image, config).unwrap();
        std::io::stdout()
            .execute(crossterm::cursor::MoveUp(printed_height as u16))
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fixtures {
        fn resources_tests() -> std::path::PathBuf {
            let mut pathbuf = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            pathbuf.push("resources/tests");
            pathbuf
        }

        pub fn bold_and_brash() -> image::DynamicImage {
            let mut pathbuf = resources_tests();
            pathbuf.push("bold and brash.jpg");
            image::io::Reader::open(pathbuf).unwrap().decode().unwrap()
        }
    }

    #[test]
    fn print() {
        let img = fixtures::bold_and_brash();
        viuer::print(&img, &Default::default()).unwrap();
    }

    #[test]
    fn animate() {
        let img = fixtures::bold_and_brash();
        let config = Default::default();

        let frames = (0..359).map(|rotation| img.huerotate(rotation));

        print_sequence(frames, &config)
    }

    #[test]
    fn generate_checkerboard() {
        let function = noise::Checkerboard::default(); // 0s or 1s
        let function = noise::ScaleBias::new(&function).set_scale(std::u16::MAX as f64);
        let buf = image::ImageBuffer::from_fn(10, 10, |w, h| {
            let f = function.get([w as f64, h as f64]);
            // println!("{} ({}), {} ({}), -> {} ({})", w, w as f64, h, h as f64, f, f as u16);
            image::Luma([f as u16])
        });
        let buf = image::DynamicImage::ImageLuma16(buf);
        viuer::print(&buf, &Default::default()).unwrap();
    }

    #[test]
    fn noise_ranges() {
        struct Function<'f, T> {
            function: &'f dyn NoiseFn<T>,
            name: &'f str,
        }

        let checkerboard = noise::Checkerboard::default();
        let billow = noise::Billow::default();

        let functions: Vec<Function<[f64; 2]>>= vec![Function{function: &checkerboard, name:"checkerboard"}, Function{function: &billow, name:"billow"}];

        // let mut vec: Vec<Function<[f64;2]>> = Vec::new();
        // vec.push(Function {function: Box::new(noise::Checkerboard::default()), name: "checkerboard"});
        // vec.push(Function {function: Box::new(noise::Billow::default()), name: "billow"});

        for function in functions {
            let buf = image::ImageBuffer::from_fn(10, 10, |w, h| {let f = function.function.get([w as f64, h as f64]); image::Luma([f as u16])});
            println!("{:>15}, max: {:?}, min: {:?}", function.name, buf.pixels().map(|p| p.0).max(), buf.pixels().map(|p|p.0).min());
        }
    }
}
