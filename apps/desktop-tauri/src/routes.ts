export const areas = ["Home", "Clinical", "Research", "Intelligence", "Operations", "Governance", "Utility"] as const;
export type Area = (typeof areas)[number];

export const routes = [
  { id: "Home", label: "Home", area: "Home", icon: "home", description: "Local workspace overview", aliases: ["command center"] },
  { id: "Patients", label: "Patients", area: "Clinical", icon: "patients", description: "Longitudinal patient workspace", aliases: ["patient", "timeline"] },
  { id: "Documents", label: "Documents", area: "Clinical", icon: "documents", description: "Bounded document intake", aliases: ["files"] },
  { id: "Insights", label: "Insights", area: "Clinical", icon: "insights", description: "Population insights", aliases: ["population"] },
  { id: "Evidence", label: "Evidence", area: "Clinical", icon: "evidence", description: "Pinned comparative evidence ledger", aliases: ["proof", "comparison"] },
  { id: "Projects", label: "Projects", area: "Research", icon: "projects", description: "Projects and artifact references", aliases: ["project"] },
  { id: "Data", label: "Data", area: "Research", icon: "data", description: "Governed sources and snapshots", aliases: ["datasets", "sources"] },
  { id: "Browse", label: "Browse", area: "Research", icon: "research", description: "Governed browse evidence", aliases: ["browser"] },
  { id: "Analytics", label: "Analytics", area: "Research", icon: "analytics", description: "Read-only snapshot analytics", aliases: ["sql"] },
  { id: "Knowledge", label: "Knowledge", area: "Research", icon: "research", description: "Indexed source revision search", aliases: ["search"] },
  { id: "Research OS", label: "Research OS", area: "Research", icon: "workspace", description: "Research OS surfaces", aliases: ["hub", "compute"] },
  { id: "Models", label: "Models", area: "Intelligence", icon: "models", description: "Local model fabric", aliases: ["model center", "packs"] },
  { id: "MedAgent", label: "MedAgent", area: "Intelligence", icon: "runtime", description: "Governed agent run records", aliases: ["agents"] },
  { id: "Model Fleet", label: "Model Fleet", area: "Intelligence", icon: "models", description: "Independent model lanes", aliases: ["fleet"] },
  { id: "Workflows", label: "Workflows", area: "Operations", icon: "workflows", description: "Review-first workflows", aliases: ["workflow"] },
  { id: "Tasks", label: "Tasks", area: "Operations", icon: "notes", description: "Derived review tasks", aliases: ["review queue"] },
  { id: "Messages", label: "Messages", area: "Operations", icon: "notes", description: "Local message previews", aliases: ["message"] },
  { id: "Collaboration", label: "Collaboration", area: "Operations", icon: "workspace", description: "Local rooms and threads", aliases: ["rooms", "threads"] },
  { id: "Audio", label: "Audio", area: "Operations", icon: "runtime", description: "Local audio lineage", aliases: ["audioflow"] },
  { id: "Privacy", label: "Privacy", area: "Governance", icon: "privacy", description: "Data classes and egress decisions", aliases: ["privacy gate"] },
  { id: "Audit Trail", label: "Audit Trail", area: "Governance", icon: "notes", description: "Disclosure history", aliases: ["audit"] },
  { id: "Exports", label: "Exports", area: "Governance", icon: "data", description: "Loss-aware interchange", aliases: ["fhir", "export"] },
  { id: "Integrations", label: "Integrations", area: "Governance", icon: "workspace", description: "Brokered integration posture", aliases: ["connections"] },
  { id: "Settings", label: "Settings", area: "Utility", icon: "settings", description: "Appearance and readiness", aliases: ["preferences"] },
  { id: "About", label: "About", area: "Utility", icon: "about", description: "Product and attribution", aliases: ["version", "licenses"] },
] as const satisfies ReadonlyArray<{ id: string; label: string; area: Area; icon: string; description: string; aliases: readonly string[] }>;

export type Route = (typeof routes)[number];
export type RouteId = Route["id"];

export function isRouteId(input: string): input is RouteId {
  return routes.some((route) => route.id === input);
}

export function searchRoutes(query: string): readonly Route[] {
  if ([...query].length > 128) return [];
  const terms = query.trim().toLocaleLowerCase("en").split(/\s+/).filter(Boolean);
  return routes.filter((route) => {
    const haystack = [route.id, route.label, route.description, ...route.aliases].join(" ").toLocaleLowerCase("en");
    return terms.every((term) => haystack.includes(term));
  });
}
