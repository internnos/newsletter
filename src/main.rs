use std::net::TcpListener;

use zero2prod::startup::run;
use zero2prod::configurations::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    let configuration = get_configuration().expect("Failed to read config file");
    // configuration.database.connection_string();
    let address = format!("{}:{}", configuration.database.host, configuration.application_port);
    let listener = TcpListener::bind(address);
    run(listener?)?.await
}


