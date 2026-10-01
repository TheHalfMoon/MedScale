import type { ButtonHTMLAttributes, ReactNode } from "react";
import markBlack from "../../../../crates/medscale-desktop/ui/assets/medscale-mark.svg";
import markWhite from "../../../../crates/medscale-desktop/ui/assets/medscale-mark-white.svg";
import { areas, routes, type Route, type RouteId } from "../routes";
import { MedScaleIcon } from "./MedScaleIcon";

export type Theme = "dark" | "light";
export type LiteralState = "Unavailable" | "Unknown" | "Limited" | "Present" | "Absent" | "Conflict" | "Unsupported" | "Unhealthy evidence" | "Incomparable units";

export function MedScaleStatus({ state, explanation }: { state: LiteralState; explanation?: string }) {
  return <span className="medscale-status" data-state={state} title={explanation}><span className="state-marker" aria-hidden="true" />{state}</span>;
}

export function MedScaleButton({ className = "", ...props }: ButtonHTMLAttributes<HTMLButtonElement>) {
  return <button type="button" className={`medscale-button ${className}`} {...props} />;
}

export function MedScaleNavItem({ route, active, onNavigate }: { route: Route; active: boolean; onNavigate: (id: RouteId) => void }) {
  return <button type="button" className={`nav-item ${active ? "is-active" : ""}`} aria-current={active ? "page" : undefined} onClick={() => onNavigate(route.id)}><MedScaleIcon name={route.icon} /><span>{route.label}</span></button>;
}

export function MedScaleRuntimeState() {
  return <div className="sidebar-foot"><MedScaleIcon name="runtime" /><div><strong>Synthetic-only preview</strong><span>Workspace unavailable</span></div></div>;
}

export function MedScaleSidebar({ theme, routeId, inert, onNavigate, onSearch, shortcut }: { theme: Theme; routeId: RouteId; inert: boolean; onNavigate: (id: RouteId) => void; onSearch: () => void; shortcut: string }) {
  return <aside className="sidebar" aria-label="MedScale navigation" inert={inert}>
    <div className="brand"><img className="brand-mark" src={theme === "dark" ? markWhite : markBlack} alt="" /><div><strong>MedScale</strong><small>LOCAL CLINICAL INTELLIGENCE</small></div></div>
    <button className="palette-trigger" onClick={onSearch} type="button" aria-label="Search MedScale routes"><MedScaleIcon name="search" /><span>Quick search</span><kbd>{shortcut}</kbd></button>
    <nav className="route-nav" aria-label="Routes">{areas.map((area) => <div className="nav-group" key={area}><div className="nav-heading">{area}</div>{routes.filter((route) => route.area === area).map((route) => <MedScaleNavItem key={route.id} route={route} active={route.id === routeId} onNavigate={onNavigate} />)}</div>)}</nav>
    <MedScaleRuntimeState />
  </aside>;
}

export function MedScaleHeader({ route, theme, onTheme }: { route: Route; theme: Theme; onTheme: (theme: Theme) => void }) {
  return <header className="topbar"><div className="breadcrumb"><span>No workspace</span><strong>{route.label}</strong></div><div className="topbar-actions"><span className="local-badge">Local preview</span><div className="theme-switch" role="group" aria-label="Appearance for this session">{(["dark", "light"] as const).map((value) => <button key={value} type="button" aria-pressed={theme === value} onClick={() => onTheme(value)}>{value === "dark" ? "Dark" : "Light"}</button>)}</div></div></header>;
}

export function MedScaleEmptyState({ title, children, icon = "workspace" }: { title: string; children: ReactNode; icon?: Route["icon"] }) {
  return <div className="medscale-empty-state"><MedScaleIcon name={icon} /><div><h2>{title}</h2><div className="empty-state-copy">{children}</div></div></div>;
}

export function MedScaleSection({ title, children, description }: { title: string; children: ReactNode; description?: ReactNode }) {
  return <section className="medscale-section"><div className="section-heading"><h2>{title}</h2>{description}</div>{children}</section>;
}

export function MedScaleSearch({ label, unavailableReason }: { label: string; unavailableReason: string }) {
  return <div className="medscale-search" title={unavailableReason}><MedScaleIcon name="search" /><input type="search" aria-label={label} placeholder={label} disabled /><span>Unavailable</span></div>;
}

export function MedScaleTable({ label, columns, children }: { label: string; columns: readonly string[]; children: ReactNode }) {
  return <div className="medscale-table-scroll" role="region" aria-label={`${label} table`} tabIndex={0}><table className="medscale-table"><caption className="sr-only">{label}</caption><thead><tr>{columns.map((column) => <th scope="col" key={column}>{column}</th>)}</tr></thead><tbody>{children}</tbody></table></div>;
}

export function MedScaleRow({ children }: { children: ReactNode }) {
  return <tr className="medscale-row">{children}</tr>;
}
