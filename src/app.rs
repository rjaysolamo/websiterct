use crate::content::{Project, COMMUNITIES, EXPERIENCE, PROJECTS, SKILLS};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en"><head>
            <meta charset="utf-8"/>
            <meta name="viewport" content="width=device-width, initial-scale=1"/>
            <meta name="theme-color" content="#101210"/>
            <link rel="icon" href="/favicon.svg" type="image/svg+xml"/>
            <AutoReload options=options.clone()/><HydrationScripts options/><MetaTags/>
        </head><body><App/></body></html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/rjay-portfolio.css"/>
        <Router><Routes fallback=NotFound><Route path=StaticSegment("") view=Home/></Routes></Router>
    }
}

#[component]
fn Header() -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <a class="skip-link" href="#main">"Skip to content"</a>
        <header class="header"><div class="container nav-wrap">
            <a href="/" class="wordmark" aria-label="Rjay Solamo home"><span class="brand-icon">"r"<span>"s"</span>"."</span><span>"rjay solamo"<span class="muted">" / developer"</span></span></a>
            <button class="menu-toggle" aria-label="Toggle navigation" aria-controls="navigation" aria-expanded=move || open.get().to_string() on:click=move |_| open.update(|v| *v = !*v)> {move || if open.get() { "Close −" } else { "Menu +" }}</button>
            <nav id="navigation" class:open=move || open.get() aria-label="Main navigation">
                {[("About", "/#about"), ("Projects", "/#projects"), ("Experience", "/#experience"), ("Writing", "/#writing")].into_iter().map(|(name, href)| view! { <a href=href on:click=move |_| open.set(false)>{name}</a> }).collect_view()}
                <a class="nav-contact" href="/#contact" on:click=move |_| open.set(false)>"Let’s talk "<span>"↗"</span></a>
            </nav>
        </div></header>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <Title text="Rjay Solamo — Rust Developer & Web3 Builder"/>
        <Meta name="description" content="Rjay Solamo is a self-taught developer in Davao City, Philippines, building with Rust, backend systems, and blockchain infrastructure. Explore projects and technical interests."/>
        <Meta property="og:title" content="Rjay Solamo — Rust Developer & Web3 Builder"/>
        <Meta property="og:description" content="Practical tools. Reliable systems. Open-source software. Explore my work in Rust, backend engineering, and Web3."/>
        <Meta property="og:type" content="website"/>
        <Header/>
        <main id="main">
            <section class="hero container" aria-labelledby="hero-title">
                <div class="hero-copy"><div class="eyebrow"><span class="live-dot"></span>"BUILDING WITH PURPOSE"</div>
                    <h1 id="hero-title">"Rjay Solamo"<span>"Code. Understand."<br/>"Build something "<em>"real."</em></span></h1>
                    <p class="hero-role">"Rust Developer "<span>"·"</span>" Backend Engineer "<span>"·"</span>" Web3 Builder"</p>
                    <p class="hero-intro">"I’m a self-taught software developer focused on Rust, backend systems, blockchain infrastructure, and open-source software. I build practical tools and explore how reliable systems can solve real-world problems."</p>
                    <div class="hero-actions"><a class="button primary" href="#projects">"View projects "<span>"↗"</span></a><a class="button secondary" href="https://github.com/rjaysolamo">"Explore GitHub "<span>"↗"</span></a><a class="text-link" href="#contact">"Contact me"</a></div>
                    <div class="location"><span>"◎"</span>" Davao City, Philippines "<i></i>" Learning. Building. Sharing."</div>
                </div>
                <figure class="hero-portrait">
                    <div class="portrait-frame"><img src="/rjay-solamo.jpg" alt="Portrait of Rjay Solamo" width="208" height="236" fetchpriority="high"/></div>
                    <figcaption><span>"Rjay Solamo"</span><span>"Davao City, Philippines"</span></figcaption>
                </figure>
            </section>
            <div class="tech-strip"><div class="container"><span class="mono">"IN THE TOOLKIT"</span>{["Rust", "Axum", "Leptos", "Solidity", "TypeScript", "PostgreSQL", "Web3"].into_iter().map(|t| view! { <span class="tech"><span class="tech-symbol">"◇"</span>{t}</span> }).collect_view()}</div></div>
            <section id="projects" class="section container">
                <SectionHeading number="01" label="SELECTED WORK" title="Ideas, turned into software."/>
                <div class="section-intro"><p>"A collection of things I’m building, exploring, and learning from."</p><a class="text-link" href="https://github.com/rjaysolamo?tab=repositories">"All repositories ↗"</a></div>
                <div class="projects">{PROJECTS.iter().copied().map(|project| view! { <ProjectCard project/> }).collect_view()}</div>
                <div class="tools-note"><span class="tool-symbol">">_"</span><div><h3>"Rust CLI & infrastructure tools"</h3><p>"An ongoing area of interest: command-line utilities, node tooling, logging, and backend infrastructure. Specific tools will be added when their scope and contributions are verified."</p></div><span class="mono">"EXPLORING"</span></div>
            </section>
            <section id="about" class="section about-section"><div class="container about-grid"><div><SectionHeading number="02" label="THE DEVELOPER" title="A self-taught path. A builder’s mindset."/><div class="about-location">"Based in Davao City"<br/><span>"Building beyond boundaries."</span></div></div><div class="about-copy"><p class="large-copy">"I didn’t start with a computer-related degree. I started with curiosity—and kept building."</p><p>"Independent study, documentation, experimentation, and AI-assisted learning brought me into software development. Practical projects are where I turn that learning into understanding."</p><p>"Today, I’m focused on systems programming with Rust, backend APIs, and blockchain infrastructure. I’m interested in how systems work beneath the surface: how data moves, how contracts behave, and how software becomes reliable."</p><p>"Open-source collaboration and sharing technical knowledge are part of that journey. There’s always something more to learn."</p><a class="text-link" href="https://github.com/rjaysolamo">"Follow the work on GitHub ↗"</a></div></div>
                <div class="container skills-grid">{SKILLS.iter().enumerate().map(|(i, (name, skills))| view! { <div class="skill-group"><span class="skill-number">{format!("0{} /", i + 1)}</span><h3>{*name}</h3><ul>{skills.iter().map(|s| view! { <li>{*s}</li> }).collect_view()}</ul></div> }).collect_view()}</div>
            </section>
            <section id="experience" class="section container"><SectionHeading number="03" label="EXPERIENCE & COMMUNITY" title="Building alongside others."/><p class="section-description">"Software development, open-source collaboration, and community knowledge sharing."</p><p class="editorial-note">"History awaiting confirmation: the dates, roles, responsibilities, and affiliations below are user-supplied and must be confirmed before publication."</p><div class="experience-grid"><div><h3 class="column-label">"DEVELOPMENT"</h3>{EXPERIENCE.iter().map(|(date, name, role)| view! { <article class="experience-row"><span class="mono">{*date}</span><div><h4>{*name}</h4><p>{*role}</p><span class="pending">"To be confirmed"</span></div></article> }).collect_view()}</div><div><h3 class="column-label">"COMMUNITY · NOT EMPLOYMENT"</h3>{COMMUNITIES.iter().map(|(name, role)| view! { <article class="community-row"><span class="small-cross">"+"</span><div><h4>{*name}</h4><p>{*role}</p></div></article> }).collect_view()}<p class="muted small">"Dates and current affiliations to be confirmed."</p></div></div></section>
            <section id="writing" class="section writing-section"><div class="container"><SectionHeading number="04" label="NOTES & EXPLORATIONS" title="Learning in the open."/><p class="section-description">"A space for technical notes, research, and lessons from building. No articles published here yet."</p><div class="writing-list">{[("01", "Rust systems programming", "From practical projects to a deeper understanding of systems."), ("02", "Following the flow on-chain", "Blockchain investigation methodologies and EVM transaction tracing."), ("03", "Building reliable backends", "Application architecture and smart contract security fundamentals.")].into_iter().map(|(n,t,d)| view! { <article class="writing-row"><span class="mono">{n}</span><div><h3>{t}</h3><p>{d}</p></div><span class="draft-label">"PLANNED TOPIC"</span></article> }).collect_view()}</div></div></section>
            <section id="contact" class="section container contact-section"><div><div class="eyebrow">"05 / GET IN TOUCH"</div><h2>"Good software starts"<br/>"with a "<em>"conversation."</em></h2><p>"Have a Rust project, an interesting systems problem, or an open-source idea? Let’s connect."</p><a class="button primary" href="https://github.com/rjaysolamo">"Connect on GitHub ↗"</a></div><aside><span class="mono">"GITHUB & OPEN SOURCE"</span><a class="github-handle" href="https://github.com/rjaysolamo">"@rjaysolamo ↗"</a><p>"Explore my repositories, follow the work, or contribute to a project."</p><p class="small muted">"Additional contact channels will be added when confirmed."</p></aside></section>
        </main><footer class="container footer"><a class="footer-name" href="/">"rs."<span>" Rjay Solamo"</span></a><span>"Built with Rust, Leptos & a little curiosity."</span><a href="#main">"Back to top ↑"</a></footer>
    }
}

#[component]
fn SectionHeading(number: &'static str, label: &'static str, title: &'static str) -> impl IntoView {
    view! { <div class="eyebrow section-label"><span>{number}" /"</span>{label}</div><h2>{title}</h2> }
}

#[component]
fn ProjectCard(project: Project) -> impl IntoView {
    view! { <article class="project-card">
        <div class=format!("project-visual {}", project.visual) aria-hidden="true"><div class="visual-top"><span>{project.category}</span><span>"↗"</span></div>
            {match project.visual {
                "index" => view! { <div class="flow-art"><span class="flow-node node-a">"0x…a1"</span><span class="flow-node node-b">"0x…8f"</span><span class="flow-node node-center">"◎"</span><span class="flow-node node-c">"0x…2d"</span><span class="flow-node node-d">"0x…6e"</span><svg viewBox="0 0 400 150" preserveAspectRatio="none"><path d="M65 40L200 75L330 35M70 120L200 75L330 120" fill="none" stroke="currentColor" stroke-width="1" stroke-dasharray="4 5"/></svg></div> }.into_any(),
                "uwi" => view! { <div class="remittance-art"><span class="country">"WORLD"<b>"◎"</b></span><div class="transfer-line"><span>"→ → →"</span><b>"uwi"<i>"fin"</i></b></div><span class="country">"HOME"<b>"⌂"</b></span></div> }.into_any(),
                _ => view! { <div class="payment-art"><div class="signal-ring"></div><div class="signal-ring second"></div><div class="payment-symbol">"A"<span>"↗"</span></div><span class="payment-caption">"CONNECT BEYOND CONNECTION"</span></div> }.into_any(),
            }}
        </div><div class="project-body"><div class="project-title"><h3>{project.name}</h3><span class="status"><span></span>{project.status}</span></div><p>{project.description}</p><div class="tags">{project.stack.iter().map(|t| view! { <span>{*t}</span> }).collect_view()}</div><div class="project-links">{project.repository.map(|href| view! { <a href=href aria-label=format!("View {} repository on GitHub", project.name)>"View repository ↗"</a> })}{project.demo.map(|href| view! { <a href=href>"Live demo ↗"</a> })}</div></div>
    </article> }
}

#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    if let Some(response) = use_context::<leptos_axum::ResponseOptions>() {
        response.set_status(axum::http::StatusCode::NOT_FOUND);
    }
    view! { <Title text="Page not found — Rjay Solamo"/><Meta name="robots" content="noindex"/><Header/><main id="main" class="container not-found"><div class="eyebrow">"404 / PAGE NOT FOUND"</div><h1>"A path yet to be built."</h1><p>"This page doesn’t exist. Head back to the portfolio to explore my work."</p><a class="button primary" href="/">"Back to home ↗"</a></main> }
}
