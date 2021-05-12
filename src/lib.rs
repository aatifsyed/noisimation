use crossterm::ExecutableCommand;
use image::{self, ImageBuffer};
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

pub fn make_image(
    from: &dyn NoiseFn<[f64; 3]>,
    across_width: u32,
    across_height: u32,
    at_depth: f64,
) -> image::ImageBuffer<image::Luma<u16>, Vec<u16>> {
    image::ImageBuffer::from_fn(across_width, across_height, |x, y| {
        let value = from.get([x as f64, y as f64, at_depth]);
        image::Luma([value as u16])
    })
}

pub fn make_volume<'d, D>(
    from: &'d dyn NoiseFn<[f64; 3]>,
    across_width: u32,
    across_height: u32,
    at_depths: D,
) -> impl Iterator<Item = ImageBuffer<image::Luma<u16>, Vec<u16>>> + 'd
where
    D: 'd + IntoIterator<Item = f64>,
{
    let v = at_depths
        .into_iter()
        .map(move |depth| make_image(from, across_width, across_height, depth));
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::DynamicImage;
    use itertools_num::linspace;

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
    fn generate_checkerboard() {
        let function = noise::Checkerboard::default(); // 0s or 1s
        let function = noise::ScaleBias::new(&function).set_scale(std::u16::MAX as f64);
        let image = make_image(&function, 10, 10, 0.0);
        image::DynamicImage::ImageLuma16(image).print_image(&Default::default());
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

    #[test]
    fn volume() {
        let function = noise::OpenSimplex::default();
        let function = noise::ScaleBias::new(&function).set_scale(std::u16::MAX as f64);
        make_volume(&function, 100, 100, linspace(0.0, 10.0, 100))
            .map(|i| DynamicImage::ImageLuma16(i))
            .print_images(&Default::default());
    }
}
