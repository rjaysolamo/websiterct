use serde::Serialize;

/// Portfolio copy is static; no external API or database is needed.
#[derive(Clone, Copy, Serialize)]
pub struct Project {
    pub name: &'static str,
    pub category: &'static str,
    pub description: &'static str,
    pub stack: &'static [&'static str],
    pub status: &'static str,
    pub repository: Option<&'static str>,
    pub demo: Option<&'static str>,
    pub visual: &'static str,
}

pub const PROJECTS: &[Project] = &[
    Project { name: "InDexAI", category: "BLOCKCHAIN INTELLIGENCE", description: "Making on-chain activity easier to understand. An open-source investigation platform exploring transaction indexing, address relationships, and fund-flow analysis. Investigation labels and visual tracing are planned capabilities.", stack: &["Rust", "Axum", "PostgreSQL", "EVM RPC", "Next.js", "TypeScript"], status: "In development", repository: Some("https://github.com/rjaysolamo/InDexAI"), demo: None, visual: "index" },
    Project { name: "UwiFin", category: "CROSS-BORDER FINTECH", description: "An exploration of simpler cross-border transfers for Filipinos sending money home. The technical direction combines a Rust backend with Web3 settlement. In development; no production transfer service is claimed.", stack: &["Rust", "Next.js", "TypeScript", "Solidity", "MariaDB"], status: "In development", repository: Some("https://github.com/rjaysolamo/uwifin"), demo: None, visual: "uwi" },
    Project { name: "AirChainPay", category: "EXPERIMENTAL PAYMENTS", description: "Exploring offline-first payment interactions through QR codes, Bluetooth Low Energy, and EVM-compatible test networks. An experimental testnet wallet, with offline interactions under exploration.", stack: &["Rust", "React Native", "Solidity", "EVM"], status: "Testnet experiment", repository: Some("https://github.com/rjaysolamo/airchainpay"), demo: None, visual: "air" },
];

pub const SKILLS: &[(&str, &[&str])] = &[
    (
        "Systems & backend",
        &[
            "Rust",
            "Axum",
            "Tokio",
            "REST APIs",
            "Asynchronous programming",
        ],
    ),
    (
        "Web development",
        &["Leptos", "TypeScript", "JavaScript", "HTML & CSS"],
    ),
    (
        "Blockchain & Web3",
        &[
            "Solidity",
            "EVM networks",
            "Smart contracts",
            "DeFi concepts",
            "Blockchain data analysis",
            "Fund-flow investigation",
        ],
    ),
    (
        "Infrastructure & tools",
        &["Git & GitHub", "PostgreSQL", "SQL", "Linux & Termux"],
    ),
];

// User-supplied history, explicitly awaiting confirmation before publication.
pub const EXPERIENCE: &[(&str, &str, &str)] = &[
    (
        "2025–2026",
        "Tekkio, LLC",
        "Rust development & cluster-node tooling",
    ),
    (
        "2024–2025",
        "Reverion Technology",
        "Smart contract & backend development",
    ),
];
pub const COMMUNITIES: &[(&str, &str)] = &[
    (
        "Davao DeFi Devcore",
        "Technical education, talks & workshops",
    ),
    (
        "Blockchain Practitioners Association of the Philippines",
        "Community management",
    ),
    ("Base Philippines", "Community contribution"),
    (
        "Core DAO Philippines",
        "Regional ambassador & ecosystem engagement",
    ),
    ("peaq Philippines", "Ambassador & ecosystem participation"),
];
