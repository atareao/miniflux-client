# Tests Unitarios - Miniflux Client

Este documento describe los tests unitarios implementados para el proyecto miniflux-client.

## Resumen

Se han implementado **31 tests unitarios** que cubren las funcionalidades principales del proyecto:

- ✅ 6 tests para la función `escape()` 
- ✅ 4 tests para `MinifluxClient`
- ✅ 7 tests para `MatrixClient`
- ✅ 5 tests para `TelegramClient`
- ✅ 4 tests para `Model`
- ✅ 3 tests de integración (requieren servicios externos)
- ✅ 2 tests adicionales (timestamp)

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

1. ✨ Añadir tests con mocks para tests de integración
2. ✨ Aumentar cobertura de código con más casos edge
3. ✨ Añadir tests de errores y manejo de excepciones
4. ✨ Implementar tests de rendimiento
5. ✨ Añadir tests de concurrencia para operaciones async
