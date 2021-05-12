use itertools_num::linspace;
use noise::{self};
use noisimation::PrintImages;
use structopt::StructOpt;
use strum::VariantNames;
use strum_macros::{Display as EnumDisplay, EnumString, EnumVariantNames};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(possible_values = FilterOpt::VARIANTS)]
    function: FilterOpt,
    #[structopt(short, long)]
    bias: f64,
    #[structopt(short, long)]
    scale: f64,
    #[structopt(short, long)]
    width: u32,
    #[structopt(short, long)]
    height: u32,
    #[structopt(short, long)]
    floor: f64,
    #[structopt(short, long)]
    ceiling: f64,
    #[structopt(short = "n", long)]
    slices: usize,
}

#[derive(Debug, EnumDisplay, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
enum FilterOpt {
    Billow(noise::Billow),
    Checkerboard(noise::Checkerboard),
    Fbm(noise::Fbm),
    OpenSimplex(noise::OpenSimplex),
    Perlin(noise::Perlin),
    SuperSimpex(noise::SuperSimplex),
    Value(noise::Value),
}

impl noise::NoiseFn<[f64; 3]> for Box<FilterOpt> {
    fn get(&self, point: [f64; 3]) -> f64 {
        match self.as_ref() {
            FilterOpt::Billow(f) => f.get(point),
            FilterOpt::Checkerboard(f) => f.get(point),
            FilterOpt::Fbm(f) => f.get(point),
            FilterOpt::OpenSimplex(f) => f.get(point),
            FilterOpt::Perlin(f) => f.get(point),
            FilterOpt::SuperSimpex(f) => f.get(point),
            FilterOpt::Value(f) => f.get(point),
        }
    }
}
fn main() -> std::result::Result<(), ()> {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    let function = Box::new(opt.function);
    let function = noise::ScaleBias::new(&function)
        .set_bias(opt.bias)
        .set_scale(opt.scale);
    noisimation::make_volume(
        &function,
        opt.width,
        opt.height,
        linspace(opt.floor, opt.ceiling, opt.slices),
    )
    .map(|i| image::DynamicImage::ImageLuma16(i))
    .print_images(&Default::default());
    Ok(())
}
