use crate::app::{ProjectCard, SectionHeading};
use crate::content::*;
use leptos::prelude::*;

#[component]
pub(crate) fn About() -> impl IntoView {
    view! {
        <section id="about" class="section about-section" aria-labelledby="about-title"><div class="container about-grid">
            <div><SectionHeading number="01" label="About" title="About Me" id="about-title"/>
                <blockquote>"Building secure systems for the onchain world."</blockquote>
                <p class="mono small muted">"SELF-TAUGHT. ALWAYS LEARNING."</p>
            </div>
            <div class="about-copy">
                <p class="large-copy">"I’m Rjay Solamo, a Web3 Engineer and Blockchain Developer focused on building secure, reliable, and practical decentralized systems."</p>
                <p>"My work spans Rust, Solidity, and blockchain application development, with experience across smart contracts, backend infrastructure, blockchain applications, and developer tooling. I enjoy working close to the protocol layer — understanding how systems work underneath the abstractions and turning complex technical problems into clean, maintainable solutions."</p>
                <p>"I’m also deeply interested in cryptography and secure systems, particularly zero-knowledge proofs, MPC, digital signatures, hashing, key management, and cryptographic protocols."</p>
                <p>"I believe great blockchain software isn’t just about putting something onchain — it’s about building systems that are secure, verifiable, efficient, and built to last."</p>
                <p>"I came to software without a computer-related degree, learning through independent study, practical projects, documentation, experimentation, and AI-assisted learning. Building and sharing knowledge remain at the heart of that journey."</p>
            </div>
        </div></section>
    }
}

#[component]
pub(crate) fn Philosophy() -> impl IntoView {
    view! {
        <section id="philosophy" class="section container" aria-labelledby="philosophy-title">
            <SectionHeading number="02" label="Engineering Philosophy" title="How I Build" id="philosophy-title"/>
            <div class="principles-grid">{PRINCIPLES.iter().enumerate().map(|(index, (title, description))| view! {
                <article class="principle"><span class="mono principle-number">{format!("0{}", index + 1)}</span><h3>{*title}</h3><p>{*description}</p></article>
            }).collect_view()}</div>
        </section>
    }
}

#[component]
pub(crate) fn TechnicalStack() -> impl IntoView {
    view! {
        <section id="skills" class="section stack-section" aria-labelledby="skills-title"><div class="container">
            <div class="section-top"><div><SectionHeading number="03" label="Technical Stack" title="Technologies I Build With" id="skills-title"/><p class="section-description">"Rust at the core. The right tools for each layer."</p></div><span class="section-aside mono">"SELF-DESCRIBED EXPERIENCE"</span></div>
            <div class="stack-layout"><div class="skills-grid">
                {SKILLS.iter().map(|group| view! {
                    <article class="skill-group"><h3>{group.name}</h3><dl>{group.skills.iter().map(|(name, level)| view! {
                        <div class:primary-skill=*name == "Rust"><dt>{*name}</dt><dd>{*level}</dd></div>
                    }).collect_view()}</dl></article>
                }).collect_view()}
            </div><aside class="developer-dashboard" aria-labelledby="dashboard-title">
                <div class="panel-header"><h3 id="dashboard-title">"Developer Dashboard"</h3><span aria-hidden="true">"⌘"</span></div>
                <p class="small muted">"A qualitative view of my current toolkit."</p>
                <div class="dashboard-list">{DASHBOARD.iter().map(|(name, label, marks)| view! {
                    <div class="dashboard-row"><div><strong class:rust-accent=*name == "Rust">{*name}</strong><span>{*label}</span></div><div class="skill-marks" aria-hidden="true">{(0..4).map(|index| view! { <span class:filled=index < *marks></span> }).collect_view()}</div></div>
                }).collect_view()}</div>
                <p class="dashboard-disclaimer">"* Bars represent qualitative proficiency, not fabricated percentages."</p>
                <p class="small muted">"TypeScript and JavaScript describe previous or existing project work. This website uses Rust for application logic, with HTML and custom CSS for presentation."</p>
                <div class="rust-stack"><span class="mono rust-accent">"THIS SITE’S RUST STACK"</span><p>"Leptos · Axum · Tokio · Serde"</p><p class="muted">"Also used in project work: PostgreSQL, SQL, Linux & Termux."</p></div>
            </aside></div>
        </div></section>
    }
}

#[component]
pub(crate) fn Projects() -> impl IntoView {
    view! {
        <section id="projects" class="section container" aria-labelledby="projects-title">
            <div class="section-top"><div><SectionHeading number="04" label="Featured Projects" title="Selected Work" id="projects-title"/><p class="section-description">"A portfolio of blockchain platforms, protocols, and experiments — spanning payments, identity, governance, and reputation."</p></div><a class="text-link" href="https://github.com/rjaysolamo?tab=repositories">"All repositories ↗"</a></div>
            <div class="projects">{PROJECTS.iter().copied().enumerate().map(|(index, project)| view! { <ProjectCard project index/> }).collect_view()}</div>
        </section>
    }
}

#[component]
pub(crate) fn Architecture() -> impl IntoView {
    view! {
        <section id="architecture" class="section architecture-section" aria-labelledby="architecture-title"><div class="container">
            <SectionHeading number="05" label="Project Architecture" title="From Code to Blockchain" id="architecture-title"/>
            <p class="section-description">"Every onchain action travels through a stack of layers. Here’s how I think about the path from a user’s intent to committed onchain state."</p>
            <div class="architecture-caption"><code>"intent → validation → execution → state"</code><span>"CONCEPTUAL MODEL"</span></div>
            <p class="small muted">"Select a layer to expand its explanation. Verification may happen at several layers; the sequence is a conceptual guide, not a universal execution order."</p>
            <ol class="architecture-flow">{LAYERS.iter().enumerate().map(|(index, (title, code, description))| view! {
                <li><details class="layer"><summary><span class="layer-number">{format!("0{}", index + 1)}</span><span class="layer-name">{*title}</span><code>{*code}</code><span class="expand-icon" aria-hidden="true">"+"</span></summary><p>{*description}</p></details></li>
            }).collect_view()}</ol>
        </div></section>
    }
}

#[component]
pub(crate) fn Cryptography() -> impl IntoView {
    view! {
        <section id="cryptography" class="section container" aria-labelledby="crypto-title">
            <SectionHeading number="06" label="Cryptography" title="Interested in the Mathematics Behind Security" id="crypto-title"/>
            <p class="section-description">"Cryptography sits at the intersection of mathematics, computer science, and security. I’m particularly interested in how cryptographic primitives can be used to build more secure decentralized systems."</p>
            <div class="crypto-grid"><div>
                <div class="crypto-examples"><div class="panel-header"><span class="mono">"PRIMITIVES / SYMBOLIC NOTATION"</span><span class="diagram-type">"ILLUSTRATION"</span></div>
                    <div class="formula"><code>"H(x) → digest"</code><span>"01 / HASH"</span></div>
                    <div class="formula"><code>"σ = Sign(sk, m)"</code><span>"02 / SIGN"</span></div>
                    <div class="formula"><code>"Verify(pk, m, σ) → true"</code><span>"03 / VERIFY"</span></div>
                    <div class="formula"><code>"Commit → Prove → Verify"</code><span>"04 / PROVE"</span></div>
                    <p class="small muted">"Illustrative notation only. No hashing, signing, or proof generation is executed here; verification succeeds only for valid inputs."</p>
                </div>
                <figure class="merkle-diagram"><figcaption>"MERKLE PROOF / CONCEPTUAL FLOW"</figcaption><div class="merkle-flow"><span>"Merkle Root"</span><i aria-hidden="true">"+"</i><span>"Leaf + Proof"</span><i aria-hidden="true">"→"</i><strong>"Verification"</strong></div><p>"Recompute the path from a leaf and its sibling hashes; compare the result with the trusted root."</p></figure>
            </div><dl class="crypto-concepts">{CRYPTO_CONCEPTS.iter().map(|(name, purpose)| view! { <div><dt>{*name}</dt><dd>{*purpose}</dd></div> }).collect_view()}</dl></div>
            <p class="editorial-note">"Presented as technical interests and active areas of development — not research credentials."</p>
        </section>
    }
}

#[component]
pub(crate) fn ExperienceSection() -> impl IntoView {
    view! {
        <section id="experience" class="section experience-section" aria-labelledby="experience-title"><div class="container">
            <SectionHeading number="07" label="Experience" title="Roles & Contributions" id="experience-title"/>
            <p class="section-description">"Rust infrastructure, smart contracts, and building with the developer community. Role descriptions supplied by Rjay; only supplied dates are shown."</p>
            <div class="experience-list">{EXPERIENCE.iter().map(|experience| view! {
                <article class="experience-row"><div class="experience-date"><span class="kind">{experience.kind}</span><p>{experience.date}</p>{(!experience.location.is_empty()).then(|| view! { <p>{experience.location}</p> })}</div><div><h3>{experience.role}<span>" — "{experience.organization}</span></h3><p>{experience.description}</p></div></article>
            }).collect_view()}</div>
            <details class="community-details"><summary>"Earlier community & ecosystem participation"<span class="muted">"Affiliations awaiting confirmation"</span></summary><div class="community-grid">{COMMUNITIES.iter().map(|(name, role)| view! { <article><h3>{*name}</h3><p>{*role}</p></article> }).collect_view()}</div><p class="small muted">"These are community roles, not employment. Dates and current affiliations have not been confirmed."</p></details>
        </div></section>
    }
}

#[component]
pub(crate) fn Achievements() -> impl IntoView {
    view! {
        <section id="achievements" class="section container" aria-labelledby="achievements-title">
            <SectionHeading number="08" label="Achievements" title="Built. Competed. Shipped." id="achievements-title"/>
            <p class="section-description">"Recognition from building and shipping at blockchain hackathons and builder programs."</p>
            <div class="achievements-grid">{ACHIEVEMENTS.iter().map(|(placement, event, tags)| view! {
                <article class="achievement"><span class="award-label mono">{*placement}</span><h3>{*event}</h3><div class="tags">{tags.iter().map(|tag| view! { <span>{*tag}</span> }).collect_view()}</div></article>
            }).collect_view()}</div>
            <p class="small muted source-note">"Owner-listed results, matching my "<a href="https://github.com/rjaysolamo/rjay/blob/main/data/achievements.ts">"existing public portfolio records ↗"</a>". Independent event documentation has not yet been linked."</p>
        </section>
    }
}

#[component]
pub(crate) fn CurrentFocus() -> impl IntoView {
    view! {
        <section id="focus" class="section focus-section" aria-labelledby="focus-title"><div class="container">
            <SectionHeading number="09" label="Current Focus" title="Currently Exploring" id="focus-title"/>
            <div class="focus-list">{FOCUS.iter().enumerate().map(|(index, (title, description))| view! { <article><span class="mono">{format!("0{}", index + 1)}</span><h3>{*title}</h3><p>{*description}</p><span class="focus-arrow" aria-hidden="true">"↗"</span></article> }).collect_view()}</div>
        </div></section>
    }
}

#[component]
pub(crate) fn OpenSource() -> impl IntoView {
    view! {
        <section id="open-source" class="section container" aria-labelledby="open-source-title">
            <div class="section-top"><div><SectionHeading number="10" label="Open Source" title="Code is the Portfolio." id="open-source-title"/><p class="section-description">"Explore the repositories, experiments, and systems behind the work."</p></div><a class="button secondary" href=GITHUB>"@rjaysolamo ↗"</a></div>
            <div class="repository-grid">{REPOSITORIES.iter().map(|repository| view! {
                <article class="repository"><div><h3><a href=repository.url>{repository.name}<span aria-hidden="true">" ↗"</span></a></h3><span class="repository-icon" aria-hidden="true">"{ }"</span></div><p>{repository.description}</p><span class="repo-language" class:rust-accent=repository.language == "Rust"><span aria-hidden="true">"● "</span>{repository.language}</span></article>
            }).collect_view()}</div>
            <p class="small muted source-note">"Repository links and primary languages checked against GitHub on 23 October 2026. Languages describe GitHub’s primary classification, not every component. No activity or popularity metrics are displayed."</p>
        </section>
    }
}

#[component]
pub(crate) fn Writing() -> impl IntoView {
    view! {
        <section id="writing" class="section writing-section" aria-labelledby="writing-title"><div class="container">
            <div class="eyebrow section-label">"TECHNICAL NOTES"</div><h2 id="writing-title">"Learning in the open."</h2><p class="section-description">"Research directions and lessons from building. No articles published here yet."</p>
            <div class="writing-list">{WRITING.iter().map(|(title, description)| view! { <article class="writing-row"><div><h3>{*title}</h3><p>{*description}</p></div><span class="draft-label">"PLANNED TOPIC"</span></article> }).collect_view()}</div>
        </div></section>
    }
}

#[component]
pub(crate) fn Contact() -> impl IntoView {
    view! {
        <section id="contact" class="section container contact-section" aria-labelledby="contact-title"><div>
            <SectionHeading number="11" label="Contact" title="Let’s Build Something Onchain." id="contact-title"/>
            <p class="large-copy">"Have a blockchain project, infrastructure problem, or technical idea worth exploring? Let’s talk."</p>
            <p>"I’m open to roles, collaborations, and interesting problems in blockchain, smart contracts, and backend systems. The fastest way to reach me is any channel below."</p>
            <a class="button primary" href=format!("mailto:{EMAIL}")>"Email Me ↗"</a>
        </div><address class="contact-channels"><span class="availability"><span class="live-dot" aria-hidden="true"></span>"Open to opportunities"</span><dl><div><dt>"EMAIL"</dt><dd><a href=format!("mailto:{EMAIL}")>{EMAIL}" ↗"</a></dd></div><div><dt>"GITHUB"</dt><dd><a href=GITHUB>"github.com/rjaysolamo ↗"</a></dd></div><div><dt>"LINKEDIN"</dt><dd><a href=LINKEDIN>"linkedin.com/in/rjaysolamo ↗"</a></dd></div><div><dt>"LOCATION"</dt><dd>"Davao City, Philippines"</dd></div></dl></address></section>
    }
}
