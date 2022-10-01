# Job Search Application
This is a quick application to demonstrate an implementation of gRPC and Actix Web server. This was created as a skills test for a past job application.

This application allows you to upload a persons fullname and CV. This is then stored in memory of the gRPC server. This can then be queried/viewed back in the webpage.

## To Run
You must start the job-search-server binary, and then start job-search-client
```
cargo run --bin job-search-server
cargo run --bin job-search-client
```

You can access the web page on `http://localhost:8080`
