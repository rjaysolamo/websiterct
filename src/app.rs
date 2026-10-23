use crate::content::{EMAIL, GITHUB, Project};
use crate::observer::NetworkObserver;
use crate::sections::*;
use leptos::prelude::*;
use leptos_meta::{Meta, MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="theme-color" content="#101512"/>
                <link rel="icon" href="/favicon.svg" type="image/svg+xml"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body><App/></body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/rjay-portfolio.css"/>
        <Router>
            <Routes fallback=NotFound><Route path=StaticSegment("") view=Home/></Routes>
        </Router>
    }
}

#[component]
fn Header() -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <a class="skip-link" href="#main">"Skip to content"</a>
        <header class="header" on:keydown=move |event| {
            if event.key() == "Escape" { open.set(false); }
        }>
            <div class="container nav-wrap">
                <a href="/" class="wordmark" aria-label="Rjay Solamo home">
                    <span class="brand-icon" aria-hidden="true">"rs"<span>"."</span></span>
                    <span>"RJAY SOLAMO"<small>"RUST / WEB3 ENGINEERING"</small></span>
                </a>
                <button class="menu-toggle" aria-label="Toggle navigation" aria-controls="navigation"
                    aria-expanded=move || open.get().to_string()
                    on:click=move |_| open.update(|value| *value = !*value)>
                    {move || if open.get() { "Close −" } else { "Menu +" }}
                </button>
                <nav id="navigation" class:open=move || open.get() aria-label="Main navigation">
                    {[("About", "/#about"), ("Skills", "/#skills"), ("Projects", "/#projects"), ("Experience", "/#experience")]
                        .into_iter().map(|(name, href)| view! {
                            <a href=href on:click=move |_| open.set(false)>{name}</a>
                        }).collect_view()}
                    <a class="nav-contact" href="/#contact" on:click=move |_| open.set(false)>"Contact Me "<span aria-hidden="true">"↗"</span></a>
                </nav>
            </div>
        </header>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <Title text="Rjay Solamo — Web3 Engineer & Rust Developer"/>
        <Meta name="description" content="Rjay Solamo, Web3 Engineer and Blockchain Builder in Davao City. Rust, Solidity, backend systems, smart contracts, and cryptographic technologies. Open to opportunities."/>
        <Meta property="og:title" content="Rjay Solamo — Building Secure Systems for the Onchain World"/>
        <Meta property="og:description" content="Rust engineering, smart contracts, and the cryptographic foundations of reliable onchain systems. Explore projects, engineering philosophy, and open-source work."/>
        <Meta property="og:type" content="website"/>
        <Meta name="twitter:card" content="summary"/>
        <Header/>
        <main id="main">
            <Hero/>
            <About/>
            <Philosophy/>
            <TechnicalStack/>
            <Projects/>
            <Architecture/>
            <Cryptography/>
            <ExperienceSection/>
            <Achievements/>
            <CurrentFocus/>
            <OpenSource/>
            <Writing/>
            <Contact/>
        </main>
        <Footer/>
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="hero container" aria-labelledby="hero-title">
            <div class="hero-copy">
                <p class="eyebrow"><span class="live-dot" aria-hidden="true"></span>"OPEN TO OPPORTUNITIES"</p>
                <p class="hero-role">"Web3 Engineer / Blockchain Builder"</p>
                <h1 id="hero-title">"Building Secure"<br/>"Systems for the"<br/><em>"Onchain World."</em></h1>
                <p class="hero-intro">"I’m Rjay Solamo, a Web3 Engineer focused on "<strong>"Rust"</strong>", Solidity, smart contracts, backend systems, and cryptographic technologies."</p>
                <div class="hero-actions">
                    <a class="button primary" href="#projects">"View Projects "<span aria-hidden="true">"↗"</span></a>
                    <a class="button secondary" href=GITHUB>"GitHub "<span aria-hidden="true">"↗"</span></a>
                    <a class="text-link" href="#contact">"Contact Me "<span aria-hidden="true">"→"</span></a>
                </div>
                <p class="location"><span aria-hidden="true">"◎"</span>" Davao City, Philippines"<span class="location-divider" aria-hidden="true">"/"</span>"Rust at the core."</p>
            </div>
            <div class="hero-system">
                <figure class="hero-portrait">
                    <div class="portrait-frame"><img src="/rjay-solamo.jpg" alt="Portrait of Rjay Solamo" width="208" height="236" fetchpriority="high"/></div>
                    <figcaption><span>"RJAY SOLAMO"</span><p>"Software. Systems."<br/>"The layers underneath."</p><span class="mono muted">"Davao City · Philippines"</span></figcaption>
                </figure>
                <NetworkObserver/>
            </div>
        </section>
        <div class="tech-strip"><div class="container"><span class="mono">"ENGINEERING TOOLKIT"</span>
            {["Rust", "Solidity", "Axum", "Leptos", "Tokio", "EVM", "Cryptography"].into_iter().map(|technology| view! {
                <span class="tech" class:rust-accent=technology == "Rust"><span aria-hidden="true">"◇ "</span>{technology}</span>
            }).collect_view()}
        </div></div>
        <div class="container supporting-statement"><p>"I build software where "<span>"correctness, security,"</span>" and "<span>"decentralization"</span>" matter."</p><p>"My work spans the full path of an onchain system — from backend services and APIs, through smart contracts, down to the cryptographic primitives that make it trustworthy. I care about the layers most people abstract away."</p></div>
    }
}

#[component]
pub(crate) fn SectionHeading(
    number: &'static str,
    label: &'static str,
    title: &'static str,
    id: &'static str,
) -> impl IntoView {
    view! {
        <div class="eyebrow section-label"><span>{number}" /"</span>{label}</div>
        <h2 id=id>{title}</h2>
    }
}

#[component]
pub(crate) fn ProjectCard(project: Project, index: usize) -> impl IntoView {
    view! {
        <article class="project-card">
            <div class=format!("project-visual {}", project.visual) aria-hidden="true">
                <div class="visual-top"><span>{format!("PROJECT {:02}", index + 1)}</span><span>{project.category}</span></div>
                {match project.visual {
                    "payment" => view! { <div class="project-flow"><span>"WALLET"</span><i>"→"</i><strong>"SIGN"</strong><i>"→"</i><span>"RELAY"</span></div><p class="visual-caption">"OFFLINE INTENT / ONCHAIN SETTLEMENT"</p> }.into_any(),
                    "credential" => view! { <div class="credential-art"><span>"CREDENTIAL / SUI"</span><b>"{ owner, issuer, proof }"</b><span>"ISSUE → VERIFY"</span></div> }.into_any(),
                    "governance" => view! { <div class="project-flow"><span>"ELECTION"</span><i>"→"</i><strong>"VOTE"</strong><i>"→"</i><span>"STATE"</span></div><p class="visual-caption">"TRANSPARENT RULES / VERIFIABLE STATE"</p> }.into_any(),
                    _ => view! { <div class="project-flow"><span>"WORK"</span><i>"→"</i><strong>"PROOF"</strong><i>"→"</i><span>"REPUTATION"</span></div><p class="visual-caption">"A CONCEPT FOR VERIFIABLE CONTRIBUTIONS"</p> }.into_any(),
                }}
            </div>
            <div class="project-body">
                <p class="status">{project.status}</p>
                <h3>{project.name}</h3>
                <p class="project-description">{project.description}</p>
                <h4 class="feature-label">"Design scope"</h4>
                <ul class="feature-list">{project.features.iter().map(|feature| view! { <li>{*feature}</li> }).collect_view()}</ul>
                <p class="project-note">{project.note}</p>
                <div class="tags">{project.stack.iter().map(|technology| view! { <span class:rust-accent=*technology == "Rust">{*technology}</span> }).collect_view()}</div>
                <div class="project-links">
                    {project.repository.map(|href| view! { <a href=href aria-label=format!("View {} repository on GitHub", project.name)>"Repository ↗"</a> })}
                    {project.demo.map(|href| view! { <a href=href aria-label=format!("Open {} demo", project.name)>"Demo ↗"</a> }.into_any()).unwrap_or_else(|| view! { <span class="muted">"Demo coming soon"</span> }.into_any())}
                </div>
                {project.related_source.map(|(label, href)| view! { <a class="related-source" href=href>{label}" ↗"</a> })}
            </div>
        </article>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="footer"><div class="container">
            <div class="footer-top"><div><a href="/" class="footer-name">"RJAY SOLAMO"</a><p>"Web3 Engineer · Smart Contract Developer · Rust Developer"</p><p class="muted">"Building at the intersection of software, blockchain, and cryptography."</p></div><a class="text-link" href="#main">"Back to top ↑"</a></div>
            <nav aria-label="Footer navigation">
                {[("About", "/#about"), ("Projects", "/#projects"), ("Experience", "/#experience"), ("Skills", "/#skills"), ("Achievements", "/#achievements"), ("GitHub", GITHUB), ("Contact", "/#contact")].into_iter().map(|(label, href)| view! { <a href=href>{label}</a> }).collect_view()}
            </nav>
            <div class="footer-bottom"><span>"© 2026 Rjay Solamo. All rights reserved."</span><span>"Leptos SSR · Axum · Rust / WebAssembly · HTML & CSS"</span><a href=format!("mailto:{EMAIL}")>"Say hello ↗"</a></div>
        </div></footer>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    if let Some(response) = use_context::<leptos_axum::ResponseOptions>() {
        response.set_status(axum::http::StatusCode::NOT_FOUND);
    }
    view! {
        <Title text="Page not found — Rjay Solamo"/>
        <Meta name="robots" content="noindex"/>
        <Header/>
        <main id="main" class="container not-found">
            <div class="eyebrow">"404 / PAGE NOT FOUND"</div>
            <h1>"A path yet to be built."</h1>
            <p>"This page doesn’t exist. Head back to the portfolio to explore my work."</p>
            <a class="button primary" href="/">"Back to home ↗"</a>
        </main>
        <Footer/>
    }
}
