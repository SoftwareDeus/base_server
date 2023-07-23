use base_server::Endpoint;

pub struct Info;

impl Endpoint for Info {
    fn get(&self) -> &str {
        "<!DOCTYPE html>
        <html>
        <body>
        
        <h1>Info GET</h1>
        
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