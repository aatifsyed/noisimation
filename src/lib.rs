use crossterm::ExecutableCommand;
use image;
use noise::{self, NoiseFn};
use viuer;

pub fn print_sequence<I>(images: I, config: &viuer::Config) -> ()
where
    I: IntoIterator<Item = image::DynamicImage>,
{
    let mut images = images.into_iter().peekable();
    std::io::stdout().execute(crossterm::terminal::ScrollDown(crossterm::terminal::size().unwrap().1)).unwrap();

    while let Some(image) = images.next() {
        let (_printed_width, printed_height) = viuer::print(&image, config).unwrap();

        if images.peek().is_some() {
            std::io::stdout()
                .execute(crossterm::cursor::MoveUp(printed_height as u16))
                .unwrap();
        }
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
    fn generate_value_noise() {
        let function = noise::Value::default(); // -1, +1
        let function = noise::Abs::new(&function);
        // let function = noise::ScaleBias::new(&function).set_bias(1.0);
        // let function = noise::Exponent::new(&function).set_exponent(10.0);
        let function = noise::ScaleBias::new(&function).set_scale((std::u16::MAX/2) as f64);
        let buf = image::ImageBuffer::from_fn(100, 100, |w, h| {
            let f = function.get([w as f64, h as f64]);
            image::Luma([f as u16])
        });
        let buf = image::DynamicImage::ImageLuma16(buf);
        viuer::print(&buf, &Default::default()).unwrap();
    }
    
    
    #[test]
    fn noise_ranges() {
        macro_rules! range_of_noise_fn {
            ($noise_fn:expr, $name:expr) => {
                let v = (0..1000).map(|i| $noise_fn.get([i as f64, 10.0])).collect::<Vec<_>>();
                let max = v.iter().fold(f64::MIN, |orig, other| f64::max(orig, *other));
                let min = v.iter().fold(f64::MAX, |orig, other| f64::min(orig, *other));
                println!("{:7.3?}\t{:7.3?}\t({})", min, max, $name);
            };
        }

        range_of_noise_fn!(noise::Billow::default(), "billow");
        range_of_noise_fn!(noise::Checkerboard::default(), "checkerboard");
        range_of_noise_fn!(noise::Fbm::default(), "fbm");
        range_of_noise_fn!(noise::OpenSimplex::default(), "opensimplex");
        range_of_noise_fn!(noise::Perlin::default(), "perlin");
        range_of_noise_fn!(noise::SuperSimplex::default(), "supersimplex");
        range_of_noise_fn!(noise::Value::default(), "value");
    }
}
