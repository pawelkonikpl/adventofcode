use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

// ============================================================================
// JSON-RPC 2.0 Structures
// ============================================================================

#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

// ============================================================================
// MCP Protocol Structures
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct ServerInfo {
    name: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerCapabilities {
    tools: Option<ToolsCapability>,
    resources: Option<ResourcesCapability>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ToolsCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    list_changed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResourcesCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    list_changed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tool {
    name: String,
    description: String,
    #[serde(rename = "inputSchema")]
    input_schema: Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct Resource {
    uri: String,
    name: String,
    description: Option<String>,
    #[serde(rename = "mimeType")]
    mime_type: Option<String>,
}

// ============================================================================
// MCP Server Implementation
// ============================================================================

struct McpServer {
    tools: HashMap<String, Tool>,
    resources: HashMap<String, Resource>,
}

impl McpServer {
    fn new() -> Self {
        let mut server = Self {
            tools: HashMap::new(),
            resources: HashMap::new(),
        };
        server.register_default_tools();
        server.register_default_resources();
        server
    }

    fn register_default_tools(&mut self) {
        // Przykładowe narzędzie: kalkulator
        self.tools.insert(
            "calculator".to_string(),
            Tool {
                name: "calculator".to_string(),
                description: "Performs basic arithmetic operations (add, subtract, multiply, divide)".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "operation": {
                            "type": "string",
                            "enum": ["add", "subtract", "multiply", "divide"],
                            "description": "The arithmetic operation to perform"
                        },
                        "a": {
                            "type": "number",
                            "description": "First number"
                        },
                        "b": {
                            "type": "number",
                            "description": "Second number"
                        }
                    },
                    "required": ["operation", "a", "b"]
                }),
            },
        );

        // Przykładowe narzędzie: generator tekstu
        self.tools.insert(
            "text_generator".to_string(),
            Tool {
                name: "text_generator".to_string(),
                description: "Generates sample text with specified parameters".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "template": {
                            "type": "string",
                            "description": "Template for text generation (use {name}, {date}, etc.)"
                        },
                        "variables": {
                            "type": "object",
                            "description": "Variables to replace in template"
                        }
                    },
                    "required": ["template"]
                }),
            },
        );

        // Przykładowe narzędzie: konwerter jednostek
        self.tools.insert(
            "unit_converter".to_string(),
            Tool {
                name: "unit_converter".to_string(),
                description: "Converts values between different units (temperature, length, weight)".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "value": {
                            "type": "number",
                            "description": "Value to convert"
                        },
                        "from_unit": {
                            "type": "string",
                            "enum": ["celsius", "fahrenheit", "kelvin", "meters", "feet", "kilometers", "miles", "kg", "lbs"],
                            "description": "Unit to convert from"
                        },
                        "to_unit": {
                            "type": "string",
                            "enum": ["celsius", "fahrenheit", "kelvin", "meters", "feet", "kilometers", "miles", "kg", "lbs"],
                            "description": "Unit to convert to"
                        }
                    },
                    "required": ["value", "from_unit", "to_unit"]
                }),
            },
        );
    }

    fn register_default_resources(&mut self) {
        self.resources.insert(
            "file://server-info".to_string(),
            Resource {
                uri: "file://server-info".to_string(),
                name: "Server Information".to_string(),
                description: Some("Information about this MCP server".to_string()),
                mime_type: Some("text/plain".to_string()),
            },
        );
    }

    async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let id = request.id.clone();

        match request.method.as_str() {
            "initialize" => self.handle_initialize(id),
            "tools/list" => self.handle_tools_list(id),
            "tools/call" => self.handle_tools_call(id, request.params),
            "resources/list" => self.handle_resources_list(id),
            "resources/read" => self.handle_resources_read(id, request.params),
            _ => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: format!("Method not found: {}", request.method),
                    data: None,
                }),
            },
        }
    }

    fn handle_initialize(&self, id: Option<Value>) -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "protocolVersion": "2024-11-05",
                "serverInfo": {
                    "name": "Rust MCP Server",
                    "version": "0.1.0"
                },
                "capabilities": {
                    "tools": {
                        "listChanged": true
                    },
                    "resources": {
                        "listChanged": true
                    }
                }
            })),
            error: None,
        }
    }

    fn handle_tools_list(&self, id: Option<Value>) -> JsonRpcResponse {
        let tools: Vec<&Tool> = self.tools.values().collect();
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({ "tools": tools })),
            error: None,
        }
    }

    fn handle_tools_call(&self, id: Option<Value>, params: Option<Value>) -> JsonRpcResponse {
        let params = match params {
            Some(p) => p,
            None => {
                return JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: "Invalid params".to_string(),
                        data: None,
                    }),
                }
            }
        };

        let tool_name = match params.get("name").and_then(|v| v.as_str()) {
            Some(name) => name,
            None => {
                return JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: "Missing tool name".to_string(),
                        data: None,
                    }),
                }
            }
        };

        let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

        // Wykonaj odpowiednie narzędzie
        let result = match tool_name {
            "calculator" => self.execute_calculator(&arguments),
            "text_generator" => self.execute_text_generator(&arguments),
            "unit_converter" => self.execute_unit_converter(&arguments),
            _ => Err(format!("Unknown tool: {}", tool_name)),
        };

        match result {
            Ok(content) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(json!({
                    "content": [
                        {
                            "type": "text",
                            "text": content
                        }
                    ]
                })),
                error: None,
            },
            Err(err) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32603,
                    message: err,
                    data: None,
                }),
            },
        }
    }

    fn execute_calculator(&self, args: &Value) -> Result<String, String> {
        let operation = args
            .get("operation")
            .and_then(|v| v.as_str())
            .ok_or("Missing operation")?;
        let a = args.get("a").and_then(|v| v.as_f64()).ok_or("Missing a")?;
        let b = args.get("b").and_then(|v| v.as_f64()).ok_or("Missing b")?;

        let result = match operation {
            "add" => a + b,
            "subtract" => a - b,
            "multiply" => a * b,
            "divide" => {
                if b == 0.0 {
                    return Err("Division by zero".to_string());
                }
                a / b
            }
            _ => return Err(format!("Unknown operation: {}", operation)),
        };

        Ok(format!("{} {} {} = {}", a, operation, b, result))
    }

    fn execute_text_generator(&self, args: &Value) -> Result<String, String> {
        let template = args
            .get("template")
            .and_then(|v| v.as_str())
            .ok_or("Missing template")?;
        let variables = args.get("variables").cloned().unwrap_or(json!({}));

        let mut result = template.to_string();

        if let Some(vars) = variables.as_object() {
            for (key, value) in vars {
                let placeholder = format!("{{{}}}", key);
                let replacement = match value {
                    Value::String(s) => s.clone(),
                    _ => value.to_string(),
                };
                result = result.replace(&placeholder, &replacement);
            }
        }

        Ok(result)
    }

    fn execute_unit_converter(&self, args: &Value) -> Result<String, String> {
        let value = args
            .get("value")
            .and_then(|v| v.as_f64())
            .ok_or("Missing value")?;
        let from_unit = args
            .get("from_unit")
            .and_then(|v| v.as_str())
            .ok_or("Missing from_unit")?;
        let to_unit = args
            .get("to_unit")
            .and_then(|v| v.as_str())
            .ok_or("Missing to_unit")?;

        let result = match (from_unit, to_unit) {
            // Temperature conversions
            ("celsius", "fahrenheit") => value * 9.0 / 5.0 + 32.0,
            ("fahrenheit", "celsius") => (value - 32.0) * 5.0 / 9.0,
            ("celsius", "kelvin") => value + 273.15,
            ("kelvin", "celsius") => value - 273.15,
            ("fahrenheit", "kelvin") => (value - 32.0) * 5.0 / 9.0 + 273.15,
            ("kelvin", "fahrenheit") => (value - 273.15) * 9.0 / 5.0 + 32.0,

            // Length conversions
            ("meters", "feet") => value * 3.28084,
            ("feet", "meters") => value / 3.28084,
            ("kilometers", "miles") => value * 0.621371,
            ("miles", "kilometers") => value / 0.621371,

            // Weight conversions
            ("kg", "lbs") => value * 2.20462,
            ("lbs", "kg") => value / 2.20462,

            // Same unit
            _ if from_unit == to_unit => value,

            _ => {
                return Err(format!(
                    "Conversion from {} to {} not supported",
                    from_unit, to_unit
                ))
            }
        };

        Ok(format!("{} {} = {:.2} {}", value, from_unit, result, to_unit))
    }

    fn handle_resources_list(&self, id: Option<Value>) -> JsonRpcResponse {
        let resources: Vec<&Resource> = self.resources.values().collect();
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({ "resources": resources })),
            error: None,
        }
    }

    fn handle_resources_read(&self, id: Option<Value>, params: Option<Value>) -> JsonRpcResponse {
        let params = match params {
            Some(p) => p,
            None => {
                return JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: "Invalid params".to_string(),
                        data: None,
                    }),
                }
            }
        };

        let uri = match params.get("uri").and_then(|v| v.as_str()) {
            Some(u) => u,
            None => {
                return JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: "Missing uri".to_string(),
                        data: None,
                    }),
                }
            }
        };

        // Zwróć treść zasobu
        let content = match uri {
            "file://server-info" => {
                "Rust MCP Server v0.1.0\n\nThis server provides:\n- Calculator tool\n- Text generator tool\n- Unit converter tool".to_string()
            }
            _ => {
                return JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: format!("Resource not found: {}", uri),
                        data: None,
                    }),
                }
            }
        };

        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "contents": [
                    {
                        "uri": uri,
                        "mimeType": "text/plain",
                        "text": content
                    }
                ]
            })),
            error: None,
        }
    }
}

// ============================================================================
// Main Server Loop
// ============================================================================

#[tokio::main]
async fn main() -> Result<()> {
    let server = McpServer::new();
    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    eprintln!("MCP Server started. Waiting for requests...");

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;

        if bytes_read == 0 {
            break; // EOF
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        eprintln!("Received: {}", trimmed);

        match serde_json::from_str::<JsonRpcRequest>(trimmed) {
            Ok(request) => {
                let response = server.handle_request(request).await;
                let response_json = serde_json::to_string(&response)?;
                stdout.write_all(response_json.as_bytes()).await?;
                stdout.write_all(b"\n").await?;
                stdout.flush().await?;
                eprintln!("Sent: {}", response_json);
            }
            Err(e) => {
                eprintln!("Failed to parse request: {}", e);
                let error_response = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: None,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32700,
                        message: "Parse error".to_string(),
                        data: Some(json!(e.to_string())),
                    }),
                };
                let response_json = serde_json::to_string(&error_response)?;
                stdout.write_all(response_json.as_bytes()).await?;
                stdout.write_all(b"\n").await?;
                stdout.flush().await?;
            }
        }
    }

    eprintln!("MCP Server stopped.");
    Ok(())
}
