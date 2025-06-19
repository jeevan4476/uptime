use poem::{
    EndpointExt, Route, Server, get, handler, listener::TcpListener, middleware::Tracing, web::Path,
};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/hello/:name", get(hello)).with(Tracing);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hellow world")
        .run(app)
        .await
}
