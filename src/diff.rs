use anyhow::Result;
use piwis_val::{Measurement, ValueEnum, VehicleAnalysisLog};

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
    include_extended_errors: bool,
    #[clap(long)]
    #[arg(default_value_t = true)]
    include_coding: bool,
}

#[derive(Debug, Default)]
pub struct DiffConfig {
    pub(crate) include_mistakes: bool,
    pub(crate) include_values: bool,
    pub(crate) include_identification: bool,
    pub(crate) include_coding: bool,
    pub(crate) include_extended_errors: bool,
}

impl DiffConfig {
    pub fn new(include_coding: bool, include_mistakes: bool, include_identification: bool, include_values: bool, include_extended_errors: bool) -> DiffConfig {
        DiffConfig {
            include_mistakes,
            include_values,
            include_identification,
            include_coding,
            include_extended_errors,
        }
    }
}



macro_rules! printp0 {
    ($vec:expr, $($arg:tt)*) => {{
        let joined = $vec.join(" // ");
        println!("{} {}", joined, format!($($arg)*));
    }};
}

fn select_measurement(measurements: &Vec<Measurement>, title: &String) -> Option<Measurement> {
    for m in measurements {
        if m.get_title().as_str() == title.as_str() {
            return Some(m.clone());
        }
    }
    None
}

fn select_value(other_values: &Vec<ValueEnum>, label: &String) -> Option<ValueEnum> {
    if let Some(other_value) = other_values.iter().find(|v| &v.get_label().as_str() == &label.as_str()) {
        return Some(other_value.clone());
    }
    None
}

fn should_compare(m: &Measurement, cfg: &DiffConfig) -> bool {
    match m {
        Measurement::Codierung(_) => cfg.include_coding,
        Measurement::Identifikation(_) => cfg.include_identification,
        Measurement::Fehler(_) => cfg.include_mistakes,
        Measurement::Messwerte(_) => cfg.include_values,
        Measurement::ErweiterterFehlerspeicher(_) => cfg.include_extended_errors,
    }
}

fn print_measurements_diff(p0: &mut Vec<String>, measurements: &Vec<Measurement>, other_measurements: &Vec<Measurement>, diff_config: &DiffConfig) {
    for measurement in measurements {
        if !should_compare(&measurement, diff_config) {
            continue;
        }
        p0.push(measurement.get_title().clone());
        let Some(other_measurement) = select_measurement(&other_measurements, &measurement.get_title()) else {
            printp0!(p0, ":: measurement was not found in second VAL");
            continue;
        };

        match (&measurement.get_submeasurements(), &other_measurement.get_submeasurements()) {
            (Some(nested_measurements), Some(other_nested_measurements)) =>
                print_measurements_diff(p0, nested_measurements, other_nested_measurements, diff_config),
            (Some(_), None) => printp0!(p0, ":: sub-measurements were not found in second VAL"),
            _ => (),
        }

        print_values_diff(p0, &measurement.get_values(), &other_measurement.get_values());
        p0.pop();
    }
}

fn print_values_diff(p0: &mut Vec<String>, values: &Option<&Vec<ValueEnum>>, other_values: &Option<&Vec<ValueEnum>>) {
    if !values.is_none() && other_values.is_none() {
        printp0!(p0, ":: values were not found in second VAL");
        return;
    }
    match (&values, &other_values) {
         (Some(values), Some(other_values)) => {
            for value in *values {
                p0.push(value.get_text().clone());
                let Some(other_value) = select_value(other_values, &value.get_label()) else {
                    printp0!(p0, ":: value was not found in second VAL");
                    p0.pop();
                    continue;
                };
                if value.get_value() != other_value.get_value() {
                    printp0!(p0, ":: '{}' -> '{}'",
                    value.get_value().unwrap_or(&"<undefined>".to_string()),
                    other_value.get_value().unwrap_or(&"<undefined>".to_string()));
                }
                p0.pop();
            }
        }
        (Some(_), None) => {
            printp0!(p0, ":: values were not found in second VAL");
            return;
        }
        (None, Some(_)) => {
            printp0!(p0, ":: values were not found in first VAL");
            return;
        }
        _ => (),
    }
}

pub fn diff(args: &DiffArgs) -> Result<()> {
    let val1 = &VehicleAnalysisLog::from_zip(&args.zip1)?;
    let val2 = &VehicleAnalysisLog::from_zip(&args.zip2)?;

    let diff_config = &mut DiffConfig::new(args.include_coding,
                                           args.include_mistakes,
                                           args.include_identification,
                                           args.include_values,
                                           args.include_extended_errors);
    let mut missing_sections2 = vec![];

    let mut p0 = vec![];
    for section in val1.result.sections.iter() {
        p0.push(section.get_title().clone());
        let Some(other_section) = val2.get_section_by_title(&section.get_title()) else {
            missing_sections2.push(section.get_title().clone());
            p0.pop();
            continue;
        };
        print_measurements_diff(&mut p0, &section.get_measurements(), &other_section.get_measurements(), diff_config);
        p0.pop();
    }

    if missing_sections2.len() > 0 {
        println!("Missing section(s) in second VAL: {}", missing_sections2.join(","));
    }

    Ok(())
}

