use std::sync::{Arc, Mutex};

use tonic::codegen::http::request;
use tonic::{transport::Server, Request, Response, Status};

use job::{ApplicationRequest, GenericResponse};
use job::application_server::{Application, ApplicationServer};


pub mod job {
    tonic::include_proto!("job"); // Include the 'job' proto package
}

#[derive(Debug, Default)]
pub struct Service {
    pub applications: Arc<Mutex<Vec<ApplicationRequest>>>
}


#[tonic::async_trait]
impl Application for Service {
    async fn create_application(&self, request: Request<ApplicationRequest>) -> Result<Response<GenericResponse>, Status> {
        // Log that an application was received (In a real app this would be done with a proper logger)
        println!("Application request: {:?}", request);

        let mut apps = self.applications.lock().unwrap();

        apps.push(request.into_inner());

        // Create a reply
        let reply = job::GenericResponse{
            message: "aaa".into(),
            timestamp: 2556
        };

        println!("{:?}", apps);

        Ok(Response::new(reply))

    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let svc = Service::default();

    Server::builder()
        // .add_service(GreeterServer::new(svc))
        .add_service(ApplicationServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}