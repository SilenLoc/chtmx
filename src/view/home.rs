use actix_web::Result as AwResult;
use actix_web::{HttpRequest, get};
use maud::html;

#[get("/ui/home")]
pub async fn home_page(req: HttpRequest) -> AwResult<maud::Markup> {
    let content = html! {
        div {
            h1 class="f2 f1-l fw2 white-90 mb0 lh-title" { "This is your super impressive headline" }
            h2 class="fw1 f3 white-80 mt3 mb4" { "Now a subheadline where explain your wonderful new startup even more" }
            a class="f6 no-underline grow dib v-mid bg-blue white ba b--blue ph3 pv2 mb3" href="/" { "Call to Action" }
            span class="dib v-mid ph3 white-70 mb3" { "or" }
            a class="f6 no-underline grow dib v-mid white ba b--white ph3 pv2 mb3" href="" { "Secondary call to action" }
        }
    };

    // Check if this is an htmx request
    if req.headers().get("HX-Request").is_some() {
        Ok(content)
    } else {
        Ok(super::render_layout(&content))
    }
}
