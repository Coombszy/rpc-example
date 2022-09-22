// use job::application_client::ApplicationClient;
// use job::ApplicationGeneric;

use actix_web::App;
use actix_web::{
    HttpServer,
};
use actix_files as fs;
use actix_web_lab::web as web_lab;

mod web;
use web::routes::health;

pub mod job {
    tonic::include_proto!("job");
}

#[tokio::main]
async fn main() ->  std::io::Result<()> {
    // let mut client = ApplicationClient::connect("http://[::1]:50051").await?;

    // let request = tonic::Request::new(ApplicationGeneric {
    //     id: 1,
    //     fullname: "aaa".into(),
    //     cv: "abc".into(),
    //     timestamp: 11111
    // });

    // let response = client.create_application(request).await?;

    // println!("RESPONSE={:?}", response);

    // Ok(())

    let host: String = "0.0.0.0".to_string(); // This should be loaded via config
    let port: u16 = 8080;

    println!("Staring web server! {}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .service(health)
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(web_lab::redirect("/", "/static/index.html"))
    })
    .bind((host,port))?
    .run()
    .await


}