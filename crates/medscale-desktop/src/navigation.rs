//! Closed desktop route registry. Route selection never interprets a query as an action.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Area {
    Home,
    Clinical,
    Research,
    Intelligence,
    Operations,
    Governance,
    Utility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Route {
    Home,
    Patients,
    Documents,
    Projects,
    Data,
    Insights,
    Models,
    Evidence,
    Workflows,
    Tasks,
    Messages,
    Collaboration,
    MedAgent,
    ModelFleet,
    Browse,
    Audio,
    Analytics,
    Knowledge,
    ResearchOs,
    Privacy,
    AuditTrail,
    Exports,
    Integrations,
    Settings,
    About,
}

#[derive(Clone, Copy, Debug)]
pub struct RouteDefinition {
    pub route: Route,
    pub id: &'static str,
    pub label: &'static str,
    pub area: Area,
    pub description: &'static str,
    pub aliases: &'static [&'static str],
}

pub const ROUTES: [RouteDefinition; 25] = [
    RouteDefinition {
        route: Route::Home,
        id: "Home",
        label: "Command Center",
        area: Area::Home,
        description: "Local synthetic workspace overview",
        aliases: &["home", "dashboard"],
    },
    RouteDefinition {
        route: Route::Patients,
        id: "Patients",
        label: "Patients",
        area: Area::Clinical,
        description: "Synthetic longitudinal patient workspace",
        aliases: &["patient", "timeline"],
    },
    RouteDefinition {
        route: Route::Documents,
        id: "Documents",
        label: "Documents",
        area: Area::Clinical,
        description: "Bounded local document intake",
        aliases: &["files", "intake"],
    },
    RouteDefinition {
        route: Route::Projects,
        id: "Projects",
        label: "Projects",
        area: Area::Research,
        description: "Core-backed project and artifact references",
        aliases: &["project"],
    },
    RouteDefinition {
        route: Route::Data,
        id: "Data",
        label: "Data",
        area: Area::Research,
        description: "Governed sources and snapshots",
        aliases: &["datasets", "sources"],
    },
    RouteDefinition {
        route: Route::Insights,
        id: "Insights",
        label: "Insights",
        area: Area::Clinical,
        description: "Synthetic population insights",
        aliases: &["population"],
    },
    RouteDefinition {
        route: Route::Models,
        id: "Models",
        label: "Models",
        area: Area::Intelligence,
        description: "Local model fabric and provenance",
        aliases: &["model center", "packs"],
    },
    RouteDefinition {
        route: Route::Evidence,
        id: "Evidence",
        label: "Evidence",
        area: Area::Clinical,
        description: "Pinned comparative proof ledger, not live literature or clinical advice",
        aliases: &["comparison", "proof"],
    },
    RouteDefinition {
        route: Route::Workflows,
        id: "Workflows",
        label: "Workflows",
        area: Area::Operations,
        description: "Review-first workflow studio",
        aliases: &["workflow"],
    },
    RouteDefinition {
        route: Route::Tasks,
        id: "Tasks",
        label: "Tasks",
        area: Area::Operations,
        description: "Derived outbox review tasks",
        aliases: &["review queue"],
    },
    RouteDefinition {
        route: Route::Messages,
        id: "Messages",
        label: "Messages",
        area: Area::Operations,
        description: "Local message previews without external transport",
        aliases: &["message"],
    },
    RouteDefinition {
        route: Route::Collaboration,
        id: "Collaboration",
        label: "Collaboration",
        area: Area::Operations,
        description: "Local rooms and threads",
        aliases: &["rooms", "threads"],
    },
    RouteDefinition {
        route: Route::MedAgent,
        id: "MedAgent",
        label: "MedAgent",
        area: Area::Intelligence,
        description: "Governed agent run records",
        aliases: &["agents"],
    },
    RouteDefinition {
        route: Route::ModelFleet,
        id: "Model Fleet",
        label: "Model Fleet",
        area: Area::Intelligence,
        description: "Independent model lanes and comparison",
        aliases: &["fleet"],
    },
    RouteDefinition {
        route: Route::Browse,
        id: "Browse",
        label: "Browse",
        area: Area::Research,
        description: "Governed read-only browse evidence",
        aliases: &["browser"],
    },
    RouteDefinition {
        route: Route::Audio,
        id: "Audio",
        label: "Audio",
        area: Area::Operations,
        description: "Local audio and transcript lineage",
        aliases: &["audioflow"],
    },
    RouteDefinition {
        route: Route::Analytics,
        id: "Analytics",
        label: "Analytics",
        area: Area::Research,
        description: "Read-only SQL over exact snapshots",
        aliases: &["sql"],
    },
    RouteDefinition {
        route: Route::Knowledge,
        id: "Knowledge",
        label: "Knowledge",
        area: Area::Research,
        description: "Lexical search over indexed source revisions",
        aliases: &["search", "research"],
    },
    RouteDefinition {
        route: Route::ResearchOs,
        id: "Research OS",
        label: "Research OS",
        area: Area::Research,
        description: "Read-only and reversible Research OS surfaces",
        aliases: &["hub", "compute", "extensions"],
    },
    RouteDefinition {
        route: Route::Privacy,
        id: "Privacy",
        label: "Privacy",
        area: Area::Governance,
        description: "Data classes and egress decisions",
        aliases: &["privacy gate"],
    },
    RouteDefinition {
        route: Route::AuditTrail,
        id: "Audit Trail",
        label: "Audit Trail",
        area: Area::Governance,
        description: "Synthetic disclosure history",
        aliases: &["audit"],
    },
    RouteDefinition {
        route: Route::Exports,
        id: "Exports",
        label: "Exports",
        area: Area::Governance,
        description: "Loss-aware interchange posture",
        aliases: &["fhir", "export"],
    },
    RouteDefinition {
        route: Route::Integrations,
        id: "Integrations",
        label: "Integrations",
        area: Area::Governance,
        description: "Brokered integration posture",
        aliases: &["connections"],
    },
    RouteDefinition {
        route: Route::Settings,
        id: "Settings",
        label: "Settings",
        area: Area::Utility,
        description: "Presentation and readiness settings",
        aliases: &["preferences"],
    },
    RouteDefinition {
        route: Route::About,
        id: "About",
        label: "About",
        area: Area::Utility,
        description: "Product and attribution details",
        aliases: &["version", "licenses"],
    },
];

impl Route {
    pub fn from_id(id: &str) -> Option<Self> {
        ROUTES
            .iter()
            .find(|entry| entry.id == id)
            .map(|entry| entry.route)
    }

    pub fn definition(self) -> &'static RouteDefinition {
        ROUTES
            .iter()
            .find(|entry| entry.route == self)
            .expect("every route has one definition")
    }
}

pub fn search_routes(query: &str) -> Vec<&'static RouteDefinition> {
    let terms: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
    ROUTES
        .iter()
        .filter(|entry| {
            terms.iter().all(|term| {
                entry.id.to_lowercase().contains(term)
                    || entry.label.to_lowercase().contains(term)
                    || entry.description.to_lowercase().contains(term)
                    || entry.aliases.iter().any(|alias| alias.contains(term))
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{ROUTES, Route, search_routes};
    use std::collections::HashSet;

    #[test]
    fn registry_is_bijective_and_exact() {
        assert_eq!(ROUTES.len(), 25);
        let mut ids = HashSet::new();
        let mut variants = HashSet::new();
        for entry in ROUTES {
            assert!(ids.insert(entry.id));
            assert!(variants.insert(format!("{:?}", entry.route)));
            assert_eq!(Route::from_id(entry.id), Some(entry.route));
            assert_eq!(entry.route.definition().id, entry.id);
        }
        assert_eq!(Route::from_id("patients"), None);
        assert_eq!(Route::from_id("../Patients"), None);
    }

    #[test]
    fn matching_is_bounded_to_route_metadata() {
        assert_eq!(search_routes("command center")[0].id, "Home");
        assert_eq!(search_routes("fhir")[0].id, "Exports");
        assert!(search_routes("delete patient").is_empty());
    }
}
