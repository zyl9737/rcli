use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: u8,
}
pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Player = result?;
        ret.push(record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_csv() {
        let result = process_csv("assets/juventus.csv", "output_test.json");
        assert!(result.is_ok());

        // 清理测试文件
        let _ = std::fs::remove_file("output_test.json");
    }
}
