use argon2::{
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
    password_hash::{SaltString, rand_core::OsRng},
};

/// 构建 Argon2 实例（博客场景平衡安全性与性能）
///
/// m_cost=4096 KiB (4 MiB), t_cost=1, p_cost=1
/// 单次验证约 50-100ms，比默认参数快 3-5 倍
fn build_argon2() -> Argon2<'static> {
    let params =
        Params::new(4096, 1, 1, Some(Params::DEFAULT_OUTPUT_LEN)).expect("Argon2 参数无效");
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

/// 生成加密密码
pub fn hash_password(passowrd: &str) -> anyhow::Result<String> {
    // 1. 设置随机盐值
    let salt = SaltString::generate(&mut OsRng);
    // 2. Argon2 配置
    let argon2 = build_argon2();
    // 3.生成密码哈希
    let password_hash = argon2
        .hash_password(passowrd.as_bytes(), &salt)
        .map_err(|e| anyhow::format_err!("密码哈希失败: {}", e))?;
    Ok(password_hash.to_string())
}

/// 验证密码
pub fn verify_password(passowrd: &str, hash: &str) -> anyhow::Result<bool> {
    // 1. 解析存储的哈希字符串
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| anyhow::format_err!("无效的密码哈希: {}", e))?;
    // 2. 验证密码（使用自定义参数，比默认快 3-5 倍）
    Ok(build_argon2()
        .verify_password(passowrd.as_bytes(), &parsed_hash)
        .is_ok())
}
