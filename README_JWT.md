# 🚀 Sistema de Autenticación JWT - Guía Rápida

## ✨ ¿Qué se implementó?

Un sistema completo de autenticación con **JSON Web Tokens (JWT)** para tu aplicación web en Rocket (Rust), incluyendo:

- ✅ Autenticación segura con JWT
- ✅ Guardianes (Guards) para proteger rutas
- ✅ Sistema de roles (usuario/administrador)
- ✅ API RESTful completa
- ✅ Interfaz de prueba interactiva
- ✅ Documentación extensa

---

## 🚀 Inicio Rápido (5 minutos)

### 1. Compilar y ejecutar
```powershell
cd backend
cargo run
```

### 2. Probar el sistema
Abre en tu navegador:
```
http://localhost:8000/frontend/jwt-test.html
```

### 3. Iniciar sesión
- **Email**: tu_usuario@ejemplo.com
- **Password**: tu_contraseña

¡Listo! Ya puedes probar todas las funcionalidades.

---

## 📁 Archivos Importantes

| Archivo | Descripción |
|---------|-------------|
| **`IMPLEMENTACION_COMPLETA.md`** | 📘 **EMPIEZA AQUÍ** - Resumen completo |
| **`JWT_README.md`** | 📖 Documentación detallada del sistema |
| **`SECURITY_BEST_PRACTICES.md`** | 🔒 Guía de seguridad |
| **`DIAGRAMA_FLUJO.md`** | 📊 Diagramas visuales del flujo |
| **`test_requests.http`** | 🧪 Colección de pruebas HTTP |
| **`test_jwt.ps1`** | 🔧 Script de pruebas PowerShell |
| **`.env.example`** | ⚙️ Variables de entorno de ejemplo |

---

## 📚 Documentación

### Para Empezar
1. Lee **`IMPLEMENTACION_COMPLETA.md`** para un resumen general
2. Revisa **`DIAGRAMA_FLUJO.md`** para entender el flujo
3. Consulta **`JWT_README.md`** para detalles técnicos

### Para Producción
4. Lee **`SECURITY_BEST_PRACTICES.md`** ANTES de subir a producción
5. Configura las variables de entorno según `.env.example`

---

## 🔑 Uso Básico

### Desde JavaScript/Frontend
```javascript
// 1. Login
const response = await fetch('/api/login', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
        email: 'usuario@ejemplo.com',
        password: 'contraseña123'
    })
});

const data = await response.json();
const token = data.token;

// 2. Guardar token
localStorage.setItem('jwt_token', token);

// 3. Usar en peticiones
fetch('/api/protected', {
    headers: {
        'Authorization': 'Bearer ' + token
    }
});
```

### Desde PowerShell
```powershell
# Ejecutar script de prueba
.\test_jwt.ps1
```

---

## 🛠️ Estructura del Código

```
backend/src/
├── utils/
│   └── jwt.rs              # ⭐ Sistema JWT y guardianes
├── routes/
│   ├── login.rs            # ⭐ Rutas de autenticación
│   └── jwt_examples.rs     # 📝 Ejemplos adicionales
├── database/
│   └── usuarios.rs         # 👤 Modelo de usuarios
└── lib.rs                  # 🔧 Configuración principal

frontend/
└── jwt-test.html           # 🧪 Interfaz de prueba
```

---

## 🔐 Rutas Disponibles

### Públicas
- `GET /login` - Página de login
- `POST /api/login` - Login con JSON

### Protegidas (requieren login)
- `GET /api/protected` - Ruta de ejemplo
- `GET /api/me` - Info del usuario actual

### Admin (requieren rol admin)
- `GET /api/admin` - Ruta de admin
- `GET /api/admin/dashboard` - Dashboard admin

---

## 📊 Códigos de Estado

| Código | Significado |
|--------|-------------|
| **200** | ✅ Éxito |
| **401** | 🔒 No autorizado (token inválido/ausente) |
| **403** | 🚫 Prohibido (sin permisos) |
| **500** | ⚠️ Error del servidor |

---

## ⚠️ IMPORTANTE para Producción

1. **Cambiar la clave secreta JWT** en `jwt.rs`
2. **Usar HTTPS** obligatoriamente
3. **Configurar variables de entorno**
4. **Implementar rate limiting**
5. **Revisar checklist de seguridad**

Ver **`SECURITY_BEST_PRACTICES.md`** para más detalles.

---

## 🧪 Probar el Sistema

### Opción 1: Interfaz Web (Recomendado)
```
http://localhost:8000/frontend/jwt-test.html
```

### Opción 2: Script PowerShell
```powershell
.\test_jwt.ps1
```

### Opción 3: cURL
```bash
curl -X POST http://localhost:8000/api/login \
  -H "Content-Type: application/json" \
  -d '{"email":"usuario@ejemplo.com","password":"pass"}'
```

---

## 🎯 Próximos Pasos

### Corto Plazo
- [ ] Probar con tu base de datos
- [ ] Integrar con tu frontend existente
- [ ] Ajustar tiempos de expiración

### Medio Plazo
- [ ] Implementar refresh tokens
- [ ] Agregar logout con blacklist
- [ ] Configurar CORS
- [ ] Implementar rate limiting

### Largo Plazo
- [ ] Agregar 2FA
- [ ] Implementar OAuth2
- [ ] Sistema de recuperación de contraseña

---

## 🆘 Solución de Problemas

### No compila
```powershell
cargo clean
cargo build
```

### Token no funciona
- Verifica el header: `Authorization: Bearer <token>`
- Verifica que no haya expirado (24h por defecto)
- Verifica que el formato sea correcto

### Error 403 en rutas admin
- Tu usuario no es administrador
- Verifica el campo `isadmin` en la BD

---

## 📖 Recursos

- [JWT.io](https://jwt.io) - Debugger de JWT
- [Rocket Docs](https://rocket.rs) - Framework Rust
- [RFC 7519](https://tools.ietf.org/html/rfc7519) - Especificación JWT

---

## 📝 Licencia

Este código es parte de tu proyecto universitario.

---

## 🎉 ¡Todo Listo!

Tu sistema JWT está completamente implementado y documentado.

**Para empezar:**
```powershell
cd backend
cargo run
```

Luego abre: `http://localhost:8000/frontend/jwt-test.html`

---

**¿Dudas?** Consulta los archivos de documentación listados arriba. 📚
