use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct Application {
    pub fullname: String,
    pub creation_time: String,
    pub cv_link: String,
}

