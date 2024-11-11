use std::fs::File;
use std::io::BufReader;
use anyhow::Context;
use chrono::{FixedOffset, NaiveDateTime};
use serde::{Deserialize, Deserializer, Serialize};
use serde_untagged::UntaggedEnumVisitor;

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct VehicleAnalysisLog {
    #[serde(rename = "RESULTSHEADER")]
    pub results_header: ResultsHeader,
    pub result: ValResult,
}

impl VehicleAnalysisLog {
    pub fn from_zip(filename: &str) -> anyhow::Result<VehicleAnalysisLog> {
        let file = File::open(&filename)?;
        let mut archive = zip::ZipArchive::new(file)?;
        for i in 0..archive.len() {
            let file = archive.by_index(i)?;
            if file.name().ends_with(".xml") {
                let reader = BufReader::new(file);
                let val = &mut quick_xml::de::Deserializer::from_reader(reader);
                let deserialized: VehicleAnalysisLog = serde_path_to_error::deserialize(val).context("Failed deserializing")?;
                return Ok(deserialized);
            }
        }
        Err(anyhow::anyhow!("Could not find FAP xml in zip."))
    }

    pub fn get_section_by_title(&self, title: &str) -> Option<&Section> {
        self.result.sections.iter().find(|s| s.get_title() == title)
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ResultsHeader {
    pub country: Country,
    #[serde(rename = "CARDEALER")]
    pub car_dealer: CarDealer,
    pub vehicle: Vehicle,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Country {
    pub regulation: String,
    pub language: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CarDealer {
    pub name: String,
    pub company: String,
    pub address: String,
    pub zip: String,
    pub city: String,
    pub tel: String,
    #[serde(rename = "DEALERNO")]
    pub dealer_no: String,
    pub order: String,
    #[serde(rename = "WARRANTYNO")]
    pub warranty_no: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Vehicle {
    pub ident: VehicleIdentity,
    pub data: VehicleData,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct VehicleIdentity {
    pub vin: String,
    pub registration: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct VehicleData {
    pub odometer: UnitString,
    #[serde(rename = "OPERATINGTIME")]
    pub operating_time: UnitString,
    #[serde(rename = "ORDERTYPE")]
    pub order_type: String,
    pub model: Option<String>,
    #[serde(rename = "MODELTYPE")]
    pub model_type: String,
    #[serde(rename = "ENGINETYPE")]
    pub engine_type: String,
    #[serde(rename = "COUNTRYCODE")]
    pub country_code: String,
    #[serde(rename = "GEARBOXTYPE")]
    pub gearbox_type: String,
    #[serde(rename = "ONBOARDVOLTAGE")]
    pub onboard_voltage: UnitString,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct UnitString {
    #[serde(rename = "@UNIT")]
    pub unit: String,
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ValResult {
    #[serde(rename = "@OBJECT")]
    pub object: String,
    #[serde(rename = "@METHOD")]
    pub method: String,
    pub title: String,
    pub header: Header,
    #[serde(rename = "SECTION")]
    pub sections: Vec<Section>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Header {
    #[serde(with = "time_format")]
    pub start_test: NaiveDateTime,
    #[serde(with = "time_format")]
    pub end_test: NaiveDateTime,
    #[serde(with = "timezone_format")]
    pub timezone: FixedOffset,
    #[serde(rename = "PROTOKOLLTYPE")]
    pub protocol_type: String,
    pub equipment: Equipment,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Equipment {
    #[serde(rename = "@TYPE")]
    pub _type: String,
    pub title: String,
    pub manufacturer: String,
    pub model: String,
    pub serial_no: String,
    pub firmware: String,
    pub version: String,
    #[serde(rename = "PT2GVERSION")]
    pub pt2g_version: String,
    pub br_pdx: String,
    pub pdu_api: String,
    pub samdiax_version: String,
    pub system: String,
    pub java: String,
    pub mode: String,
}

mod time_format {
    use super::*;
    use serde::de::{self, Visitor};
    use std::fmt;
    use serde::{Deserializer, Serializer};

    struct TimeVisitor;

    impl<'de> Visitor<'de> for TimeVisitor {
        type Value = NaiveDateTime;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a time string in the format DD.MM.YYYY HH:MM:SS")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            NaiveDateTime::parse_from_str(value, "%d.%m.%Y %H:%M:%S").map_err(de::Error::custom)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TimeVisitor)
    }

    pub fn serialize<S>(time: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = time.format("%d.%m.%Y %H:%M:%S").to_string();
        serializer.serialize_str(&s)
    }
}

mod timezone_format {
    use super::*;
    use serde::de::{self, Visitor};
    use std::fmt;
    use std::str::FromStr;
    use serde::{Deserializer, Serializer};

    struct TimezoneVisitor;

    impl<'de> Visitor<'de> for TimezoneVisitor {
        type Value = FixedOffset;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a timezone string in the format GMT±HH:MM")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if let Some(offset) = value.strip_prefix("GMT") {
                FixedOffset::from_str(offset).map_err(de::Error::custom)
            } else {
                Err(de::Error::custom("invalid timezone format"))
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<FixedOffset, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TimezoneVisitor)
    }

    pub fn serialize<S>(offset: &FixedOffset, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("GMT{}", offset);
        serializer.serialize_str(&s)
    }
}

#[derive(Serialize, Debug)]
#[serde(deny_unknown_fields, tag = "@OBJECT")]
pub enum Section {
    ECU(ECUSection)
}

impl Section {
    pub fn get_title(&self) -> &String {
        match self {
            Section::ECU(section) => &section.title,
        }
    }
    pub fn get_measurements(&self) -> &Vec<Measurement> {
        match self {
            Section::ECU(section) => &section.measurements,
        }
    }
    #[allow(dead_code)]
    fn get_measurement_by_title(&self, title: &String) -> Option<Measurement> {
        match self {
            Section::ECU(section) => get_measurement_by_title(&section.measurements, title),
        }
    }
}

impl<'de> Deserialize<'de> for Section {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        UntaggedEnumVisitor::new()
            .map(|map| {
                let value: CommonSection = map.deserialize()?;
                match value.object.as_str() {
                    "ECU" => Ok(Section::ECU(ECUSection::from(value))),
                    _ => {
                        Err(serde::de::Error::custom(format!("'{}' not implemented", value.object.as_str())))
                    }
                }
            })
            .deserialize(deserializer)
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CommonSection {
    #[serde(rename = "@OBJECT")]
    pub object: String,
    #[serde(rename = "TITLE")]
    pub title: String,
    #[serde(rename = "MEAS")]
    pub measurements: Vec<Measurement>,
}

#[derive(Serialize, Debug)]
pub struct ECUSection {
    #[serde(rename = "TITLE")]
    pub title: String,
    #[serde(rename = "MEAS")]
    pub measurements: Vec<Measurement>,
}

impl From<CommonSection> for ECUSection {
    fn from(m: CommonSection) -> Self {
        ECUSection {
            title: m.title,
            measurements: m.measurements,
        }
    }
}

fn get_measurement_by_title(measurements: &Vec<Measurement>, title: &String) -> Option<Measurement> {
    for m in measurements {
        if m.get_title() == title {
            return Some(m.clone());
        }
    }
    None
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Measurement {
    Codierung(MeasurementCoding),
    Identifikation(MeasurementIdentification),
    Fehler(MeasurementMistake),
    Messwerte(MeasurementMeasuredValues),
    ErweiterterFehlerspeicher(MeasurementExtendedErrorMemory),
}

impl Measurement {
    pub fn get_title(&self) -> &String {
        match self {
            Measurement::Codierung(m) => &m.title,
            Measurement::Identifikation(m) => &m.title,
            Measurement::Fehler(m) => &m.title,
            Measurement::Messwerte(m) => &m.title,
            Measurement::ErweiterterFehlerspeicher(m) => &m.title,
        }
    }

    pub fn get_values(&self) -> Option<&Vec<ValueEnum>> {
        match self {
            Measurement::Codierung(m) => m.values.as_ref(),
            Measurement::Identifikation(m) => m.values.as_ref(),
            Measurement::Fehler(m) => m.values.as_ref(),
            Measurement::Messwerte(m) => m.values.as_ref(),
            Measurement::ErweiterterFehlerspeicher(m) => m.values.as_ref(),
        }
    }

    #[allow(dead_code)]
    fn get_value_by_label(&self, label: &String) -> Option<&ValueEnum> {
        match self.get_values() {
            Some(values) => values.iter().find(|v| match v {
                ValueEnum::Num(n) => &n.label == label,
                ValueEnum::Alpha(a) => &a.label == label,
            }),
            _ => None,
        }
    }

    pub fn get_submeasurements(&self) -> Option<&Vec<Measurement>> {
        match self {
            Measurement::Fehler(m) => m.measurements.as_ref(),
            _ => None,
        }
    }

    #[allow(dead_code)]
    fn get_submeasurement_by_title(&self, title: &String) -> Option<Measurement> {
        match self {
            Measurement::Fehler(m) => match &m.measurements {
                Some(measurements) => get_measurement_by_title(&measurements, title),
                _ => None,
            }
            _ => None
        }
    }
}

impl<'de> Deserialize<'de> for Measurement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        UntaggedEnumVisitor::new()
            .map(|map| {
                let value: CommonMeasurement = map.deserialize()?;
                match value.object.as_str() {
                    "Codierung" => Ok(Measurement::Codierung(MeasurementCoding::from(value))),
                    "Identifikation" => Ok(Measurement::Identifikation(MeasurementIdentification::from(value))),
                    "Fehler" => Ok(Measurement::Fehler(MeasurementMistake::from(value))),
                    "Messwerte" => Ok(Measurement::Messwerte(MeasurementMeasuredValues::from(value))),
                    "Erweiterter Fehlerspeicher" => Ok(Measurement::ErweiterterFehlerspeicher(MeasurementExtendedErrorMemory::from(value))),
                    _ => {
                        Err(serde::de::Error::custom(format!("'{}' not implemented", value.object.as_str())))
                    }
                }
            })
            .deserialize(deserializer)
    }
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
struct CommonMeasurement {
    #[serde(rename = "@OBJECT")]
    object: String,
    #[serde(rename = "TITLE")]
    title: String,
    #[serde(rename = "VALUE")]
    values: Option<Vec<ValueEnum>>,
    #[serde(rename = "MEAS")]
    measurements: Option<Vec<Measurement>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct MeasurementCoding {
    pub title: String,
    pub values: Option<Vec<ValueEnum>>,
}

impl From<CommonMeasurement> for MeasurementCoding {
    fn from(m: CommonMeasurement) -> Self {
        if m.measurements != None {
            panic!("unexpected measurements for MeasurementCoding");
        }
        MeasurementCoding {
            title: m.title,
            values: m.values,
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct MeasurementIdentification {
    pub title: String,
    pub values: Option<Vec<ValueEnum>>,
}

impl From<CommonMeasurement> for MeasurementIdentification {
    fn from(m: CommonMeasurement) -> Self {
        if m.measurements != None {
            panic!("unexpected measurements for MeasurementIdentification");
        }
        MeasurementIdentification {
            title: m.title,
            values: m.values,
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct MeasurementMeasuredValues {
    pub title: String,
    pub values: Option<Vec<ValueEnum>>,
}

impl From<CommonMeasurement> for MeasurementMeasuredValues {
    fn from(m: CommonMeasurement) -> Self {
        if m.measurements != None {
            panic!("unexpected measurements for MeasurementMeasuredValues");
        }
        MeasurementMeasuredValues {
            title: m.title,
            values: m.values,
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct MeasurementMistake {
    pub title: String,
    pub values: Option<Vec<ValueEnum>>,
    pub measurements: Option<Vec<Measurement>>,
}

impl From<CommonMeasurement> for MeasurementMistake {
    fn from(m: CommonMeasurement) -> Self {
        MeasurementMistake {
            title: m.title,
            values: m.values,
            measurements: m.measurements,
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct MeasurementExtendedErrorMemory {
    pub title: String,
    pub values: Option<Vec<ValueEnum>>,
}

impl From<CommonMeasurement> for MeasurementExtendedErrorMemory {
    fn from(m: CommonMeasurement) -> Self {
        MeasurementExtendedErrorMemory {
            title: m.title,
            values: m.values,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE", tag = "@FORMAT")]
pub enum ValueEnum {
    Num(NumberValue),
    Alpha(AlphaValue),
}

impl ValueEnum {
    pub fn get_label(&self) -> &String {
        match self {
            ValueEnum::Num(n) => &n.label,
            ValueEnum::Alpha(a) => &a.label,
        }
    }

    pub fn get_text(&self) -> &String {
        match self {
            ValueEnum::Num(n) => &n.text,
            ValueEnum::Alpha(a) => &a.text,
        }
    }

    pub fn get_unit(&self) -> Option<&String> {
        match self {
            ValueEnum::Num(n) => n.unit.as_ref(),
            _ => None,
        }
    }

    pub fn get_value(&self) -> Option<&String> {
        match self {
            ValueEnum::Num(n) => Some(&n.value),
            ValueEnum::Alpha(a) => a.value.as_ref(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct NumberValue {
    #[serde(rename = "@TEXT")]
    pub text: String,
    #[serde(rename = "@UNIT")]
    pub unit: Option<String>,
    #[serde(rename = "@LABEL")]
    pub label: String,
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(deny_unknown_fields, rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AlphaValue {
    #[serde(rename = "@TEXT")]
    pub text: String,
    #[serde(rename = "@LABEL")]
    pub label: String,
    #[serde(rename = "$text")]
    pub value: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let val = VehicleAnalysisLog::from_zip("tests/data/FAP_XXXXXXXXXXXXXXXXX_20240804_132559_23.0.1.zip").unwrap();
        assert_eq!(val.results_header.vehicle.ident.vin, "XXXXXXXXXXXXXXXXX");
        assert_eq!(val.result.header.equipment.pt2g_version, "42.200.010");
        assert_eq!(val.result.header.timezone, FixedOffset::west_opt(7 * 3600).unwrap());

        let section = &val.get_section_by_title("Gateway (A7.1)").unwrap();
        let m = &section.get_measurement_by_title(&"Control unit, coding".to_string()).unwrap();
        let value = m.get_value_by_label(&"Batteriewechsel_Technologie_zwei.Scannercode".to_string()).unwrap();
        assert_eq!(m.get_title(), "Control unit, coding");
        assert_eq!(value, &ValueEnum::Alpha(AlphaValue {
            text: "Battery change: Scanner code".to_string(),
            label: "Batteriewechsel_Technologie_zwei.Scannercode".to_string(),
            value: Some("205 BA24H9F0EGE".to_string()),
        }));


        let section = &val.get_section_by_title("Airbag (variant: A2.8)").unwrap();
        let m = &section.get_measurement_by_title(&"Fault".to_string()).unwrap();
        let submeasurement = m.get_submeasurement_by_title(&"erweiterter Fehlerspeicher".to_string()).unwrap();
        let value = submeasurement.get_value_by_label(&"Priority".to_string()).unwrap();
        assert_eq!(m.get_title(), "Fault");
        assert_eq!(value, &ValueEnum::Alpha(AlphaValue {
            text: "Hinweis_Prio".to_string(),
            label: "Priority".to_string(),
            value: Some("2".to_string()),
        }));
    }
}
