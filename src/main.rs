use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db: mongodb::Database = connect_mongodb().await;
    let app: Router<_> = Router::new()
        .route("/", get(root))
        .route("/auth/signin", post(signin))
        .with_state(db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
#[derive(Debug, Deserialize, Serialize)]
struct SigninPayload {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    password: String,
}
async fn signin(
    State(db): State<Database>,
    Json(payload): Json<SigninPayload>,
) -> Result<Json<Value>, AppError> {
    println!("{:?} ", payload);
    let payload_username = payload.username;
    let payload_password: String = payload.password;
    let user_collection: mongodb::Collection<User> = db.collection("user");
    let result = user_collection
        .find_one(doc! { "username": payload_username }, None)
        .await;

    match result {
        Ok(Some(_user)) => {
            let app_error = AppError(anyhow::Error::msg("An error occurred"));
            Err(app_error)
        }
        Ok(None) => Ok(Json(json!({ "data": "username is existed" }))),
        Err(e) => {
            println!("error parsing header: {e:?}");
            let app_error = AppError(anyhow::Error::msg("An error occurred 123132"));
            Err(app_error)
        }
    }

    // check had username
    // create a new user document ;
}
async fn root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn connect_mongodb() -> mongodb::Database {
    let mongodb_uri = std::env::var("MONGO_DB_URI").expect("MONGO_DB_URI must be set.");
    let db_name = std::env::var("DB_NAME").expect("DB NAME must be set.");
    let client_options = ClientOptions::parse(mongodb_uri)
        .await
        .expect("Can not parse client options");
    //TODO: use match instead of unwarp;
    let client = Client::with_options(client_options).unwrap();
    let db: mongodb::Database = client.database(db_name.as_str());
    println!("Connect db success");
    db
}

struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}
