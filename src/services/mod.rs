use actix_web::{get, HttpResponse, web};
use actix_web::http::header::ContentType;
use error_mapper::{map_to_new_error};
use tokio::sync::broadcast::Sender;
use crate::functions;

pub(super) fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_endpoint)
        .service(stop_execution)
        .service(exec_command);
}

#[get("exec")]
async fn exec_command(sender: web::Data<Sender<()>>) -> HttpResponse {

    //  Send the signal to the processing thread
    if let Err(e) = sender.send(()).map_err(|e| map_to_new_error!(e)) {
        println!("{}", e);
    };

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Signal received, sending it to the processing thread")

}

#[get("/hello")]
async fn hello_endpoint() -> HttpResponse {

    println!("Hello ma dude!");
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Hello ma dude!")

}

#[get("/stop")]
async fn stop_execution() -> HttpResponse {

    println!("Shutting process down");

    //  Created a new thread to shut down the whole process after a 5 seconds delay
    tokio::spawn(functions::stop_process());

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body("Shutting down...")

}