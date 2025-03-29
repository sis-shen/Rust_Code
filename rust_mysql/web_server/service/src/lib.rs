use axum::Json;
use axum::extract::State;
use entity::account::Entity as Account;
use entity::account;
use axum::{Router, routing::get,routing::post};
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use dotenv::dotenv;
use std::env;


use std::fmt::format;
// 新增状态结构体
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

/// 异步程序的主函数
#[tokio::main]
pub async fn main()-> anyhow::Result<()> {
    dotenv().expect(".env文件加载失败");

    let db_url = env::var("DATABASE_URL").unwrap();
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");
    println!("获取到的url:{}",db_url.clone());
    let db = get_db(&db_url).await.unwrap();    

    let shared_state = AppState{db};

    //构建路由
    let app = Router::new()
    .route("/users",get(get_info))
    .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener,app).await?;
    Ok(())
}

async fn get_db(db_url:&str)->Result<DatabaseConnection, sea_orm::DbErr>{

    let db = Database::connect(db_url).await?;
    Ok(db)
}

async fn get_index()->String{
    "Hello".to_string()
}

async fn get_info()->String{
    let contents = tokio::fs::read_to_string("./res/html/users_info.html").await.unwrap_or("页面丢失了？_？".to_string());
    contents
}

async fn get_all_users(
    State(state):State<AppState>,
)-> Result<Json<Vec<account::Model>>,String>{
    Account::find()
    .all(&state.db)
    .await
    .map(Json)
    .map_err(|e| format!("查询失败{}",e))
}