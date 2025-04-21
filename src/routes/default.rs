use crate::models::{AppError, Token};
use axum::extract::Path;
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use std::sync::LazyLock;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use tera::{Context, Tera};
use crate::routes::TOKEN_COOKIE;

static TERA: LazyLock<Tera> = LazyLock::new(|| {
   let mut tera = match Tera::new("www/**/*") {
       Ok(t) => t,
       Err(e) => panic!("{:?}", e),
    };

    tera.autoescape_on(vec![".html"]);

    tera
});

pub async fn default(Path(file): Path<String>, jar: CookieJar) -> Result<impl IntoResponse, AppError> {

    let mut ctx = Context::new();

    if let Some(cookie) = jar.get(TOKEN_COOKIE) {
        let decoded = match BASE64_STANDARD.decode(cookie.value().as_bytes()) {
            Ok(decoded) => decoded,
            Err(e) => return Err(AppError::from(e)),
        };

        let token = match Token::try_from(decoded.as_slice()) {
            Ok(token) => token,
            Err(e) => return Err(AppError::from(e)),
        };

        ctx.insert("user", &token);
    }

    let content = match TERA.render(&file, &ctx) {
        Ok(content) => content,
        Err(_) => return Ok(StatusCode::NOT_FOUND.into_response())
    };

    let mime = mime_guess::from_path(&file).first_or_text_plain();

    let mut headers = HeaderMap::new();
    headers.append(
        header::CONTENT_TYPE,
        mime.essence_str().parse()?,
    );

    Ok((headers, content).into_response())
}