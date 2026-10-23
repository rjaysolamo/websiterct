// Match the library limit for Leptos's nested view types in optimized builds.
#![recursion_limit = "256"]

#[cfg(feature = "ssr")]
use anyhow::{Context, Result, ensure};

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<()> {
    use axum::{Router, http::header, routing::get};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use rjay_portfolio::app::*;

    let conf = get_configuration(None).context("Failed to load Leptos server configuration")?;
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    // A configured origin avoids publishing invented deployment URLs.
    let origin = site_origin(std::env::var_os("SITE_ORIGIN"))?;
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
        .route("/pkg/rjay-portfolio.css", get(stylesheet))
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

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("Failed to bind HTTP server to {addr}"))?;
    log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .context("HTTP server terminated unexpectedly")?;
    Ok(())
}

/// Build-generated presentation is embedded so deployment needs no source stylesheet.
#[cfg(feature = "ssr")]
async fn stylesheet() -> impl axum::response::IntoResponse {
    (
        [
            (axum::http::header::CONTENT_TYPE, "text/css; charset=utf-8"),
            (axum::http::header::CACHE_CONTROL, "no-cache"),
        ],
        include_str!(concat!(env!("OUT_DIR"), "/portfolio.css")),
    )
}

/// Missing configuration retains the local default; malformed configured values fail safely.
#[cfg(feature = "ssr")]
fn site_origin(value: Option<std::ffi::OsString>) -> Result<String> {
    use axum::http::Uri;

    let origin = match value {
        None => "http://localhost:3000".to_owned(),
        Some(value) => value
            .into_string()
            .map_err(|_| anyhow::anyhow!("SITE_ORIGIN must contain valid Unicode"))?,
    };
    let origin = origin.trim_end_matches('/');
    let uri: Uri = origin
        .parse()
        .context("Failed to parse SITE_ORIGIN as a URI")?;
    ensure!(
        matches!(uri.scheme_str(), Some("http" | "https"))
            && uri.authority().is_some()
            && (uri.path() == "/" || uri.path().is_empty())
            && uri.query().is_none()
            && !origin.contains('@')
            && !origin.contains('#'),
        "SITE_ORIGIN must be an HTTP(S) origin without a path, query, fragment, or credentials"
    );
    Ok(origin.to_owned())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::site_origin;

    #[test]
    fn missing_origin_keeps_local_default() -> anyhow::Result<()> {
        assert_eq!(site_origin(None)?, "http://localhost:3000");
        Ok(())
    }

    #[test]
    fn valid_origins_preserve_scheme_host_port_and_normalize_slashes() -> anyhow::Result<()> {
        for (input, expected) in [
            ("https://example.test/", "https://example.test"),
            ("http://localhost:4000", "http://localhost:4000"),
            ("http://[::1]:3000/", "http://[::1]:3000"),
        ] {
            assert_eq!(site_origin(Some(input.into()))?, expected);
        }
        Ok(())
    }

    #[test]
    fn invalid_origins_fail_without_echoing_configuration() {
        for input in [
            "",
            "example.test",
            "ftp://example.test",
            "https://example.test/path",
            "https://example.test?key=private-value",
            "https://example.test/#fragment",
            "https://private-user:private-password@example.test",
        ] {
            let error = site_origin(Some(input.into())).expect_err("invalid origin must fail");
            let diagnostic = format!("{error:#}");
            assert!(diagnostic.contains("SITE_ORIGIN"));
            assert!(!diagnostic.contains("private-"));
        }
    }

    #[test]
    fn malformed_uri_preserves_parse_error_source() {
        let error =
            site_origin(Some("https://[broken".into())).expect_err("malformed URI must fail");
        assert!(
            error
                .downcast_ref::<axum::http::uri::InvalidUri>()
                .is_some()
        );
        assert_eq!(error.to_string(), "Failed to parse SITE_ORIGIN as a URI");
    }

    #[cfg(unix)]
    #[test]
    fn invalid_unicode_is_rejected_instead_of_defaulting() {
        use std::os::unix::ffi::OsStringExt;
        let value = std::ffi::OsString::from_vec(vec![0xff]);
        let error = site_origin(Some(value)).expect_err("invalid Unicode must fail");
        assert_eq!(error.to_string(), "SITE_ORIGIN must contain valid Unicode");
    }
}
