# Script de prueba para el sistema JWT
# Uso: .\test_jwt.ps1

Write-Host "==================================================" -ForegroundColor Cyan
Write-Host "   SCRIPT DE PRUEBA - SISTEMA JWT" -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host ""

# Configuración
$baseUrl = "http://localhost:8000"
$email = "usuario@ejemplo.com"
$password = "contraseña123"

# Función para mostrar respuestas
function Show-Response {
    param($response, $title)
    Write-Host ""
    Write-Host "--- $title ---" -ForegroundColor Yellow
    $response | ConvertTo-Json -Depth 10 | Write-Host -ForegroundColor Green
    Write-Host ""
}

# Función para manejar errores
function Show-Error {
    param($error, $title)
    Write-Host ""
    Write-Host "--- ERROR: $title ---" -ForegroundColor Red
    Write-Host $error -ForegroundColor Red
    Write-Host ""
}

try {
    # Test 1: Login
    Write-Host "[1/6] Probando LOGIN..." -ForegroundColor Cyan
    $loginBody = @{
        email = $email
        password = $password
    } | ConvertTo-Json

    $loginResponse = Invoke-RestMethod -Uri "$baseUrl/api/login" `
        -Method Post `
        -ContentType "application/json" `
        -Body $loginBody `
        -ErrorAction Stop

    if ($loginResponse.success) {
        Show-Response $loginResponse "LOGIN EXITOSO ✓"
        $token = $loginResponse.token
        Write-Host "Token guardado: $($token.Substring(0, 50))..." -ForegroundColor Green
    } else {
        Show-Error $loginResponse.message "LOGIN FALLIDO"
        exit 1
    }

    # Headers con el token
    $headers = @{
        Authorization = "Bearer $token"
    }

    # Test 2: Ruta protegida
    Write-Host "[2/6] Probando RUTA PROTEGIDA..." -ForegroundColor Cyan
    try {
        $protectedResponse = Invoke-RestMethod -Uri "$baseUrl/api/protected" `
            -Headers $headers `
            -ErrorAction Stop
        Show-Response $protectedResponse "RUTA PROTEGIDA ✓"
    } catch {
        Show-Error $_.Exception.Message "RUTA PROTEGIDA"
    }

    # Test 3: Información del usuario
    Write-Host "[3/6] Probando GET /api/me..." -ForegroundColor Cyan
    try {
        $meResponse = Invoke-RestMethod -Uri "$baseUrl/api/me" `
            -Headers $headers `
            -ErrorAction Stop
        Show-Response $meResponse "INFORMACIÓN DEL USUARIO ✓"
    } catch {
        Show-Error $_.Exception.Message "GET /api/me"
    }

    # Test 4: Ruta de administrador
    Write-Host "[4/6] Probando RUTA DE ADMINISTRADOR..." -ForegroundColor Cyan
    try {
        $adminResponse = Invoke-RestMethod -Uri "$baseUrl/api/admin" `
            -Headers $headers `
            -ErrorAction Stop
        Show-Response $adminResponse "RUTA DE ADMINISTRADOR ✓"
    } catch {
        if ($_.Exception.Response.StatusCode -eq 403) {
            Write-Host "⚠️  Acceso denegado - Usuario no es administrador (esperado si no eres admin)" -ForegroundColor Yellow
        } else {
            Show-Error $_.Exception.Message "RUTA DE ADMINISTRADOR"
        }
    }

    # Test 5: Dashboard de administrador
    Write-Host "[5/6] Probando DASHBOARD DE ADMINISTRADOR..." -ForegroundColor Cyan
    try {
        $dashboardResponse = Invoke-RestMethod -Uri "$baseUrl/api/admin/dashboard" `
            -Headers $headers `
            -ErrorAction Stop
        Show-Response $dashboardResponse "DASHBOARD DE ADMINISTRADOR ✓"
    } catch {
        if ($_.Exception.Response.StatusCode -eq 403) {
            Write-Host "⚠️  Acceso denegado - Usuario no es administrador (esperado si no eres admin)" -ForegroundColor Yellow
        } else {
            Show-Error $_.Exception.Message "DASHBOARD DE ADMINISTRADOR"
        }
    }

    # Test 6: Prueba sin token (debe fallar)
    Write-Host "[6/6] Probando ACCESO SIN TOKEN (debe fallar)..." -ForegroundColor Cyan
    try {
        $noTokenResponse = Invoke-RestMethod -Uri "$baseUrl/api/protected" `
            -ErrorAction Stop
        Write-Host "❌ ERROR: Debería haber fallado sin token" -ForegroundColor Red
    } catch {
        if ($_.Exception.Response.StatusCode -eq 401) {
            Write-Host "✓ Correcto: Acceso denegado sin token (401 Unauthorized)" -ForegroundColor Green
        } else {
            Show-Error $_.Exception.Message "ACCESO SIN TOKEN"
        }
    }

    # Resumen
    Write-Host ""
    Write-Host "==================================================" -ForegroundColor Cyan
    Write-Host "   PRUEBAS COMPLETADAS" -ForegroundColor Cyan
    Write-Host "==================================================" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Token JWT:" -ForegroundColor Yellow
    Write-Host $token -ForegroundColor White
    Write-Host ""
    Write-Host "Para usar en otras herramientas:" -ForegroundColor Yellow
    Write-Host "Authorization: Bearer $token" -ForegroundColor White
    Write-Host ""

} catch {
    Write-Host ""
    Write-Host "==================================================" -ForegroundColor Red
    Write-Host "   ERROR CRÍTICO" -ForegroundColor Red
    Write-Host "==================================================" -ForegroundColor Red
    Write-Host ""
    Write-Host $_.Exception.Message -ForegroundColor Red
    Write-Host ""
    Write-Host "Verifica que:" -ForegroundColor Yellow
    Write-Host "1. El servidor esté ejecutándose (cargo run)" -ForegroundColor White
    Write-Host "2. El puerto 8000 esté disponible" -ForegroundColor White
    Write-Host "3. Las credenciales sean correctas" -ForegroundColor White
    Write-Host "4. La base de datos esté conectada" -ForegroundColor White
    Write-Host ""
}

# Pausa para ver los resultados
Write-Host "Presiona cualquier tecla para salir..." -ForegroundColor Gray
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
