use actix_web::Responder;
use maud::{DOCTYPE, PreEscaped, html};

pub async fn index() -> impl Responder {
    let markup = html! {
        (DOCTYPE)
        html class="h-100" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "CHTMX" }
                link rel="stylesheet" href="/assets/t.css";
                script src="/assets/h.js" {}
            }
            body class="w-100 h-100 sans-serif ma0" {
                header class="sans-serif h-100" {
                    div class="cover bg-center h-100" style="background-image: url(/assets/background.jpg)" {
                        div class="bg-black-10 h-100 flex flex-column" {
                            nav class="dt w-100 mw8 center" {
                                div class="dtc v-mid pa3" {
                                    a href="/" class="link white-90 hover-white no-underline fw6 f4" {
                                        "CHTMX"
                                    }
                                }
                                div class="dtc v-mid tr pa3" {
                                    a class="f6 fw4 hover-white no-underline white-70 dn dib-ns pv2 ph3" href="/" { "How it Works" }
                                    a class="f6 fw4 hover-white no-underline white-70 dn dib-l pv2 ph3" href="/" { "About" }
                                    div class="dib v-mid ml3"
                                        hx-get="/health/db/status"
                                        hx-trigger="load"
                                        hx-swap="innerHTML" {
                                        span class="white-70 f6" { "Loading..." }
                                    }
                                }
                            }
                            div class="flex-auto flex items-center justify-center tc ph3" {
                                div {
                                    h1 class="f2 f1-l fw2 white-90 mb0 lh-title" { "This is your super impressive headline" }
                                    h2 class="fw1 f3 white-80 mt3 mb4" { "Now a subheadline where explain your wonderful new startup even more" }
                                    a class="f6 no-underline grow dib v-mid bg-blue white ba b--blue ph3 pv2 mb3" href="/" { "Call to Action" }
                                    span class="dib v-mid ph3 white-70 mb3" { "or" }
                                    a class="f6 no-underline grow dib v-mid white ba b--white ph3 pv2 mb3" href="" { "Secondary call to action" }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    PreEscaped(markup.into_string())
}
