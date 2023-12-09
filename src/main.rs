use axum::{
    extract::State,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client, Database};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db: mongodb::Database = connect_mongodb().await;
    let app: Router<_> = Router::new()
        .route("/", get(root))
        .route("/auth/signin", post(signin))
        .with_state(db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
#[derive(Debug, Deserialize)]
struct SigninPayload {
    username: String,
}

async fn signin(db: State<Database>, Json(payload): Json<SigninPayload>) {
    println!("{:?} ", payload);
    // check had username
    // create a new user document ;
}
async fn root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn connect_mongodb() -> mongodb::Database {
    let mongodb_uri = std::env::var("MONGO_DB_URI").expect("MONGO_DB_URI must be set.");
    let db_name = std::env::var("DB_NAME").expect("MONGO_DB_URI must be set.");
    let client_options = ClientOptions::parse(mongodb_uri)
        .await
        .expect("Can not parse client options");
    //TODO: use match instead of unwarp;
    let client = Client::with_options(client_options).unwrap();
    let db: mongodb::Database = client.database(db_name.as_str());
    println!("Connect db success");
    db
}
