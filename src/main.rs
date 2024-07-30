use std::net::{UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;
use local_ip_address::local_ip;
use crate::requisition::Requisition;

mod commands;
mod requisition;
mod json_sender;
mod locales;

fn main() -> std::io::Result<()> {
    
    match local_ip() {
        Ok(ip) => {
            println!("Running on {}:11000", ip);
            let socket = UdpSocket::bind((ip.to_string(), 11000))?;
            
            loop {
                let mut metadata_buffer = [0; 8];
                let (amt, src) = socket.recv_from(&mut metadata_buffer).unwrap();
                let metadata_length = u64::from_le_bytes(metadata_buffer);

                let mut buffer = vec![0; metadata_length as usize];
                socket.recv_from(&mut buffer).expect("TODO: panic message");
                
                let json = String::from_utf8_lossy(&buffer[..]);
                println!("{}", json);
                let cleaned_json = json.trim_end_matches('\0');
                let requisition = serde_json::from_str::<Requisition>(&cleaned_json).expect("cwe");
                
                resolve_requisition(requisition, &socket);
            }
        },
        Err(e) => eprintln!("Falha ao pegar ip local: {}", e),
    }

    Ok(())
}

fn resolve_requisition(requisition: Requisition, socket: &UdpSocket) {
    return match requisition.command() {
        "set" => commands::set(requisition, socket),
        "get" => commands::get(requisition, socket),
        _ => println!("Método não reconhecido")
    }
}
