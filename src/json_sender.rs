use std::net::{SocketAddr, UdpSocket};
use serde_json::json;
use crate::locales::Locale;

pub struct JsonSender {
  
}

impl JsonSender {
    
    pub fn make_response_json(locale: Locale) -> String {
        let data = json!({
            "locate": locale.locate,
            "status": locale.status
        });

        let json_string = serde_json::to_string(&data).expect("Falha ao serializar JSON");

        return json_string
    }

    pub fn send_json_to_client(json: String, connection: &UdpSocket, addr: SocketAddr) {
        let metadata = json.len() as u64;
        let bytes = json.as_bytes();
        
        connection.connect(addr).expect("TODO: panic message");

        connection.send(&metadata.to_le_bytes()).expect("TODO: panic message");
        connection.send(bytes).unwrap();
    }

    pub fn send_json_error_to_client(connection: &UdpSocket, addr: SocketAddr) {
        let json = json!({
            "error": "Locale nao encontrado"
        }).to_string();
        
        let metadata = json.len() as u64;
        let bytes = json.as_bytes();

        connection.connect(addr).expect("TODO: panic message");

        connection.send(&metadata.to_le_bytes()).expect("TODO: panic message");
        connection.send(bytes).unwrap();
    }
}