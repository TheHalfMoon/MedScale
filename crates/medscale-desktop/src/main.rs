//! Native MedScale Desktop shell — Slint, no WebView/Tauri (Spec 060).

use medscale_core::{CoreFacade, build_doctor_report, privacy_proof_artifact_present};
use std::env;
use std::hint::black_box;
use std::io::{self, Write};
use std::process::ExitCode;
use std::thread;
use std::time::Duration;

use slint::{ComponentHandle, ModelRc, VecModel};

mod patient_workspace;
mod population_insights;

slint::include_modules!();

const PERF_IDLE_MAX_MS: u64 = 10_000;

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
    ui.set_product_version(report.version.into());

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

    // Spec 061 keeps patient presentation read-only and routes consequential work to review surfaces.
    // Consequential operations are routed to their owning review surfaces; no action is committed here.
    let weak = ui.as_weak();
    let insights_for_actions = insights.clone();
    ui.on_ui_action(move |action| {
        let Some(ui) = weak.upgrade() else {
            return;
        };
        let action = action.as_str();
        if let Some(query) = action.strip_prefix("insights-assistant:") {
            ui.set_insights_assistant_answer(insights_for_actions.answer_query(query).into());
            ui.set_active_route("Insights".into());
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
    use super::perf_idle_ms;

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
    fn no_tauri_in_manifest() {
        let manifest =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml")).unwrap();
        assert!(!manifest.contains("tauri"));
    }
}
