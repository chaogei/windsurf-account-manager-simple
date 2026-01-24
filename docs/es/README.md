# Windsurf Account Manager - Simple

Una aplicación de escritorio para la gestión de múltiples cuentas de Windsurf desarrollada con Tauri + Vue 3 + TypeScript, diseñada para administrar varias cuentas de Windsurf y ofrecer funciones como el restablecimiento de créditos, consultas de facturación y cambio de cuenta con un solo clic.

> ⚠️ **Aviso de Software Gratuito**: Este software es completamente gratuito. ¡Si pagaste por él, has sido estafado!

## 📱 Grupos de Comunidad

<p align="center">
  <img src="../../public/交流群.png" alt="Código QR de WeChat" width="300">
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="../../public/QQ群.jpg" alt="Código QR de QQ" width="300">
</p>

---

## 🖥️ Interfaz del Software

<p align="center">
  <img src="../../public/主页.png" alt="Página de inicio del software" width="800">
</p>

---

## ✨ Características

### 🔐 Gestión de Cuentas
- ✅ **Agregar/Editar/Eliminar Cuentas** - Operaciones CRUD completas para cuentas.
- ✅ **Gestión de Grupos** - Soporte para grupos personalizados para organizar fácilmente múltiples cuentas.
- ✅ **Sistema de Etiquetas** - Añade etiquetas personalizadas a las cuentas.
- ✅ **Estado en Tiempo Real** - Muestra el tipo de plan, saldo de créditos, fecha de vencimiento, etc.
- ✅ **Operaciones por Lote** - Selecciona múltiples cuentas para restablecimiento o eliminación masiva.
- ✅ **Almacenamiento Seguro** - Utiliza cifrado AES-256-GCM con claves almacenadas en el llavero del sistema.

### 💳 Restablecimiento de Créditos
- ✅ **Restablecimiento con un Clic** - Restablece los créditos a través de las API de actualización de asientos.
- ✅ **Rotación Inteligente de Asientos** - Alterna automáticamente entre 3, 4 y 5 asientos.
- ✅ **Restablecimiento por Lote** - Soporta el restablecimiento concurrente de múltiples cuentas (hasta 5).
- ✅ **Restablecimiento de Equipo** - Restablecimiento con un solo clic para todos los miembros de un equipo.
- ✅ **Programación Automática** - Configura tareas programadas para restablecimientos automáticos.

### 👥 Gestión de Equipos
- ✅ **Ver Miembros del Equipo** - Lista toda la información de los miembros del equipo.
- ✅ **Invitar Miembros** - Invita a nuevos miembros al equipo vía correo electrónico.
- ✅ **Eliminar Miembros** - Elimina miembros específicos del equipo.
- ✅ **Gestión de Cuotas de Equipo** - Gestión unificada de los créditos de los miembros del equipo.

### 🔄 Cambio con un Clic
- ✅ **Cambio Rápido de Cuenta** - Cambia rápidamente a otras cuentas de Windsurf.
- ✅ **Actualización Automática de Token** - Utiliza automáticamente el `refresh_token` para obtener un nuevo `access_token`.
- ✅ **Activador de Callback OAuth** - Completa automáticamente el inicio de sesión a través del protocolo `windsurf://`.
- ✅ **Restablecimiento de ID de Máquina** - Restablece el identificador del dispositivo (requiere privilegios de administrador).

### 🔧 Parche de Cambio Fluido (Seamless)
- ✅ **Detección Automática de Ruta** - Encuentra automáticamente la ubicación de instalación de Windsurf.
- ✅ **Aplicar Parche con un Clic** - Modifica `extension.js` para un cambio sin interrupciones.
- ✅ **Eliminar Límite de Tiempo** - Elimina la restricción de tiempo de espera OAuth de 180s.
- ✅ **Respaldo Automático** - Realiza copias de seguridad de los archivos originales antes de aplicar el parche (mantiene hasta 3 versiones).
- ✅ **Restaurar Estado Original** - Restauración con un clic desde los archivos de respaldo.
- ✅ **Reinicio Automático de Windsurf** - Reinicia Windsurf automáticamente tras aplicar el parche.

### 💰 Pagos
- ✅ **Generación de Tarjetas Virtuales** - Genera información de tarjetas de crédito virtuales para pruebas de pago.
- ✅ **BIN Personalizado** - Soporte para números BIN o rangos BIN personalizados.
- ✅ **Ventana de Pago Privada** - Abre las páginas de pago de Stripe en una ventana de incógnito independiente.
- ✅ **Alipay/WeChat Pay** - Soporte para métodos de pago domésticos (Donaciones).

### 📊 Consultas de Datos
- ✅ **Información de Facturación** - Consulta plan, cuotas, uso, etc.
- ✅ **Estado de Suscripción** - Muestra el tipo de suscripción, fecha de vencimiento y próxima fecha de cobro.
- ✅ **Estadísticas de Uso** - Visualiza el uso de créditos y la cuota restante.
- ✅ **Actualización Global** - Actualización con un clic de toda la información de las cuentas.

### ⚙️ Configuración del Sistema
- ✅ **Configuración de Proxy** - Soporte para ajustes de proxy HTTP.
- ✅ **Modo API Ligero** - Utiliza `GetPlanStatus` en lugar de `GetCurrentUser` para reducir las peticiones.
- ✅ **Resultados Detallados** - Opción para mostrar la respuesta detallada de la API.
- ✅ **Logs de Operaciones** - Registra todo el historial de operaciones, exportable.

### 🔒 Seguridad de Datos
- ✅ **Llavero del Sistema** - Almacena las claves de cifrado en el Administrador de Credenciales de Windows.
- ✅ **Cifrado AES-256-GCM** - Toda la información sensible está cifrada.
- ✅ **Almacenamiento Local** - Los datos se guardan solo localmente.
- ✅ **Auditoría** - Registros completos para la auditoría de operaciones.

## Stack Tecnológico

### Frontend
- **Framework**: Vue 3 + TypeScript
- **Componentes UI**: Element Plus
- **Gestión de Estado**: Pinia
- **Herramienta de Build**: Vite
- **Estilos**: CSS3 + Tema Element Plus

### Backend
- **Framework**: Tauri 2.x
- **Lenguaje**: Rust
- **Cifrado**: AES-256-GCM
- **Gestión de Claves**: Administrador de Credenciales de Windows / Keyring
- **Peticiones de Red**: Reqwest
- **Runtime Asíncrono**: Tokio

## Instalación y Ejecución

### Requisitos
- Node.js 16+
- Rust 1.70+
- Windows 10/11 (Actualmente solo soporta Windows)

### Entorno de Desarrollo

```bash
# Clonar el proyecto
git clone [url-del-repositorio]
cd windsurf-account-manager

# Instalar dependencias
npm install

# Ejecutar en modo desarrollo
npm run tauri dev
```

### Versión de Producción (Build)

```bash
# Crear instalador de Windows
npm run tauri build
```

El instalador se encontrará en `src-tauri/target/release/bundle/` tras el build.

## Licencia
AGPL-3.0

## Descargo de Responsabilidad
Esta herramienta es solo para uso educativo y personal. Por favor, cumpla con los Términos de Servicio de Windsurf. El autor no se hace responsable de los problemas derivados del uso de esta herramienta.
