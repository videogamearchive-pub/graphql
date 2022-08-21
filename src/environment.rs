use lazy_static::lazy_static;

lazy_static! {
    pub static ref HTTP_HOST: String =
        std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_owned());
    pub static ref HTTP_PORT: String = std::env::var("PORT").unwrap_or_else(|_| "8080".to_owned());
    pub static ref HTTP_ADDRESS: String = format!("{}:{}", *HTTP_HOST, *HTTP_PORT);
    pub static ref DATABASE_URL: String = std::env::var("DATABASE_URL").expect("DATABASE_URL");
}
