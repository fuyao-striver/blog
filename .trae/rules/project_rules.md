# 项目规则

本文件记录了用户对项目开发的特定要求，请在所有后续开发中严格遵守。

## Rust 代码规范

### 模块组织（Rust 2021+ 风格）

**禁止使用 `mod.rs` 文件**，采用 Rust 2021+ 的模块组织方式：

```
src/
├── model.rs          # 模块声明文件（pub mod user;）
├── model/
│   └── user.rs       # 子模块实现
├── repository.rs     # 模块声明文件（pub mod user_repo;）
├── repository/
│   └── user_repo.rs  # 子模块实现
```

**错误示例（旧风格）：**
```
src/
├── model/
│   ├── mod.rs        # ❌ 禁止使用
│   └── user.rs
```

**正确示例（新风格）：**
```
src/
├── model.rs          # ✅ 模块声明放在同级 .rs 文件中
├── model/
│   └── user.rs
```

### 依赖管理

**添加依赖时使用 `cargo add` 命令**，禁止直接修改 `Cargo.toml` 文件。

**错误示例：**
```toml
# ❌ 禁止手动编辑 Cargo.toml 添加依赖
[dependencies]
serde = { version = "1", features = ["derive"] }
```

**正确示例：**
```bash
# ✅ 使用 cargo add 命令
cargo add serde -F derive
cargo add tokio -F rt-multi-thread,macros
```

### 代码注释

**生成的代码必须添加必要的中文注释**，包括模块说明、结构体说明、函数说明、关键逻辑注释等。

**示例：**
```rust
/// 用户登录表单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginForm {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
}

/// 用户登录
/// 
/// # 参数
/// - `state`: 应用状态
/// - `form`: 登录表单
/// 
/// # 返回
/// 成功返回JWT Token，失败返回错误信息
pub async fn login(...) -> Result<String> {
    // 根据用户名查询用户
    let user = user_repo::find_by_username(...);
    ...
}
```

## 日志规范

**生成的后端接口必须包含完整的日志输出**，包括请求日志、响应日志和SQL日志。

### 请求日志

每个接口请求必须记录以下信息：
- 请求路径（path）
- 请求方法（method）
- 请求参数（query/path/body）
- 请求时间戳

**示例：**
```rust
pub async fn login(
    State(state): State<AppState>,
    Json(form): Json<LoginForm>,
) -> Result<Json<ApiResponse<Token>>> {
    // 记录请求日志
    tracing::info!(
        path = "/api/login",
        method = "POST",
        username = %form.username,
        "用户登录请求"
    );
    
    // 业务逻辑...
}
```

### 响应日志

每个接口响应必须记录以下信息：
- 响应状态码
- 响应结果（成功/失败）
- 响应JSON数据
- 响应时间戳
- 请求耗时（可选）

**示例：**
```rust
pub async fn login(...) -> Result<Json<ApiResponse<Token>>> {
    let start = std::time::Instant::now();
    
    // 业务逻辑...
    let token = generate_token(&user)?;
    let response = ApiResponse::success(token);
    
    // 记录响应日志
    tracing::info!(
        path = "/api/login",
        method = "POST",
        status = 200,
        success = true,
        response = ?response,
        elapsed_ms = start.elapsed().as_millis(),
        "用户登录成功"
    );
    
    Ok(Json(response))
}
```

### SQL日志

所有数据库操作必须记录SQL执行日志：
- SQL语句（必须使用 `format_sql` 格式化为单行）
- 执行参数
- 执行耗时
- 影响行数（可选）

**重要：SQL语句必须使用 `crate::utils::sql::format_sql` 函数格式化，将多行SQL压缩为单行，提高日志可读性。**

**错误示例：**
```rust
// ❌ 直接输出多行SQL，日志中包含大量 \n 换行符，影响阅读
tracing::debug!(
    sql = sql,
    "执行用户查询"
);
```

**正确示例：**
```rust
use crate::utils::sql::format_sql;

pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<User>> {
    let sql = "SELECT * FROM users WHERE username = $1";
    
    // 记录SQL日志（使用 format_sql 格式化）
    tracing::debug!(
        sql = %format_sql(sql),
        param = %username,
        "执行用户查询"
    );
    
    let start = std::time::Instant::now();
    let user = sqlx::query_as::<_, User>(sql)
        .bind(username)
        .fetch_optional(pool)
        .await?;
    
    // 记录SQL执行结果
    tracing::debug!(
        sql = %format_sql(sql),
        elapsed_ms = start.elapsed().as_millis(),
        found = user.is_some(),
        "SQL查询完成"
    );
    
    Ok(user)
}
```

**对于多行SQL同样适用：**
```rust
use crate::utils::sql::format_sql;

pub async fn find_permissions_by_user_id(pool: &PgPool, user_id: i32) -> Result<Vec<String>, sqlx::Error> {
    let sql = r#"
        SELECT DISTINCT m.perms 
        FROM t_menu m
        INNER JOIN t_role_menu rm ON m.id = rm.menu_id
        INNER JOIN t_user_role ur ON rm.role_id = ur.role_id
        WHERE ur.user_id = $1 AND m.perms IS NOT NULL AND m.perms != ''
        ORDER BY m.perms
        "#;
    
    // 记录SQL日志（自动格式化为单行）
    tracing::debug!(
        sql = %format_sql(sql),
        user_id = user_id,
        "执行查询用户权限列表"
    );
    
    // 执行查询...
}
```

**日志输出效果：**
```
DEBUG blog_api::repository::menu_repo: 执行查询用户权限列表 sql="SELECT DISTINCT m.perms FROM t_menu m INNER JOIN t_role_menu rm ON m.id = rm.menu_id INNER JOIN t_user_role ur ON rm.role_id = ur.role_id WHERE ur.user_id = $1 AND m.perms IS NOT NULL AND m.perms != '' ORDER BY m.perms" user_id=1
```

### 日志级别使用

- `tracing::error!`: 错误信息，系统异常
- `tracing::warn!`: 警告信息，潜在问题
- `tracing::info!`: 重要业务信息，如请求/响应日志
- `tracing::debug!`: 调试信息，如SQL日志
- `tracing::trace!`: 详细追踪信息
