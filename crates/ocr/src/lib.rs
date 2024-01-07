use tracing::info;

pub struct Ocr {}

impl Ocr {
    pub async fn init() {
        println!("Ezocr init!");
        info!("Ezocr init!")
    }
}
