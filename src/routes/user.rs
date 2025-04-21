use crate::models::{AppError, Model, Token, User, UserCreateInfo};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::Form;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use base64::prelude::*;

pub static TOKEN_COOKIE: &str = "ritw-token";

pub async fn signup_post(
    jar: CookieJar,
    Form(user): Form<UserCreateInfo>,
) -> Result<impl IntoResponse, AppError> {
    let user = match User::create(&User::from(user)).await {
        Ok(user) => user,
        Err(e) => return Ok((StatusCode::CONFLICT, e).into_response()),
    };

    let token: Vec<u8> = Token::new(&user).try_into().map_err(AppError::from)?;
    let token_str = BASE64_STANDARD.encode(&token);

    let cookie = Cookie::build((TOKEN_COOKIE, token_str)).http_only(true);

    let jar = jar.add(cookie);
    Ok((jar, Redirect::to("/")).into_response())
}

pub async fn signin_post(
    jar: CookieJar,
    Form(dto): Form<UserCreateInfo>,
) -> Result<impl IntoResponse, AppError> {
    let user = User::from(dto);
    let origin = match User::read(user.id()).await {
        Ok(user) => user,
        Err(e) => return Ok((StatusCode::NOT_FOUND, jar, e).into_response()),
    };

    if !user.password().eq(origin.password()) {
        return Ok((StatusCode::FORBIDDEN, jar, "Password incorrect").into_response());
    }

    let token: Vec<u8> = Token::new(&user).try_into().map_err(AppError::from)?;
    let token_str = BASE64_STANDARD.encode(&token);

    let cookie = Cookie::build((TOKEN_COOKIE, token_str)).http_only(true);

    let jar = jar.add(cookie);
    Ok((jar, Redirect::to("/")).into_response())
}

pub async fn signout_post(jar: CookieJar) -> impl IntoResponse {
    let jar = jar.remove(Cookie::from(TOKEN_COOKIE));
    (jar, Redirect::to("/"))
}
