mod google_auth;
mod google_sheet;

extern crate google_sheets4 as sheets4;

use sheets4::{Result};
use crate::google_auth::get_auth;
use crate::google_sheet::GoogleSheet;

#[tokio::main]
async fn main() -> Result<()> {
    let auth = get_auth().await?;

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
