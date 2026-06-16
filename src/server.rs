//! خادم الويب العربي - سورة البعث المتقدمة

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/// خادم ويب بسيط بالبيان
pub struct BayanServer {
    port: u16,
    routes: Vec<(String, String)>,
}

impl BayanServer {
    pub fn new(port: u16) -> Self {
        BayanServer { port, routes: vec![] }
    }

    /// إضافة مسار
    pub fn route(&mut self, path: &str, response: &str) {
        self.routes.push((path.to_string(), response.to_string()));
    }

    /// تشغيل الخادم
    pub fn start(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port))?;
        println!("🕌 خادم البيان يعمل على: http://localhost:{}", self.port);

        for stream in listener.incoming() {
            let stream = stream?;
            let routes = self.routes.clone();
            thread::spawn(move || handle_client(stream, routes));
        }
        Ok(())
    }
}

fn handle_client(mut stream: TcpStream, routes: Vec<(String, String)>) {
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer);
    let request = String::from_utf8_lossy(&buffer);

    let first_line = request.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    let path = if parts.len() > 1 { parts[1] } else { "/" };

    let response = routes.iter()
        .find(|(p, _)| p == path)
        .map(|(_, r)| r.clone())
        .unwrap_or_else(|| "🕌 أهلًا بك في خادم البيان".to_string());

    let html = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\r\n\
         <html><head><title>خادم البيان</title></head>\
         <body style='text-align:center;font-family:sans-serif;margin-top:100px'>\
         <h1>🕌 {}</h1>\
         <p>مشغل بلغة البيان - الكود قرآن</p>\
         </body></html>",
        response
    );

    let _ = stream.write_all(html.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let server = BayanServer::new(3030);
        assert_eq!(server.port, 3030);
    }

    #[test]
    fn test_add_route() {
        let mut server = BayanServer::new(3030);
        server.route("/", "مرحباً");
        assert_eq!(server.routes.len(), 1);
    }
}
