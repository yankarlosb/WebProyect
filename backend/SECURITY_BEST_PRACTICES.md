# 🔒 MEJORES PRÁCTICAS DE SEGURIDAD PARA JWT

## 1. 🔑 Gestión de Claves Secretas

### ❌ MAL - Clave hardcodeada
```rust
const JWT_SECRET: &str = "mi_clave_secreta";
```

### ✅ BIEN - Usar variables de entorno
```rust
use std::env;

pub fn get_jwt_secret() -> String {
    env::var("JWT_SECRET")
        .expect("JWT_SECRET debe estar definida en las variables de entorno")
}
```

### Generar una clave segura
```bash
# En Linux/Mac
openssl rand -base64 64

# En Windows PowerShell
[Convert]::ToBase64String((1..64 | ForEach-Object { Get-Random -Minimum 0 -Maximum 256 }))
```

---

## 2. ⏰ Expiración de Tokens

### Configuración Recomendada

- **Access Token**: 15 minutos - 1 hora (para operaciones normales)
- **Refresh Token**: 7-30 días (para renovar access tokens)
- **Remember Me Token**: 30-90 días (opcional, con validación extra)

### Implementación
```rust
// Token de corta duración
const ACCESS_TOKEN_DURATION: usize = 900; // 15 minutos

// Token de larga duración
const REFRESH_TOKEN_DURATION: usize = 604800; // 7 días
```

---

## 3. 🛡️ Almacenamiento en el Cliente

### Opciones y Seguridad

| Método | Pros | Contras | Recomendación |
|--------|------|---------|---------------|
| **localStorage** | Fácil de usar, persistente | Vulnerable a XSS | ⚠️ Usar solo con sanitización |
| **sessionStorage** | Se borra al cerrar pestaña | Vulnerable a XSS | ⚠️ Igual que localStorage |
| **Cookie HttpOnly** | Inmune a XSS | Vulnerable a CSRF | ✅ Recomendado con SameSite |
| **Memory (variable JS)** | Más seguro | Se pierde al refrescar | ⚠️ Para sesiones cortas |

### Ejemplo: Cookie HttpOnly (Recomendado)
```rust
use rocket::http::{Cookie, SameSite};

#[post("/api/login")]
pub fn login(/* ... */) -> (Json<Response>, Cookies) {
    let token = create_jwt(&claims)?;
    
    let cookie = Cookie::build(("jwt", token))
        .http_only(true)      // No accesible desde JavaScript
        .secure(true)         // Solo HTTPS
        .same_site(SameSite::Strict)  // Previene CSRF
        .path("/")
        .max_age(Duration::days(7))
        .finish();
    
    cookies.add_private(cookie);
    // ...
}
```

---

## 4. 🚫 Blacklist de Tokens (Logout)

### Problema
Los JWT son stateless - no puedes "invalidarlos" simplemente.

### Soluciones

#### Opción A: Blacklist en Base de Datos
```rust
// Tabla en PostgreSQL
CREATE TABLE token_blacklist (
    id SERIAL PRIMARY KEY,
    token_jti VARCHAR(255) UNIQUE NOT NULL,  -- JWT ID
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

// Agregar JTI (JWT ID) a los claims
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub jti: String,  // JWT ID único
    // ... otros campos
}

// Al logout, agregar a blacklist
pub async fn logout(token: &str, db: &Database) {
    let claims = decode_jwt(token)?;
    db.add_to_blacklist(claims.jti, claims.exp).await;
}

// Verificar en cada request
pub async fn validate_token(token: &str, db: &Database) -> Result<Claims> {
    let claims = decode_jwt(token)?;
    
    if db.is_blacklisted(&claims.jti).await {
        return Err("Token invalidado");
    }
    
    Ok(claims)
}
```

#### Opción B: Redis para Blacklist (Más rápido)
```rust
use redis::Commands;

pub async fn add_to_blacklist(token_jti: &str, expires_at: usize) {
    let mut con = redis_connection();
    let ttl = expires_at - current_timestamp();
    let _: () = con.setex(format!("bl:{}", token_jti), ttl, "1").unwrap();
}

pub async fn is_blacklisted(token_jti: &str) -> bool {
    let mut con = redis_connection();
    con.exists(format!("bl:{}", token_jti)).unwrap_or(false)
}
```

---

## 5. 🔄 Refresh Tokens

### Estrategia Recomendada

1. **Access Token** (corto): Para peticiones normales (15-60 min)
2. **Refresh Token** (largo): Para obtener nuevos access tokens (7-30 días)

### Implementación
```rust
#[derive(Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: usize,
}

pub fn create_token_pair(user: &User) -> TokenPair {
    let access_claims = Claims::with_expiration(
        user.id, user.email.clone(), user.name.clone(),
        user.is_admin, 900  // 15 minutos
    );
    
    let refresh_claims = Claims::with_expiration(
        user.id, user.email.clone(), user.name.clone(),
        user.is_admin, 604800  // 7 días
    );
    
    TokenPair {
        access_token: create_jwt(&access_claims).unwrap(),
        refresh_token: create_jwt(&refresh_claims).unwrap(),
        expires_in: 900,
    }
}

#[post("/api/refresh", data = "<refresh>")]
pub fn refresh(refresh: Json<RefreshRequest>) -> Result<Json<TokenPair>> {
    let claims = decode_jwt(&refresh.refresh_token)?;
    
    // Crear nuevo par de tokens
    let new_pair = create_token_pair(&user);
    
    // Opcional: Invalidar el refresh token usado (rotación)
    // add_to_blacklist(&claims.jti);
    
    Ok(Json(new_pair))
}
```

---

## 6. 🌐 HTTPS y CORS

### HTTPS Obligatorio
```rust
// En producción, redirigir HTTP a HTTPS
#[get("/<_..>", rank = 10)]
fn redirect_to_https(uri: &Origin) -> Redirect {
    if cfg!(not(debug_assertions)) {  // Solo en producción
        Redirect::permanent(format!("https://tu-dominio.com{}", uri))
    }
}
```

### CORS Seguro
```rust
use rocket_cors::{AllowedOrigins, CorsOptions};

fn setup_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "https://tu-dominio.com",
        "https://app.tu-dominio.com"
    ]);
    
    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error al crear CORS")
}
```

---

## 7. 🚦 Rate Limiting

### Prevenir Ataques de Fuerza Bruta
```rust
// Usando una librería como governor o simple en memoria

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        RateLimiter {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window: Duration::from_secs(window_secs),
        }
    }
    
    pub fn check_rate_limit(&self, identifier: &str) -> bool {
        let mut requests = self.requests.lock().unwrap();
        let now = Instant::now();
        
        let entry = requests.entry(identifier.to_string()).or_insert_with(Vec::new);
        
        // Limpiar requests antiguos
        entry.retain(|&time| now.duration_since(time) < self.window);
        
        // Verificar límite
        if entry.len() >= self.max_requests {
            return false;
        }
        
        entry.push(now);
        true
    }
}

// Usar en el guardián de login
#[post("/api/login")]
pub fn login(
    credentials: Json<LoginRequest>,
    rate_limiter: &State<RateLimiter>
) -> Result<Json<TokenPair>> {
    if !rate_limiter.check_rate_limit(&credentials.email) {
        return Err(Status::TooManyRequests);
    }
    
    // Continuar con el login...
}
```

---

## 8. 📝 Logging y Auditoría

### Registrar Eventos de Seguridad
```rust
use log::{info, warn, error};

#[post("/api/login")]
pub async fn login(credentials: Json<LoginRequest>) -> Result<Json<TokenPair>> {
    info!("Intento de login para: {}", credentials.email);
    
    match authenticate(&credentials.email, &credentials.password).await {
        Ok(user) => {
            info!("Login exitoso para: {} (ID: {})", credentials.email, user.id);
            // ...
        }
        Err(_) => {
            warn!("Login fallido para: {} - Credenciales inválidas", credentials.email);
            // ...
        }
    }
}

#[get("/api/admin")]
pub fn admin_route(admin: AdminUser) -> Json<Response> {
    info!("Acceso a ruta admin por: {} ({})", admin.0.name, admin.0.email);
    // ...
}
```

---

## 9. 🧪 Validación y Sanitización

### Validar Entrada del Usuario
```rust
use validator::{Validate, ValidationError};

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8, max = 100))]
    pub password: String,
}

#[post("/api/login", data = "<credentials>")]
pub fn login(credentials: Json<LoginRequest>) -> Result<Json<TokenPair>> {
    // Validar
    credentials.validate()
        .map_err(|_| Status::BadRequest)?;
    
    // Continuar...
}
```

---

## 10. 📋 Checklist de Seguridad

### Antes de Producción

- [ ] Clave JWT en variable de entorno (no hardcodeada)
- [ ] Tokens con expiración adecuada (15-60 min access, 7-30 días refresh)
- [ ] HTTPS habilitado y forzado
- [ ] Cookies con HttpOnly, Secure, SameSite
- [ ] CORS configurado correctamente
- [ ] Rate limiting en endpoints de autenticación
- [ ] Logging de eventos de seguridad
- [ ] Validación de entrada
- [ ] Blacklist de tokens (para logout)
- [ ] Bcrypt con cost adecuado (12-14)
- [ ] Refresh token rotation
- [ ] Monitoreo de intentos de login fallidos
- [ ] Backup y recuperación de base de datos
- [ ] Actualizar dependencias regularmente

### Comandos Útiles
```bash
# Auditar dependencias
cargo audit

# Verificar vulnerabilidades
cargo outdated

# Actualizar dependencias
cargo update
```

---

## 📚 Recursos Adicionales

- [OWASP JWT Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/JSON_Web_Token_for_Java_Cheat_Sheet.html)
- [RFC 7519 - JWT](https://datatracker.ietf.org/doc/html/rfc7519)
- [Rocket Security Guide](https://rocket.rs/v0.5/guide/security/)

---

**⚠️ IMPORTANTE**: La seguridad es un proceso continuo. Mantén tus dependencias actualizadas y revisa periódicamente las mejores prácticas.
