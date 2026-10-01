import type { Route } from "../routes";

export type IconName = Route["icon"] | "search" | "commands" | "chevron-right";

// First-party geometry: 24-unit grid, 2-unit stroke, square ends and no fill.
const paths: Record<IconName, string> = {
  home: "M4 10 12 3l8 7v11H4Z M9 21v-7h6v7",
  patients: "M9 4h6v6H9Z M5 21v-5l4-3h6l4 3v5 M9 21v-4m6 4v-4",
  documents: "M5 3h10l4 4v14H5Z M15 3v5h4 M8 12h8m-8 4h6",
  insights: "M4 20V4 M4 20h16 M8 16l4-5 4 2 4-7",
  evidence: "M5 3h14v18H5Z M8 7h8m-8 4h8m-8 4h3 M13 17l2 2 3-4",
  projects: "M3 6h7l2 3h9v12H3Z M3 6V3h8l2 3h8v3",
  data: "M4 14h4v7H4Z M10 9h4v12h-4Z M16 3h4v18h-4Z",
  research: "M4 4h7v7H4Z M13 13h7v7h-7Z M11 7h6v6 M7 11v6h6",
  analytics: "M4 4v16h16 M8 16V9m4 7V5m4 11v-4m4 4V7",
  workspace: "M3 3h7v7H3Z M14 3h7v7h-7Z M3 14h7v7H3Z M14 14h7v7h-7Z",
  models: "M5 6 12 2l7 4v12l-7 4-7-4Z M5 6l7 4 7-4 M12 10v12",
  runtime: "M7 3h10v18H7Z M3 7h4m-4 5h4m-4 5h4m10-10h4m-4 5h4m-4 5h4 M10 8h4v8h-4Z",
  workflows: "M3 3h6v6H3Z M15 3h6v6h-6Z M9 15h6v6H9Z M9 6h6 M6 9v9h3m9-9v9h-3",
  notes: "M5 3h14v18H5Z M8 7h8m-8 5h8m-8 5h5",
  privacy: "M4 5 12 2l8 3v7l-8 10-8-10Z M12 2v20",
  settings: "M4 6h16M4 12h16M4 18h16 M8 3v6m8 0v6M10 15v6",
  about: "M12 3a9 9 0 1 0 0 18 9 9 0 0 0 0-18 M12 10v7m0-11v1",
  search: "M10 3a7 7 0 1 0 0 14 7 7 0 0 0 0-14 M15 15l6 6",
  commands: "M4 5l5 5-5 5 M12 18h8",
  "chevron-right": "M9 5l7 7-7 7",
};

export function MedScaleIcon({ name, className = "" }: { name: IconName; className?: string }) {
  return <svg className={`route-icon ${className}`} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="butt" strokeLinejoin="miter" aria-hidden="true" focusable="false"><path d={paths[name]} /></svg>;
}
