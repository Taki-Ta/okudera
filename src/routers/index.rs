use askama::Template;
use salvo::{
    writing:: Text,  Response
};
use salvo::prelude::*;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    content: &'a str,
}

#[handler]
pub async fn index(res: &mut Response) {
    res.render(Text::Html(IndexTemplate { content: "Hello, Salvo!" }.to_string()))
}