use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

/// 生成加密密码
pub fn hash_password(passowrd: &str) -> anyhow::Result<String> {
    // 1. 设置随机盐值
    let salt = SaltString::generate(&mut OsRng);
    // 2. Argon2 配置
    let argon2 = Argon2::default();
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
    // 2. 验证密码
    Ok(Argon2::default()
        .verify_password(passowrd.as_bytes(), &parsed_hash)
        .is_ok())
}
