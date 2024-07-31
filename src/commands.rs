use std::fs::File;
use std::io::{Read, Write};
use std::net::{SocketAddr, UdpSocket};

use crate::json_sender::JsonSender;
use crate::locales::{Locale, Locales};
use crate::requisition::Requisition;

pub fn set(requisition: Requisition, socket: &UdpSocket, addr: SocketAddr) {
    
    let mut file = File::open("src/local.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    
    let mut locales: Locales = serde_json::from_str(&data).expect("JSON inválido");
    
    let updated_data = serde_json::to_string_pretty(&locales).expect("Erro ao serializar JSON");
    let mut file = File::create("src/local.json").unwrap();
    file.write_all(updated_data.as_bytes()).unwrap();

    match find_locale_on_json(requisition, &mut locales, "set".to_string()) {
        Ok(response_locale) => {
            let json = JsonSender::make_response_json(response_locale);
            JsonSender::send_json_to_client(json, socket, addr);
        } Err(e) => {

        }
    }
}

pub fn get(requisition: Requisition, socket: &UdpSocket, addr: SocketAddr) {

    let mut file = File::open("src/local.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut locales: Locales = serde_json::from_str(&data).expect("JSON inválido");

    match find_locale_on_json(requisition, &mut locales, "get".to_string()) {  
        Ok(response_locale) => {
            let json = JsonSender::make_response_json(response_locale);
            JsonSender::send_json_to_client(json, socket, addr);
        } Err(e) => {
            
        }
    }
}

#[derive(Debug)]
enum LocaleNotFound {
    LocaleNotFound,
}

fn find_locale_on_json(requisition: Requisition, locales: &mut Locales, mode: String) -> Result<Locale, LocaleNotFound> {
    
    match mode.as_str() { 
        "set" => {
            for locale in &mut locales.locale_list {
                if locale.locate == requisition.locate() {

                    locale.status = match requisition.value().as_ref().unwrap().as_str() {
                        "on" => "on".parse().unwrap(),
                        "off" => "off".parse().unwrap(),
                        _ => "deu ruim".parse().unwrap()
                    };

                    return Ok(locale.clone());
                }
            }
        },
        "get" => {
            return Ok(locales.locale_list.clone()
                .into_iter()
                .find(|l| l.locate == requisition.locate())
                .unwrap());
        },
        _ => {}
    }
    
    return Err(LocaleNotFound::LocaleNotFound);
}