use base_server::Endpoint;

pub struct Home;

impl Endpoint for Home {
    fn get(&self) -> &str {
        "<!DOCTYPE html>
        <html>
        <body>
        
        <h1>Home GET</h1>
        
        </body>
        </html>"
    }

    fn post(&self)-> &str {
        "test"
    }

    fn delete(&self)-> &str{
        "test"
    }
}