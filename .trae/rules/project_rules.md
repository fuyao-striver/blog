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
