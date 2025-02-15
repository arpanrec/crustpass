use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::{prelude::BASE64_STANDARD, Engine};

pub async fn aes256_enc(key_str_base64: &String, iv_str_base64: &String, plaintext: &String) -> String {
    let key_decoded: Vec<u8> = BASE64_STANDARD.decode(key_str_base64.as_bytes()).unwrap();
    let key: [u8; 32] = key_decoded.try_into().unwrap();

    let iv_decoded: Vec<u8> = BASE64_STANDARD.decode(iv_str_base64.as_bytes()).unwrap();
    let iv: [u8; 16] = iv_decoded.try_into().unwrap();

    let plaintext_bin: Vec<u8> = plaintext.as_bytes().to_vec();
    type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;

    let mut buf = vec![0u8; plaintext_bin.len() + 16];
    let pt_len = plaintext_bin.len();
    buf[..pt_len].copy_from_slice(&plaintext_bin);
    let ct = Aes256CbcEnc::new(&key.into(), &iv.into())
        .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
        .unwrap();
    let ct_base64 = BASE64_STANDARD.encode(&ct);
    ct_base64.to_string()
}

pub async fn aes256_dec(
    key_str_base64: &String,
    iv_str_base64: &String,
    encrypted_text_base64: &String,
) -> String {
    let key_decoded: Vec<u8> = BASE64_STANDARD.decode(key_str_base64.as_bytes()).unwrap();
    let key: [u8; 32] = key_decoded.try_into().unwrap();

    let iv_decoded: Vec<u8> = BASE64_STANDARD.decode(iv_str_base64.as_bytes()).unwrap();
    let iv: [u8; 16] = iv_decoded.try_into().unwrap();

    let encrypted_text_decoded: Vec<u8> = BASE64_STANDARD
        .decode(encrypted_text_base64.as_bytes())
        .unwrap();
    let encrypted_text_bin: Vec<u8> = encrypted_text_decoded.to_vec();
    type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

    let mut buf = vec![0u8; encrypted_text_bin.len()];
    let pt = Aes256CbcDec::new(&key.into(), &iv.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(&encrypted_text_bin, &mut buf)
        .unwrap();
    let pt_str = String::from_utf8_lossy(&pt);
    pt_str.to_string()
}
