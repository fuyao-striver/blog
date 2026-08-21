use axum::{Router, routing::get};
use blog_api::utils::log::app_log;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // 读取.env文件获取环境变量
    dotenvy::dotenv().expect("读取.env文件失败!");

    app_log();

    // 创建一个路由
    let app = Router::new()
        .route("/", get(root))
        // 添加TraceLayer中间件
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(tower_http::trace::DefaultMakeSpan::new().include_headers(false))
                .on_request(tower_http::trace::DefaultOnRequest::new())
                .on_response(tower_http::trace::DefaultOnResponse::new()),
        );

    // 监听端口并启动服务
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1315")
        .await
        .expect("端口监听失败!");
    axum::serve(listener, app).await.expect("服务启动失败!");
}

async fn root() -> &'static str {
    "hello world"
}
