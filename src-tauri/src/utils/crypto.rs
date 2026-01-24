use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use keyring::Entry;
use rand::{RngCore, thread_rng};

#[allow(dead_code)]
const APP_NAME: &str = "WindsurfAccountManager";
#[allow(dead_code)]
const KEY_NAME: &str = "MasterKey";

#[allow(dead_code)]
pub struct CryptoService {
    cipher: Aes256Gcm,
}

#[allow(dead_code)]
impl CryptoService {
    #[allow(dead_code)]
    pub fn new() -> Result<Self> {
        let key = Self::get_or_create_key()?;
        let key_bytes = STANDARD.decode(&key)?;
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        
        Ok(Self { cipher })
    }

    fn get_or_create_key() -> Result<String> {
        let entry = Entry::new(APP_NAME, KEY_NAME)?;
        
        match entry.get_password() {
            Ok(key) => Ok(key),
            Err(_) => {
                // 生成新的密钥
                let mut key = vec![0u8; 32];
                thread_rng().fill_bytes(&mut key);
                let key_base64 = STANDARD.encode(&key);
                
                // 保存到系统密钥链
                entry.set_password(&key_base64)?;
                
                Ok(key_base64)
            }
        }
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String> {
        // 生成随机nonce (96-bit)
        let mut nonce_bytes = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // 加密
        let ciphertext = self.cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;
        
        // 组合nonce和密文
        let mut combined = Vec::new();
        combined.extend_from_slice(&nonce_bytes);
        combined.extend_from_slice(&ciphertext);
        
        // 返回base64编码
        Ok(STANDARD.encode(&combined))
    }

    pub fn decrypt(&self, ciphertext: &str) -> Result<String> {
        // 解码base64
        let combined = STANDARD.decode(ciphertext)?;
        
        // 检查最小长度
        if combined.len() < 12 {
            return Err(anyhow::anyhow!("Invalid ciphertext"));
        }
        
        // 分离nonce和密文
        let (nonce_bytes, ciphertext) = combined.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        // 解密
        let plaintext = self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;
        
        // 转换为字符串
        String::from_utf8(plaintext)
            .map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))
    }

    pub fn set_master_password(&self, _password: &str) -> Result<()> {
        // 这里可以实现主密码功能，用密码派生密钥
        // 暂时不实现，使用系统密钥链即可
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let service = CryptoService::new().unwrap();
        let plaintext = "Hello, World!";
        
        let encrypted = service.encrypt(plaintext).unwrap();
        let decrypted = service.decrypt(&encrypted).unwrap();
        
        assert_eq!(plaintext, decrypted);
    }
}
