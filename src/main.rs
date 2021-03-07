use std::net::TcpListener;
//use std::net::TcpStream;
use std::io::prelude::*;
//use std::thread;

mod users;
use users::UserConnection;

fn main() {
    println!("Chat-cat server");
    
    if let Ok(listener) = TcpListener::bind("192.168.1.5:7878"){
        
        let mut all_connections: Vec<UserConnection> = Vec::new();
        let max_connections = 1;
        let mut user_counter = 0;
        
        for stream in listener.incoming(){
            let mut stream = stream.unwrap(); //To improve
            
            /*
             * Every time a user connects to the server, if there is room left, its stream is saved in 
             * an object. This allows us to loop through all the connections in
             * order to then broadcast the messages.
             */
            if all_connections.len() < max_connections{
             
                println!("Adding user: {:?}", stream.peer_addr());

                all_connections.push(UserConnection::new(stream, user_counter));
                user_counter += 1;

                println!("User added successfully!");
                println!("Number of users: {}", all_connections.len());
                println!("--------------------");

                all_connections = all_connections.into_iter().filter(|user| user.is_alive).collect();
                
            }else{
                
                let negative_response = format!("Connection refused due to server limits; limit: {}, users: {}", max_connections, all_connections.len());
                
                if let Err(error) = stream.write(negative_response.as_bytes()){
                    println!("Error in writing: {}", error);
                }
            }
        }
        
    }else{
        println!("Error in server startup");
    }
}
