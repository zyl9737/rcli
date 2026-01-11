use crate::opts::OutputFormat;
use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();

        ret.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(output, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_csv_json() {
        let result = process_csv(
            "assets/juventus.csv",
            "output_test.json".to_string(),
            OutputFormat::Json,
        );
        assert!(result.is_ok());
        std::fs::remove_file("output_test.json").ok();
    }
}
