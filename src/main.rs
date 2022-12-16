use actix::Actor;
use actix_web::web;

mod lobby;
mod messages;
mod routes;
mod ws;

use lobby::Lobby;
use routes::ws_connection;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_server = web::Data::new(Lobby::default().start()); // create and spin up a lobby

    HttpServer::new(move || {
        App::new()
            .app_data(chat_server.clone()) // register the lobby
            .service(ws_connection) // register our route. rename with "as" import or naming conflict
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
