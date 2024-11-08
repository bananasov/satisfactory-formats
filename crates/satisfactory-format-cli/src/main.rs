use std::io::Read;

use satisfactory_blueprints::{blueprint::BlueprintFileHeader, config::BlueprintConfigFile};
use scroll::Pread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config_file = std::fs::File::open("data/blueprints/2x foundry.sbpcfg")?;
    let mut config_buffer = Vec::new();
    config_file.read_to_end(&mut config_buffer)?;

    let mut blueprint_file = std::fs::File::open("data/blueprints/2x foundry.sbp")?;
    let mut blueprint_buffer = Vec::new();
    blueprint_file.read_to_end(&mut blueprint_buffer)?;

    let config: BlueprintConfigFile = config_buffer.gread_with(&mut 0, scroll::LE)?;
    println!("{:#?}", config);

    let blueprint: BlueprintFileHeader = blueprint_buffer.gread_with(&mut 0, scroll::LE)?;
    println!("{:#?}", blueprint);

    Ok(())
}
