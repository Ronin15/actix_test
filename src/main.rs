use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(
        "<!DOCTYPE html>

        <head>
            <title>Landing Page</title>
        </head>
        <body>
        <body style='background-color:#2c2c2c;'>
            <div style='text-align:center; padding: 10%; color: #d3d3d3;'>
                <h1>Welcome to Our Landing Page</h1>
                <p>Discover the best content and connect with us. We are here to provide you with the best experience.</p>
                <img src='../resources/img/HammerForge.jpg' alt='Landing Page Image'>
                <a href='#' class='button'>Get Started</a>
            </div>
        </body>
        </html>
",
    )
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
