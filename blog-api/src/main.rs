use axum::{Router, routing::post};
use blog_api::{
    AppState,
    handler::user_handler::login,
    utils::{log::app_log, sql::connect_mysql},
};
use tower_http::trace::TraceLayer;
use tracing::Level;

#[tokio::main]
async fn main() {
    // 读取.env文件获取环境变量
    dotenvy::dotenv().expect("读取.env文件失败!");

    app_log();

    // 连接数据库
    let pool = connect_mysql().expect("数据库连接失败!");

    let state = AppState::new(pool);
    // 创建一个路由
    let app = Router::new()
        .route("/login", post(login))
        // 添加TraceLayer中间件
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(tower_http::trace::DefaultMakeSpan::new().include_headers(false))
                .on_request(tower_http::trace::DefaultOnRequest::new().level(Level::INFO))
                .on_response(tower_http::trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state);

    // 监听端口并启动服务
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1315")
        .await
        .expect("端口监听失败!");
    axum::serve(listener, app).await.expect("服务启动失败!");
}
