use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use base_server::{ThreadPool, RouteMapper};

mod routes;
use routes::home::home;
use routes::info::info;

const IP_ADDRESS: &str = "127.0.0.1";
const PORT: &str = "7878";

#[derive(Clone)]
pub struct Server {
    pub mapper: Arc<Mutex<RouteMapper>>,
}

impl Server {
    fn new() -> Arc<Mutex<Server>> {
        Arc::new(Mutex::new(Server {
            mapper: Arc::new(Mutex::new(RouteMapper::new())),
        }))
    }

    fn run(&mut self) {
        self.setup_routes();

        let listener = TcpListener::bind(format!("{}:{}", IP_ADDRESS, PORT))
            .expect("Could not bind to the specified address and port");

        let pool = ThreadPool::new(4);
        let server_arc = Arc::new(Mutex::new(self.clone())); // Create an Arc to share the Server instance
        for stream in listener.incoming() {
            let stream = stream.expect("Connection failed!");
            let server_ref = Arc::clone(&server_arc);
            pool.execute(move || {
                Server::handle_connection(&server_ref, stream);
            });
        }
    }

    fn setup_routes(&mut self) {
        self.mapper.lock().unwrap().add_route("/home", Box::new(home::Home));
        self.mapper.lock().unwrap().add_route("/info", Box::new(info::Info));
        self.mapper.lock().unwrap().print_routes();
    }

    fn handle_connection(server: &Arc<Mutex<Server>>, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).expect("Could not read into buffer");

        // Convert the buffer to a string to parse the request
        let request = String::from_utf8_lossy(&buffer);

        // Split the request by lines
        let lines: Vec<&str> = request.split("\r\n").collect();

        // Get the first line of the request
        if let Some(first_line) = lines.get(0) {
            let parts: Vec<&str> = first_line.split_whitespace().collect();
            if parts.len() >= 2 {
                let method = parts[0]; // GET, POST, etc.
                let path = parts[1]; // /path

                // Handle the route based on the path and method using the Mapper
                Server::handle_route(server, &stream, method, path);
            }
        }
        stream.flush().expect("Failed to flush stream");
    }
    fn respond_with_file(mut stream: &TcpStream, response: &str) {
        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", response.len(), response);
        stream.write(response.as_bytes()).expect("Failed to write response to stream");
    }

    fn handle_route(server: &Arc<Mutex<Server>>, stream: &TcpStream, method: &str, path: &str) {
        // Use the Mapper to handle the route with the corresponding method
        let mapper = server.lock().unwrap().mapper.clone();
        let mapper_locked = mapper.lock().unwrap();
        if let Some(endpoint) = mapper_locked.get_route_handler(path) {
            let response = match method {
                "GET" => endpoint.get(),
                "POST" => endpoint.post(),
                "DELETE" => endpoint.delete(),
                _ => {
                    "HTTP/1.1 405 METHOD NOT ALLOWED"
                }
            };
            Server::respond_with_file(stream, response);
        } else {
            Server::respond_with_file(stream, "<html><body><h1>404 NOT FOUND</h1></body></html>");
        }
    }
}

fn main() {
    let server = Server::new();
    let mut server_ref = server.lock().unwrap();
    server_ref.run();
}
