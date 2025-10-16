use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::http::CookieJar;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// Clave secreta para firmar los tokens (en producción, debe estar en variables de entorno)
const JWT_SECRET: &str = "tu_clave_secreta_muy_segura_cambiar_en_produccion";

// Estructura de los claims del JWT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,      // Subject (user ID)
    pub email: String,    // Email del usuario
    pub name: String,     // Nombre del usuario
    pub is_admin: bool,   // Si es administrador
    pub exp: usize,       // Expiration time (timestamp)
    pub iat: usize,       // Issued at (timestamp)
}

impl Claims {
    /// Crea un nuevo claim con una expiración de 24 horas
    pub fn new(user_id: i32, email: String, name: String, is_admin: bool) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        Claims {
            sub: user_id.to_string(),
            email,
            name,
            is_admin,
            iat: now,
            exp: now + 86400, // 24 horas = 86400 segundos
        }
    }

    /// Crea un token con una expiración personalizada en segundos
    pub fn with_expiration(
        user_id: i32,
        email: String,
        name: String,
        is_admin: bool,
        expiration_secs: usize,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        Claims {
            sub: user_id.to_string(),
            email,
            name,
            is_admin,
            iat: now,
            exp: now + expiration_secs,
        }
    }
}

/// Genera un token JWT a partir de los claims
pub fn create_jwt(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    let header = Header::new(Algorithm::HS256);
    encode(
        &header,
        claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
}

/// Decodifica y valida un token JWT
pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &validation,
    )?;
    Ok(token_data.claims)
}

// ============================================================================
// GUARDIANES DE AUTENTICACIÓN
// ============================================================================

/// Guardián que valida que el usuario esté autenticado
/// Extrae el token del header Authorization: Bearer <token> O de la cookie jwt_token
pub struct AuthenticatedUser(pub Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Intentar obtener el token del header Authorization primero
        let token_from_header = request
            .headers()
            .get_one("Authorization")
            .and_then(|header| {
                if header.starts_with("Bearer ") {
                    Some(header[7..].to_string())
                } else {
                    None
                }
            });

        // Si no hay token en el header, intentar obtenerlo de la cookie
        let token = match token_from_header {
            Some(t) => Some(t),
            None => {
                let cookies = request.cookies();
                cookies.get("jwt_token").map(|c| c.value().to_string())
            }
        };

        match token {
            Some(token) => match decode_jwt(&token) {
                Ok(claims) => Outcome::Success(AuthenticatedUser(claims)),
                Err(_) => Outcome::Error((Status::Unauthorized, ())),
            },
            None => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

/// Guardián que valida que el usuario sea administrador
pub struct AdminUser(pub Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Primero verificamos que esté autenticado
        let auth_user = match request.guard::<AuthenticatedUser>().await {
            Outcome::Success(user) => user,
            Outcome::Error(e) => return Outcome::Error(e),
            Outcome::Forward(f) => return Outcome::Forward(f),
        };

        // Verificamos que sea administrador
        if auth_user.0.is_admin {
            Outcome::Success(AdminUser(auth_user.0))
        } else {
            Outcome::Error((Status::Forbidden, ()))
        }
    }
}

// ============================================================================
// RESPUESTAS JSON PARA AUTENTICACIÓN
// ============================================================================

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub token: Option<String>,
    pub user: Option<UserInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}

impl LoginResponse {
    pub fn success(token: String, claims: &Claims) -> Self {
        LoginResponse {
            success: true,
            message: "Login exitoso".to_string(),
            token: Some(token),
            user: Some(UserInfo {
                id: claims.sub.clone(),
                name: claims.name.clone(),
                email: claims.email.clone(),
                is_admin: claims.is_admin,
            }),
        }
    }

    pub fn error(message: String) -> Self {
        LoginResponse {
            success: false,
            message,
            token: None,
            user: None,
        }
    }
}
