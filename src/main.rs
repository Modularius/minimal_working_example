#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::prelude::*;
    use leptos::config::get_configuration;
    use leptos_meta::MetaTags;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use minimal_working_example::app::*;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    HttpServer::new(move || {
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        println!("listening on http://{}", &addr);

        App::new()
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone() root = "http://localhost/subfolder" />
                                <MetaTags/>
                            </head>
                            <body>
                                <App/>
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(not(feature = "ssr"))]
fn main() {

}