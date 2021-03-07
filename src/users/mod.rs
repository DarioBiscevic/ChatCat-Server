use std::net::TcpStream;
use std::thread;
use std::sync::{Mutex, Arc};
use std::time::{Duration};

use std::io::prelude::*;

///Struct that holds the connection between a user and the server
pub struct UserConnection{
	pub id:       usize,
	connection:   thread::JoinHandle<()>,
}

impl UserConnection{
	
	pub fn new(mut stream: TcpStream, id: usize, ids_to_remove: Arc<Mutex<Vec<usize>>>) -> Self{
		
		let list_ids = ids_to_remove.clone();
		
		UserConnection{
			id,
			
			connection: thread::spawn(move ||{
				//After the connection function finishes, queue the user 
				handle_connection(stream);
				
				let mut list = list_ids.lock().unwrap();
				list.push(id);
			})
		}
	}
	
}

//TODO: add error handling
fn handle_connection(mut stream: TcpStream){
	
	let mut buffer = [0; 512];
	
	if let Err(error) = stream.read(&mut buffer){
		println!("Error reading data: {}", error);
	}
	
	println!("Data received: {}", std::str::from_utf8(&buffer).unwrap());
	
}
