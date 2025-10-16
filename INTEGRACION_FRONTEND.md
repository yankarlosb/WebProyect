# 🎨 Integración del Sistema JWT con tu Frontend Existente

Esta guía te ayudará a integrar el sistema JWT con tus páginas HTML existentes.

---

## 📁 Tus Archivos Frontend Actuales

```
frontend/
├── login.html          - Tu página de login actual
├── login-es.css        - Estilos del login
├── principal.html      - Página principal
├── principal-es.css    - Estilos de la principal
├── balance.html        - Página de balance
└── balance.css         - Estilos del balance
```

---

## 🔧 Paso 1: Modificar login.html para usar JWT

### Opción A: Mantener formulario tradicional (más fácil)

Tu `login.html` actual probablemente usa un formulario que envía a `/login`:

```html
<form method="POST" action="/login">
    <input type="email" name="email" required>
    <input type="password" name="password" required>
    <button type="submit">Iniciar Sesión</button>
</form>
```

**✅ Esto ya funciona con tu implementación actual**

El backend redirige a `/frontend/balance.html` después del login exitoso.

### Opción B: Usar JWT con JavaScript (más seguro)

Modifica tu `login.html` para usar la API JWT:

```html
<!-- En tu login.html, agregar antes de </body> -->
<script>
// Capturar el formulario
document.getElementById('loginForm').addEventListener('submit', async (e) => {
    e.preventDefault();
    
    const email = document.getElementById('email').value;
    const password = document.getElementById('password').value;
    
    try {
        // Llamar a la API JWT
        const response = await fetch('/api/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ email, password })
        });
        
        const data = await response.json();
        
        if (data.success) {
            // Guardar el token
            localStorage.setItem('jwt_token', data.token);
            localStorage.setItem('user_name', data.user.name);
            localStorage.setItem('user_email', data.user.email);
            
            // Redirigir a la página principal
            window.location.href = '/frontend/balance.html';
        } else {
            // Mostrar error
            alert('Error: ' + data.message);
        }
    } catch (error) {
        alert('Error de conexión: ' + error.message);
    }
});
</script>
```

**Cambios necesarios en tu HTML:**
```html
<!-- Añadir id al formulario -->
<form id="loginForm">
    <input type="email" id="email" name="email" required>
    <input type="password" id="password" name="password" required>
    <button type="submit">Iniciar Sesión</button>
</form>
```

---

## 🔧 Paso 2: Proteger tus páginas existentes

### Agregar verificación en balance.html y principal.html

Añade este script al inicio de cada página protegida:

```html
<!-- Agregar después de <body> en balance.html y principal.html -->
<script>
// Verificar autenticación al cargar la página
window.addEventListener('DOMContentLoaded', async () => {
    const token = localStorage.getItem('jwt_token');
    
    if (!token) {
        // No hay token, redirigir al login
        window.location.href = '/login';
        return;
    }
    
    try {
        // Verificar que el token sea válido
        const response = await fetch('/api/me', {
            headers: {
                'Authorization': 'Bearer ' + token
            }
        });
        
        if (!response.ok) {
            // Token inválido, redirigir al login
            localStorage.removeItem('jwt_token');
            window.location.href = '/login';
            return;
        }
        
        // Token válido, obtener info del usuario
        const userData = await response.json();
        
        // Opcional: Mostrar nombre del usuario
        const userNameElement = document.getElementById('user-name');
        if (userNameElement) {
            userNameElement.textContent = userData.user;
        }
        
    } catch (error) {
        console.error('Error verificando token:', error);
        window.location.href = '/login';
    }
});
</script>
```

---

## 🔧 Paso 3: Agregar botón de Cerrar Sesión

En tus páginas `balance.html` y `principal.html`:

```html
<!-- Agregar botón de logout -->
<button id="logoutBtn" onclick="logout()">Cerrar Sesión</button>

<!-- Agregar script -->
<script>
function logout() {
    // Limpiar datos del localStorage
    localStorage.removeItem('jwt_token');
    localStorage.removeItem('user_name');
    localStorage.removeItem('user_email');
    
    // Redirigir al login
    window.location.href = '/login';
}
</script>
```

---

## 🔧 Paso 4: Hacer peticiones protegidas desde tu frontend

Si necesitas hacer peticiones a tu backend desde JavaScript:

```javascript
// Función helper para hacer peticiones autenticadas
async function fetchProtected(url, options = {}) {
    const token = localStorage.getItem('jwt_token');
    
    if (!token) {
        window.location.href = '/login';
        return;
    }
    
    const headers = {
        ...options.headers,
        'Authorization': 'Bearer ' + token
    };
    
    const response = await fetch(url, {
        ...options,
        headers
    });
    
    if (response.status === 401 || response.status === 403) {
        // Token expirado o sin permisos
        localStorage.removeItem('jwt_token');
        window.location.href = '/login';
        return;
    }
    
    return response;
}

// Ejemplo de uso:
async function obtenerDatos() {
    try {
        const response = await fetchProtected('/api/datos');
        const data = await response.json();
        console.log(data);
    } catch (error) {
        console.error('Error:', error);
    }
}
```

---

## 📝 Ejemplo Completo: balance.html Modificado

```html
<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Balance</title>
    <link rel="stylesheet" href="balance.css">
</head>
<body>
    <header>
        <h1>Balance</h1>
        <div>
            <span id="user-name">Usuario</span>
            <button id="logoutBtn" onclick="logout()">Cerrar Sesión</button>
        </div>
    </header>
    
    <main>
        <!-- Tu contenido aquí -->
    </main>
    
    <!-- Script de autenticación -->
    <script>
        // Verificar autenticación
        window.addEventListener('DOMContentLoaded', async () => {
            const token = localStorage.getItem('jwt_token');
            
            if (!token) {
                window.location.href = '/login';
                return;
            }
            
            try {
                const response = await fetch('/api/me', {
                    headers: { 'Authorization': 'Bearer ' + token }
                });
                
                if (!response.ok) {
                    localStorage.removeItem('jwt_token');
                    window.location.href = '/login';
                    return;
                }
                
                const userData = await response.json();
                document.getElementById('user-name').textContent = userData.user;
                
            } catch (error) {
                console.error('Error:', error);
                window.location.href = '/login';
            }
        });
        
        // Función de logout
        function logout() {
            localStorage.removeItem('jwt_token');
            window.location.href = '/login';
        }
    </script>
</body>
</html>
```

---

## 🎨 Mejora Visual: Indicador de Sesión

Agrega un indicador visual del estado de sesión:

```css
/* En tu CSS */
.user-info {
    display: flex;
    align-items: center;
    gap: 10px;
}

.user-avatar {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background: #007bff;
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
}

.logout-btn {
    padding: 8px 16px;
    background: #dc3545;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

.logout-btn:hover {
    background: #c82333;
}
```

```html
<!-- En tu HTML -->
<div class="user-info">
    <div class="user-avatar" id="user-avatar">U</div>
    <span id="user-name">Usuario</span>
    <button class="logout-btn" onclick="logout()">Cerrar Sesión</button>
</div>

<script>
// Agregar inicial del usuario en el avatar
const userData = await response.json();
const userName = userData.user;
document.getElementById('user-name').textContent = userName;
document.getElementById('user-avatar').textContent = userName.charAt(0).toUpperCase();
</script>
```

---

## 🔄 Flujo Completo Recomendado

```
1. Usuario visita balance.html
   ↓
2. JavaScript verifica si hay token
   ↓
3. Si NO hay token → redirige a /login
   ↓
4. Usuario hace login en /login
   ↓
5. JavaScript guarda token en localStorage
   ↓
6. Redirige a /frontend/balance.html
   ↓
7. Verifica token con /api/me
   ↓
8. Si es válido, carga la página
   ↓
9. Usuario navega entre páginas (token persiste)
   ↓
10. Click en "Cerrar Sesión"
    ↓
11. Limpia localStorage
    ↓
12. Redirige a /login
```

---

## 🚀 Implementación Rápida (5 minutos)

### Archivo: frontend/auth.js (crear este archivo nuevo)
```javascript
// auth.js - Incluir en todas tus páginas protegidas

const AUTH = {
    // Verificar autenticación
    async check() {
        const token = localStorage.getItem('jwt_token');
        if (!token) {
            window.location.href = '/login';
            return false;
        }
        
        try {
            const response = await fetch('/api/me', {
                headers: { 'Authorization': 'Bearer ' + token }
            });
            
            if (!response.ok) {
                this.logout();
                return false;
            }
            
            const data = await response.json();
            return data;
        } catch (error) {
            this.logout();
            return false;
        }
    },
    
    // Cerrar sesión
    logout() {
        localStorage.removeItem('jwt_token');
        window.location.href = '/login';
    },
    
    // Hacer petición autenticada
    async fetch(url, options = {}) {
        const token = localStorage.getItem('jwt_token');
        const headers = {
            ...options.headers,
            'Authorization': 'Bearer ' + token
        };
        
        const response = await fetch(url, { ...options, headers });
        
        if (response.status === 401 || response.status === 403) {
            this.logout();
        }
        
        return response;
    }
};

// Auto-verificar al cargar cualquier página
window.addEventListener('DOMContentLoaded', async () => {
    const userData = await AUTH.check();
    if (userData) {
        // Opcional: Mostrar nombre del usuario
        const elem = document.getElementById('user-name');
        if (elem) elem.textContent = userData.user;
    }
});
```

### Luego, en tus páginas protegidas:
```html
<!-- En balance.html, principal.html, etc. -->
<head>
    <!-- Tu CSS existente -->
    <link rel="stylesheet" href="balance.css">
</head>
<body>
    <!-- Tu contenido existente -->
    
    <!-- Agregar antes de </body> -->
    <script src="/frontend/auth.js"></script>
</body>
```

---

## ✅ Checklist de Integración

### Login Page
- [ ] Modificar formulario para usar API JWT
- [ ] Guardar token en localStorage
- [ ] Redirigir a página principal después del login

### Páginas Protegidas (balance, principal)
- [ ] Incluir script de verificación
- [ ] Verificar token al cargar
- [ ] Redirigir a login si no hay token
- [ ] Agregar botón de logout

### Opcional
- [ ] Crear archivo auth.js compartido
- [ ] Mostrar nombre del usuario
- [ ] Agregar avatar/inicial
- [ ] Manejar expiración de token
- [ ] Agregar indicador de carga

---

## 🆘 Solución de Problemas

### "El token no persiste entre páginas"
✅ Verifica que uses localStorage.setItem() después del login

### "Me redirige al login constantemente"
✅ Verifica que el token esté guardado: console.log(localStorage.getItem('jwt_token'))

### "CORS error en el navegador"
✅ Verifica que las páginas se sirvan desde el mismo dominio (localhost:8000)

### "Token expirado muy rápido"
✅ El token expira en 24 horas, puedes ajustarlo en jwt.rs

---

## 🎯 Próximo Paso Recomendado

**Opción Fácil (5 minutos):**
1. Crea `frontend/auth.js` con el código de arriba
2. Agrega `<script src="/frontend/auth.js"></script>` en balance.html y principal.html
3. ¡Listo! Ya tienes protección JWT

**Opción Completa (15 minutos):**
1. Modifica login.html para usar la API JWT
2. Crea auth.js
3. Modifica todas las páginas protegidas
4. Agrega botones de logout
5. Prueba el flujo completo

---

**¡Tu sistema JWT está listo para integrarse con tu frontend! 🚀**
