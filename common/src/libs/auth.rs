use bcrypt::{hash, verify, DEFAULT_COST};
use sha2::{Sha256, Digest};

/// パスワードを bcrypt でハッシュ化します
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

/// パスワードがハッシュと一致するか検証します
pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

/// メールアドレスをハッシュ化（SHA-256）します
/// 本システムではメール送信を行わないため、比較専用のハッシュとして保存します
pub fn hash_email(email: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(email.trim().to_lowercase());
    let result = hasher.finalize();
    format!("{:x}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "my_secure_password";
        let hashed = hash_password(password).unwrap();
        assert!(verify_password(password, &hashed).unwrap());
        assert!(!verify_password("wrong_password", &hashed).unwrap());
    }

    #[test]
    fn test_email_hashing() {
        let email1 = "Test@Example.com ";
        let email2 = "test@example.com";
        assert_eq!(hash_email(email1), hash_email(email2));
    }
}
