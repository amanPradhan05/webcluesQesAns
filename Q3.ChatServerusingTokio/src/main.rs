use std::collections::HashMap;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio::time::{self, Duration};

type Clients = Mutex<HashMap<String, mpsc::UnboundedSender<String>>>;

#[tokio::main]
async fn main() {
  
    let clients: Clients = Mutex::new(HashMap::new());

   
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server started at 127.0.0.1:8080");

    
    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        let clients = clients.clone();

        
        tokio::spawn(async move {
           
            let (tx, mut rx) = mpsc::unbounded_channel::<String>();

          
            {
                let mut map = clients.lock().await;
                map.insert(addr.to_string(), tx);
            }

            
            let mut buf_reader = BufReader::new(socket);
            loop {
                let mut buf = String::new();
                match buf_reader.read_line(&mut buf).await {
                    Ok(0) => break,
                    Ok(_) => {
                        
                        buf.pop();
                        let mut map = clients.lock().await;

                        for (client_addr, client_tx) in map.iter_mut() {
                            if client_addr != &addr.to_string() {
                                if let Err(_) = client_tx.send(buf.clone()) {
                                 
                                    map.remove(client_addr);
                                }
                            }
                        }
                    }
                    Err(_) => {
                       
                        continue;
                    }
                }
            }

            {
                let mut map = clients.lock().await;
                map.remove(&addr.to_string());
            }
        });
    }
}
