use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::net::{UdpSocket};
use std::path::Path;
use crate::json_sender::JsonSender;
use crate::locales::Locales;
use crate::requisition::Requisition;

pub fn set(requisition: Requisition, mut socket: &UdpSocket) {
    
    let mut file = File::open("src/local.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    
    let mut locales: Locales = serde_json::from_str(&data).expect("JSON inválido");
    
    for locale in &mut locales.locales {
        if locale.locate == requisition.locate() {

            locale.status = match requisition.value().as_ref().unwrap().as_str() {
                "on" => "on".parse().unwrap(),
                "off" => "off".parse().unwrap(),
                _ => "deu ruim".parse().unwrap()
            };
                
            break;
        }
    }
    
    let updated_data = serde_json::to_string_pretty(&locales).expect("Erro ao serializar JSON");

    file.write_all(updated_data.as_bytes()).unwrap();
    
    println!("{}", updated_data);
}

pub fn get(requisition: Requisition, mut socket: &UdpSocket) {
    
}