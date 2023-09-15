use actix_web::{App, HttpServer, web};
use error_mapper::{map_to_new_error, TheResult};

mod services;
mod functions;

#[tokio::main]
async fn main() -> TheResult<()> {

    let (sender, receiver) = tokio::sync::broadcast::channel::<()>(10);

    //  If I run the task by awaiting it, it'll await until it's finished and never start the API
    //  Need to move it into an async lambda function and await it inside the spawn call
    tokio::spawn(async move {
        if let Err(e) = functions::command_receiver(receiver).await {
            println!("{}", e);
        };
    });

    HttpServer::new(move || {
        let sender_clone = sender.clone();
            App::new().service(
                web::scope("/general")
                    .configure(services::services)
            ).app_data(web::Data::new(sender_clone))
        })
        .workers(8)
            .bind(("127.0.0.1", 8069))
            .map_err(|e| map_to_new_error!(e))?
            .run()
            .await
            .map_err(|e| map_to_new_error!(e))

}
