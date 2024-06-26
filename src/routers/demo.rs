use crate::app_writer::AppResult;
use askama::Template;
use salvo::{oapi::endpoint, writing::Text, Request, Response};

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

#[endpoint]
pub async fn hello(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let hello_tmpl = HelloTemplate {
        name: req.param::<&str>("name").unwrap_or("World"),
    };
    res.render(Text::Html(hello_tmpl.render().unwrap()));
    Ok(())
}

#[allow(unused_imports)]
mod tests {
    use crate::config::CFG;
    use salvo::test::{ResponseExt, TestClient};
    use salvo::Service;

    #[tokio::test]
    async fn test_hello_world() {
        let service = Service::new(crate::routers::router());

        let content = TestClient::get(format!(
            "http://{}",
            &CFG.server.address.replace("0.0.0.0", "127.0.0.1")
        ))
        .send(&service)
        .await
        .take_string()
        .await
        .unwrap();
        assert_eq!(content, "Hello World from salvo");
    }
}
