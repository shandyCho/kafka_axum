// Login 및 token 생성
async fn login(State(state): State<ApplicationState>, Json(login_info): Json<LoginInfo>) -> String {
     let hashed_password = "12345";
     let user_idx = 1;
     let is_valid = verify(login_info.password, hashed_password).unwrap();

     if is_valid {
        let claims = Claims {
                sub: user_idx, 
                exp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as usize + 60 * 60 };

        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET.as_bytes())).unwrap();
        token
    } else {
        "Log in failed!".to_string()
    }        
}

// HTTP Response 헤더에 포함된 토큰을 삽입
async fn auth(mut req: Request, next: Next) -> Response {
  match  req.headers().get("authorization") {
      None => Response::new(Body::new("No token".to_string())),
      Some(header_value) => {
        let token = header_value.to_str().unwrap();
        match decode(token, &DecodingKey::from_secret(SECRET.as_bytes()), &Validation::default()) {
          Err(e) => Response::new(Body::new(e.to_string())),
          Ok(token_data) => {
            let claims : Claims = token_data.claims;
            req.extensions_mut().insert(claims.sub.to_string());
            let req = next.run(req).await;
            req
          }
      }
      }
  }
  
}