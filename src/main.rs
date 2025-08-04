mod entity;
mod loader;
use std::time::Instant;

use crate::loader::BaseLoader;

fn main() {
    let s = Instant::now();
    let filename = "./pbf/kanto-latest.osm.pbf";
    let path = std::path::Path::new(filename);

    let mut loader = loader::LoaderForVehicle::new();
    loader.load(path);
    println!("{:?}", s.elapsed());
}
