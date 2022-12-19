use actix::Addr;
use actix_web::{get, web::Data, web::Path, web::Payload, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::lobby::Lobby;
use crate::ws::WsConn;

#[get("/{group_id}")]
pub async fn ws_connection(
    req: HttpRequest,
    stream: Payload,
    group_id: Path<Uuid>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let ws = WsConn::new(group_id.into_inner(), srv.get_ref().clone());

    ws::start(ws, &req, stream)
}
