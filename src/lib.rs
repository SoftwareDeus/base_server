use std::{thread, sync::{mpsc, Arc, Mutex}, collections::HashMap};

pub struct ThreadPool{
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool of a given size (number of threads) panic if size is 0
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool{
            threads: workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).expect("Failed to send job to thread")
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>, 
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop { // Use move to capture the receiver
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job();
        });

        Worker {
            id,
            thread,
        }
    }
}

pub trait Endpoint: Send + Sync {
    fn get(&self) -> &str;
    fn post(&self) -> &str;
    fn delete(&self) -> &str;
}

pub struct RouteMapper {
    routes: HashMap<String, Box<dyn Endpoint + Send + Sync>>,
}
impl RouteMapper {
    pub fn new() -> RouteMapper {
        RouteMapper {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, path: &str, endpoint: Box<dyn Endpoint + Send + Sync>) {
        self.routes.insert(path.to_string(), endpoint);
    }

    pub fn print_routes(&self) {
        // Print routes and endpoints
        for (path, _endpoint) in &self.routes {
            println!("{}", path);
        }
    }

    pub fn get_route_handler(&self, path: &str) -> Option<&Box<dyn Endpoint + Send + Sync>> {
        println!("Routes: {}", &self.routes.len());
        if let Some(endpoint) = &self.routes.get(path) {
            println!("Found route: {}", path);
            println!("Endpoint: {:?}", endpoint.get());
            return Some(endpoint);
        }
        None
    }
}