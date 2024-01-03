mod postgre_module;

#[tokio::main]
async fn main() {
    postgre_module::pg::migration_user().await.expect("Gagal Migrations");
}
