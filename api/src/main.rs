use std::sync::{Arc, Mutex};

use poem::{EndpointExt, Route, Server, get, listener::TcpListener, post};

use routes::{
    user::{sign_in, sign_up},
    website::{create_website, get_website},
};
use store::store::Store;
pub mod req_inputs;
pub mod req_outpus;
pub mod routes;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    let s = Arc::new(Mutex::new(Store::new().unwrap()));
    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/user/signup", post(sign_up))
        .at("user/signin", post(sign_in))
        .data(s);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hellow world")
        .run(app)
        .await
}

//either multiple immutable references or one immutable refenrence
