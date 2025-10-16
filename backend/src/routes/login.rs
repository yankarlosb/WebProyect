use crate::*;
use crate::utils::jwt::{create_jwt, Claims, LoginResponse, AuthenticatedUser, AdminUser};
use rocket::{get, post, catch};
use rocket::response::content;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(FromForm)]
pub struct LoginForm {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginJson {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct ApiResponse {
    message: String,
    alert: String,
}

/// Página de login (HTML)
#[get("/login")]
pub async fn login_get() -> Option<NamedFile> {
    NamedFile::open("../frontend/login.html").await.ok()
}

/// Login con formulario HTML - Establece cookie JWT y redirecciona
#[post("/login", data = "<user>")]
pub async fn login_form(
    user: rocket::form::Form<LoginForm>,
    db: &State<AppState>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, Status> {
    let email = &user.email;
    let password = &user.password;

    // Buscar el usuario en la base de datos
    let entity = usuarios::Entity::find()
        .filter(usuarios::Column::Email.eq(email))
        .one(&db.db)
        .await
        .map_err(|_| Status::InternalServerError)?;

    let user_entity = match entity {
        Some(u) => u,
        None => return Err(Status::Unauthorized), // Usuario no encontrado
    };

    // Verificar la contraseña
    let verify = bcrypt::verify(password, &user_entity.token).unwrap_or(false);
    if !verify {
        return Err(Status::Unauthorized);
    }

    // Crear los claims del JWT
    let claims = Claims::new(
        user_entity.id,
        user_entity.email.clone(),
        user_entity.name.clone(),
        user_entity.isadmin.unwrap_or(false),
    );

    // Generar el token JWT
    let token = create_jwt(&claims).map_err(|_| Status::InternalServerError)?;

    // Crear cookie HttpOnly con el token JWT (más seguro que localStorage)
    let mut cookie = Cookie::new("jwt_token", token);
    cookie.set_http_only(true); // No accesible desde JavaScript (protege contra XSS)
    cookie.set_same_site(SameSite::Lax); // Protección CSRF
    cookie.set_path("/");
    cookie.set_max_age(Duration::hours(24)); // Expira en 24 horas
    
    // En producción, descomentar esto para usar solo con HTTPS:
    // cookie.set_secure(true);
    
    cookies.add(cookie);

    // Redireccionar a la página de balance (ruta protegida)
    Ok(Redirect::to("/balance"))
}

/// Login con JSON (devuelve token JWT)
#[post("/api/login", format = "json", data = "<credentials>")]
pub async fn login_json(
    credentials: Json<LoginJson>,
    db: &State<AppState>,
) -> Json<LoginResponse> {
    let email = &credentials.email;
    let password = &credentials.password;

    // Buscar el usuario en la base de datos
    let entity = match usuarios::Entity::find()
        .filter(usuarios::Column::Email.eq(email))
        .one(&db.db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Json(LoginResponse::error("Usuario no encontrado".to_string()));
        }
        Err(_) => {
            return Json(LoginResponse::error("Error del servidor".to_string()));
        }
    };

    // Verificar la contraseña
    let verify = bcrypt::verify(password, &entity.token).unwrap_or(false);
    if !verify {
        return Json(LoginResponse::error("Contraseña incorrecta".to_string()));
    }

    // Crear los claims del JWT
    let claims = Claims::new(
        entity.id,
        entity.email.clone(),
        entity.name.clone(),
        entity.isadmin.unwrap_or(false),
    );

    // Generar el token
    match create_jwt(&claims) {
        Ok(token) => Json(LoginResponse::success(token, &claims)),
        Err(_) => Json(LoginResponse::error("Error al generar el token".to_string())),
    }
}

/// Página de balance - Solo usuarios autenticados
#[get("/balance")]
pub async fn balance_page(_user: AuthenticatedUser) -> Option<NamedFile> {
    // El guardián valida automáticamente la cookie JWT
    // Si no está autenticado, devuelve 401 y Rocket maneja el error
    NamedFile::open("../frontend/balance.html").await.ok()
}

/// Logout - Elimina la cookie JWT y redirecciona al login
#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Redirect {
    // Eliminar la cookie JWT
    cookies.remove(Cookie::build("jwt_token"));
    
    // Redireccionar al login
    Redirect::to("/login")
}

/// Catcher para error 401 (No autorizado) - Muestra alerta y redirige
#[catch(401)]
pub fn unauthorized() -> content::RawHtml<&'static str> {
    content::RawHtml(r#"
        <!DOCTYPE html>
        <html>
        <head><meta charset="UTF-8"></head>
        <body>
            <script>
                alert('⚠️ Por favor, inicie sesión para acceder a esta página.');
                window.location.href = '/login';
            </script>
        </body>
        </html>
    "#)
}