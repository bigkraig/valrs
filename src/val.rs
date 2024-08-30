use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct Config {
    pub(crate) include_mistakes: bool,
    pub(crate) include_values: bool,
    pub(crate) include_identification: bool,
    pub(crate) include_coding: bool,

    current_position: Vec<String>,
}

impl Config {
    pub fn new(include_coding: bool, include_mistakes: bool, include_identification: bool, include_values: bool) -> Config {
        Config {
            include_mistakes,
            include_values,
            include_identification,
            include_coding,
            current_position: vec![],
        }
    }

    pub fn print_pos(&self, s: &str) {
        println!("{} // {}", self.current_position.join(" // "), s);
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct VehicleAnalysisLog {
    #[serde(rename = "RESULTSHEADER")]
    header: ResultsHeader,
    #[serde(rename = "RESULT")]
    result: ValResult,
}

impl VehicleAnalysisLog {
    pub(crate) fn get_section(&self, section_name: &String) -> Option<&ECUSection> {
        for section in &self.result.sections {
            if section.title() == section_name {
                return Some(section);
            }
        }
        None
    }

    pub(crate) fn sections(&self) -> &Vec<ECUSection> {
        return &self.result.sections;
    }

    pub(crate) fn compare_sections(&self, cfg: &mut Config, p0: &VehicleAnalysisLog, skip: Option<Vec<&String>>) -> Vec<&String> {
        let mut compared_sections = vec![];
        for section in self.sections() {
            if let Some(s) = &skip {
                if s.contains(&section.title()) {
                    continue;
                }
            }

            if section.compare(cfg, &p0) {
                compared_sections.push(section.title());
            }
        }
        compared_sections
    }
}

#[derive(Deserialize, Serialize, Debug)]
// #[serde(deny_unknown_fields)]
struct ResultsHeader {
    #[serde(rename = "VEHICLE")]
    vehicle: Vehicle,
}

#[derive(Deserialize, Serialize, Debug)]
// #[serde(deny_unknown_fields)]
struct Vehicle {
    #[serde(rename = "IDENT")]
    identity: VehicleIdentity,
}

#[derive(Deserialize, Serialize, Debug)]
// #[serde(deny_unknown_fields)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct VehicleIdentity {
    vin: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
struct ValResult {
    #[serde(rename = "@OBJECT")]
    object: String,
    #[serde(rename = "@METHOD")]
    method: String,
    title: Title,
    header: Header,
    #[serde(rename = "SECTION")]
    sections: Vec<ECUSection>, // using the OBJECT flag twice blows shit up
}

#[derive(Deserialize, Serialize, Debug)]
// #[serde(deny_unknown_fields)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct Header {
    start_test: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ECUSection {
    #[serde(rename = "@OBJECT")]
    object: String,
    #[serde(rename = "TITLE")]
    _title: Title,
    #[serde(rename = "MEAS")]
    measurements: Vec<Measurement>, // same. cant use object specific measurement types as a list
}

fn find_measurement<'a>(measurements: &'a Vec<Measurement>, title: &String) -> Option<&'a Measurement> {
    for m in measurements {
        if m.title() == title {
            return Some(m);
        }
    }
    None
}

impl ECUSection {
    pub(crate) fn title(&self) -> &String {
        return &self._title.title;
    }

    pub(crate) fn compare(&self, cfg: &mut Config, other: &VehicleAnalysisLog) -> bool {
        if let Some(other_section) = other.get_section(&self.title()) {
            cfg.current_position = vec![self.title().clone()];
            for measurement in &self.measurements {
                if !measurement.should_compare(&cfg) {
                    continue;
                }
                measurement.compare(cfg, &other_section.measurements);
            }
            return true;
        } else {
            println!("Section {} not found in other result", self.title());
            return false;
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
struct Title {
    #[serde(rename = "$text")]
    title: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
struct Measurement {
    #[serde(rename = "@OBJECT")]
    object: String,
    #[serde(rename = "TITLE")]
    _title: Title,
    #[serde(rename = "VALUE")]
    values: Option<Vec<Value>>,
    #[serde(rename = "MEAS")]
    measurements: Option<Vec<Measurement>>,
}

impl Measurement {
    pub(crate) fn should_compare(&self, cfg: &Config) -> bool {
        match self.object.as_str() {
            "Identifikation" => cfg.include_identification,
            "Fehler" => cfg.include_mistakes,
            "Messwerte" => cfg.include_values,
            "Codierung" => cfg.include_coding,
            _ => true
        }
    }

    pub(crate) fn title(&self) -> &String {
        return &self._title.title;
    }

    pub(crate) fn compare(&self, cfg: &mut Config, other_measurements: &Vec<Measurement>) -> bool {
        let cur = &cfg.current_position;
        if let Some(other) = find_measurement(other_measurements, &self.title()) {
            cfg.current_position = cur.clone();
            cfg.current_position.push(self.title().clone());

            if let Some(values) = &self.values {
                for value in values {
                    if let Some(other_values) = &other.values {
                        for other_value in other_values {
                            if value.text == other_value.text {
                                if value.value != other_value.value {
                                    cfg.print_pos(format!("{} // {} -> {}", value.text.as_str(), value.value.as_deref().unwrap_or("N/A"), other_value.value.as_deref().unwrap_or("N/A")).as_str());
                                }
                            }
                        }
                    } else {
                        cfg.print_pos(format!("{} value not found in other measurement", value.text.as_str()).as_str());
                    }
                }
            }

            if let Some(measurements) = &self.measurements {
                if let Some(other_measurements) = &other.measurements {
                    for measurement in measurements {
                        measurement.compare(cfg, other_measurements);
                    }
                } else {
                    cfg.print_pos(format!("Nested {} measurements not found in other section", self.title()).as_str());
                }
            }

            return true;
        } else {
            cfg.print_pos(format!("Measurement {} not found in other section", self.title()).as_str());
            return false;
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
struct Value {
    #[serde(rename = "@TEXT")]
    text: String,
    #[serde(rename = "@UNIT")]
    unit: Option<String>,
    #[serde(rename = "@FORMAT")]
    format: String,
    #[serde(rename = "@LABEL")]
    label: String,
    #[serde(rename = "$text")]
    value: Option<String>,
}
