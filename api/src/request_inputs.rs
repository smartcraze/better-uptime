use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
pub struct  CreateWebsiteInput{
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetWebsiteInput{
    pub website_id: String,
}
