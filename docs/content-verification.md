# Portfolio content verification

Checked 23 October 2026 against the public GitHub API and repository source. These are bounded source checks, not security audits or proof of production readiness. No repository metrics are stored or displayed.

## Featured projects

| Project | Source | What was checked | Publication limits |
| --- | --- | --- | --- |
| AirChainPay | [Main repository](https://github.com/rjaysolamo/airchainpay) | README, source tree, Solidity contracts, nested Rust relay and wallet core | Experimental/testnet positioning retained. Offline/Bluetooth capabilities are exploratory, not a verified production service. Relay TODO documents incomplete protobuf compression. |
| AirChainPay Rust relay | [Source directory](https://github.com/rjaysolamo/airchainpay/tree/main/airchainpay-relay-rust/airchainpay-relay) | Cargo manifest, Rust source tree, TODO.md | `airchainpay-relay-rust` is present inside the main repository. A separate repository of that name returned 404; no invented separate URL. |
| CredForge | [Repository](https://github.com/rjaysolamo/credforge) | README and `sources/credforge.move`: issue_credential, revoke_credential, is_valid_owner, is_trusted | Prototype. README reports testnet IDs, but no independent chain verification was performed. Revocation requires mutable access to wallet-owned credentials; issuer-side UI revocation has a documented limitation. |
| University Onchain Voting | [Repository](https://github.com/rjaysolamo/blockchin-voting) | GitHub fork metadata and `contracts/BlockchainVoting.sol`: createElection, registerCandidate, castVote, getResultsByPosition | Repository is a fork of RICKYLER/blockchin-voting. Personal contribution scope and deployment are not independently verified. No sole-authorship claim. |
| Proof of Contribution | Owner-supplied concept | No matching public repository found in the profile listing | Concept only; features are design scope, not completed functionality. No repository, demo, or contract URL fabricated. |

AirChainPay and CredForge homepage URLs from GitHub metadata returned HTTP 200. This only verifies availability of a page, not functional demos, wallet interactions, or deployed contracts. They are therefore not published as working demo links. Demo labels remain “coming soon.”

## Open-source repository entries

GitHub-reported primary language, not an assertion about every component:

| Repository | Primary language |
| --- | --- |
| [uwifin](https://github.com/rjaysolamo/uwifin) | Rust |
| [InDexAI](https://github.com/rjaysolamo/InDexAI) | TypeScript |
| [SqueezeAT](https://github.com/rjaysolamo/SqueezeAT) | Rust |
| [airchainpay](https://github.com/rjaysolamo/airchainpay) | TypeScript |
| [web3lab](https://github.com/rjaysolamo/web3lab) | TypeScript |
| [rjay](https://github.com/rjaysolamo/rjay) | TypeScript |

Descriptions come from GitHub metadata where available; web3lab and rjay use restrained descriptions of their purpose. No stars, follower counts, recent activity claims, or contribution graphs are shown. This avoids stale hardcoded metrics and does not require a runtime GitHub API dependency.

## Owner-supplied records

- Identity, email, availability, skills, and qualitative levels use the latest owner instructions. They are self-assessments, not objective measurements.
- Reverion Technology: Smart Contract Developer, December 2024–June 2025, Davao City. Uses latest supplied dates in place of earlier broad dates.
- Tekkio: Rust Developer. No dates newly confirmed, so earlier unconfirmed 2025–2026 dates are not presented as fact. Describes Rust infrastructure, CLI and node work; does not classify Tekkio as a blockchain company.
- Davao DeFi Devcore: Core Developer, community contribution; no dates invented.
- Web3 Davao: Co-Founder, community leadership; current affiliation is explicitly awaiting verification.
- Earlier BPAP, Base Philippines, Core DAO Philippines, and peaq Philippines affiliations remain available in an expandable community section, clearly pending confirmation and distinct from employment.
- Award records match [the owner's earlier public portfolio data](https://github.com/rjaysolamo/rjay/blob/main/data/achievements.ts): Champion — Mindanao Blockchain AI Hackathon; Second Place — Base Batches Mindanao; Third Place — Sui Builder Program Davao. Displayed as **owner-listed results**, with a source link and an explicit statement that independent event documentation has not been linked. No dates, prize amounts, or event URLs invented. Matching a personal portfolio record does not independently verify organizer results.
- LinkedIn URL [linkedin.com/in/rjaysolamo](https://www.linkedin.com/in/rjaysolamo) is reused from [the existing owner-authored site configuration](https://github.com/rjaysolamo/rjay/blob/main/lib/site.ts), not guessed. LinkedIn returned HTTP 999 to the automated availability check; profile availability and identity were not independently verified there.
- Existing technical notes are retained as planned topics, not published articles.

## Illustrations and interaction boundaries

- Hero network diagram is a conceptual illustration, not live blockchain data.
- Architecture order follows the requested conceptual model; the explanatory note states that verification can occur at multiple layers.
- Cryptographic formulae and Merkle proof flow are symbolic illustrations. No cryptographic operations run on user input.
- Expandable architecture layers and community history use semantic HTML `details`/`summary`, rendered by Rust and usable by keyboard without handwritten scripts.
- Mobile navigation state and Escape handling use Leptos Rust compiled to WebAssembly.
- Email links open the user's email application. There is no server contact form, message storage, or fake delivery confirmation.
