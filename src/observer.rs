//! A deterministic, local illustration. No RPC traffic or cryptographic execution.
use leptos::prelude::*;

const STEP_SECONDS: f32 = 1.6;
const EVENTS: [&str; 12] = [
    "Peer connection established",
    "Transaction received at RPC",
    "Transaction propagated to peer",
    "Validator received transaction",
    "SHA256 digest illustrated",
    "Signature creation illustrated",
    "Signature verification illustrated",
    "Block validation completed (simulated)",
    "Block #2049 formed",
    "Transaction confirmed",
    "Onchain state updated",
    "Cycle complete; replay follows",
];
const PHASES: [&str; 4] = ["PENDING", "PROPAGATING", "VERIFYING", "CONFIRMED"];
const CRYPTO: [&str; 4] = ["SHA256(...)", "SIGN(...)", "VERIFY(...)", "VALID"];
const NODES: [(&str, &str, &str); 5] = [
    (
        "RPC",
        "NODE 01",
        "RPC endpoint · receives the sample transaction",
    ),
    (
        "PEER",
        "NODE 02",
        "Peer · propagates the sample transaction",
    ),
    (
        "VERIFY",
        "VALIDATOR",
        "Validator · illustrates signature and block checks",
    ),
    (
        "BLOCK",
        "BUILDER",
        "Block builder · groups the verified sample into a block",
    ),
    (
        "ONCHAIN",
        "STATE",
        "State replica · illustrates the committed result",
    ),
];
// Indices refer to NODES. The four transaction hops use edges 0, 2, 4 and 5.
const EDGES: [(usize, usize, &str); 7] = [
    (0, 1, "M55 70L190 42"),
    (0, 2, "M55 70L190 170"),
    (1, 2, "M190 42V170"),
    (1, 3, "M190 42L345 70"),
    (2, 3, "M190 170L345 70"),
    (3, 4, "M345 70V188"),
    (2, 4, "M190 170L345 188"),
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Simulation {
    step: usize,
}

impl Simulation {
    fn advance(&mut self) {
        self.step = (self.step + 1) % EVENTS.len();
    }

    fn phase(self) -> usize {
        match self.step {
            0..=1 => 0,
            2..=3 => 1,
            4..=8 => 2,
            _ => 3,
        }
    }

    fn crypto_stage(self) -> Option<usize> {
        (self.step >= 4).then(|| (self.step - 4).min(3))
    }

    fn active_node(self) -> usize {
        match self.step {
            0..=1 => 0,
            2 => 1,
            3..=7 => 2,
            8 => 3,
            _ => 4,
        }
    }

    fn active_edge(self) -> Option<usize> {
        match self.step {
            2 => Some(0),
            3 => Some(2),
            8 => Some(4),
            9 => Some(5),
            _ => None,
        }
    }

    fn latest_block(self) -> u16 {
        if self.step >= 8 { 2049 } else { 2048 }
    }

    fn recent_events(self) -> impl Iterator<Item = (usize, &'static str)> {
        EVENTS
            .iter()
            .copied()
            .enumerate()
            .take(self.step + 1)
            .skip(self.step.saturating_sub(4))
    }
}

#[component]
pub(crate) fn NetworkObserver() -> impl IntoView {
    let simulation = RwSignal::new(Simulation::default());
    let paused = RwSignal::new(false);
    let visible = RwSignal::new(true);
    let foreground = RwSignal::new(true);
    let reduced = RwSignal::new(false);
    let ready = RwSignal::new(false);
    let selected = RwSignal::new(None::<usize>);
    let inspected = RwSignal::new(None::<usize>);
    let log_view = RwSignal::new(false);
    let transaction_open = RwSignal::new(false);
    let generation = RwSignal::new(0_u32);
    let panel = NodeRef::<leptos::html::Section>::new();
    let running =
        move || ready.get() && !paused.get() && visible.get() && foreground.get() && !reduced.get();

    #[cfg(feature = "hydrate")]
    panel.on_load(move |element| {
        connect_browser(element.into(), visible, foreground, reduced);
        ready.set(true);
    });

    view! {
        <section class="observer" node_ref=panel aria-labelledby="observer-title"
            data-running=move || running().to_string()
            data-step=move || simulation.get().step.to_string()
            data-phase=move || simulation.get().phase().to_string()
            class:has-selection=move || selected.get().is_some()>
            <div class="observer-header">
                <div><code id="observer-title">"network.observe()"</code><p>"NETWORK OBSERVER"</p></div>
                <span class="observer-badge">"SIMULATION"</span>
            </div>
            <div class="observer-toolbar">
                <div class="observer-views" role="group" aria-label="Observer view">
                    <button type="button" aria-pressed=move || (!log_view.get()).to_string()
                        disabled=move || !ready.get() on:click=move |_| log_view.set(false)>"Network"</button>
                    <button type="button" aria-pressed=move || log_view.get().to_string()
                        disabled=move || !ready.get() on:click=move |_| log_view.set(true)>"Event log"</button>
                </div>
                <span class="observer-runtime" role="status">{move || {
                    if !ready.get() { "Static preview" }
                    else if reduced.get() { "Manual · reduced motion" }
                    else if paused.get() { "Paused" }
                    else if !visible.get() || !foreground.get() { "Auto-paused" }
                    else { "Running" }
                }}</span>
            </div>
            <For each=move || [generation.get()] key=|key| *key children=move |_| view! {
                <div class="observer-cycle">
                    // CSS supplies the clock; the Rust handler advances the bounded model.
                    // Pausing CSS freezes both the visual and the next state transition.
                    <span class="observer-clock" aria-hidden="true"
                        on:animationiteration=move |_| {
                            if running() { simulation.update(Simulation::advance); }
                        }></span>
                    <div class="observer-network-view" hidden=move || log_view.get()>
                        <div class="observer-graph" role="group" aria-label="Simulated blockchain nodes">
                            <svg viewBox="0 0 420 230" preserveAspectRatio="none" aria-hidden="true">
                                {EDGES.into_iter().enumerate().map(|(index, (from, to, path))| view! {
                                    <path d=path class="observer-edge"
                                        class:connected=move || selected.get().is_some_and(|node| node == from || node == to)
                                        class:carrying=move || simulation.get().active_edge() == Some(index)/>
                                }).collect_view()}
                                <circle r="3" cx="55" cy="70" class="observer-particle hop-one"/>
                                <circle r="3" cx="190" cy="42" class="observer-particle hop-two"/>
                                <circle r="3" cx="190" cy="170" class="observer-particle hop-three"/>
                                <circle r="3" cx="345" cy="70" class="observer-particle hop-four"/>
                            </svg>
                            {NODES.into_iter().enumerate().map(|(index, (name, label, role))| view! {
                                <button type="button" class="observer-node" data-node=index
                                    title=role aria-label=format!("{name}: {role}. Select to highlight connections.")
                                    aria-pressed=move || (selected.get() == Some(index)).to_string()
                                    disabled=move || !ready.get()
                                    class:active=move || simulation.get().active_node() == index
                                    on:mouseenter=move |_| inspected.set(Some(index))
                                    on:mouseleave=move |_| inspected.set(None)
                                    on:focus=move |_| inspected.set(Some(index))
                                    on:blur=move |_| inspected.set(None)
                                    on:click=move |_| selected.update(|value| *value = if *value == Some(index) { None } else { Some(index) })>
                                    <span>{name}</span><small>{label}</small>
                                </button>
                            }).collect_view()}
                            <span class="observer-graph-label">"PEER CONNECTION / TRANSACTION FLOW"</span>
                        </div>
                        <p class="observer-node-detail" aria-live="polite">{move || inspected.get().or(selected.get()).map(|node| NODES[node].2)
                            .unwrap_or("Select a node to inspect its role and connections.")}</p>
                        <div class="observer-chain" aria-label="Illustrative block creation">
                            <span>"BLOCK"<strong>"#2047"</strong></span><i aria-hidden="true">"→"</i>
                            <span>"BLOCK"<strong>"#2048"</strong></span><i class:committed=move || { simulation.get().step >= 8 } aria-hidden="true">"→"</i>
                            <span class="observer-new-block" class:committed=move || { simulation.get().step >= 8 }>
                                {move || if simulation.get().step >= 8 { "NEW BLOCK" } else { "FORMING" }}
                                <strong>{move || if simulation.get().step >= 8 { "#2049" } else { "· · ·" }}</strong>
                            </span>
                        </div>
                    </div>
                    <div class="observer-log-view" hidden=move || !log_view.get()>
                        <div class="observer-log-heading"><span>"RECENT OBSERVER EVENTS"</span><span>"SIMULATED T+"</span></div>
                        <ol class="observer-log" aria-label="Recent simulated events, elapsed simulation time">
                            {move || simulation.get().recent_events().map(|(index, event)| view! {
                                <li><time>{format!("T+{:04.1}s", index as f32 * STEP_SECONDS)}</time><span>{event}</span></li>
                            }).collect_view()}
                        </ol>
                        <p class="observer-log-note">"A fixed 12-step replay. Last five events shown; no real timestamps or network traffic."</p>
                    </div>
                </div>
            }/>
            <div class="observer-transaction">
                <button type="button" class="observer-tx-button" disabled=move || !ready.get()
                    aria-expanded=move || transaction_open.get().to_string() aria-controls="observer-tx-details"
                    on:click=move |_| transaction_open.update(|value| *value = !*value)>
                    <span>"TX "<code>"0x7a9f…"</code></span><span>{move || PHASES[simulation.get().phase()]}" ↗"</span>
                </button>
                <ol class="observer-lifecycle" aria-label="Illustrative transaction lifecycle">
                    {PHASES.into_iter().enumerate().map(|(index, phase)| view! {
                        <li class:current=move || simulation.get().phase() == index
                            class:complete=move || { simulation.get().phase() > index }>{phase}</li>
                    }).collect_view()}
                </ol>
                <div id="observer-tx-details" class="observer-tx-details" hidden=move || !transaction_open.get()>
                    <p><code>"transaction.id: 0x7a9f…"</code>" / illustrative identifier"</p>
                    <p>"RPC → peer → validator → block → onchain state. The same sample replays; no funds move."</p>
                    <p>{move || EVENTS[simulation.get().step]}</p>
                </div>
            </div>
            <div class="observer-verification">
                <div class="observer-log-heading"><span>"BLOCK VALIDATION"</span><span>"ILLUSTRATIVE"</span></div>
                <ol aria-label="Illustrative cryptographic stages">
                    {CRYPTO.into_iter().enumerate().map(|(index, stage)| view! {
                        <li class:verified=move || simulation.get().crypto_stage().is_some_and(|current| index <= current)
                            class:checking=move || simulation.get().crypto_stage() == Some(index)>{stage}</li>
                    }).collect_view()}
                </ol>
            </div>
            <dl class="observer-metrics" aria-label="Simulated network metadata">
                <div><dt>"Nodes"</dt><dd>{NODES.len()}</dd></div>
                <div><dt>"Connections"</dt><dd>{EDGES.len()}</dd></div>
                <div><dt>"TX observed"</dt><dd>{move || usize::from(simulation.get().step > 0)}" / replay"</dd></div>
                <div><dt>"Latest block"</dt><dd>"#"{move || simulation.get().latest_block()}</dd></div>
            </dl>
            <div class="observer-hash"><code>"block.hash: "{move || if simulation.get().step >= 8 { "0xb10c…2049" } else { "0xb10c…2048" }}</code>
                <span>{move || if simulation.get().step >= 7 { "VALID · SIMULATED" } else { "AWAITING CHECK" }}</span></div>
            <div class="observer-controls" role="group" aria-label="Simulation playback">
                <button type="button" class="observer-pause" disabled=move || !ready.get() || reduced.get()
                    aria-pressed=move || paused.get().to_string() on:click=move |_| paused.update(|value| *value = !*value)>
                    {move || if paused.get() { "Resume" } else { "Pause" }}
                </button>
                <button type="button" disabled=move || !ready.get() on:click=move |_| {
                    paused.set(true);
                    simulation.update(Simulation::advance);
                    generation.update(|value| *value = value.wrapping_add(1));
                }>"Next step"</button>
                <button type="button" disabled=move || !ready.get() on:click=move |_| {
                    paused.set(true);
                    simulation.set(Simulation::default());
                    selected.set(None);
                    inspected.set(None);
                    transaction_open.set(false);
                    log_view.set(false);
                    generation.update(|value| *value = value.wrapping_add(1));
                }>"Reset"</button>
                <span>"LOCAL REPLAY"</span>
            </div>
            <p class="observer-disclaimer">"Simulated data & cryptography · No RPC connection, signing, or hashing. Motion pauses offscreen. Use Next step for a static walkthrough."</p>
            <noscript><p class="observer-disclaimer">"Static preview. Enable the browser’s script support to load the Rust/WASM controls."</p></noscript>
        </section>
    }
}

/// Browser APIs are bound from Rust/WASM; no hand-written JavaScript or polling.
#[cfg(feature = "hydrate")]
fn connect_browser(
    element: web_sys::Element,
    visible: RwSignal<bool>,
    foreground: RwSignal<bool>,
    reduced: RwSignal<bool>,
) {
    use leptos::__reexports::send_wrapper::SendWrapper;
    use wasm_bindgen::{JsCast, closure::Closure};
    use web_sys::{IntersectionObserver, IntersectionObserverEntry, js_sys::Array};

    let intersection = Closure::<dyn FnMut(Array)>::new(move |entries: Array| {
        if let Some(entry) = entries.get(0).dyn_ref::<IntersectionObserverEntry>() {
            visible.set(entry.is_intersecting());
        }
    });
    let observer = match IntersectionObserver::new(intersection.as_ref().unchecked_ref()) {
        Ok(observer) => Some(observer),
        Err(_) => {
            leptos::logging::warn!(
                "Observer visibility API unavailable; offscreen pausing is disabled."
            );
            None
        }
    };
    if let Some(observer) = &observer {
        observer.observe(&element);
    }
    let media = match window().match_media("(prefers-reduced-motion: reduce)") {
        Ok(Some(query)) => Some(query),
        Ok(None) | Err(_) => {
            leptos::logging::warn!(
                "Motion preference API unavailable; using the manual observer walkthrough."
            );
            None
        }
    };
    reduced.set(media.as_ref().is_none_or(|query| query.matches()));
    let media_query = media.clone();
    let motion_change = Closure::<dyn FnMut()>::new(move || {
        reduced.set(media_query.as_ref().is_none_or(|query| query.matches()));
    });
    if let Some(query) = &media {
        query.set_onchange(Some(motion_change.as_ref().unchecked_ref()));
    }
    foreground.set(!document().hidden());
    let visibility = window_event_listener(leptos::ev::visibilitychange, move |_| {
        foreground.set(!document().hidden());
    });
    // Keep callbacks alive until unmount, then disconnect and drop them together.
    let resources = SendWrapper::new((observer, intersection, media, motion_change, visibility));
    on_cleanup(move || {
        let (observer, _intersection, media, _motion_change, visibility) = resources.take();
        if let Some(observer) = observer {
            observer.disconnect();
        }
        if let Some(media) = media {
            media.set_onchange(None);
        }
        visibility.remove();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_replays_without_growing_state_or_log() {
        let mut simulation = Simulation::default();
        for step in 0..120 {
            assert_eq!(simulation.step, step % EVENTS.len());
            let events: Vec<_> = simulation.recent_events().collect();
            assert!(events.len() <= 5);
            assert_eq!(events.last().map(|event| event.0), Some(simulation.step));
            simulation.advance();
        }
        assert_eq!(simulation, Simulation::default());
    }

    #[test]
    fn verification_precedes_block_and_confirmation() {
        let mut simulation = Simulation::default();
        let mut phases = Vec::new();
        for _ in 0..EVENTS.len() {
            phases.push(simulation.phase());
            if simulation.crypto_stage() != Some(3) {
                assert_eq!(simulation.latest_block(), 2048);
                assert_ne!(simulation.phase(), 3);
            }
            if simulation.phase() == 3 {
                assert_eq!(simulation.latest_block(), 2049);
                assert_eq!(simulation.crypto_stage(), Some(3));
            }
            simulation.advance();
        }
        phases.dedup();
        assert_eq!(phases, vec![0, 1, 2, 3]);
    }

    #[test]
    fn transaction_hops_are_connected_and_end_at_onchain_state() {
        let mut destination = 0;
        for step in [2, 3, 8, 9] {
            let simulation = Simulation { step };
            let (from, to, _) = EDGES[simulation.active_edge().unwrap()];
            assert_eq!(from, destination);
            destination = to;
            assert_eq!(simulation.active_node(), to);
        }
        assert_eq!(destination, NODES.len() - 1);
        assert!(
            EDGES
                .iter()
                .all(|(from, to, _)| *from < NODES.len() && *to < NODES.len())
        );
    }
}
