use poem::{
    get, handler, listener::TcpListener, post, web::{Json, Path}, Route, Server
};

use crate::{request_inputs::{CreateWebsiteInput, GetWebsiteInput}, request_outputs::CreateWebsiteOutput};


pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_website(Path(website_id): Path<String>) -> Json<GetWebsiteInput> {
    let response = GetWebsiteInput{
        website_id: website_id.to_string(),
    };

    Json(response)
}

// converting struct->json json to struct 


#[handler] 
fn create_website(Json(data):Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let url = data.url;

    let response = CreateWebsiteOutput{
        id:String::from("ID")
    };

    // save the website to the database
    // diesel ORM

    Json(response)

}


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {


    // create the routes
    // business logic
    let app = Route::new()
    .at("/website/:website_id", get(get_website))
    .at("/website", post(create_website));




    // create and run the server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}