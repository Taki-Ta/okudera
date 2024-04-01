use crate::{
    app_writer::{AppResult, AppWriter, ErrorResponseBuilder},
    dtos::user::{
        UserAddRequest, UserLoginRequest, UserLoginResponse, UserResponse, UserUpdateRequest,
    },
    middleware::jwt::decode_token,
    services::user,
};
// use alloc::sync;
use askama::Template;
use salvo::{
    http::cookie::Cookie,
    oapi::{
        endpoint,
        extract::{JsonBody, PathParam},
    },
    writing::{Redirect, Text},
    Request, Response,
};
use salvo::{prelude::*, Writer};
use tracing::info;

// #[derive(Template)]
// #[template(path = "login.html")]
// struct LoginTemplate {}

// #[endpoint(tags("comm"))]
// pub async fn login_page(res: &mut Response) -> AppResult<()> {
//     let cookies = res.cookies();
//     let cookie = cookies.get("jwt_token");
//     if let Some(cookie) = cookie {
//         let token = cookie.value().to_string();
//         if decode_token(&token) {
//             res.render(Redirect::other("/users"));
//             return Ok(());
//         }
//     }
//     let hello_tmpl = LoginTemplate {};
//     res.render(Text::Html(hello_tmpl.render().unwrap()));
//     Ok(())
// }

#[derive(Template)]
#[template(path = "user_list_page.html")]
pub struct UserListPageTemplate {}

#[derive(Template)]
#[template(path = "user_list.html")]
pub struct UserListTemplate {}

#[endpoint]
pub async fn user_list_page(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let is_fragment = req.headers().get("X-Fragment-Header");
    match is_fragment {
        Some(_) => {
            let hello_tmpl = UserListTemplate {};
            res.render(Text::Html(hello_tmpl.render().unwrap()));
        }
        None => {
            let hello_tmpl = UserListPageTemplate {};
            res.render(Text::Html(hello_tmpl.render().unwrap()));
        }
    }
    Ok(())
}
#[endpoint(tags("comm"))]
pub async fn post_login(req: JsonBody<UserLoginRequest>, res: &mut Response) {
    let result: AppResult<UserLoginResponse> = user::login(req.0).await;
    match result {
        Ok(data) => {
            let jwt_token = data.token.clone();
            let cookie = Cookie::build(("jwt_token", jwt_token))
                .path("/")
                .http_only(true)
                //set cookie expire time
                .expires(time::OffsetDateTime::now_utc() + time::Duration::seconds(data.exp))
                .build();
            res.add_cookie(cookie);
            info!("user {} login in", &data.username);
            info!("add user jwttoken 【{}】", &data.token);
            res.render(Redirect::found("/index"));
        }
        Err(e) => ErrorResponseBuilder::with_err(e).into_response(res),
    }
}

#[endpoint(tags("users"))]
pub async fn post_add_user(new_user: JsonBody<UserAddRequest>) -> AppWriter<UserResponse> {
    let result = user::add_user(new_user.0).await;
    AppWriter(result)
}

#[endpoint(tags("users"),
parameters(
    ("id", description = "user id"),
))]
pub async fn put_update_user(req: &mut Request) -> AppResult<AppWriter<UserResponse>> {
    let req: UserUpdateRequest = req.extract().await?;
    let result = user::update_user(req).await;
    Ok(AppWriter(result))
}

#[endpoint(tags("users"))]
pub async fn delete_user(id: PathParam<String>) -> AppWriter<()> {
    let result = user::delete_user(id.0).await;
    AppWriter(result)
}

#[endpoint(tags("users"))]
pub async fn get_users() -> AppWriter<Vec<UserResponse>> {
    let result = user::users().await;
    AppWriter(result)
}

#[handler]
pub async fn login_page(&self, res: &mut Response) {
    let cookies = res.cookies();
    let cookie = cookies.get("jwt_token");
    if let Some(cookie) = cookie {
        let token = cookie.value().to_string();
        if decode_token(&token) {
            res.render(Redirect::other("/index"));
        }
    }
    let content = tokio::fs::read_to_string("pages/login.html").await.unwrap();
    res.render(Text::Html(content))
}

#[endpoint(tags("users"))]
pub async fn logout(res: &mut Response) -> AppResult<()> {
    let cookies = res.cookies_mut(); // Change the type of `cookies` to `CookiesMut`
    if cookies.get("jwt_token").is_some() {
        let cookie = Cookie::build(("jwt_token", ""))
            .path("/")
            .http_only(true)
            .expires(time::OffsetDateTime::now_utc() - time::Duration::days(1))
            .build();
        //remove cookie
        res.add_cookie(cookie);
    }
    res.render(Redirect::other("/login"));
    Ok(())
}

#[cfg(test)]
mod test {}
