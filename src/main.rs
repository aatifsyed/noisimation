use noise;
use noisimation::PrintImage;
use structopt::StructOpt;
use strum::VariantNames;
use strum_macros::{Display as EnumDisplay, EnumString, EnumVariantNames};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(possible_values = FilterOpt::VARIANTS)]
    function: FilterOpt,
}

#[derive(Debug, EnumDisplay, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
enum FilterOpt {
    Checkerboard(noise::Checkerboard),
    Value(noise::Value),
}

impl noise::NoiseFn<[f64; 3]> for FilterOpt {
    fn get(&self, point: [f64; 3]) -> f64 {
        match self {
            FilterOpt::Checkerboard(f) => f.get(point),
            FilterOpt::Value(f) => f.get(point),
        }
    }
}
fn main() -> std::result::Result<(), ()> {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    let filter = &opt.function as &dyn noise::NoiseFn<[f64; 3]>;
    Ok(())
}
