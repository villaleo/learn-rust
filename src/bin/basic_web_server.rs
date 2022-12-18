use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

const LOCALHOST: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running app on {}:{}", LOCALHOST, PORT);
    HttpServer::new(|| {
        App::new()
            // service() for the handlers using routing macros
            .service(hello)
            .service(echo)
            // .route() for manually routed handlers
            .route("/hey", web::get().to(say_hey))
    })
        .bind((LOCALHOST, PORT))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    println!("Endpoint / hit!");
    HttpResponse::Ok().body("Hello world!\n")
}

#[post("/echo")]
async fn echo(name: String) -> impl Responder {
    println!("Endpoint /echo hit!");
    let message = format!("Hello, {}!\n", name);
    HttpResponse::Ok().body(message)
}

async fn say_hey() -> impl Responder {
    println!("Endpoint /hey hit!");
    HttpResponse::Ok().body("Hey!\n")
}
