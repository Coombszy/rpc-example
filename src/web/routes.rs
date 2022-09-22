use std::{
    collections::HashMap,
    io::Write,
    str::from_utf8,
    time::{SystemTime, UNIX_EPOCH},
};

use actix_multipart::Multipart;
use actix_web::{
    get, post,
    web::{self, Data},
    Error, HttpResponse,
};

use futures_util::{stream::StreamExt as _, TryStreamExt};

use tonic::Request;

use crate::{
    job::{ApplicationGeneric, ApplicationsGeneric},
    web::structs::Application,
    AppState,
};

use uuid::Uuid;

// Check that the backend gRPC server is alive and healthy
#[get("/health")]
pub async fn health(data: Data<AppState>) -> Result<HttpResponse, Error> {
    let mut rpc = data.rpc_client.lock().unwrap();
    let response = rpc.health_check(Request::new(crate::job::Empty {})).await;
    drop(rpc);

    if response.is_ok() {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(actix_web::error::ErrorInternalServerError(
            "Failed to reach gRPC server",
        ))
    }
}

// Create an application
#[post("/application")]
pub async fn create_application(
    data: Data<AppState>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let mut map: HashMap<String, Vec<u8>> = HashMap::new();

    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        let mut vec_stream = vec![];
        while let Some(chunk) = field.next().await {
            vec_stream.write_all(&chunk.unwrap()).unwrap();
        }

        map.insert(field.name().to_string(), vec_stream);
    }

    // Create gRPC request
    let request = Request::new(crate::job::ApplicationGeneric {
        id: Uuid::new_v4().to_string(),
        cv: map.get("cv").unwrap().to_vec(),
        fullname: from_utf8(map.get("fullname").unwrap()).unwrap().to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    });

    let mut rpc = data.rpc_client.lock().unwrap();
    let response = rpc.create_application(request).await;
    drop(rpc);

    if response.is_ok() {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(actix_web::error::ErrorInternalServerError(
            "Failed to create new application",
        ))
    }
}

// Get all applications
#[get("/applications")]
pub async fn get_applications(data: Data<AppState>) -> Result<HttpResponse, Error> {
    // Create gRPC request
    let request = Request::new(crate::job::GetApplicationsRequest {});

    let mut rpc = data.rpc_client.lock().unwrap();
    let response = rpc.get_applications(request).await;
    drop(rpc);

    if response.is_ok() {
        // Get all applications and convert them to frontend friendly json
        let apps: ApplicationsGeneric = response.unwrap().into_inner();

        // Applications suitable for serde serialization
        let mut fe_apps: Vec<Application> = Vec::new();
        for a in apps.applications {
            fe_apps.push(Application {
                fullname: a.fullname,
                creation_time: a.timestamp.to_string(),
                cv_link: "aaa".to_string(),
            })
        }

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(fe_apps))
    } else {
        Err(actix_web::error::ErrorInternalServerError(
            "Failed to create new application",
        ))
    }
}

// Get application cv pdf
#[get("/cv/{id}")]
pub async fn get_cv(data: Data<AppState>, path: web::Path<String>) -> Result<HttpResponse, Error> {
    // Create gRPC request
    let request = Request::new(crate::job::GetApplicationRequest {
        id: path.into_inner(),
    });

    let mut rpc = data.rpc_client.lock().unwrap();
    let response = rpc.get_application(request).await;
    drop(rpc);

    if response.is_ok() {
        // Get application cv bytes and return as pdf!
        let app: ApplicationGeneric = response.unwrap().into_inner();

        Ok(HttpResponse::Ok()
            .content_type("application/pdf")
            .body(app.cv))
    } else {
        Err(actix_web::error::ErrorInternalServerError(
            "Failed to create new application",
        ))
    }
}
