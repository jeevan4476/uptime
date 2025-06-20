use poem::{
    EndpointExt, Route, Server, get, handler,
    listener::TcpListener,
    middleware::Tracing,
    post,
    web::{Json, Path},
};

use crate::{req_inputs::CreateWebsiteInput, req_outpus::CreateWebsiteOutput};

pub mod req_inputs;
pub mod req_outpus;
use store::store::Store;

#[handler]
fn get_website(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

#[handler]
fn create_websites(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let s = Store::;
    let id = s.create_website();
    let reponse = CreateWebsiteOutput { id };

    Json(reponse)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_websites));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hellow world")
        .run(app)
        .await
}
