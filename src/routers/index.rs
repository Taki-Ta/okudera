use askama::Template;
use salvo::prelude::*;
use salvo::{writing::Text, Response};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    #[allow(dead_code)]
    content: String,
}

#[handler]
pub async fn index(res: &mut Response) {
    let content = tokio::fs::read_to_string("templates/index_content.html")
        .await
        .unwrap();
    res.render(Text::Html(IndexTemplate { content }.render().unwrap()))
}
