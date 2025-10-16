# 🔒 Sistema de Autenticación con Cookies JWT - IMPLEMENTADO

## ✅ ¿Qué se implementó?

Tu aplicación ahora tiene **autenticación completa con JWT usando cookies HttpOnly**. Solo los usuarios autenticados pueden acceder a las páginas protegidas.

---

## 🎯 Estructura de Rutas

### 📖 Rutas PÚBLICAS (sin autenticación)
```
GET  /login              → Página de login
POST /login              → Procesar login (formulario HTML)
```

### 🔒 Páginas PROTEGIDAS (requieren autenticación)
```
GET  /balance            → Página de balance (protegida)
GET  /principal          → Página principal (protegida)
GET  /logout             → Cerrar sesión
```

### 🔐 API PROTEGIDA (requieren autenticación)
```
GET  /api/me             → Información del usuario actual
GET  /api/protected      → Ruta de ejemplo protegida
POST /api/login          → Login con JSON (opcional, para APIs)
```

### 👑 Rutas de ADMINISTRADOR (requieren rol admin)
```
GET  /api/admin          → Ruta exclusiva para admins
GET  /api/admin/dashboard → Dashboard administrativo
```

### 📁 Archivos Estáticos
```
GET  /frontend/<archivo> → CSS, JS, imágenes (públicos)
```

---

## 🔄 Flujo de Autenticación

```
1. Usuario visita /balance sin estar autenticado
   ↓
2. Guardián detecta que NO hay cookie JWT
   ↓
3. Devuelve 401 Unauthorized
   ↓
4. Catcher captura el 401 y redirige a /login
   ↓
5. Usuario llena formulario y envía POST /login
   ↓
6. Backend valida credenciales
   ↓
7. Si son correctas:
   - Crea JWT
   - Establece cookie HttpOnly
   - Redirige a /balance
   ↓
8. Ahora /balance funciona porque la cookie se envía automáticamente
   ↓
9. Usuario hace clic en /logout
   ↓
10. Cookie se elimina y redirige a /login
```

---

## 🍪 ¿Cómo funciona la Cookie JWT?

### Características de la Cookie:
```rust
Cookie {
    name: "jwt_token",
    value: "eyJhbGc...",        // Token JWT
    http_only: true,             // ✅ No accesible desde JavaScript (seguro)
    same_site: Lax,              // ✅ Protección CSRF
    path: "/",                   // ✅ Disponible en toda la app
    max_age: 24 horas,           // ✅ Expira automáticamente
    secure: false                // ⚠️ En producción: true (solo HTTPS)
}
```

### Ventajas:
- ✅ **Automática**: El navegador envía la cookie en cada request
- ✅ **Segura**: HttpOnly = inmune a XSS
- ✅ **Sin JavaScript**: No necesitas código extra en el frontend
- ✅ **Estándar**: Funciona con cualquier framework frontend

---

## 🧪 Cómo Probar

### 1. Ejecutar el Servidor
```powershell
cd backend
cargo run
```

### 2. Verificar Rutas
Deberías ver en la consola:
```
Routes:
   >> (login_get) GET /login
   >> (login_form) POST /login
   >> (balance_page) GET /balance
   >> (principal_page) GET /principal
   >> (logout) GET /logout
   >> ...
Catchers:
   >> (unauthorized) 401
```

### 3. Probar Flujo Completo

#### A) Intentar acceder sin autenticación:
```
http://localhost:8000/balance
```
**Resultado esperado:** Redirige a `/login` ✅

#### B) Hacer login:
```
http://localhost:8000/login
```
- Ingresa email y contraseña
- Haz clic en "Login"

**Resultado esperado:** 
- Cookie JWT establecida ✅
- Redirige a `/balance` ✅
- Página se muestra correctamente ✅

#### C) Navegar libremente:
```
http://localhost:8000/balance   → ✅ Funciona
http://localhost:8000/principal → ✅ Funciona
```

#### D) Cerrar sesión:
```
http://localhost:8000/logout
```
**Resultado esperado:**
- Cookie eliminada ✅
- Redirige a `/login` ✅

#### E) Intentar acceder de nuevo:
```
http://localhost:8000/balance
```
**Resultado esperado:** Redirige a `/login` ✅

---

## 🔍 Verificar Cookie en el Navegador

### Chrome/Edge/Firefox:
1. Abre DevTools (F12)
2. Ve a la pestaña **Application** (Chrome) o **Storage** (Firefox)
3. En el menú izquierdo: **Cookies** → `http://localhost:8000`
4. Deberías ver:

```
Name: jwt_token
Value: eyJhbGc...
Domain: localhost
Path: /
HttpOnly: ✓
Secure: (vacío en desarrollo)
SameSite: Lax
Expires: (24 horas desde login)
```

---

## 🛠️ Modificar tus Páginas HTML (Opcional)

Si quieres agregar un botón de logout en tus páginas:

### En `balance.html` o `principal.html`:
```html
<nav>
    <a href="/balance">Balance</a>
    <a href="/principal">Principal</a>
    <a href="/logout">Cerrar Sesión</a>
</nav>
```

### Si quieres mostrar info del usuario (opcional):
```html
<script>
// Obtener información del usuario actual
fetch('/api/me', {
    credentials: 'include' // Incluye cookies automáticamente
})
.then(res => res.json())
.then(data => {
    document.getElementById('username').textContent = data.user;
})
.catch(() => {
    // Si falla, no está autenticado
    window.location.href = '/login';
});
</script>

<p>Bienvenido, <span id="username">Cargando...</span></p>
```

---

## 🔒 Seguridad

### ✅ Implementado:
- Cookie HttpOnly (protege contra XSS)
- SameSite=Lax (protege contra CSRF)
- Expiración de tokens (24 horas)
- Validación automática en cada request
- Redirección automática si no está autenticado

### ⚠️ Para Producción:
1. **Activar `secure: true`** en las cookies (solo HTTPS)
2. **Cambiar JWT_SECRET** a variable de entorno
3. **Usar HTTPS** obligatoriamente
4. **Implementar rate limiting** en el login
5. **Agregar logs** de intentos de login

---

## 📝 Estructura del Código

### Backend:
```
src/routes/login.rs
├── login_get()           → GET /login
├── login_form()          → POST /login (establece cookie)
├── balance_page()        → GET /balance (protegida)
├── principal_page()      → GET /principal (protegida)
├── logout()              → GET /logout
└── unauthorized()        → Catcher 401

src/utils/jwt.rs
├── Claims               → Estructura del token
├── create_jwt()         → Genera token
├── decode_jwt()         → Valida token
└── AuthenticatedUser    → Guardián (lee cookie automáticamente)

src/lib.rs
├── Registro de rutas
└── Registro de catchers
```

---

## 🎯 Ventajas de Esta Implementación

### 1. **Cero JavaScript requerido**
- Las cookies se manejan automáticamente
- Perfecto para HTML tradicional

### 2. **Más seguro que localStorage**
- HttpOnly = No accesible desde JS
- Protege contra XSS

### 3. **Flexible**
- Puedes agregar JavaScript después si lo necesitas
- Compatible con cualquier framework (React, Vue, etc.)

### 4. **Automático**
- El navegador envía la cookie en cada request
- No necesitas código extra

### 5. **Protección completa**
- Todas las páginas excepto login están protegidas
- Redirección automática si no está autenticado

---

## 🆘 Solución de Problemas

### No me deja acceder a /balance
✅ **Esperado**: Si no has hecho login, te redirige a `/login`

### La cookie no se establece
- Verifica que las credenciales sean correctas
- Revisa la consola del backend para errores
- Verifica en DevTools → Application → Cookies

### Después de login sigo en /login
- Verifica que la cookie se haya establecido
- Revisa que el email/password sean correctos en la BD
- Verifica que bcrypt valide correctamente

### El guardián no funciona
- Verifica que la cookie se llame `jwt_token`
- Verifica que el token no haya expirado (24h)
- Revisa la consola del backend

---

## 📚 Recursos

- **JWT.io** - Debugger de JWT: https://jwt.io
- **RFC 6265** - HTTP Cookies: https://tools.ietf.org/html/rfc6265
- **OWASP** - Session Management: https://owasp.org

---

## ✨ Resultado Final

Tu aplicación ahora tiene:
- ✅ **Login funcional** con formularios HTML
- ✅ **Autenticación segura** con cookies JWT HttpOnly
- ✅ **Páginas protegidas** (/balance, /principal)
- ✅ **Redirección automática** si no está autenticado
- ✅ **Logout funcional** que elimina la sesión
- ✅ **Sin JavaScript requerido** (100% automático)

**¡Tu aplicación está segura y lista para usar! 🎉**

---

**Para probar ahora:**
```
1. cargo run
2. Abre http://localhost:8000/balance
3. Te redirigirá a /login
4. Ingresa credenciales
5. ¡Acceso concedido! 🚀
```
