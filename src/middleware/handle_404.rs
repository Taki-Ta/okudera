use askama::Template;
use salvo::{handler, prelude::StatusCode, writing::Text, Depot, FlowCtrl, Request, Response};

#[derive(Template)]
#[template(path = "handle_404.html")]
struct Handle404;

#[handler]
pub async fn handle_404(&self, _req: &Request, _depot: &Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    if let Some(StatusCode::NOT_FOUND) = res.status_code {
        let handle404 = Handle404;
        res.render(Text::Html(handle404.render().unwrap()));
        ctrl.skip_rest();
    }
}
