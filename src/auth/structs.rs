use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Claims {
    exp: usize,
    sub: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct LoginInfo {
    identifier: String,
    password: String,
}