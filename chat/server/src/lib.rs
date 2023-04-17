pub mod message;

use anyhow::Result;

use http_server::*;
use messaging::*;
use slight_http_handler_macro::register_handler;
use slight_http_server_macro::on_server_init;

wit_bindgen_rust::import!("wit/http-server.wit");
wit_bindgen_rust::export!("wit/http-server-export.wit");
wit_bindgen_rust::import!("wit/messaging.wit");
wit_error_rs::impl_error!(http_server::HttpRouterError);
wit_error_rs::impl_error!(MessagingError);

#[on_server_init]
fn main() -> Result<()> {
    let router = Router::new()?;
    let router_with_route = router
        .get("/login", "handle_login")?
        .put("/user/:sub_token/send", "handle_send")?
        .get("/user/:sub_token/receive/latest", "handle_get")?;
    println!("Server is running on port 3000");
    let _ = Server::serve("0.0.0.0:3000", &router_with_route)?;
    Ok(())
}

#[register_handler]
fn handle_login(req: Request) -> Result<Response, HttpError> {
    let sub = Sub::open("my-messaging").unwrap();
    let sub_tok1 = sub.subscribe("global-chat").unwrap();
    Ok(Response {
        headers: Some(req.headers),
        body: Some(sub_tok1.as_bytes().to_vec()),
        status: 200,
    })
}

#[register_handler]
fn handle_send(req: Request) -> Result<Response, HttpError> {
    assert_eq!(req.method, Method::Put);
    let sub_token = req
        .params
        .into_iter()
        .find(|(k, _)| k == "sub_token")
        .unwrap()
        .1;
    let ps = Pub::open("my-messaging").unwrap();
    let message = req.body.unwrap();
    let message = message::Message::new(sub_token, message);
    ps.publish(&serde_json::to_vec(&message).unwrap(), "global-chat")
        .unwrap();

    Ok(Response {
        headers: Some(req.headers),
        body: None,
        status: 200,
    })
}

#[register_handler]
fn handle_get(req: Request) -> Result<Response, HttpError> {
    let sub_token = req
        .params
        .into_iter()
        .find(|(k, _)| k == "sub_token")
        .unwrap()
        .1;
    let sub = Sub::open("my-messaging").unwrap();
    let msg = match sub.receive(&sub_token) {
        Ok(msg) => msg,
        Err(_) => vec![],
    };

    if msg.is_empty() {
        Ok(Response {
            headers: Some(req.headers),
            body: None,
            status: 200,
        })
    } else {
        Ok(Response {
            headers: Some(req.headers),
            body: Some(msg),
            status: 200,
        })
    }
}
