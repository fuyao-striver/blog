//! SQL工具模块
//!
//! 提供SQL相关的工具函数

/// 格式化SQL语句，将多行SQL压缩成单行
///
/// 将SQL语句中的多余空白字符（换行符、制表符、多个空格等）
/// 替换为单个空格，使SQL日志更易读。
///
/// # 参数
/// - `sql`: SQL语句
///
/// # 返回
/// 格式化后的单行SQL
///
/// # 示例
/// ```
/// let sql = r#"
///     SELECT *
///     FROM users
///     WHERE id = 1
/// "#;
/// let formatted = format_sql(sql);
/// assert_eq!(formatted, "SELECT * FROM users WHERE id = 1");
/// ```
pub fn format_sql(sql: &str) -> String {
    sql.split_whitespace().collect::<Vec<_>>().join(" ")
}
