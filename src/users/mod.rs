use std::net::TcpStream;
use std::thread;

use std::io::prelude::*;

///Struct that holds the connection between a user and the server
pub struct UserConnection{
	pub id:       usize,
	pub is_alive: bool,
	connection:   thread::JoinHandle<()>,
}

impl UserConnection{
	
	pub fn new(mut stream: TcpStream, id: usize) -> Self{
		UserConnection{
			id: id,
			
			is_alive: true,
			
			/* 
			 * Somehow, when the thread finishes, i.e. the connection is terminated,
			 * is_alive must become false. But how?
			 */
			connection: thread::spawn(||{
				handle_connection(stream);
				
			})
		}
	}
	
}

//TODO: add error handling
fn handle_connection(mut stream: TcpStream){
	/* CONNECTION TEST SNIPPET */
	
	println!("'handle_connection' activated; reading in");
	
	let mut buffer = [0; 512];
	
	if let Err(error) = stream.read(&mut buffer){
		println!("Error reading data: {}", error);
	}
	
	println!("Data received: {}", std::str::from_utf8(&buffer).unwrap());
	
}
