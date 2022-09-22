use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tonic::{transport::Server, Request, Response, Status};

use job::{ApplicationGeneric, ApplicationsGeneric, GenericResponse, GetApplicationRequest, GetApplicationsRequest};
use job::application_server::{Application, ApplicationServer};


pub mod job {
    tonic::include_proto!("job"); // Include the 'job' proto package
}

#[derive(Debug, Default)]
pub struct Service {
    pub applications: Arc<Mutex<HashMap<i32, ApplicationGeneric>>>
}


#[tonic::async_trait]
impl Application for Service {

    // Create new applications
    async fn create_application(&self, request: Request<ApplicationGeneric>) -> Result<Response<GenericResponse>, Status> {
        // Log that an application was received (In a real app this would be done with a proper logger)
        println!("Application create request: {:?}", request);

        // Get application hashmap and inset new application
        let mut apps = self.applications.lock().unwrap();
        let application = request.into_inner();
        apps.insert(application.id, application);

        // Create a reply
        let reply = job::GenericResponse{
            message: "aaa".into(),
        };

        Ok(Response::new(reply))
    }

    // Get application using ID
    async fn get_application(&self, request: Request<GetApplicationRequest>) -> Result<Response<ApplicationGeneric>, Status> {
        // Log that an application was received (In a real app this would be done with a proper logger)
        println!("Application query by id request: {:?}", request);

        let application_id = request.into_inner().id;

        // Get application hashmap and inset new application
        let hashmap = self.applications.lock().unwrap();
        let application = hashmap.get(&application_id);

        match application {
            Some(app) => Ok(Response::new(app.clone())),
            _ => Err(Status::not_found("Could not find application with given id"))
        }
    }

    // Gets all applications
    async fn get_applications(&self, request: Request<GetApplicationsRequest>) -> Result<Response<ApplicationsGeneric>, Status> {
        // Log that an application was received (In a real app this would be done with a proper logger)
        println!("Application query all request: {:?}", request);

        // Get application hashmap and inset new application
        let applications = self.applications.lock().unwrap().values().cloned().collect();

        Ok(Response::new(ApplicationsGeneric { applications: applications }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let svc = Service::default();

    Server::builder()
        .add_service(ApplicationServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}