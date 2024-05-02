use jsonwebtoken::TokenData;
use regex::Regex;
use sha256::digest;
use crate::model::auth::TokenClaims;

pub fn validate_firstname_lastname(firstname : String, lastname: String) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
    let caps = re.is_match(&lastname) & re.is_match(&firstname);
    caps
}

pub fn validate_email(email: String) -> bool {
    let re = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
    let caps = re.is_match(&email);
    caps
}

pub fn validate_phone(phone: String) -> bool {
    let re = Regex::new(r"^[0-9]*$").unwrap();
    let caps = re.is_match(&phone);
    caps
}

pub fn validate_password(password : String) -> bool{
    let regex_one_uppercase = Regex::new(r"[a-z]{1,}").unwrap();
    let one_uppercase = regex_one_uppercase.is_match(&password);
    let regex_one_lowercase = Regex::new(r"[A-Z]{1,}").unwrap();
    let one_lowercase = regex_one_lowercase.is_match(&password);
    let regex_one_digit = Regex::new(r"[0-9]{1,}").unwrap();
    let one_digit = regex_one_digit.is_match(&password);
    let regex_length = Regex::new(r".{8,}").unwrap();
    let length = regex_length.is_match(&password);
    let regex_one_symbol = Regex::new(r"[*@!#%&()^~{}/.,?><=+-_]{1,}").unwrap();
    let one_symbol = regex_one_symbol.is_match(&password);
    let caps = one_uppercase & one_lowercase & one_digit & length & one_symbol;
    caps
}

pub fn validate_user(username: String) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
    let caps = re.is_match(&username);
    caps
}

pub fn check_sha256_salt(password1 : String, password2: String, salt: String) -> bool {
    let hash_password2 = digest(format!("{}{}", password2, salt));

    password1 == hash_password2
}

pub fn validate_department_name(name: String) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9 ]+$").unwrap();
    let caps = re.is_match(&name);
    caps
}

pub fn hash_sha256 (password: String , salt: String) -> String {
    digest(format!("{}{}", password, salt))
}

pub fn verify_jwt_token(
    token: &str,
) -> Result<TokenData<TokenClaims>, jsonwebtoken::errors::Error> {

    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);

    let decoded = jsonwebtoken::decode::<TokenClaims>(
        token,
        &jsonwebtoken::DecodingKey::from_rsa_pem(include_bytes!("public_key.pem"))?,
        &validation,
    );

    decoded
}