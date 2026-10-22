#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use axum::{
        http::{header, Uri},
        routing::get,
        Router,
    };
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use rjay_portfolio::app::*;

    let conf = get_configuration(None)?;
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    // A configured origin avoids publishing invented deployment URLs.
    let origin = std::env::var("SITE_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".into());
    let origin = origin.trim_end_matches('/');
    let uri: Uri = origin.parse()?;
    if !matches!(uri.scheme_str(), Some("http" | "https"))
        || uri.authority().is_none()
        || uri.path() != "/" && !uri.path().is_empty()
        || uri.query().is_some()
        || origin.contains('@')
    {
        return Err("SITE_ORIGIN must be an HTTP(S) origin without a path or credentials".into());
    }
    let escaped_origin = origin
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\"', "&quot;")
        .replace('\'', "&apos;");
    let sitemap = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"><url><loc>{escaped_origin}/</loc></url></urlset>"#
    );
    let robots = format!("User-agent: *\nAllow: /\nSitemap: {origin}/sitemap.xml\n");

    let app = Router::new()
        .route(
            "/sitemap.xml",
            get(move || {
                let sitemap = sitemap.clone();
                async move {
                    (
                        [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
                        sitemap,
                    )
                }
            }),
        )
        .route(
            "/robots.txt",
            get(move || {
                let robots = robots.clone();
                async move { robots }
            }),
        )
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
