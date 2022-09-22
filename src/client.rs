use std::sync::{Arc, Mutex};

use job::application_client::ApplicationClient;
use job::ApplicationGeneric;

use actix_web::App;
use actix_web::{
    HttpServer,
};
use actix_files as fs;
use actix_web_lab::web as web_lab;

mod web;
use tonic::transport::Channel;
use web::routes::health;
use web::routes::create_application;
use web::routes::get_applications;
use web::routes::get_cv;

pub mod job {
    tonic::include_proto!("job");
}


pub struct AppState {
    pub rpc_client: Arc<Mutex<ApplicationClient<Channel>>>
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    
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
    let port: u16 = 8080; // This should be loaded via config

    // This backend ip:port should be loaded via config, Additionally, should have some actually error handling if this connection cannot be established
    let client = Arc::new(Mutex::new(ApplicationClient::connect("http://[::1]:50051").await.unwrap()));

    println!("Staring web server! {}:{}", host, port);

    HttpServer::new(move || {
        App::new()
        .app_data(
            actix_web::web::Data::new(AppState {
                rpc_client: client.clone()
            })
        )
        .service(fs::Files::new("/static", "./static").show_files_listing())
        .service(web_lab::redirect("/", "/static/index.html"))
        .service(health)
        .service(create_application)
        .service(get_applications)
        .service(get_cv)
        
    })
    .bind((host,port))?
    .run()
    .await

}