use job::application_client::ApplicationClient;
use job::ApplicationGeneric;

pub mod job {
    tonic::include_proto!("job");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ApplicationClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(ApplicationGeneric {
        id: 1,
        fullname: "aaa".into(),
        cv: "abc".into(),
        timestamp: 11111
    });

    let response = client.create_application(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}