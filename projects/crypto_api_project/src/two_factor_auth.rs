use crate::api_library::Result;
use google_authenticator::GoogleAuthenticator;

pub fn get_code(secret: &str) -> Result<String> {
    let auth = GoogleAuthenticator::new();
    let code = auth.get_code(&secret, 0).unwrap();
    Ok(code)
}
