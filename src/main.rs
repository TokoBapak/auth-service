mod postgre_module;

fn main() {
    postgre_module::pg::migration_user().await.expect("Gagal Migrations");
}
