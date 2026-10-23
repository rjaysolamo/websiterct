use serde::Serialize;

pub const GITHUB: &str = "https://github.com/rjaysolamo";
pub const EMAIL: &str = "rjaysolamo@gmail.com";
// Reused from the owner's existing public portfolio configuration.
pub const LINKEDIN: &str = "https://www.linkedin.com/in/rjaysolamo";

/// Static, owner-supplied copy; repository links were checked on 2026-10-23.
#[derive(Clone, Copy, Serialize)]
pub struct Project {
    pub name: &'static str,
    pub category: &'static str,
    pub description: &'static str,
    pub features: &'static [&'static str],
    pub stack: &'static [&'static str],
    pub status: &'static str,
    pub note: &'static str,
    pub repository: Option<&'static str>,
    pub related_source: Option<(&'static str, &'static str)>,
    pub demo: Option<&'static str>,
    pub visual: &'static str,
}

pub const PROJECTS: &[Project] = &[
    Project {
        name: "AirChainPay",
        category: "Blockchain Payments",
        description: "A self-custodial multi-chain payment platform exploring offline transaction workflows and Bluetooth-based communication.",
        features: &[
            "Self-custodial architecture",
            "Multi-chain payment concepts",
            "Offline transaction workflows",
            "Bluetooth communication",
            "Cryptographic signing",
        ],
        stack: &[
            "Rust",
            "Solidity",
            "TypeScript",
            "Blockchain",
            "Cryptography",
        ],
        status: "Experimental · Testnet",
        note: "Offline workflows remain experimental. The Rust relay documents incomplete transaction compression; this is not a production payment service.",
        repository: Some("https://github.com/rjaysolamo/airchainpay"),
        related_source: Some((
            "Rust relay source",
            "https://github.com/rjaysolamo/airchainpay/tree/main/airchainpay-relay-rust/airchainpay-relay",
        )),
        demo: None,
        visual: "payment",
    },
    Project {
        name: "CredForge",
        category: "Decentralized Credentials",
        description: "A blockchain-based credential platform designed for issuing, verifying, and revoking digital credentials on Sui.",
        features: &[
            "Credential issuance",
            "Credential verification",
            "Revocation design",
            "Onchain registry",
            "Soulbound credentials",
        ],
        stack: &["Sui", "Move", "TypeScript"],
        status: "Prototype",
        note: "The repository documents a testnet deployment, not independently verified here. Issuer-side revocation has a documented ownership limitation.",
        repository: Some("https://github.com/rjaysolamo/credforge"),
        related_source: None,
        demo: None,
        visual: "credential",
    },
    Project {
        name: "University Onchain Voting",
        category: "Onchain Governance",
        description: "A decentralized university voting platform designed to provide transparent election management and blockchain-based voting.",
        features: &[
            "Election management",
            "Candidate registration",
            "Onchain voting",
            "Smart contracts",
            "Transparent election state",
        ],
        stack: &["Solidity", "TypeScript", "Smart Contracts"],
        status: "Prototype · Fork",
        note: "Linked source is a fork of RICKYLER/blockchin-voting. Election and voting methods are present; deployment and individual contribution scope are not verified.",
        repository: Some("https://github.com/rjaysolamo/blockchin-voting"),
        related_source: None,
        demo: None,
        visual: "governance",
    },
    Project {
        name: "Proof of Contribution",
        category: "Web3 Infrastructure",
        description: "A concept for transforming community contributions and completed work into verifiable onchain reputation.",
        features: &[
            "Proof of contribution",
            "Onchain reputation",
            "Community rewards",
            "Verifiable work history",
        ],
        stack: &["Solidity", "TypeScript", "Cryptography"],
        status: "Concept",
        note: "Proposed capabilities and technology direction. A public repository, working demo, and deployed contracts have not been verified.",
        repository: None,
        related_source: None,
        demo: None,
        visual: "reputation",
    },
];

pub struct SkillGroup {
    pub name: &'static str,
    pub skills: &'static [(&'static str, &'static str)],
}

pub const SKILLS: &[SkillGroup] = &[
    SkillGroup {
        name: "Languages",
        skills: &[
            ("Rust", "Primary"),
            ("Solidity", "Strong"),
            ("TypeScript", "Primary · project work"),
            ("JavaScript", "Strong · project work"),
            ("Move", "Working Knowledge"),
        ],
    },
    SkillGroup {
        name: "Blockchain",
        skills: &[
            ("Ethereum / EVM", "Strong"),
            ("Sui", "Working Knowledge"),
            ("Move", "Working Knowledge"),
            ("Smart Contracts", "Strong"),
            ("Web3", "Strong"),
            ("Onchain Applications", "Strong"),
        ],
    },
    SkillGroup {
        name: "Backend Engineering",
        skills: &[
            ("REST APIs", "Strong"),
            ("WebSockets", "Working Knowledge"),
            ("CLI Applications", "Strong"),
            ("Linux", "Strong"),
            ("Git", "Primary"),
            ("Backend Architecture", "Strong"),
        ],
    },
    SkillGroup {
        name: "Cryptography",
        skills: &[
            ("Hash Functions", "Strong"),
            ("SHA Family", "Strong"),
            ("Digital Signatures", "Strong"),
            ("secp256k1", "Working Knowledge"),
            ("EIP-712", "Working Knowledge"),
            ("Merkle Trees", "Strong"),
            ("Zero-Knowledge Proofs", "Exploring"),
            ("MPC", "Exploring"),
        ],
    },
];

// The discrete marks communicate self-described categories, never measured scores.
pub const DASHBOARD: &[(&str, &str, u8)] = &[
    ("Rust", "Primary", 4),
    ("Solidity", "Strong", 3),
    ("TypeScript", "Primary · project work", 4),
    ("Web3", "Primary", 4),
    ("Smart Contracts", "Strong", 3),
    ("Cryptography", "Working Knowledge", 2),
];

pub const PRINCIPLES: &[(&str, &str)] = &[
    (
        "Security First",
        "Design systems with security and failure scenarios in mind.",
    ),
    (
        "Understand the Protocol",
        "Don’t just use the abstraction. Understand what happens underneath it.",
    ),
    (
        "Simple Systems",
        "Prefer clear architecture over unnecessary complexity.",
    ),
    (
        "Verify Everything",
        "Test assumptions, validate data, and verify critical operations.",
    ),
];

pub const LAYERS: &[(&str, &str, &str)] = &[
    (
        "User",
        "intent",
        "Initiates an action and decides what to authorize.",
    ),
    (
        "Frontend",
        "interface",
        "Presents the interface and wallet interaction. Makes the intended action understandable before signing.",
    ),
    (
        "API / Backend",
        "coordinate",
        "Processes requests, validates inputs, and coordinates services without assuming the client is trusted.",
    ),
    (
        "Smart Contract",
        "rules",
        "Enforces onchain business rules and checks authorized state transitions.",
    ),
    (
        "Cryptographic Verification",
        "verify",
        "Validates signatures and proofs as required by the protocol.",
    ),
    (
        "Blockchain",
        "execute",
        "Executes and records transactions under the network’s consensus rules.",
    ),
    (
        "Onchain State",
        "commit",
        "Stores the resulting verifiable state.",
    ),
];

pub const CRYPTO_CONCEPTS: &[(&str, &str)] = &[
    ("Hash functions", "Integrity and commitments"),
    ("Digital signatures", "Authenticity"),
    ("Zero-knowledge proofs", "Privacy"),
    ("MPC", "Distributed trust"),
    ("Merkle trees", "Efficient proofs"),
    ("Finite fields", "Algebraic structure"),
    ("Secure key management", "Custody"),
];

pub struct Experience {
    pub role: &'static str,
    pub organization: &'static str,
    pub kind: &'static str,
    pub date: &'static str,
    pub location: &'static str,
    pub description: &'static str,
}

pub const EXPERIENCE: &[Experience] = &[
    Experience {
        role: "Rust Developer",
        organization: "Tekkio",
        kind: "Development",
        date: "Dates not supplied",
        location: "",
        description: "Rust backend systems, CLI development, node configuration, node logging, and statistics and query systems. Infrastructure and node-related development.",
    },
    Experience {
        role: "Smart Contract Developer",
        organization: "Reverion Technology",
        kind: "Development",
        date: "December 2024 – June 2025",
        location: "Davao City, Philippines",
        description: "Smart contracts, blockchain applications, Web3 integration, and backend systems.",
    },
    Experience {
        role: "Core Developer",
        organization: "Davao DeFi Devcore",
        kind: "Community contribution",
        date: "Dates not supplied",
        location: "Davao City, Philippines",
        description: "Blockchain development, Web3 architecture, decentralized applications, and the developer ecosystem. Technical education, talks, and workshops.",
    },
    Experience {
        role: "Co-Founder",
        organization: "Web3 Davao",
        kind: "Community leadership",
        date: "Current status to be verified",
        location: "Davao City, Philippines",
        description: "Web3 developer community, blockchain education, technical events, developer ecosystem, and blockchain projects. Current affiliation awaits confirmation.",
    },
];

pub const COMMUNITIES: &[(&str, &str)] = &[
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

pub const FOCUS: &[(&str, &str)] = &[
    (
        "Zero-Knowledge",
        "Privacy-preserving computation and proof systems.",
    ),
    (
        "MPC",
        "Secure multi-party computation and distributed cryptographic protocols.",
    ),
    (
        "Blockchain Infrastructure",
        "Reliable systems for decentralized applications.",
    ),
    (
        "Rust",
        "High-performance backend and blockchain infrastructure.",
    ),
    (
        "Smart Contract Security",
        "Safer contract architecture and verification.",
    ),
];

// Matches the owner's existing rjay/data/achievements.ts records.
// These are owner-listed placements, not independently verified event results.
pub const ACHIEVEMENTS: &[(&str, &str, &[&str])] = &[
    (
        "Champion",
        "Mindanao Blockchain AI Hackathon",
        &["Blockchain", "AI", "Hackathon"],
    ),
    (
        "Second Place",
        "Base Batches Mindanao",
        &["Blockchain", "AI", "Hackathon"],
    ),
    (
        "Third Place",
        "Sui Builder Program Davao",
        &["Sui", "Move", "Blockchain Development"],
    ),
];

pub struct Repository {
    pub name: &'static str,
    pub description: &'static str,
    pub language: &'static str,
    pub url: &'static str,
}

// Primary languages are GitHub's reported classifications, checked 2026-10-23.
// No counts, popularity metrics, activity graphs, or live API calls.
pub const REPOSITORIES: &[Repository] = &[
    Repository {
        name: "uwifin",
        description: "A Web3 financial application exploring cross-border transfers for Filipinos sending money home. Rust backend and Web3 settlement in development.",
        language: "Rust",
        url: "https://github.com/rjaysolamo/uwifin",
    },
    Repository {
        name: "InDexAI",
        description: "AI-assisted blockchain indexer for transaction tracing, wallet analysis, and crypto investigations. Investigation capabilities are in development.",
        language: "TypeScript",
        url: "https://github.com/rjaysolamo/InDexAI",
    },
    Repository {
        name: "SqueezeAT",
        description: "A local-first AI gateway for optimizing LLM API usage and supporting coding agents.",
        language: "Rust",
        url: "https://github.com/rjaysolamo/SqueezeAT",
    },
    Repository {
        name: "airchainpay",
        description: "Self-custodial multi-chain mobile wallet and payment experiments. Includes a Rust relay and wallet core.",
        language: "TypeScript",
        url: "https://github.com/rjaysolamo/airchainpay",
    },
    Repository {
        name: "web3lab",
        description: "Web3 application and infrastructure experiments. Explore the source for the current implementation.",
        language: "TypeScript",
        url: "https://github.com/rjaysolamo/web3lab",
    },
    Repository {
        name: "rjay",
        description: "Earlier portfolio source and web development work.",
        language: "TypeScript",
        url: "https://github.com/rjaysolamo/rjay",
    },
];

pub const WRITING: &[(&str, &str)] = &[
    (
        "Rust systems programming",
        "Learning through practical projects and understanding systems beneath the abstractions.",
    ),
    (
        "Following the flow onchain",
        "Blockchain investigation methodologies and EVM transaction tracing.",
    ),
    (
        "Building reliable backends",
        "Application architecture and smart contract security fundamentals.",
    ),
];
