use clap::{App, Arg, SubCommand};
use std::path::PathBuf;
use std::env;
mod google_auth;
mod google_sheet;
mod google_drive;
mod util;

extern crate google_sheets4 as sheets4;

use crate::google_auth::get_auth;
use crate::google_sheet::GoogleSheet;
use std::fs;
use anyhow::{Context, Result};
use prettytable::{format, row, Cell, Row, Table};
use crate::google_drive::GoogleDrive;
use crate::util::get_client_secret_path;

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
        .subcommand(SubCommand::with_name("list")
            .about("List contents of a Google Sheet")
            .arg(Arg::with_name("sheet_name")
                .short('n')
                .long("name")
                .value_name("GOOGLE_SHEET_NAME")
                .help("Sets google sheet name for filter")
                .takes_value(true)
                .required(true)))
        .subcommand(SubCommand::with_name("get")
            .about("Get data from a Google Sheet")
            .arg(Arg::with_name("sheet-id")
                .short('s')
                .long("sheet-id")
                .value_name("SHEET_ID")
                .help("Sets the Google Sheet ID")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("range")
                .short('r')
                .long("range")
                .value_name("RANGE")
                .help("Sets the range to fetch (default: A1:R100)")
                .takes_value(true)))
        .get_matches();

    /* init 명령어 핸들러 */
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

            let config: serde_json::Value = serde_json::from_str(&fs::read_to_string(CONFIG_FILE)?).context("Fail to read client secret")?;
            let client_secret_path = PathBuf::from(config["client_secret_path"].as_str().unwrap());

            /* init 시 구글 인증까지 완료 */
            get_auth(&client_secret_path).await?;
            println!("Authenticated with client secret successfully");
            return Ok(());
        }
    }

    /* list 명령어 핸들러 */
    if let Some(matches) = matches.subcommand_matches("list") {
        if let Some(name) = matches.value_of("sheet_name") {
            let client_secret_path = get_client_secret_path()?;

            let auth = get_auth(&client_secret_path).await?;
            let google_drive = GoogleDrive::new(auth)?;

            let list = google_drive.list_spreadsheets(name).await?;

            // 테이블 생성
            let mut table = Table::new();
            table.add_row(row!["No.", "Title", "Spreadsheet ID"]);

            for (index, file) in list.iter().enumerate() {
                if let (Some(name), Some(id)) = (&file.name, &file.id) {
                    table.add_row(row![index + 1, name, id]);
                }
            }

            // 테이블 출력
            table.printstd();
            return Ok(());
        }
    }

    if let Some(matches) = matches.subcommand_matches("get") {
        let spreadsheet_id = matches.value_of("sheet-id").unwrap();
        let range = matches.value_of("range").unwrap_or("A1:R100");

        let client_secret_path = get_client_secret_path()?;

        let auth = get_auth(&client_secret_path).await?;

        let google_sheet = GoogleSheet::new(auth)?;

        // 스프레드시트 정보 검증하기
        let sheet_name = "Sheet1";
        let target_sheet_id: Option<i32> = google_sheet.get_sheet_id(spreadsheet_id, sheet_name).await?;
        if target_sheet_id.is_none() {
            debug_println!("Sheet '{}'을 찾을 수 없습니다.", sheet_name);
            return Ok(());
        }

        // 5. 데이터 가져오기
        let full_range = format!("{}!{}", sheet_name, range);
        let values = google_sheet.get_values(spreadsheet_id, &full_range).await?;

        // 6. 결과 출력
        if let Some(data) = values.1.values {
            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_BOX_CHARS);

            // 열 헤더 추가 (A, B, C, ...)
            let mut header_row = Row::new(vec![Cell::new("")]);  // 왼쪽 상단 빈 셀
            for i in 0..data[0].len() {
                header_row.add_cell(Cell::new(&format!("{}", (b'A' + i as u8) as char)));
            }
            table.add_row(header_row);

            // 데이터 행 추가
            for (row_index, row) in data.iter().enumerate() {
                let mut table_row = Row::new(vec![Cell::new(&format!("{}", row_index + 1))]);  // 행 번호
                for cell in row {
                    table_row.add_cell(Cell::new(&cell.as_str().unwrap_or("").to_string()));
                }
                table.add_row(table_row);
            }

            // 테이블 출력
            table.printstd();
            return Ok(());
        }
    }

    println!("Error: Invalid command");
    println!("Run 'gsheet --help' for usage information");
    std::process::exit(1);
}
