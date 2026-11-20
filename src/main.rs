use spreadsheet_to_json::*;
use spreadsheet_to_json::error::GenericError;
use spreadsheet_to_json::serde_json::json;

fn main() -> Result<(), GenericError> {
  println!("oke");
  let opts = OptionSet::new("../assets/excel/file_table.xlsx").sheet_index(1);
  let result = process_spreadsheet_direct(&opts);
  let json_value = match result {
    Err(msg_code) => json!({ "error": true, "key": msg_code.to_string() }),
    Ok(data_set) => data_set.to_json() // full result set
  };
  println!("{}", json_value);
  Ok(())
}
