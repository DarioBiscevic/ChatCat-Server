use std::net::TcpListener;
//use std::net::TcpStream;
use std::io::prelude::*;
use std::sync::{Mutex, Arc};
use std::thread;

mod users;
use users::UserConnection;

fn main() {
    println!("Chat-cat server");
    
    if let Ok(listener) = TcpListener::bind("192.168.1.5:7878"){
        
        //Vector that contains all the user structs
        let mut all_connections: Arc<Mutex<Vec<UserConnection>>> = Arc::new(Mutex::new(Vec::new()));
        
        let max_connections = 10; //Maximum allowed connections
        let mut user_counter = 0; //User id counter
        
        let mut remove_queue = Arc::new(Mutex::new(Vec::new()));
        
        /*
         * Thread that constantly checks for users that must be removed and
         * removes them.
         */
        let connections = all_connections.clone();
        let queue = remove_queue.clone();
        thread::spawn(move ||{
            loop{
                connections
                    .lock()
                    .unwrap()
                    .retain(|user|{
                        if queue.lock().unwrap().contains(&user.id){
                            println!("Removing user {}", user.id);
                        }
                        !queue.lock().unwrap().contains(&user.id)
                    });
                queue.lock().unwrap().clear();
            }
        });
        
        for stream in listener.incoming(){
            let mut stream = stream.unwrap(); //To improve
            
            /*
             * Every time a user connects to the server, if there is room left, its stream is saved in 
             * an object. This allows us to loop through all the connections in
             * order to then broadcast the messages.
             */
            if all_connections.lock().unwrap().len() < max_connections{
             
                println!("Adding user: {:?}", stream.peer_addr());

                all_connections.lock().unwrap().push(
                    UserConnection::new(stream, user_counter, Arc::clone(&remove_queue))
                );
                
                user_counter += 1;

                //println!("User added successfully!");
                println!("Number of users: {}", all_connections.lock().unwrap().len());
                //println!("--------------------");
                
            }else{
                
                let negative_response =
                    format!("Connection refused due to server limits; limit: {}, users: {}",
                        max_connections,
                        all_connections.lock().unwrap().len()
                );
                
                if let Err(error) = stream.write(negative_response.as_bytes()){
                    println!("Error in writing: {}", error);
                }
            }
        }
        
    }else{
        println!("Error in server startup");
    }
}
