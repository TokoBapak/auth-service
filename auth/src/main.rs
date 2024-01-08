use database::pg;
#[tokio::main]
async fn main() {
    pg::migration_user().await.expect("Gagal Migrations");
}
