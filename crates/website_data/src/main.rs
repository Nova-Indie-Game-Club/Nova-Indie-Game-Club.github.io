use website_data::*;

#[tokio::main]
async fn main() {
    run_notion_data_collection().await.unwrap();
}
