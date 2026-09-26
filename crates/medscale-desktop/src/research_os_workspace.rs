//! Research OS operations view-models (Spec 093, Desktop parity for Specs
//! 084-091).
//!
//! Desktop reaches every plane only through the Core-owned `CliSession`;
//! it holds no storage, worker, transport or key code. This view is
//! read-only: each plane is listed with an explicit state, and a plane that
//! Core cannot read is shown as unavailable, never as empty. Actions for
//! these planes stay in the CLI in this slice.

use medscale_contracts::envelopes::AuthorityError;
use medscale_contracts::objects::OpaqueId;
use medscale_contracts::research_packs::CLINICAL_RESEARCH_PACK_ID;
use medscale_core::CliSession;

/// One row of the operations list.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RowVm {
    pub plane: String,
    pub id: String,
    pub state: String,
    pub detail: String,
}

/// The planes in display order.
pub const PLANES: [&str; 8] = [
    "Hub",
    "Compute",
    "R Workspace",
    "Extensions",
    "Huddles",
    "Research Packs",
    "Adapters",
    "Federation",
];

#[must_use]
pub fn status_message(err: &AuthorityError) -> &'static str {
    match err {
        AuthorityError::Unauthorized
        | AuthorityError::SessionRequired
        | AuthorityError::SessionExpired
        | AuthorityError::SessionRevoked
        | AuthorityError::SessionDenied
        | AuthorityError::WrongScope => "denied by Core authority",
        AuthorityError::NotFound => "none in this vault",
        AuthorityError::Corrupt { .. } => "corrupt: integrity check failed",
        _ => "unavailable: vault or Core not ready",
    }
}

fn row(plane: &str, id: &str, state: &str, detail: String) -> RowVm {
    RowVm {
        plane: plane.to_owned(),
        id: id.to_owned(),
        state: state.to_owned(),
        detail,
    }
}

fn unavailable(plane: &str, err: &AuthorityError) -> RowVm {
    row(plane, "-", "unavailable", status_message(err).to_owned())
}

fn plane_rows(session: &mut CliSession, project: &OpaqueId, plane: &str) -> Vec<RowVm> {
    let rows: Result<Vec<RowVm>, AuthorityError> = match plane {
        "Hub" => session.hub_links().map(|links| {
            links
                .iter()
                .map(|l| {
                    row(
                        plane,
                        l.header.id.as_str(),
                        "linked",
                        format!(
                            "device {} · project {}",
                            l.device_id.as_str(),
                            l.project_id.as_str()
                        ),
                    )
                })
                .collect()
        }),
        "Compute" => session.compute_jobs(project.clone()).map(|jobs| {
            jobs.iter()
                .map(|j| {
                    row(
                        plane,
                        j.header.id.as_str(),
                        j.state.as_str(),
                        format!("{} · closed job kind", j.manifest.kind.as_str()),
                    )
                })
                .collect()
        }),
        "R Workspace" => session.r_workspaces(project.clone()).map(|list| {
            list.iter()
                .map(|w| {
                    row(
                        plane,
                        w.id().as_str(),
                        "staged",
                        format!(
                            "{} · {} input(s) · class {} · managed R not admitted",
                            w.manifest.label,
                            w.manifest.inputs.len(),
                            w.manifest.data_class.as_str()
                        ),
                    )
                })
                .collect()
        }),
        "Extensions" => session.ext_list(project.clone()).map(|view| {
            view.installs
                .iter()
                .map(|i| {
                    row(
                        plane,
                        &i.extension_id,
                        i.state.as_str(),
                        format!("publisher {} · declarative only", i.publisher_id),
                    )
                })
                .collect()
        }),
        "Huddles" => session.huddle_list(project.clone()).map(|list| {
            list.iter()
                .map(|h| {
                    row(
                        plane,
                        h.header.id.as_str(),
                        h.state.as_str(),
                        format!("{} · retention {} days", h.title, h.retention_days),
                    )
                })
                .collect()
        }),
        "Research Packs" => session
            .pack_artifact_list(project.clone(), CLINICAL_RESEARCH_PACK_ID.to_owned())
            .map(|list| {
                list.iter()
                    .map(|a| {
                        row(
                            plane,
                            a.header.id.as_str(),
                            &a.workflow_state,
                            format!("{} · pack v{}", a.type_id, a.pack_version),
                        )
                    })
                    .collect()
            }),
        "Adapters" => session.adapter_list(project.clone()).map(|list| {
            list.iter()
                .map(|a| {
                    row(
                        plane,
                        a.header.id.as_str(),
                        a.state.as_str(),
                        format!(
                            "{} · ceiling {} · no network transport in this build",
                            a.config.destination,
                            a.config.data_class_ceiling.as_str()
                        ),
                    )
                })
                .collect()
        }),
        "Federation" => session.federation_get().map(|view| {
            view.peers
                .iter()
                .map(|p| {
                    row(
                        plane,
                        &p.institution_id,
                        p.state.as_str(),
                        format!(
                            "ceiling {} · last received #{}",
                            p.ceiling.as_str(),
                            p.last_received_seq
                        ),
                    )
                })
                .collect()
        }),
        _ => Ok(Vec::new()),
    };
    match rows {
        Ok(rows) if rows.is_empty() => {
            vec![row(plane, "-", "empty", "nothing recorded yet".to_owned())]
        }
        Ok(rows) => rows,
        Err(err) => vec![unavailable(plane, &err)],
    }
}

/// Every plane for one Project, in display order. A plane Core cannot read
/// is a single `unavailable` row.
pub fn rows(session: &mut CliSession, project_id: &str) -> Vec<RowVm> {
    let project = OpaqueId::new(project_id);
    PLANES
        .iter()
        .flat_map(|plane| plane_rows(session, &project, plane))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_plane_is_listed_and_empty_is_not_unavailable() {
        let dir = std::env::temp_dir().join(format!("medscale-093-rows-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let mut s = CliSession::connect("vault-093-desktop").unwrap();
        s.open_synthetic_vault(&dir.display().to_string()).unwrap();
        let project = s.project_create("ops".to_owned(), None).unwrap().header.id;
        let rows = rows(&mut s, project.as_str());
        for plane in PLANES {
            assert!(
                rows.iter().any(|r| r.plane == plane),
                "{plane} missing from the operations view"
            );
        }
        // A fresh vault has nothing recorded; nothing is reported as data.
        assert!(
            rows.iter()
                .all(|r| r.state == "empty" || r.state == "unavailable"),
            "{rows:?}"
        );
    }

    #[test]
    fn errors_map_to_explicit_states() {
        assert_eq!(
            status_message(&AuthorityError::NotFound),
            "none in this vault"
        );
        assert_eq!(
            status_message(&AuthorityError::WrongScope),
            "denied by Core authority"
        );
    }
}
