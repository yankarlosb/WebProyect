// ============================================================================
// EJEMPLOS DE USO DE JWT EN RUST
// ============================================================================

// Este archivo contiene ejemplos de cómo extender el sistema JWT
// No es necesario compilarlo, es solo para referencia

use crate::utils::jwt::{Claims, create_jwt, decode_jwt, AuthenticatedUser, AdminUser};
use rocket::{State, http::Status};
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};

// ============================================================================
// EJEMPLO 1: Crear rutas con diferentes niveles de acceso
// ============================================================================

// Ruta pública - cualquiera puede acceder
#[get("/api/public")]
pub fn public_route() -> Json<&'static str> {
    Json("Esta ruta es pública")
}

// Ruta protegida - solo usuarios autenticados
#[get("/api/profile")]
pub fn user_profile(user: AuthenticatedUser) -> Json<UserProfile> {
    Json(UserProfile {
        name: user.0.name.clone(),
        email: user.0.email.clone(),
        role: if user.0.is_admin { "admin" } else { "user" }.to_string(),
    })
}

// Ruta de administrador - solo admins
#[get("/api/users/list")]
pub fn list_users(admin: AdminUser) -> Json<&'static str> {
    Json("Lista de usuarios - solo admin")
}

#[derive(Serialize)]
struct UserProfile {
    name: String,
    email: String,
    role: String,
}

// ============================================================================
// EJEMPLO 2: Crear tokens con expiración personalizada
// ============================================================================

// Token de corta duración (15 minutos) para operaciones sensibles
pub fn create_short_lived_token(user_id: i32, email: String, name: String, is_admin: bool) -> String {
    let claims = Claims::with_expiration(
        user_id,
        email,
        name,
        is_admin,
        900 // 15 minutos
    );
    create_jwt(&claims).unwrap()
}

// Token de larga duración (7 días) para "recordarme"
pub fn create_long_lived_token(user_id: i32, email: String, name: String, is_admin: bool) -> String {
    let claims = Claims::with_expiration(
        user_id,
        email,
        name,
        is_admin,
        604800 // 7 días
    );
    create_jwt(&claims).unwrap()
}

// ============================================================================
// EJEMPLO 3: Validar token manualmente en una función
// ============================================================================

pub fn validate_token_manually(token: &str) -> Result<Claims, &'static str> {
    match decode_jwt(token) {
        Ok(claims) => Ok(claims),
        Err(_) => Err("Token inválido o expirado"),
    }
}

// ============================================================================
// EJEMPLO 4: Ruta POST protegida (crear recurso)
// ============================================================================

#[derive(Deserialize)]
pub struct CreatePost {
    title: String,
    content: String,
}

#[derive(Serialize)]
pub struct PostResponse {
    success: bool,
    message: String,
    post_id: Option<i32>,
}

#[post("/api/posts", format = "json", data = "<post_data>")]
pub fn create_post(
    user: AuthenticatedUser,
    post_data: Json<CreatePost>,
) -> Json<PostResponse> {
    // Solo usuarios autenticados pueden crear posts
    // Aquí user.0 contiene los claims del usuario
    
    Json(PostResponse {
        success: true,
        message: format!("Post creado por {}", user.0.name),
        post_id: Some(123),
    })
}

// ============================================================================
// EJEMPLO 5: Ruta PUT protegida (actualizar recurso)
// ============================================================================

#[derive(Deserialize)]
pub struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
}

#[put("/api/users/<user_id>", format = "json", data = "<update_data>")]
pub fn update_user(
    admin: AdminUser, // Solo admins pueden actualizar usuarios
    user_id: i32,
    update_data: Json<UpdateUser>,
) -> Result<Json<&'static str>, Status> {
    // Verificar que es admin
    if !admin.0.is_admin {
        return Err(Status::Forbidden);
    }
    
    // Actualizar usuario...
    Ok(Json("Usuario actualizado"))
}

// ============================================================================
// EJEMPLO 6: Ruta DELETE protegida (eliminar recurso)
// ============================================================================

#[delete("/api/users/<user_id>")]
pub fn delete_user(
    admin: AdminUser, // Solo admins pueden eliminar usuarios
    user_id: i32,
) -> Result<Json<&'static str>, Status> {
    if !admin.0.is_admin {
        return Err(Status::Forbidden);
    }
    
    // No puede eliminarse a sí mismo
    if admin.0.sub == user_id.to_string() {
        return Err(Status::BadRequest);
    }
    
    // Eliminar usuario...
    Ok(Json("Usuario eliminado"))
}

// ============================================================================
// EJEMPLO 7: Opcional - Usar el guardián como Option para rutas flexibles
// ============================================================================

#[get("/api/content")]
pub fn flexible_content(user: Option<AuthenticatedUser>) -> Json<ContentResponse> {
    match user {
        Some(auth_user) => {
            // Usuario autenticado - contenido personalizado
            Json(ContentResponse {
                message: format!("Bienvenido, {}", auth_user.0.name),
                content: "Contenido personalizado".to_string(),
                is_authenticated: true,
            })
        }
        None => {
            // Usuario no autenticado - contenido público
            Json(ContentResponse {
                message: "Bienvenido, invitado".to_string(),
                content: "Contenido público".to_string(),
                is_authenticated: false,
            })
        }
    }
}

#[derive(Serialize)]
pub struct ContentResponse {
    message: String,
    content: String,
    is_authenticated: bool,
}

// ============================================================================
// EJEMPLO 8: Verificar permisos adicionales en la lógica de negocio
// ============================================================================

#[get("/api/posts/<post_id>")]
pub fn get_post(
    user: AuthenticatedUser,
    post_id: i32,
    db: &State<AppState>,
) -> Result<Json<Post>, Status> {
    // Buscar el post en la base de datos
    // let post = db.find_post(post_id)?;
    
    // Verificar si el usuario es dueño del post o es admin
    // if post.author_id != user.0.sub && !user.0.is_admin {
    //     return Err(Status::Forbidden);
    // }
    
    Ok(Json(Post {
        id: post_id,
        title: "Ejemplo".to_string(),
        author: user.0.name,
    }))
}

#[derive(Serialize)]
pub struct Post {
    id: i32,
    title: String,
    author: String,
}

// ============================================================================
// EJEMPLO 9: Refresh Token (renovar token)
// ============================================================================

#[derive(Deserialize)]
pub struct RefreshRequest {
    token: String,
}

#[derive(Serialize)]
pub struct RefreshResponse {
    success: bool,
    new_token: Option<String>,
    message: String,
}

#[post("/api/refresh", format = "json", data = "<refresh>")]
pub fn refresh_token(refresh: Json<RefreshRequest>) -> Json<RefreshResponse> {
    match decode_jwt(&refresh.token) {
        Ok(claims) => {
            // Crear un nuevo token con los mismos datos
            let new_claims = Claims::new(
                claims.sub.parse().unwrap_or(0),
                claims.email,
                claims.name,
                claims.is_admin,
            );
            
            match create_jwt(&new_claims) {
                Ok(token) => Json(RefreshResponse {
                    success: true,
                    new_token: Some(token),
                    message: "Token renovado exitosamente".to_string(),
                }),
                Err(_) => Json(RefreshResponse {
                    success: false,
                    new_token: None,
                    message: "Error al crear nuevo token".to_string(),
                }),
            }
        }
        Err(_) => Json(RefreshResponse {
            success: false,
            new_token: None,
            message: "Token inválido o expirado".to_string(),
        }),
    }
}

// ============================================================================
// EJEMPLO 10: Logout (invalidar token) - Usando blacklist
// ============================================================================

// Nota: Para implementar esto completamente, necesitas:
// 1. Una tabla en la base de datos para tokens invalidados
// 2. Modificar el guardián para verificar la blacklist

#[derive(Deserialize)]
pub struct LogoutRequest {
    token: String,
}

#[post("/api/logout", format = "json", data = "<logout>")]
pub async fn logout(
    user: AuthenticatedUser,
    logout: Json<LogoutRequest>,
    db: &State<AppState>,
) -> Json<&'static str> {
    // Agregar el token a la blacklist en la base de datos
    // db.add_to_blacklist(&logout.token).await;
    
    Json("Sesión cerrada exitosamente")
}

// ============================================================================
// CÓMO REGISTRAR ESTAS RUTAS EN main.rs o lib.rs
// ============================================================================

/*
use routes::examples::{
    public_route,
    user_profile,
    list_users,
    create_post,
    update_user,
    delete_user,
    flexible_content,
    get_post,
    refresh_token,
    logout
};

pub async fn run() -> Rocket<Build> {
    let db = utils::db::establish_connection().await;
    rocket::build()
        .manage(AppState { db })
        .mount("/", routes![
            // Rutas existentes...
            
            // Nuevas rutas de ejemplo
            public_route,
            user_profile,
            list_users,
            create_post,
            update_user,
            delete_user,
            flexible_content,
            get_post,
            refresh_token,
            logout,
        ])
        .mount("/frontend", FileServer::from("../frontend"))
}
*/
