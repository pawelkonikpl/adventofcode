# MCP Server - Model Context Protocol Server w Rust

Serwer MCP (Model Context Protocol) napisany w Rust, który umożliwia integrację z Claude Desktop i innymi klientami wspierającymi MCP.

A Rust-based MCP (Model Context Protocol) server that enables integration with Claude Desktop and other MCP-compatible clients.

## Funkcje / Features

- ✅ Pełna implementacja protokołu MCP 2024-11-05
- ✅ Komunikacja przez JSON-RPC 2.0
- ✅ Asynchroniczny runtime (Tokio)
- ✅ 3 wbudowane narzędzia (tools)
- ✅ Obsługa zasobów (resources)

---

- ✅ Full implementation of MCP protocol 2024-11-05
- ✅ JSON-RPC 2.0 communication
- ✅ Async runtime (Tokio)
- ✅ 3 built-in tools
- ✅ Resource support

## Dostępne narzędzia / Available Tools

### 1. Calculator (Kalkulator)
Wykonuje podstawowe operacje arytmetyczne.
Performs basic arithmetic operations.

**Parametry / Parameters:**
- `operation`: "add", "subtract", "multiply", "divide"
- `a`: pierwsza liczba / first number
- `b`: druga liczba / second number

**Przykład / Example:**
```json
{
  "operation": "add",
  "a": 5,
  "b": 3
}
```

### 2. Text Generator (Generator tekstu)
Generuje tekst na podstawie szablonu z możliwością podstawienia zmiennych.
Generates text from a template with variable substitution.

**Parametry / Parameters:**
- `template`: szablon tekstowy z placeholderami {nazwa} / text template with {name} placeholders
- `variables`: obiekt ze zmiennymi do podstawienia / object with variables to substitute

**Przykład / Example:**
```json
{
  "template": "Hello {name}, today is {date}!",
  "variables": {
    "name": "Alice",
    "date": "2024-11-06"
  }
}
```

### 3. Unit Converter (Konwerter jednostek)
Konwertuje wartości między różnymi jednostkami miary.
Converts values between different units of measurement.

**Obsługiwane jednostki / Supported units:**
- Temperatura / Temperature: celsius, fahrenheit, kelvin
- Długość / Length: meters, feet, kilometers, miles
- Waga / Weight: kg, lbs

**Parametry / Parameters:**
- `value`: wartość do konwersji / value to convert
- `from_unit`: jednostka źródłowa / source unit
- `to_unit`: jednostka docelowa / target unit

**Przykład / Example:**
```json
{
  "value": 100,
  "from_unit": "celsius",
  "to_unit": "fahrenheit"
}
```

## Instalacja / Installation

### Wymagania / Requirements

- Rust 1.70+ (zalecane najnowsza wersja / latest recommended)
- Cargo
- Claude Desktop (opcjonalnie / optional)

### Kompilacja / Build

```bash
cd mcp_server
cargo build --release
```

Skompilowany plik binarny znajdzie się w:
The compiled binary will be located at:
```
target/release/mcp_server
```

## Użycie / Usage

### Uruchomienie ręczne / Manual Run

Serwer komunikuje się przez stdin/stdout używając JSON-RPC 2.0:
The server communicates via stdin/stdout using JSON-RPC 2.0:

```bash
cargo run
```

### Integracja z Claude Desktop

1. **Znajdź plik konfiguracyjny Claude Desktop:**
   **Locate Claude Desktop config file:**

   - macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
   - Windows: `%APPDATA%\Claude\claude_desktop_config.json`
   - Linux: `~/.config/Claude/claude_desktop_config.json`

2. **Dodaj konfigurację MCP server:**
   **Add MCP server configuration:**

```json
{
  "mcpServers": {
    "rust-mcp-server": {
      "command": "/pełna/ścieżka/do/mcp_server/target/release/mcp_server",
      "args": []
    }
  }
}
```

**Lub dla wersji developerskiej / Or for development version:**

```json
{
  "mcpServers": {
    "rust-mcp-server": {
      "command": "cargo",
      "args": ["run", "--manifest-path", "/pełna/ścieżka/do/mcp_server/Cargo.toml"]
    }
  }
}
```

3. **Uruchom ponownie Claude Desktop**
   **Restart Claude Desktop**

4. **Sprawdź dostępność narzędzi:**
   **Check tool availability:**

   W Claude Desktop powinieneś zobaczyć dostępne narzędzia z tego serwera.
   In Claude Desktop, you should see available tools from this server.

## Testowanie / Testing

### Test ręczny / Manual Test

Możesz przetestować serwer ręcznie, wysyłając zapytania JSON-RPC:
You can test the server manually by sending JSON-RPC requests:

```bash
cargo run
```

Następnie wpisz (każda linia to osobne żądanie):
Then type (each line is a separate request):

```json
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}
{"jsonrpc":"2.0","id":2,"method":"tools/list"}
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"calculator","arguments":{"operation":"add","a":5,"b":3}}}
```

### Test z użyciem echo i pipe

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | cargo run
```

## Struktura projektu / Project Structure

```
mcp_server/
├── Cargo.toml          # Konfiguracja projektu i zależności
├── README.md           # Ta dokumentacja
└── src/
    └── main.rs         # Główna implementacja serwera
```

## Protokół MCP / MCP Protocol

Serwer implementuje następujące metody JSON-RPC:
The server implements the following JSON-RPC methods:

- `initialize` - Inicjalizacja połączenia / Initialize connection
- `tools/list` - Lista dostępnych narzędzi / List available tools
- `tools/call` - Wywołanie narzędzia / Call a tool
- `resources/list` - Lista dostępnych zasobów / List available resources
- `resources/read` - Odczyt zasobu / Read a resource

## Rozszerzanie serwera / Extending the Server

### Dodawanie nowego narzędzia / Adding a New Tool

1. Dodaj definicję narzędzia w `register_default_tools()`:

```rust
self.tools.insert(
    "my_tool".to_string(),
    Tool {
        name: "my_tool".to_string(),
        description: "My tool description".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "param": {
                    "type": "string",
                    "description": "Parameter description"
                }
            },
            "required": ["param"]
        }),
    },
);
```

2. Dodaj case w `handle_tools_call()`:

```rust
"my_tool" => self.execute_my_tool(&arguments),
```

3. Zaimplementuj funkcję wykonawczą:

```rust
fn execute_my_tool(&self, args: &Value) -> Result<String, String> {
    let param = args
        .get("param")
        .and_then(|v| v.as_str())
        .ok_or("Missing param")?;

    // Twoja logika
    Ok(format!("Result: {}", param))
}
```

## Debugowanie / Debugging

Serwer wysyła logi diagnostyczne do stderr (eprintln!), które możesz zobaczyć w logach Claude Desktop lub podczas ręcznego uruchomienia.

The server sends diagnostic logs to stderr (eprintln!), which you can see in Claude Desktop logs or during manual execution.

## Dokumentacja MCP / MCP Documentation

- [Model Context Protocol Specification](https://spec.modelcontextprotocol.io/)
- [MCP Documentation](https://modelcontextprotocol.io/)

## Zależności / Dependencies

- `tokio` - Asynchroniczny runtime / Async runtime
- `serde` - Serializacja danych / Data serialization
- `serde_json` - JSON parsing
- `anyhow` - Obsługa błędów / Error handling
- `async-trait` - Async traits

## Licencja / License

MIT

## Kontakt / Contact

W razie problemów, sprawdź logi serwera lub dokumentację MCP.
For issues, check server logs or MCP documentation.
