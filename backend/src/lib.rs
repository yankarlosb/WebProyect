// Re-exportar las bibliotecas principales;
use rocket::Build;
pub use rocket;
pub use rocket::routes;
pub use rocket::catchers;

pub use rocket::fs::{FileServer, NamedFile};
pub use rocket::http::Status;
pub use rocket::response::Redirect;
pub use rocket::serde::json::Json;
use rocket::Rocket;
pub use rocket::State;
pub use rocket::form::FromForm;
pub use sea_orm::{Database, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait};
pub use bcrypt;
pub use dotenvy;
pub use serde;

// Exportar los módulos de entidades
pub mod database;
pub use database::prelude::*;
pub mod utils;
pub mod routes;

// Re-exportar los módulos específicos de entidades para facilitar el acceso
pub use database::{asignaturas, usuarios};

// Importar las rutas para usar en el macro routes!
use routes::login::{
    login_get,
    login_form,
    login_json,
    balance_page,
    logout,
    unauthorized
};

pub struct AppState {
    pub db: DatabaseConnection,
}

pub async fn run() -> Rocket<Build> {
    let db = utils::db::establish_connection().await;
    rocket::build()
        .manage(AppState { db })
        .mount("/", routes![
            // Rutas públicas
            login_get,
            login_form,
            logout,
            balance_page,
            login_json
        ])
        .register("/", catchers![unauthorized])
        .mount("/frontend", FileServer::from("../frontend"))
}