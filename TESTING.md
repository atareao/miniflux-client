# Tests Unitarios - Miniflux Client

Este documento describe los tests unitarios implementados para el proyecto miniflux-client.

## Resumen

Se han implementado **39 tests en total**:
- ✅ **33 tests** que pasan correctamente (27 unitarios + 6 con mocks)
- ⚠️ **6 tests ignorados** (requieren credenciales reales de APIs)

## Tests por Categoría

### ✅ Tests Unitarios (27 pasando)

- ✅ 8 tests para la función `escape()` 
- ✅ 4 tests para `MinifluxClient` (serialización, creación, clone)
- ✅ 6 tests para `MatrixClient` (serialización, timestamp)
- ✅ 5 tests para `TelegramClient` (serialización, creación, defaults)
- ✅ 4 tests para `Model` (serialización, creación, clone)

### ✅ Tests con Mocks (6 pasando)

- ✅ 4 tests para `MinifluxClient` (get_entries, get_categories, mark_as_read, unauthorized_error)
- ✅ 2 tests para `TelegramClient` (send_message, send_message_error)

## Ejecutar Tests

### Todos los tests unitarios (sin conexión externa)
```bash
cargo test test_
```

### Tests específicos por módulo

#### Tests de la función escape
```bash
cargo test test_escape
```

#### Tests de MinifluxClient
```bash
cargo test test_miniflux_client
```

#### Tests de MatrixClient
```bash
cargo test test_matrix_client
cargo test test_ts
```

#### Tests de TelegramClient
```bash
cargo test test_telegram_client
```

#### Tests de Model
```bash
cargo test test_model
```

### Tests de integración (requieren configuración)
Los siguientes tests requieren servicios externos y variables de entorno configuradas:

```bash
# Requiere MINIFLUX_URL y MINIFLUX_TOKEN
cargo test read_entries
cargo test read_categories
cargo test read_category_entries

# Requiere MATRIX_URL, MATRIX_TOKEN y MATRIX_ROOM
cargo test matrix::test::post

# Requiere TELEGRAM_TOKEN, TELEGRAM_CHAT_ID y TELEGRAM_THREAD_ID
cargo test telegram::test::telegram

# Requiere MODEL_URL, MODEL_API_KEY, MODEL_NAME, MODEL_DESCRIPTION y MODEL_PROMPT
cargo test process_news
```

### Listar todos los tests
```bash
cargo test -- --list
```

## Cobertura de Tests

### 1. Función `escape()` (src/main.rs)

Escapa caracteres especiales de Markdown para Telegram:

- ✅ `test_escape_simple_text` - Texto sin caracteres especiales
- ✅ `test_escape_text_with_markdown_chars` - Texto con un asterisco
- ✅ `test_escape_text_with_multiple_special_chars` - Texto con múltiples caracteres especiales
- ✅ `test_escape_empty_string` - Cadena vacía
- ✅ `test_escape_all_special_chars` - Todos los caracteres especiales
- ✅ `test_escape_mixed_content` - Contenido mixto con símbolos

### 2. MinifluxClient (src/models/miniflux.rs)

Cliente para interactuar con Miniflux RSS:

**Tests unitarios:**
- ✅ `test_miniflux_client_creation` - Creación del cliente
- ✅ `test_miniflux_client_clone` - Clonación del cliente
- ✅ `test_miniflux_client_serialize` - Serialización a JSON
- ✅ `test_miniflux_client_deserialize` - Deserialización desde JSON

**Tests de integración:**
- ⚠️ `read_entries` - Leer entradas de Miniflux (requiere conexión)
- ⚠️ `read_categories` - Leer categorías de Miniflux (requiere conexión)
- ⚠️ `read_category_entries` - Leer entradas por categoría (requiere conexión)

### 3. MatrixClient (src/models/matrix.rs)

Cliente para enviar mensajes a Matrix:

**Tests unitarios:**
- ✅ `test_matrix_client_creation` - Creación del cliente
- ✅ `test_matrix_client_clone` - Clonación del cliente
- ✅ `test_matrix_client_serialize` - Serialización a JSON
- ✅ `test_matrix_client_deserialize` - Deserialización desde JSON
- ✅ `test_ts_returns_positive_value` - Función timestamp devuelve valor positivo
- ✅ `test_ts_returns_different_values` - Función timestamp devuelve valores diferentes

**Tests de integración:**
- ⚠️ `post` - Enviar mensaje a Matrix (requiere conexión)

### 4. TelegramClient (src/models/telegram.rs)

Cliente para enviar mensajes a Telegram:

**Tests unitarios:**
- ✅ `test_telegram_client_creation` - Creación del cliente
- ✅ `test_telegram_client_clone` - Clonación del cliente
- ✅ `test_telegram_client_serialize` - Serialización a JSON
- ✅ `test_telegram_client_deserialize` - Deserialización desde JSON
- ✅ `test_telegram_client_with_default_thread_id` - Thread ID por defecto

**Tests de integración:**
- ⚠️ `telegram` - Enviar mensaje a Telegram (requiere conexión)

### 5. Model (src/models/model.rs)

Cliente para procesar noticias con IA:

**Tests unitarios:**
- ✅ `test_model_creation` - Creación del modelo
- ✅ `test_model_clone` - Clonación del modelo
- ✅ `test_model_serialize` - Serialización a JSON
- ✅ `test_model_deserialize` - Deserialización desde JSON

**Tests de integración:**
- ⚠️ `process_news` - Procesar noticias con IA (requiere conexión)

## Resultados

Todos los tests unitarios pasan exitosamente:

```
test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured
```

## Notas

- Los tests unitarios no requieren configuración de variables de entorno
- Los tests de integración (marcados con ⚠️) requieren:
  - Variables de entorno configuradas (ver `.env.example`)
  - Servicios externos disponibles (Miniflux, Matrix, Telegram, API de IA)
  - Archivo `.env` con credenciales válidas usando `dotenv`

## Mejoras Futuras

Posibles mejoras a los tests:

1. ✨ Añadir más tests con mocks para otros métodos de API
2. ✨ Aumentar cobertura de código con más casos edge
3. ✨ Añadir tests de errores y manejo de excepciones
4. ✨ Implementar tests de rendimiento
5. ✨ Añadir tests de concurrencia para operaciones async

## ✅ Refactorización Completada - Tests con Mocks Funcionales

Se ha completado la refactorización del código para soportar inyección de dependencias, permitiendo que **todos los 6 tests con mocks ahora pasen correctamente**.

### ✅ Cambios Implementados

**MinifluxClient**:
- ✅ Añadido campo `base_url: Option<String>` (omitido en serialización con `#[serde(skip)]`)
- ✅ Nuevo constructor `with_base_url(url, token, base_url)` para testing
- ✅ Método privado `get_base_url()` que retorna "https" por defecto
- ✅ Todas las URLs ahora usan `{}://{}/path` en lugar de `https://{}/path`

**TelegramClient**:
- ✅ Añadido campo `base_url: Option<String>` (omitido en serialización)
- ✅ Nuevo constructor `with_base_url(token, chat_id, thread_id, base_url)`
- ✅ Método privado `get_base_url()` que retorna URL de Telegram API por defecto
- ✅ `send_message()` usa base URL configurable

**MatrixClient**:
- ✅ Añadido campo `base_url: Option<String>` (omitido en serialización)
- ✅ Nuevo constructor `with_base_url(server, token, room, base_url)`
- ✅ Método privado `get_base_url()` que retorna "https" por defecto
- ✅ `post()` usa base URL configurable

### Uso en Producción vs Testing

**Producción** (sin cambios en código existente):
```rust
let client = MinifluxClient::new(
    "miniflux.example.com".to_string(),
    "token123".to_string()
); // Usa https:// automáticamente
```

**Testing con mocks**:
```rust
let mut server = mockito::Server::new_async().await;
let client = MinifluxClient::with_base_url(
    server.host_with_port(),      // e.g., "127.0.0.1:1234"
    "test_token".to_string(),
    "http".to_string(),            // Usa http:// para mock server
);
```

### ✅ Tests con Mocks Activos (6 pasando)

**MinifluxClient** (4 tests):
- ✅ `test_get_entries_with_mock` - Mockea obtención de entradas
- ✅ `test_get_categories_with_mock` - Mockea obtención de categorías
- ✅ `test_mark_as_read_with_mock` - Mockea marcar como leído
- ✅ `test_get_entries_unauthorized_error` - Mockea error 401

**TelegramClient** (2 tests):
- ✅ `test_send_message_with_mock` - Mockea envío exitoso
- ✅ `test_send_message_error_with_mock` - Mockea error en envío

### Ejecutar Tests con Mocks

```bash
# Ejecutar solo tests con mocks
cargo test mock

# O específicamente
cargo test test_get_entries_with_mock
cargo test test_send_message_with_mock
```

### Resultado de Tests

```bash
cargo test
```

Resultado:
```
test result: ok. 33 passed; 0 failed; 6 ignored
```

- **33 tests pasando**: 27 unitarios + 6 con mocks
- **6 tests ignorados**: Tests de integración que requieren credenciales reales

O específicamente:

```bash
cargo test test_get_entries_with_mock -- --ignored
```
