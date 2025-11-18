use super::types::*;
use anyhow::Result;
use serde_json::Value;
use std::io::{self, BufRead, Write};

pub struct McpProtocolHandler {
    stdin: io::Stdin,
    stdout: io::Stdout,
}

impl McpProtocolHandler {
    pub fn new() -> Self {
        McpProtocolHandler {
            stdin: io::stdin(),
            stdout: io::stdout(),
        }
    }

    /// Read a JSON-RPC request from stdin
    pub fn read_request(&self) -> Result<Request> {
        let stdin = self.stdin.lock();
        let mut lines = stdin.lines();

        if let Some(line) = lines.next() {
            let line = line?;
            let request: Request = serde_json::from_str(&line)?;
            log::debug!("Received request: method={} id={:?}", request.method, request.id);
            Ok(request)
        } else {
            Err(anyhow::anyhow!("No input from stdin"))
        }
    }

    /// Write a JSON-RPC response to stdout
    pub fn write_response(&mut self, response: Response) -> Result<()> {
        let json = serde_json::to_string(&response)?;
        writeln!(self.stdout, "{}", json)?;
        self.stdout.flush()?;
        log::debug!("Sent response: id={:?}", response.id);
        Ok(())
    }

    /// Send success response
    pub fn send_success(&mut self, id: Option<Value>, result: Value) -> Result<()> {
        self.write_response(Response::success(id, result))
    }

    /// Send error response
    pub fn send_error(&mut self, id: Option<Value>, code: i32, message: String) -> Result<()> {
        self.write_response(Response::error(id, code, message))
    }
}

impl Default for McpProtocolHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_response_serialization() {
        let response = Response::success(Some(json!(1)), json!({"data": "test"}));
        let serialized = serde_json::to_string(&response).unwrap();

        assert!(serialized.contains("\"jsonrpc\":\"2.0\""));
        assert!(serialized.contains("\"id\":1"));
        assert!(serialized.contains("\"result\""));
    }
}
