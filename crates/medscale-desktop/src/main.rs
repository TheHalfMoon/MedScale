//! Native MedScale Desktop shell — Slint, no WebView/Tauri (Spec 060).

use medscale_core::{CliSession, CoreFacade, build_doctor_report, privacy_proof_artifact_present};
use std::cell::RefCell;
use std::env;
use std::hint::black_box;
use std::io::{self, Write};
use std::path::Path;
use std::process::ExitCode;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use slint::{ComponentHandle, ModelRc, VecModel};

mod data_workbench;
mod patient_workspace;
mod population_insights;
mod product_intelligence;
mod project_workspace;
mod utility_surfaces;
mod workflow_studio;

slint::include_modules!();

const PERF_IDLE_MAX_MS: u64 = 10_000;

fn apply_product_intelligence(ui: &AppWindow, vm: &product_intelligence::ProductIntelligenceVm) {
    ui.set_model_runtime_summary(vm.model_runtime_summary.clone().into());
    ui.set_model_runtime_boundary(vm.model_runtime_boundary.clone().into());
    ui.set_model_source_summary(vm.model_source_summary.clone().into());
    ui.set_model_inventory_summary(vm.model_inventory_summary.clone().into());
    ui.set_openmed_baseline(vm.openmed_baseline.clone().into());
    ui.set_competitive_summary(vm.competitive_summary.clone().into());
    ui.set_model_rows(ModelRc::new(VecModel::from_iter(vm.models.iter().map(
        |row| ModelStatusItem {
            scope: row.scope.clone().into(),
            task: row.task.clone().into(),
            model: row.model.clone().into(),
            source: row.source.clone().into(),
            runtime: row.runtime.clone().into(),
            device: row.device.clone().into(),
            trust: row.trust.clone().into(),
            benchmark: row.benchmark.clone().into(),
            digest: row.digest.clone().into(),
            state: row.state.clone().into(),
        },
    ))));
    ui.set_competitive_evidence_rows(ModelRc::new(VecModel::from_iter(vm.evidence.iter().map(
        |row| CompetitiveEvidenceItem {
            capability: row.capability.clone().into(),
            medscale: row.medscale.clone().into(),
            openmed: row.openmed.clone().into(),
            verdict: row.verdict.clone().into(),
            evidence: row.evidence.clone().into(),
            limitations: row.limitations.clone().into(),
        },
    ))));
}

fn refresh_model_center(
    ui: &AppWindow,
    session: &Rc<RefCell<CliSession>>,
) -> Result<(), medscale_contracts::envelopes::AuthorityError> {
    let vm = product_intelligence::ProductIntelligenceVm::from_session(&mut session.borrow_mut())?;
    apply_product_intelligence(ui, &vm);
    Ok(())
}

fn admit_model_pack(
    session: &Rc<RefCell<CliSession>>,
    path: &str,
) -> Result<medscale_contracts::packs::PackAdmitResult, medscale_contracts::envelopes::AuthorityError>
{
    let mut session = session.borrow_mut();
    session.packs_install_local(path)
}

/// Refreshes the Projects list from Core; honest empty/denied/unavailable states.
fn refresh_project_list(ui: &AppWindow, session: &Rc<RefCell<CliSession>>) {
    match project_workspace::refresh_projects(&mut session.borrow_mut()) {
        Ok(rows) => {
            if rows.is_empty() {
                ui.set_project_status("No projects yet · create one above".into());
            } else {
                ui.set_project_status(
                    format!(
                        "{} project{} · Core-backed",
                        rows.len(),
                        if rows.len() == 1 { "" } else { "s" }
                    )
                    .into(),
                );
            }
            ui.set_project_rows(ModelRc::new(VecModel::from_iter(rows.iter().map(|row| {
                ProjectRowItem {
                    id: row.id.clone().into(),
                    name: row.name.clone().into(),
                    status: row.status.clone().into(),
                    revision: row.revision.to_string().into(),
                    experiments: row.experiments.to_string().into(),
                    refs: row.refs.to_string().into(),
                    edges: row.edges.to_string().into(),
                }
            }))));
        }
        Err(err) => {
            ui.set_project_rows(ModelRc::new(VecModel::from(Vec::new())));
            ui.set_project_status(project_workspace::status_message(&err).into());
        }
    }
}

/// Refreshes the Data Source list for the active Project from Core.
fn refresh_data_sources(ui: &AppWindow, session: &Rc<RefCell<CliSession>>) {
    let project_id = ui.get_project_active_id().to_string();
    if project_id.is_empty() {
        ui.set_data_status("Open a project to inspect its data sources".into());
        ui.set_data_sources(ModelRc::new(VecModel::from(Vec::new())));
        return;
    }
    match data_workbench::refresh_sources(&mut session.borrow_mut(), &project_id) {
        Ok(rows) => {
            if rows.is_empty() {
                ui.set_data_status("No data sources yet in this project".into());
            } else {
                ui.set_data_status(
                    format!(
                        "{} source{} · Core-backed",
                        rows.len(),
                        if rows.len() == 1 { "" } else { "s" }
                    )
                    .into(),
                );
            }
            ui.set_data_sources(ModelRc::new(VecModel::from_iter(rows.iter().map(|row| {
                DataSourceRowItem {
                    id: row.id.clone().into(),
                    name: row.name.clone().into(),
                    kind: row.kind.clone().into(),
                    health: row.health.clone().into(),
                    revision: row.revision.to_string().into(),
                }
            }))));
        }
        Err(err) => {
            ui.set_data_sources(ModelRc::new(VecModel::from(Vec::new())));
            ui.set_data_status(data_workbench::status_message(&err).into());
        }
    }
}

/// Opens one data source: snapshots plus the latest workbench page.
fn open_data_source(ui: &AppWindow, session: &Rc<RefCell<CliSession>>, source_id: &str) {
    match data_workbench::source_snapshots(&mut session.borrow_mut(), source_id) {
        Ok(snapshots) => {
            ui.set_data_active_source(source_id.into());
            ui.set_data_snapshots(ModelRc::new(VecModel::from_iter(snapshots.iter().map(
                |row| SnapshotRowItem {
                    id: row.id.clone().into(),
                    rows: row.rows.to_string().into(),
                    digest: row.digest.clone().into(),
                },
            ))));
            if let Some(latest) = snapshots.first() {
                let id = latest.id.clone();
                open_data_snapshot(ui, session, &id);
            } else {
                ui.set_data_status(
                    "Source has no snapshots yet · import to materialize one".into(),
                );
            }
        }
        Err(err) => ui.set_data_status(data_workbench::status_message(&err).into()),
    }
}

/// Opens one snapshot workbench page (typed schema plus rendered rows).
fn open_data_snapshot(ui: &AppWindow, session: &Rc<RefCell<CliSession>>, snapshot_id: &str) {
    match data_workbench::workbench_page(&mut session.borrow_mut(), snapshot_id) {
        Ok(page) => {
            ui.set_data_active_snapshot(page.snapshot_id.clone().into());
            ui.set_data_schema_line(page.schema_line.clone().into());
            ui.set_data_detail(page.detail.clone().into());
            ui.set_data_rows(ModelRc::new(VecModel::from_iter(page.rows.iter().map(
                |text| WorkbenchRowItem {
                    text: text.clone().into(),
                },
            ))));
            ui.set_data_next_cursor(page.next_cursor.unwrap_or_default().into());
            ui.set_data_status(format!("Open: {}", page.snapshot_id).into());
        }
        Err(err) => ui.set_data_status(data_workbench::status_message(&err).into()),
    }
}

/// Opens one Project detail (overview, experiments, refs, relationships).
fn open_project_detail(ui: &AppWindow, session: &Rc<RefCell<CliSession>>, project_id: &str) {
    match project_workspace::project_detail(&mut session.borrow_mut(), project_id) {
        Ok(detail) => {
            ui.set_project_active_id(detail.project_id.clone().into());
            ui.set_project_active_revision(detail.revision.to_string().into());
            ui.set_project_detail_name(detail.name.clone().into());
            ui.set_project_detail_meta(
                format!(
                    "{} · {} · status {} · revision {} · {} experiments · {} refs · {} edges",
                    detail.project_id,
                    detail.name,
                    detail.status,
                    detail.revision,
                    detail.experiment_count,
                    detail.active_ref_count,
                    detail.active_edge_count
                )
                .into(),
            );
            ui.set_project_experiments(ModelRc::new(VecModel::from_iter(
                detail.experiments.iter().map(|row| ExperimentRowItem {
                    id: row.id.clone().into(),
                    name: row.name.clone().into(),
                    status: row.status.clone().into(),
                    revision: row.revision.to_string().into(),
                }),
            )));
            ui.set_project_refs(ModelRc::new(VecModel::from_iter(detail.refs.iter().map(
                |row| RefRowItem {
                    ref_id: row.ref_id.clone().into(),
                    kind: row.kind.clone().into(),
                    object_id: row.object_id.clone().into(),
                    resolution: row.resolution.clone().into(),
                },
            ))));
            ui.set_project_edges(ModelRc::new(VecModel::from_iter(detail.edges.iter().map(
                |row| EdgeRowItem {
                    edge_id: row.edge_id.clone().into(),
                    subject: row.subject.clone().into(),
                    predicate: row.predicate.clone().into(),
                    object: row.object.clone().into(),
                    revision: row.revision.to_string().into(),
                },
            ))));
            ui.set_project_status(format!("Open: {}", detail.name).into());
        }
        Err(err) => {
            ui.set_project_active_id("".into());
            ui.set_project_status(project_workspace::status_message(&err).into());
        }
    }
}

fn validated_model_pack_path(raw: &str) -> Result<&str, &'static str> {
    let path = raw.trim();
    if path.is_empty() {
        return Err("Enter an absolute local Pack directory");
    }
    if !Path::new(path).is_absolute() {
        return Err("Local Pack directory must be an absolute path");
    }
    Ok(path)
}

fn perf_idle_ms(args: &[String]) -> Result<Option<u64>, &'static str> {
    let Some(index) = args.iter().position(|arg| arg == "--perf-idle-ms") else {
        return Ok(None);
    };
    let Some(raw) = args.get(index + 1) else {
        return Err("--perf-idle-ms requires a millisecond value");
    };
    let value = raw
        .parse::<u64>()
        .map_err(|_| "invalid --perf-idle-ms value")?;
    if !(50..=PERF_IDLE_MAX_MS).contains(&value) {
        return Err("--perf-idle-ms must be between 50 and 10000");
    }
    Ok(Some(value))
}

fn evidence_theme_override() -> Option<i32> {
    match env::var("MEDSCALE_EVIDENCE_THEME")
        .ok()?
        .to_ascii_lowercase()
        .as_str()
    {
        "light" => Some(1),
        "dark" => Some(2),
        _ => None,
    }
}

fn evidence_route_override() -> Option<&'static str> {
    match env::var("MEDSCALE_EVIDENCE_ROUTE").ok()?.as_str() {
        "Home" => Some("Home"),
        "Patients" => Some("Patients"),
        "Documents" => Some("Documents"),
        "Projects" => Some("Projects"),
        "Insights" => Some("Insights"),
        "Models" => Some("Models"),
        "Evidence" => Some("Evidence"),
        "Workflows" => Some("Workflows"),
        "Tasks" => Some("Tasks"),
        "Messages" => Some("Messages"),
        "Audit Trail" => Some("Audit Trail"),
        "Exports" => Some("Exports"),
        "Integrations" => Some("Integrations"),
        "Settings" => Some("Settings"),
        "About" => Some("About"),
        _ => None,
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let smoke = args.iter().any(|arg| arg == "--smoke");
    let idle_ms = match perf_idle_ms(&args) {
        Ok(value) => value,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(2);
        }
    };

    let report = build_doctor_report(None, false, privacy_proof_artifact_present());
    if let Some(ms) = idle_ms {
        black_box(&report);
        println!("medscale-desktop perf-idle-ready");
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(ms));
        black_box(&report);
        return ExitCode::SUCCESS;
    }

    if smoke {
        println!("medscale-desktop smoke ok");
        println!("version={}", CoreFacade::version());
        println!("tauri_admitted={}", report.tauri_admitted);
        println!("desktop_shell={}", report.desktop_shell);
        return ExitCode::SUCCESS;
    }

    let ui = match AppWindow::new() {
        Ok(ui) => ui,
        Err(err) => {
            eprintln!("failed to initialize MedScale Desktop UI: {err}");
            return ExitCode::from(1);
        }
    };
    ui.set_product_version(report.version.clone().into());
    if let Some(theme) = evidence_theme_override() {
        ui.global::<Theme>().set_evidence_theme(theme);
    }
    if let Some(route) = evidence_route_override() {
        ui.set_active_route(route.into());
    }

    let patient = patient_workspace::PatientWorkspaceVm::synthetic_demo();
    ui.set_patient_name(patient.display_name.into());
    ui.set_patient_subject_ref(patient.subject_ref.into());
    ui.set_patient_birth_date(patient.birth_date.into());
    ui.set_patient_condition_summary(patient.condition_summary.into());
    ui.set_patient_coverage_summary(patient.coverage_summary.into());
    ui.set_patient_coverage_state(patient.coverage_state.into());
    ui.set_patient_medications_state(patient.medications_state.into());
    ui.set_patient_documents_state(patient.documents_state.into());
    ui.set_patient_care_plan_state(patient.care_plan_state.into());
    ui.set_patient_timeline(ModelRc::new(VecModel::from_iter(
        patient.timeline.into_iter().map(|row| PatientTimelineItem {
            date: row.date.into(),
            title: row.title.into(),
            detail: row.detail.into(),
            source: row.source.into(),
            status: row.status.into(),
        }),
    )));
    ui.set_patient_labs(ModelRc::new(VecModel::from_iter(
        patient.labs.into_iter().map(|row| PatientLabItem {
            label: row.label.into(),
            value: row.value.into(),
            unit: row.unit.into(),
            status: row.status.into(),
            source: row.source.into(),
        }),
    )));
    ui.set_patient_sources(ModelRc::new(VecModel::from_iter(
        patient.sources.into_iter().map(|row| PatientSourceItem {
            source_id: row.source_id.into(),
            note: row.note.into(),
        }),
    )));

    let insights = population_insights::PopulationInsightsVm::synthetic_demo();
    ui.set_insights_cohort_size(insights.cohort_size.to_string().into());
    ui.set_insights_review_attention(insights.review_attention_count.to_string().into());
    ui.set_insights_evidence_gaps(insights.evidence_gap_count.to_string().into());
    ui.set_insights_present_count(insights.present_count.to_string().into());
    ui.set_insights_risk_state(insights.risk_state.clone().into());
    ui.set_insights_trend_state(insights.trend_state.clone().into());
    ui.set_insights_gap_state(insights.gap_state.clone().into());
    ui.set_insights_recommendation_state(insights.recommendation_state.clone().into());
    ui.set_insights_assistant_answer(insights.assistant_default.clone().into());
    ui.set_insights_conditions(ModelRc::new(VecModel::from_iter(
        insights
            .condition_distribution
            .iter()
            .map(|row| PopulationDistributionItem {
                label: row.label.clone().into(),
                value: row.value.clone().into(),
                detail: row.detail.clone().into(),
                status: row.status.clone().into(),
            }),
    )));
    ui.set_insights_coverage(ModelRc::new(VecModel::from_iter(
        insights
            .coverage_distribution
            .iter()
            .map(|row| PopulationDistributionItem {
                label: row.label.clone().into(),
                value: row.value.clone().into(),
                detail: row.detail.clone().into(),
                status: row.status.clone().into(),
            }),
    )));
    ui.set_insights_cohorts(ModelRc::new(VecModel::from_iter(
        insights.cohorts.iter().map(|row| PopulationCohortItem {
            name: row.display_name.clone().into(),
            subject_ref: row.subject_ref.clone().into(),
            condition: row.condition.clone().into(),
            evidence_state: row.evidence_state.clone().into(),
            review_attention: row.review_attention.clone().into(),
        }),
    )));
    ui.set_insights_evidence(ModelRc::new(VecModel::from_iter(
        insights.evidence.iter().map(|row| AssistantEvidenceItem {
            source: row.source.clone().into(),
            title: row.title.clone().into(),
            snippet: row.snippet.clone().into(),
            metadata: row.metadata.clone().into(),
        }),
    )));

    let workflow = workflow_studio::WorkflowStudioVm::synthetic_demo();
    ui.set_workflow_name(workflow.workflow_name.clone().into());
    ui.set_workflow_state(workflow.workflow_state.clone().into());
    ui.set_workflow_outbox_summary(workflow.outbox_summary.clone().into());
    ui.set_workflow_action_boundary(workflow.action_boundary.clone().into());
    ui.set_workflow_task_boundary(workflow.task_boundary.clone().into());
    ui.set_workflow_message_boundary(workflow.message_boundary.clone().into());
    ui.set_workflow_review_detail(
        "Select a derived review task. No action is committed from this surface.".into(),
    );
    ui.set_workflow_steps(ModelRc::new(VecModel::from_iter(
        workflow.steps.iter().map(|row| WorkflowStepItem {
            order: row.order.clone().into(),
            title: row.title.clone().into(),
            detail: row.detail.clone().into(),
            state: row.state.clone().into(),
        }),
    )));
    ui.set_workflow_tasks(ModelRc::new(VecModel::from_iter(
        workflow.tasks.iter().map(|row| WorkflowTaskItem {
            action_id: row.action_id.clone().into(),
            title: row.title.clone().into(),
            state: row.state.clone().into(),
            payload_digest: row.payload_digest.clone().into(),
            next_step: row.next_step.clone().into(),
            reconcile_required: row.reconcile_required,
        }),
    )));
    ui.set_workflow_messages(ModelRc::new(VecModel::from_iter(
        workflow.messages.iter().map(|row| WorkflowMessageItem {
            title: row.title.clone().into(),
            body: row.body.clone().into(),
            status: row.status.clone().into(),
            related_action_id: row.related_action_id.clone().into(),
        }),
    )));

    let utilities = utility_surfaces::UtilitySurfacesVm::synthetic_demo(&report);
    ui.set_utility_audit_boundary(utilities.audit_boundary.clone().into());
    ui.set_utility_export_boundary(utilities.export_boundary.clone().into());
    ui.set_utility_settings_boundary(utilities.settings_boundary.clone().into());
    ui.set_utility_integrations_boundary(utilities.integrations_boundary.clone().into());
    ui.set_utility_missing_release_evidence(utilities.missing_release_evidence.clone().into());
    ui.set_utility_audit_rows(ModelRc::new(VecModel::from_iter(
        utilities.audit_rows.iter().map(|row| AuditTrailItem {
            record_id: row.record_id.clone().into(),
            action: row.action.clone().into(),
            subject: row.subject.clone().into(),
            detail: row.detail.clone().into(),
            status: row.status.clone().into(),
        }),
    )));
    ui.set_utility_export_rows(ModelRc::new(VecModel::from_iter(
        utilities.export_rows.iter().map(|row| UtilityStatusItem {
            label: row.label.clone().into(),
            value: row.value.clone().into(),
            detail: row.detail.clone().into(),
            status: row.status.clone().into(),
        }),
    )));
    ui.set_utility_settings_rows(ModelRc::new(VecModel::from_iter(
        utilities.settings_rows.iter().map(|row| UtilityStatusItem {
            label: row.label.clone().into(),
            value: row.value.clone().into(),
            detail: row.detail.clone().into(),
            status: row.status.clone().into(),
        }),
    )));
    ui.set_utility_integration_rows(ModelRc::new(VecModel::from_iter(
        utilities
            .integration_rows
            .iter()
            .map(|row| UtilityStatusItem {
                label: row.label.clone().into(),
                value: row.value.clone().into(),
                detail: row.detail.clone().into(),
                status: row.status.clone().into(),
            }),
    )));

    let model_session = match CliSession::connect_pack_operator("desktop-model-center") {
        Ok(session) => {
            ui.set_model_operator_status("Core session ready · local admission only".into());
            Some(Rc::new(RefCell::new(session)))
        }
        Err(err) => {
            ui.set_model_operator_status(format!("Core session unavailable: {err:?}").into());
            None
        }
    };
    let product_intelligence = if let Some(session) = &model_session {
        match product_intelligence::ProductIntelligenceVm::from_session(&mut session.borrow_mut()) {
            Ok(vm) => vm,
            Err(err) => {
                ui.set_model_operator_status(
                    format!("Core inventory unavailable at startup: {err:?}").into(),
                );
                product_intelligence::ProductIntelligenceVm::current_truth()
            }
        }
    } else {
        product_intelligence::ProductIntelligenceVm::current_truth()
    };
    apply_product_intelligence(&ui, &product_intelligence);

    // Spec 074 Projects workspace: operator session plus a local synthetic
    // vault at the platform default root. Failures stay visible as status.
    let project_session = match CliSession::connect("desktop-projects") {
        Ok(mut session) => {
            let root = CliSession::default_vault_root("desktop-projects");
            match session.open_synthetic_vault(&root.display().to_string()) {
                Ok(()) => {
                    let session = Rc::new(RefCell::new(session));
                    refresh_project_list(&ui, &session);
                    Some(session)
                }
                Err(err) => {
                    ui.set_project_status(format!("Projects unavailable: {err:?}").into());
                    None
                }
            }
        }
        Err(err) => {
            ui.set_project_status(format!("Projects unavailable: {err:?}").into());
            None
        }
    };

    // Spec 061 keeps patient presentation read-only and routes consequential work to review surfaces.
    // Consequential operations are routed to their owning review surfaces; no action is committed here.
    let weak = ui.as_weak();
    let insights_for_actions = insights.clone();
    let workflow_for_actions = workflow.clone();
    let model_session_for_actions = model_session.clone();
    let project_session_for_actions = project_session.clone();
    ui.on_ui_action(move |action| {
        let Some(ui) = weak.upgrade() else {
            return;
        };
        let action = action.as_str();
        // Spec 074 project workspace actions (Core-backed, revision-guarded).
        if action == "projects-refresh" {
            if let Some(session) = &project_session_for_actions {
                refresh_project_list(&ui, session);
                let active = ui.get_project_active_id().to_string();
                if !active.is_empty() {
                    open_project_detail(&ui, session, &active);
                }
            } else {
                ui.set_project_status("Projects unavailable: no Core session".into());
            }
            return;
        }
        if action == "projects-create" {
            let Some(session) = &project_session_for_actions else {
                ui.set_project_status("Projects unavailable: no Core session".into());
                return;
            };
            let raw_name = ui.get_project_name_input().to_string();
            let raw_desc = ui.get_project_desc_input().to_string();
            let name = match project_workspace::validate_name(&raw_name) {
                Ok(name) => name,
                Err(message) => {
                    ui.set_project_status(message.into());
                    return;
                }
            };
            let description = if raw_desc.trim().is_empty() {
                None
            } else {
                Some(raw_desc.trim().to_owned())
            };
            match project_workspace::create_project(&mut session.borrow_mut(), name, description) {
                Ok(row) => {
                    ui.set_project_name_input("".into());
                    ui.set_project_desc_input("".into());
                    refresh_project_list(&ui, session);
                    open_project_detail(&ui, session, &row.id);
                }
                Err(err) => ui.set_project_status(project_workspace::status_message(&err).into()),
            }
            return;
        }
        if let Some(project_id) = action.strip_prefix("projects-open:") {
            if let Some(session) = &project_session_for_actions {
                open_project_detail(&ui, session, project_id);
            } else {
                ui.set_project_status("Projects unavailable: no Core session".into());
            }
            return;
        }
        if action == "projects-archive" {
            let Some(session) = &project_session_for_actions else {
                ui.set_project_status("Projects unavailable: no Core session".into());
                return;
            };
            let project_id = ui.get_project_active_id().to_string();
            let revision = ui.get_project_active_revision().to_string().parse::<u64>();
            let Ok(revision) = revision else {
                ui.set_project_status("Invalid: open project has no revision".into());
                return;
            };
            match project_workspace::archive_project(&mut session.borrow_mut(), &project_id, revision) {
                Ok(_) => {
                    refresh_project_list(&ui, session);
                    open_project_detail(&ui, session, &project_id);
                }
                Err(err) => ui.set_project_status(project_workspace::status_message(&err).into()),
            }
            return;
        }
        if action == "projects-exp-create" {
            let Some(session) = &project_session_for_actions else {
                ui.set_project_status("Projects unavailable: no Core session".into());
                return;
            };
            let project_id = ui.get_project_active_id().to_string();
            if project_id.is_empty() {
                ui.set_project_status("Invalid: open a project first".into());
                return;
            }
            let raw_name = ui.get_project_experiment_input().to_string();
            let name = match project_workspace::validate_name(&raw_name) {
                Ok(name) => name,
                Err(message) => {
                    ui.set_project_status(message.into());
                    return;
                }
            };
            match project_workspace::create_experiment(&mut session.borrow_mut(), &project_id, name) {
                Ok(_) => {
                    ui.set_project_experiment_input("".into());
                    open_project_detail(&ui, session, &project_id);
                }
                Err(err) => ui.set_project_status(project_workspace::status_message(&err).into()),
            }
            return;
        }
        // Spec 075 data workbench actions (Core-backed snapshots and views).
        if action == "data-refresh" {
            if let Some(session) = &project_session_for_actions {
                refresh_data_sources(&ui, session);
                let active = ui.get_data_active_source().to_string();
                if !active.is_empty() {
                    open_data_source(&ui, session, &active);
                }
            } else {
                ui.set_data_status("Data unavailable: no Core session".into());
            }
            return;
        }
        if let Some(source_id) = action.strip_prefix("data-open:") {
            if let Some(session) = &project_session_for_actions {
                open_data_source(&ui, session, source_id);
            } else {
                ui.set_data_status("Data unavailable: no Core session".into());
            }
            return;
        }
        if let Some(snapshot_id) = action.strip_prefix("data-snapshot:") {
            if let Some(session) = &project_session_for_actions {
                open_data_snapshot(&ui, session, snapshot_id);
            } else {
                ui.set_data_status("Data unavailable: no Core session".into());
            }
            return;
        }
        if action == "data-import" {
            let Some(session) = &project_session_for_actions else {
                ui.set_data_status("Data unavailable: no Core session".into());
                return;
            };
            let source_id = ui.get_data_active_source().to_string();
            if source_id.is_empty() {
                ui.set_data_status("Invalid: open a data source first".into());
                return;
            }
            match data_workbench::import_snapshot(&mut session.borrow_mut(), &source_id) {
                Ok(row) => {
                    refresh_data_sources(&ui, session);
                    open_data_source(&ui, session, &source_id);
                    ui.set_data_status(format!("Imported snapshot {}", row.id).into());
                }
                Err(err) => ui.set_data_status(data_workbench::status_message(&err).into()),
            }
            return;
        }
        if action == "data-refresh-source" || action == "data-refresh-source-force" {
            let Some(session) = &project_session_for_actions else {
                ui.set_data_status("Data unavailable: no Core session".into());
                return;
            };
            let source_id = ui.get_data_active_source().to_string();
            if source_id.is_empty() {
                ui.set_data_status("Invalid: open a data source first".into());
                return;
            }
            let allow = action == "data-refresh-source-force";
            match data_workbench::refresh_source(&mut session.borrow_mut(), &source_id, allow) {
                Ok(summary) => {
                    refresh_data_sources(&ui, session);
                    open_data_source(&ui, session, &source_id);
                    ui.set_data_status(summary.into());
                }
                Err(err) => ui.set_data_status(data_workbench::status_message(&err).into()),
            }
            return;
        }
        if action == "data-view-grid" {
            let Some(session) = &project_session_for_actions else {
                ui.set_data_status("Data unavailable: no Core session".into());
                return;
            };
            let snapshot_id = ui.get_data_active_snapshot().to_string();
            if snapshot_id.is_empty() {
                ui.set_data_status("Invalid: open a snapshot first".into());
                return;
            }
            match data_workbench::create_grid_view(&mut session.borrow_mut(), &snapshot_id) {
                Ok(view_id) => {
                    ui.set_data_status(format!("Saved grid view {view_id}").into());
                }
                Err(err) => ui.set_data_status(data_workbench::status_message(&err).into()),
            }
            return;
        }
        if action == "data-transform-apply" {
            let Some(session) = &project_session_for_actions else {
                ui.set_data_status("Data unavailable: no Core session".into());
                return;
            };
            let snapshot_id = ui.get_data_active_snapshot().to_string();
            if snapshot_id.is_empty() {
                ui.set_data_status("Invalid: open a snapshot first".into());
                return;
            }
            let columns_raw = ui.get_data_columns_input().to_string();
            let columns = if columns_raw.trim().is_empty() {
                None
            } else {
                Some(
                    columns_raw
                        .split(',')
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(str::to_owned)
                        .collect::<Vec<_>>(),
                )
            };
            let filter = match data_workbench::parse_filter_input(
                &ui.get_data_filter_col(),
                &ui.get_data_filter_op(),
                &ui.get_data_filter_value(),
            ) {
                Ok(filter) => filter,
                Err(message) => {
                    ui.set_data_status(message.into());
                    return;
                }
            };
            let sort = data_workbench::parse_sort_input(&ui.get_data_sort_input());
            match data_workbench::apply_workbench_transform(
                &mut session.borrow_mut(),
                &snapshot_id,
                columns,
                filter,
                sort,
            ) {
                Ok(summary) => {
                    open_data_source(&ui, session, &ui.get_data_active_source());
                    ui.set_data_status(summary.into());
                }
                Err(err) => ui.set_data_status(data_workbench::status_message(&err).into()),
            }
            return;
        }
        if action == "model-refresh" {            if let Some(session) = &model_session_for_actions {
                match refresh_model_center(&ui, session) {
                    Ok(()) => ui.set_model_operator_status("Core inventory refreshed".into()),
                    Err(err) => ui.set_model_operator_status(
                        format!("Core inventory refresh failed: {err:?}").into(),
                    ),
                }
            } else {
                ui.set_model_operator_status(
                    "Core session unavailable; inventory not refreshed".into(),
                );
            }
            return;
        }
        if action == "model-admit" {
            let Some(session) = &model_session_for_actions else {
                ui.set_model_operator_status(
                    "Core session unavailable; Pack admission denied".into(),
                );
                return;
            };
            let raw_path = ui.get_model_pack_path().to_string();
            let path = match validated_model_pack_path(&raw_path) {
                Ok(path) => path,
                Err(message) => {
                    ui.set_model_operator_status(message.into());
                    return;
                }
            };
            let admission = admit_model_pack(session, path);
            match admission {
                Ok(result) if result.admitted => {
                    let pack_id = result
                        .pack_id
                        .as_ref()
                        .map_or("unknown", medscale_contracts::objects::OpaqueId::as_str);
                    ui.set_model_pack_path("".into());
                    match refresh_model_center(&ui, session) {
                        Ok(()) => ui.set_model_operator_status(
                            format!("Pack admitted by Core: {pack_id}").into(),
                        ),
                        Err(err) => ui.set_model_operator_status(
                            format!("Pack admitted by Core: {pack_id}; inventory refresh failed: {err:?}").into(),
                        ),
                    }
                }
                Ok(result) => ui.set_model_operator_status(
                    format!("Pack admission refused: {:?}", result.reason).into(),
                ),
                Err(err) => {
                    ui.set_model_operator_status(format!("Pack admission failed: {err:?}").into())
                }
            }
            return;
        }
        if let Some(query) = action.strip_prefix("insights-assistant:") {
            ui.set_insights_assistant_answer(insights_for_actions.answer_query(query).into());
            ui.set_active_route("Insights".into());
            return;
        }
        if let Some(action_id) = action.strip_prefix("workflow-review:") {
            ui.set_workflow_review_detail(workflow_for_actions.review_detail(action_id).into());
            ui.set_active_route("Tasks".into());
            return;
        }
        let route = if action == "care-plan" {
            Some("Workflows")
        } else if action == "summarize" || action == "import" || action.starts_with("search:") {
            Some("Patients")
        } else if action == "privacy-status" {
            Some("Settings")
        } else {
            None
        };
        if let Some(route) = route {
            ui.set_active_route(route.into());
        }
    });

    match ui.run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("MedScale Desktop UI exited with an error: {err}");
            ExitCode::from(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{admit_model_pack, perf_idle_ms, validated_model_pack_path};
    use medscale_core::CliSession;
    use std::cell::RefCell;
    use std::path::PathBuf;
    use std::rc::Rc;

    #[test]
    fn perf_idle_probe_is_bounded_and_explicit() {
        assert_eq!(
            perf_idle_ms(&["--perf-idle-ms".to_owned(), "250".to_owned()]),
            Ok(Some(250))
        );
        assert!(perf_idle_ms(&["--perf-idle-ms".to_owned(), "49".to_owned()]).is_err());
        assert!(perf_idle_ms(&["--perf-idle-ms".to_owned()]).is_err());
    }

    #[test]
    fn model_pack_path_requires_an_absolute_directory_path() {
        assert!(validated_model_pack_path("").is_err());
        assert!(validated_model_pack_path("relative/pack").is_err());

        let absolute = std::env::temp_dir().join("signed-pack");
        let raw = format!(" {} ", absolute.display());
        assert_eq!(
            validated_model_pack_path(&raw),
            Ok(absolute.to_str().expect("temp path must be valid UTF-8"))
        );
    }

    #[test]
    fn model_pack_admission_releases_session_borrow_before_refresh() {
        let session = Rc::new(RefCell::new(
            CliSession::connect_pack_operator("desktop-model-center-borrow-test")
                .expect("connect least-privilege Model Center session"),
        ));
        let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../evidence/008-local-ai-capability-fabric/fixtures/pack-fixture-ner-v0");
        let result = admit_model_pack(&session, &fixture.display().to_string())
            .expect("admit signed fixture through scoped borrow");
        assert!(result.admitted);
        assert!(
            session.try_borrow_mut().is_ok(),
            "admission must release RefCell borrow before Model Center refresh"
        );
    }

    #[test]
    fn no_tauri_or_runtime_crate_in_manifest() {
        let manifest =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml")).unwrap();
        assert!(!manifest.contains("tauri"));
        assert!(
            !manifest.contains("medscale-pack"),
            "Desktop must reach Pack state through Core authority, not the runtime crate"
        );
    }
}
