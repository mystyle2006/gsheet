use clap::{App, Arg, SubCommand};
use std::path::PathBuf;
use std::env;
mod google_auth;
mod google_sheet;


extern crate google_sheets4 as sheets4;

use crate::google_auth::get_auth;
use crate::google_sheet::GoogleSheet;
use std::fs;
use anyhow::{Context, Result};

const CONFIG_FILE: &str = "gsheet_config.json";
#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("gsheet")
        .version("1.0")
        .author("inno")
        .about("Google Sheets CLI")
        .subcommand(SubCommand::with_name("init")
            .about("Initialize the application")
            .arg(Arg::with_name("path")
                .short('p')
                .long("path")
                .value_name("FILE_PATH")
                .help("Sets the path to client_secret.json")
                .takes_value(true)
                .required(true)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        if let Some(path) = matches.value_of("path") {
            let client_secret_path = if PathBuf::from(path).is_absolute() {
                PathBuf::from(path)
            } else {
                env::current_dir()?.join(path)
            };
            println!("Initializing with client secret from: {:?}", client_secret_path);

            let config = serde_json::json!({
                "client_secret_path": client_secret_path.to_str().unwrap()
            });
            fs::write(CONFIG_FILE, serde_json::to_string_pretty(&config)?).context("Fail to write client secret")?;

            return Ok(());
        }
    }

    let config: serde_json::Value = serde_json::from_str(&fs::read_to_string(CONFIG_FILE)?).context("Fail to read client secret")?;
    let client_secret_path = PathBuf::from(config["client_secret_path"].as_str().unwrap());

    let auth = get_auth(&client_secret_path).await?;

    let spreadsheet_id = "16RvttLd2IbFPANGSXFQ34BsXqSu_Kev-OrQq47pG5ng"; // 실제 스프레드시트 ID로 교체
    let range = "A1:B10"; // 읽고 싶은 범위 (예: Sheet1의 A1:B10)

    let google_sheet = GoogleSheet::new(auth)?;

    // 스프레드시트 정보 검증하기
    let sheet_name = "Sheet1";
    let target_sheet_id: Option<i32> = google_sheet.get_sheet_id(spreadsheet_id, sheet_name).await?;
    if target_sheet_id.is_none() {
        eprintln!("Sheet '{}'을 찾을 수 없습니다.", sheet_name);
        return Ok(());
    }

    // 5. 데이터 가져오기
    let full_range = format!("{}!{}", sheet_name, range);
    let values = google_sheet.get_values(spreadsheet_id, &full_range).await?;

    // 6. 결과 출력
    if let Some(values) = values.1.values {
        for row in values {
            println!("{:?}", row);
        }
    } else {
        println!("데이터가 없습니다.");
    }

    Ok(())
}
