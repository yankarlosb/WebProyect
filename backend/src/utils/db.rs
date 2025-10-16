use sea_orm::{DatabaseConnection, Database};

pub async fn establish_connection() -> DatabaseConnection {
    dotenvy::dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL no encontrada en el archivo .env");
    println!("🔗 Conectando a la base de datos...");
    let db = Database::connect(&database_url)
        .await
        .expect("Error al conectar a la base de datos");

    println!("✅ Conectado a la base de datos exitosamente");
    db
}