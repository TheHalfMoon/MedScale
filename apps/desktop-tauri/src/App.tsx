import { invoke } from "@tauri-apps/api/core";
import { useEffect, useRef, useState, type KeyboardEvent as ReactKeyboardEvent } from "react";
import markBlack from "../../../crates/medscale-desktop/ui/assets/medscale-mark.svg";
import markWhite from "../../../crates/medscale-desktop/ui/assets/medscale-mark-white.svg";
import { areas, isRouteId, routes, searchRoutes, type Route, type RouteId } from "./routes";

type Theme = "dark" | "light";
type ShellStatus = { coreConnection: "unavailable"; detail: string; syntheticOnly: boolean };
type Status = { kind: "loading" } | { kind: "available"; value: ShellStatus } | { kind: "error"; message: string };

const initialTheme = (): Theme =>
  window.matchMedia?.("(prefers-color-scheme: light)").matches ? "light" : "dark";

export function App() {
  const [theme, setTheme] = useState<Theme>(initialTheme);
  const [routeId, setRouteId] = useState<RouteId>("Home");
  const [paletteOpen, setPaletteOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [selected, setSelected] = useState(0);
  const [status, setStatus] = useState<Status>({ kind: "loading" });
  const paletteInput = useRef<HTMLInputElement>(null);
  const previousFocus = useRef<HTMLElement | null>(null);
  const dialog = useRef<HTMLElement>(null);

  useEffect(() => {
    document.documentElement.dataset.theme = theme;
  }, [theme]);

  useEffect(() => {
    let alive = true;
    invoke<ShellStatus>("get_shell_status")
      .then((value) => { if (alive) setStatus({ kind: "available", value }); })
      .catch(() => { if (alive) setStatus({ kind: "error", message: "Native bridge unavailable in this preview." }); });
    return () => { alive = false; };
  }, []);

  useEffect(() => {
    function onGlobalKey(event: globalThis.KeyboardEvent) {
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "k") {
        event.preventDefault();
        setPaletteOpen((open) => {
          if (!open) previousFocus.current = document.activeElement instanceof HTMLElement ? document.activeElement : null;
          return !open;
        });
      }
    }
    window.addEventListener("keydown", onGlobalKey);
    return () => window.removeEventListener("keydown", onGlobalKey);
  }, []);

  useEffect(() => {
    if (paletteOpen) paletteInput.current?.focus();
    else previousFocus.current?.focus();
  }, [paletteOpen]);

  const current = routes.find((route) => route.id === routeId) ?? routes[0]!;
  const results = searchRoutes(query).slice(0, 12);

  function navigate(candidate: string) {
    if (!isRouteId(candidate)) return;
    setRouteId(candidate);
    setPaletteOpen(false);
    setQuery("");
    setSelected(0);
  }

  function openPalette() {
    previousFocus.current = document.activeElement instanceof HTMLElement ? document.activeElement : null;
    setQuery("");
    setSelected(0);
    setPaletteOpen(true);
  }

  function closePalette() {
    setPaletteOpen(false);
    setQuery("");
    setSelected(0);
  }

  function onPaletteKey(event: ReactKeyboardEvent<HTMLElement>) {
    if (event.key === "Escape") { event.preventDefault(); closePalette(); return; }
    if (event.key === "ArrowDown") { event.preventDefault(); setSelected((index) => Math.max(0, Math.min(index + 1, results.length - 1))); return; }
    if (event.key === "ArrowUp") { event.preventDefault(); setSelected((index) => Math.max(index - 1, 0)); return; }
    if (event.key === "Enter" && results[selected]) { event.preventDefault(); navigate(results[selected].id); return; }
    if (event.key === "Tab" && dialog.current) {
      const controls = [...dialog.current.querySelectorAll<HTMLElement>("input,button:not([disabled])")];
      const first = controls[0];
      const last = controls.at(-1);
      if (event.shiftKey && document.activeElement === first && last) { event.preventDefault(); last.focus(); }
      else if (!event.shiftKey && document.activeElement === last && first) { event.preventDefault(); first.focus(); }
    }
  }

  return (
    <div className="app-shell">
      <aside className="sidebar" aria-label="MedScale navigation">
        <div className="brand">
          <img className="brand-mark" src={theme === "dark" ? markWhite : markBlack} alt="" />
          <div><strong>MedScale</strong><small>LOCAL CLINICAL INTELLIGENCE</small></div>
        </div>
        <button className="palette-trigger" onClick={openPalette} type="button" aria-label="Search routes and commands">
          <span className="route-icon icon-search" aria-hidden="true" />
          <span>Quick search</span><kbd>⌘ K</kbd>
        </button>
        <nav className="route-nav" aria-label="Routes">
          {areas.map((area) => {
            const items = routes.filter((route) => route.area === area);
            return <div className="nav-group" key={area}>
              <div className="nav-heading">{area}</div>
              {items.map((route) => <button key={route.id} type="button" className={`nav-item ${routeId === route.id ? "is-active" : ""}`} aria-current={routeId === route.id ? "page" : undefined} onClick={() => navigate(route.id)}>
                <span className={`route-icon icon-${route.icon}`} aria-hidden="true" />
                <span>{route.label}</span>
              </button>)}
            </div>;
          })}
        </nav>
        <div className="sidebar-foot">
          <span className="status-dot" aria-hidden="true" />
          <div><strong>Synthetic-only preview</strong><span>Core connection unavailable</span></div>
        </div>
      </aside>

      <div className="workspace">
        <header className="topbar">
          <div className="breadcrumb"><span>MEDSCALE</span><span aria-hidden="true">/</span><strong>{current?.label}</strong></div>
          <div className="topbar-actions">
            <span className="local-badge">LOCAL PREVIEW</span>
            <div className="theme-switch" role="group" aria-label="Appearance for this session">
              <button type="button" aria-pressed={theme === "dark"} onClick={() => setTheme("dark")}>Dark</button>
              <button type="button" aria-pressed={theme === "light"} onClick={() => setTheme("light")}>Light</button>
            </div>
          </div>
        </header>

        <main id="main-content" className="main-content" tabIndex={-1}>
          {routeId === "Home" ? <Home onNavigate={navigate} status={status} /> : <RouteUnavailable route={current} status={status} />}
        </main>
      </div>

      {paletteOpen && <div className="palette-backdrop" onMouseDown={(event) => { if (event.target === event.currentTarget) closePalette(); }}>
        <section ref={dialog} className="palette" role="dialog" aria-modal="true" aria-label="Navigate MedScale" onKeyDown={onPaletteKey}>
          <div className="palette-search"><span className="route-icon icon-search" aria-hidden="true" /><input ref={paletteInput} value={query} maxLength={128} onChange={(event) => { setQuery(event.target.value); setSelected(0); }} placeholder="Find a route…" aria-label="Find a route" aria-controls="palette-results" autoComplete="off" /><kbd>ESC</kbd></div>
          <div id="palette-results" className="palette-results" role="listbox" aria-label="Matching routes">
            {results.length ? results.map((route, index) => <button key={route.id} type="button" className={`palette-result ${selected === index ? "is-selected" : ""}`} role="option" aria-selected={selected === index} onMouseEnter={() => setSelected(index)} onClick={() => navigate(route.id)}>
              <span className={`route-icon icon-${route.icon}`} aria-hidden="true" /><span><strong>{route.label}</strong><small>{route.description}</small></span><em>{route.area}</em>
            </button>) : <p className="no-results">No matching route. Search only covers MedScale navigation.</p>}
          </div>
          <div className="palette-foot"><span>↑ ↓ to choose · Enter to open</span><span>Navigation only</span></div>
        </section>
      </div>}
    </div>
  );
}

function CoreState({ status }: { status: Status }) {
  if (status.kind === "loading") return <span>Checking native shell…</span>;
  if (status.kind === "error") return <span>{status.message}</span>;
  return <span>{status.value.detail}</span>;
}

function Home({ onNavigate, status }: { onNavigate: (id: string) => void; status: Status }) {
  return <div className="page-container home-page">
    <div className="eyebrow"><span className="eyebrow-line" /> HOME / LOCAL WORKSPACE</div>
    <div className="home-title"><h1>Workspace overview</h1><p>Source truth, evidence and review in one local workspace.</p></div>
    <section className="workspace-notice" aria-labelledby="workspace-state-title"><div><p className="section-index">WORKSPACE STATE / UNAVAILABLE</p><h2 id="workspace-state-title">No workspace connected</h2><p><CoreState status={status} /></p></div><span className="state-label">UNAVAILABLE</span></section>
    <div className="section-heading"><div><p className="section-index">01 / CURRENT POSTURE</p><h2>Workspace state</h2></div><span>SYNTHETIC-ONLY PREVIEW</span></div>
    <table className="posture-table"><caption className="sr-only">Current preview availability</caption><thead><tr><th scope="col">Area</th><th scope="col">State</th><th scope="col">Detail</th></tr></thead><tbody>
      <tr><th scope="row">Patient context</th><td><span className="state-label">Unavailable</span></td><td>No Core-derived subject is loaded.</td></tr>
      <tr><th scope="row">Evidence and provenance</th><td><span className="state-label">Unavailable</span></td><td>No source inventory or review state has been read.</td></tr>
      <tr><th scope="row">Model runtime</th><td><span className="state-label">Unknown</span></td><td>Runtime availability has not been queried.</td></tr>
      <tr><th scope="row">Preview permissions</th><td><span className="state-label">Limited</span></td><td>Navigation and shell status only; no privileged action.</td></tr>
    </tbody></table>
    <div className="section-heading second-section"><div><p className="section-index">02 / WORK AREAS</p><h2>Explore the workspace</h2></div></div>
    <div className="work-area-list">{["Patients", "Evidence", "Projects", "Settings"].map((id) => {
      const route = routes.find((candidate) => candidate.id === id);
      if (!route) return null;
      return <button className="work-area" key={id} type="button" onClick={() => onNavigate(id)}><span className={`route-icon icon-${route.icon}`} aria-hidden="true" /><strong>{route.label}</strong><small>{route.description}</small><span className="work-area-arrow" aria-hidden="true">→</span></button>;
    })}</div>
    <p className="preview-note">Synthetic-only preview. No clinical records are available or invented.</p>
  </div>;
}

function RouteUnavailable({ route, status }: { route: Route; status: Status }) {
  return <div className="page-container route-page"><div className="eyebrow"><span className="eyebrow-line" /> {route.area.toUpperCase()} / {route.id.toUpperCase()}</div><div className="route-title-row"><div><p className="hero-kicker">MEDSCALE WORKSPACE</p><h1>{route.label}</h1><p className="hero-subtitle">{route.description}</p></div><span className="route-phase">PREVIEW</span></div><div className="unavailable-panel"><span className={`route-icon icon-${route.icon}`} aria-hidden="true" /><p className="panel-index">CURRENT STATE / UNAVAILABLE</p><h2>No workspace connected</h2><p>This preview has no Core-derived records for {route.label}. No clinical or operational data is available here yet.</p><div className="panel-status"><span className="status-dot" aria-hidden="true" /><CoreState status={status} /></div></div><p className="preview-note">Synthetic-only preview. No records are invented.</p></div>;
}
