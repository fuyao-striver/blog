use tracing_subscriber::{EnvFilter, fmt, util::SubscriberInitExt};

/// 日志
pub fn app_log() {
    // 1. 从环境变量中获取日志等级，没有就使用默认等级
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    // 2. 配置日志格式
    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(filter)
        .with_target(true) // 显示路径模块
        .with_thread_ids(true) // 显示线程id
        .with_file(true)
        .with_line_number(true)
        .finish();
    subscriber.init();
}
