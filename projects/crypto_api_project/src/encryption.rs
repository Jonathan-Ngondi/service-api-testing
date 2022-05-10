use crate::api_library::Result;
use crate::error::Error;
use hmac:: {
    Hmac,
    Mac,
    NewMac,
};
use sha2::{
    Digest,
    Sha256,
    Sha512,
};
use std::time::{
    SystemTime,
    UNIX_EPOCH,
};

type HmacSha512 = Hmac::<Sha512>;

fn sha256(input: String) -> Result<Vec<u8>> {
    let mut hash = Sha256::new();
    hash.update(input.as_bytes());
    Ok(hash.finalize().to_vec())
}

fn sha512(input: Vec<u8>, secret: &[u8]) -> Result<Vec<u8>> {
    let mut mac = HmacSha512::new_varkey(secret)?;
    mac.update(&input);
    Ok(mac.finalize().into_bytes().to_vec())
}

pub fn get_signature(
    api_secret: &str,
    path: &str,
    nonce: &str,
    post_body: &str,
) -> Result<String> {
    let mut post = String::from("");
    post.push_str(nonce);
    post.push_str(post_body);

    let mut sha256_post_encryption = sha256(post).unwrap();

    let mut byte_arr = vec![];

    byte_arr.append(&mut path.as_bytes().to_owned());
    byte_arr.append(&mut sha256_post_encryption);

    let secret = base64::decode(api_secret).map_err(Error::internal)?;
    let sha512_sign_encryption = sha512(byte_arr, &secret).unwrap();
    
    Ok(base64::encode(&sha512_sign_encryption))
}