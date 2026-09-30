import { invoke } from "@tauri-apps/api/core";
import { useEffect, useRef, useState, type KeyboardEvent as ReactKeyboardEvent } from "react";
import markBlack from "../../../crates/medscale-desktop/ui/assets/medscale-mark.svg";
import markWhite from "../../../crates/medscale-desktop/ui/assets/medscale-mark-white.svg";
import { areas, isRouteId, routes, searchRoutes, type Route, type RouteId } from "./routes";
import { parseShellStatus, type ShellStatus } from "./ipc";

type Theme = "dark" | "light";
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
    invoke<unknown>("get_shell_status").then(parseShellStatus)
      .then((value) => { if (alive) setStatus({ kind: "available", value }); })
      .catch(() => { if (alive) setStatus({ kind: "error", message: "Workspace status is unavailable here. You can explore navigation and appearance." }); });
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
  const results = query.trim() ? searchRoutes(query) : routes.filter((route) => ["Home", "Patients", "Evidence", "Projects", "Settings"].includes(route.id));

  useEffect(() => {
    if (paletteOpen) dialog.current?.querySelector('[aria-selected="true"]')?.scrollIntoView({ block: "nearest" });
  }, [paletteOpen, query, selected]);

  const shortcut = /Mac|iPhone|iPad/.test(navigator.platform) ? "⌘ K" : "Ctrl K";

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
      <aside className="sidebar" aria-label="MedScale navigation" inert={paletteOpen}>
        <div className="brand">
          <img className="brand-mark" src={theme === "dark" ? markWhite : markBlack} alt="" />
          <div><strong>MedScale</strong><small>LOCAL CLINICAL INTELLIGENCE</small></div>
        </div>
        <button className="palette-trigger" onClick={openPalette} type="button" aria-label="Search MedScale routes">
          <span className="route-icon icon-search" aria-hidden="true" />
          <span>Quick search</span><kbd>{shortcut}</kbd>
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
          <div><strong>Synthetic-only preview</strong><span>Workspace unavailable</span></div>
        </div>
      </aside>

      <div className="workspace" inert={paletteOpen}>
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
          {routeId === "Home" ? <Home onNavigate={navigate} status={status} /> : <RouteUnavailable route={current} status={status} onNavigate={navigate} />}
        </main>
      </div>

      {paletteOpen && <div className="palette-backdrop" onMouseDown={(event) => { if (event.target === event.currentTarget) closePalette(); }}>
        <section ref={dialog} className="palette" role="dialog" aria-modal="true" aria-label="Navigate MedScale" onKeyDown={onPaletteKey}>
          <div className="palette-search"><span className="route-icon icon-search" aria-hidden="true" /><input ref={paletteInput} value={query} maxLength={128} onChange={(event) => { setQuery(event.target.value); setSelected(0); }} placeholder="Find a route…" role="combobox" aria-expanded="true" aria-autocomplete="list" aria-label="Find a route" aria-controls="palette-results" aria-activedescendant={results[selected] ? `palette-route-${results[selected].id.replaceAll(" ", "-")}` : undefined} autoComplete="off" /><kbd>ESC</kbd></div>
          <div id="palette-results" className="palette-results" role="listbox" aria-label="Matching routes">
            {results.length ? results.map((route, index) => <button key={route.id} id={`palette-route-${route.id.replaceAll(" ", "-")}`} type="button" className={`palette-result ${selected === index ? "is-selected" : ""}`} role="option" aria-selected={selected === index} onFocus={() => setSelected(index)} onMouseEnter={() => setSelected(index)} onClick={() => navigate(route.id)}>
              <span className={`route-icon icon-${route.icon}`} aria-hidden="true" /><span><strong>{route.label}</strong><small>{route.description}</small></span><em>{route.area}</em>
            </button>) : <p className="no-results">No matching route. Search only covers MedScale navigation.</p>}
          </div>
          <div className="palette-foot"><span>↑ ↓ to choose · Enter to open</span><span>{query.trim() ? "Navigation only" : "Type to search all 25 routes"}</span></div>
        </section>
      </div>}
    </div>
  );
}

function CoreState({ status }: { status: Status }) {
  if (status.kind === "loading") return <span>Checking preview availability…</span>;
  if (status.kind === "error") return <span>{status.message}</span>;
  return <span>Workspace data is not connected in this preview. Explore navigation and appearance.</span>;
}

function Home({ onNavigate, status }: { onNavigate: (id: string) => void; status: Status }) {
  return <div className="page-container home-page">
    <div className="home-title"><h1>Workspace overview</h1><p>Source truth, evidence and review in one local workspace.</p></div>
    <section className="workspace-notice" aria-labelledby="workspace-state-title"><div><p className="section-index">WORKSPACE STATE / UNAVAILABLE</p><h2 id="workspace-state-title">No workspace connected</h2><p><CoreState status={status} /></p></div><span className="state-label">UNAVAILABLE</span></section>
    <div className="section-heading"><div><h2>Workspace state</h2></div><span>SYNTHETIC-ONLY PREVIEW</span></div>
    <table className="posture-table"><caption className="sr-only">Current preview availability</caption><thead><tr><th scope="col">Area</th><th scope="col">State</th><th scope="col">Detail</th></tr></thead><tbody>
      <tr><th scope="row">Patient context</th><td><span className="state-label">Unavailable</span></td><td>No patient information has been loaded.</td></tr>
      <tr><th scope="row">Evidence and provenance</th><td><span className="state-label">Unavailable</span></td><td>No source inventory or review state has been read.</td></tr>
      <tr><th scope="row">Model runtime</th><td><span className="state-label">Unknown</span></td><td>Runtime availability has not been queried.</td></tr>
      <tr><th scope="row">Preview permissions</th><td><span className="state-label">Limited</span></td><td>Navigation and shell status only; no privileged action.</td></tr>
    </tbody></table>
    <div className="section-heading second-section"><div><h2>Explore the workspace</h2></div></div>
    <div className="work-area-list">{["Patients", "Evidence", "Projects", "Settings"].map((id) => {
      const route = routes.find((candidate) => candidate.id === id);
      if (!route) return null;
      return <button className="work-area" key={id} type="button" onClick={() => onNavigate(id)}><span className={`route-icon icon-${route.icon}`} aria-hidden="true" /><strong>{route.label}</strong><small>{route.description}</small><span className="work-area-arrow" aria-hidden="true">→</span></button>;
    })}</div>
    <p className="preview-note">Synthetic-only preview. No clinical records are available or invented.</p>
  </div>;
}

function RouteUnavailable({ route, status, onNavigate }: { route: Route; status: Status; onNavigate: (id: string) => void }) {
  return <div className="page-container route-page"><div className="route-title-row"><div><h1>{route.label}</h1><p className="route-description">{route.description}</p></div><span className="route-phase">PREVIEW</span></div><section className="unavailable-panel" aria-labelledby="route-state-title"><span className="state-label">Unavailable</span><h2 id="route-state-title">Workspace data is not connected</h2><p>This preparatory preview lets you explore MedScale navigation and appearance. {route.label} records and actions are unavailable here.</p><div className="panel-status"><CoreState status={status} /></div></section><button className="back-home" type="button" onClick={() => onNavigate("Home")}>Return to Home</button><p className="preview-note">Synthetic-only preview. No records are invented.</p></div>;
}
