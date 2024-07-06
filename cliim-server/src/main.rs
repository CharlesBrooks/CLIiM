mod client_manager;
mod server;

fn main() {
    let server = server::Server::new(String::from("cliim-server"), String::from("127.0.0.1:7878"));

    server.start();
}
