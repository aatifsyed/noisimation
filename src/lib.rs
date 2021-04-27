use crossterm::ExecutableCommand;
use image;
use noise::{self, NoiseFn};
use viuer::{self};

pub trait PrintImages {
    fn print_images(self, config: &viuer::Config) -> ();
}

impl<T> PrintImages for T
where
    T: IntoIterator<Item = image::DynamicImage> + Sized,
{
    fn print_images(self, config: &viuer::Config) {
        let mut images = self.into_iter().peekable();
        scroll_down();

        while let Some(image) = images.next() {
            let (_printed_width, printed_height) = viuer::print(&image, config).unwrap();

            if images.peek().is_some() {
                std::io::stdout()
                    .execute(crossterm::cursor::MoveUp(printed_height as u16))
                    .unwrap();
            }
        }
    }
}

/// Scroll the terminal down (before displaying an image)
fn scroll_down() {
    std::io::stdout()
        .execute(crossterm::terminal::ScrollDown(
            crossterm::terminal::size().unwrap().1,
        ))
        .unwrap();
}

pub trait PrintImage {
    fn print_image(&self, config: &viuer::Config) -> ();
}

impl PrintImage for image::DynamicImage {
    fn print_image(&self, config: &viuer::Config) -> () {
        scroll_down();
        viuer::print(self, config).unwrap();
    }
}

pub trait MakeImage<P: image::Pixel, Container>
where
    P: image::Pixel,
{
    fn make_image(&self, width: u32, height: u32) -> image::ImageBuffer<P, Container>;
}

impl MakeImage<image::Luma<u16>, Vec<u16>> for dyn NoiseFn<[f64; 2]> {
    fn make_image(
        &self,
        width: u32,
        height: u32,
    ) -> image::ImageBuffer<image::Luma<u16>, Vec<u16>> {
        image::ImageBuffer::from_fn(width, height, |x, y| {
            let value = self.get([x as f64, y as f64]);
            image::Luma([value as u16])
        })
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
        fixtures::bold_and_brash().print_image(&Default::default());
    }

    #[test]
    fn animate() {
        let img = fixtures::bold_and_brash();
        let config = Default::default();

        (0..359)
            .map(|rotation| img.huerotate(rotation))
            .print_images(&config);
    }

    #[test]
    fn make_image() {
        let function = noise::Checkerboard::default(); // 0s or 1s
        let function = noise::ScaleBias::new(&function).set_scale(std::u16::MAX as f64)
            as dyn NoiseFn<[f64; 2]>;
        function.make_image(100, 100);
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
        let function = noise::ScaleBias::new(&function).set_scale((std::u16::MAX / 2) as f64);
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
                let v = (0..1000)
                    .map(|i| $noise_fn.get([i as f64, 10.0]))
                    .collect::<Vec<_>>();
                let max = v
                    .iter()
                    .fold(f64::MIN, |orig, other| f64::max(orig, *other));
                let min = v
                    .iter()
                    .fold(f64::MAX, |orig, other| f64::min(orig, *other));
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
