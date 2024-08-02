use std::net::{SocketAddr, UdpSocket};
use serde_json::{json, Value};
use crate::locales::{Locale, Locales};

pub struct JsonSender {
  
}

pub enum LocaleEnum {
    Locale(Locale),
    Locales(Locales),
}

impl JsonSender {
    
    pub fn make_response_json(locale_enum: LocaleEnum) -> String {
        let data = match locale_enum { 
            LocaleEnum::Locale(locale) => {
                json!({
                    "locate": locale.locate,
                    "status": locale.status
                })
            },
            LocaleEnum::Locales(locales) => {
                json!({
                    "locale_list": locales.locale_list
                })
            }
        };
        
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