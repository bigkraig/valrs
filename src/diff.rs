use anyhow::{Context, Result};
use std::fs::File;
use std::io::BufReader;
use crate::val::{Config, VehicleAnalysisLog};

#[derive(clap::Args, Debug)]
pub struct DiffArgs {
    zip1: String,
    zip2: String,

    #[clap(long)]
    include_values: bool,
    #[clap(long)]
    include_identification: bool,
    #[clap(long)]
    include_mistakes: bool,
    #[clap(long)]
    #[arg(default_value_t = true)]
    include_coding: bool,
}

fn load(filename: &str) -> Result<VehicleAnalysisLog> {
    let file = File::open(filename)?;
    let mut archive = zip::ZipArchive::new(file)?;
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        if file.name().starts_with("FAP_") && file.name().ends_with(".xml") {
            let reader = BufReader::new(file);
            let val = &mut quick_xml::de::Deserializer::from_reader(reader);
            let deserialized: VehicleAnalysisLog = serde_path_to_error::deserialize(val).context("Failed deserializing")?;
            return Ok(deserialized);
        }
    }
    Err(anyhow::anyhow!("Could not find FAP xml in zip."))
}

pub fn diff(args: &DiffArgs) -> Result<()> {
    let val1 = &load(&args.zip1)?;
    let val2 = &load(&args.zip2)?;

    let diff_config = &mut Config::new(args.include_coding, args.include_mistakes, args.include_identification, args.include_values);

    let compared_sections = val1.compare_sections(diff_config, val2, None);
    val2.compare_sections(diff_config, val1, Some(compared_sections));
    Ok(())
}