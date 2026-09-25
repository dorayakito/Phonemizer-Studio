use crate::models::{PhonemizerProject, TestCase, TestExecutionResult};
use crate::simulator::PhonemizerSimulator;
use crate::ui::helpers::section_header;
use egui::Ui;

pub struct TestSuiteViewState {
    pub execution_results: Vec<TestExecutionResult>,
    pub filter_passed_only: bool,
    pub filter_failed_only: bool,
    pub new_test_name: String,
    pub new_test_lyrics: String,
    pub new_test_expected: String,
}

impl Default for TestSuiteViewState {
    fn default() -> Self {
        Self {
            execution_results: Vec::new(),
            filter_passed_only: false,
            filter_failed_only: false,
            new_test_name: "Novo Teste".to_string(),
            new_test_lyrics: "la la".to_string(),
            new_test_expected: "- la, la".to_string(),
        }
    }
}

pub fn render_test_suite_view(
    ui: &mut Ui,
    project: &mut PhonemizerProject,
    state: &mut TestSuiteViewState,
) {
    section_header(
        ui,
        "Suite de Testes Automatizados de Regras Foneticas (Unit Testing)",
        Some("Defina casos de teste com entradas e saidas esperadas de aliases para garantir regressao zero ao alterar regras."),
    );

    ui.group(|ui| {
        ui.horizontal(|ui| {
            if ui.button(egui::RichText::new(" Executar Todos os Testes").color(egui::Color32::from_rgb(100, 240, 160)).strong()).clicked() {
                run_all_tests(project, state);
            }

            let total_tests = project.test_cases.len();
            let passed_count = state.execution_results.iter().filter(|r| r.passed).count();
            let failed_count = state.execution_results.iter().filter(|r| !r.passed).count();

            if !state.execution_results.is_empty() {
                ui.separator();
                ui.label(egui::RichText::new(format!("Total: {}", total_tests)).strong());
                ui.label(egui::RichText::new(format!("Passaram: {}", passed_count)).color(egui::Color32::from_rgb(100, 240, 160)).strong());
                if failed_count > 0 {
                    ui.label(egui::RichText::new(format!("Falharam: {}", failed_count)).color(egui::Color32::from_rgb(255, 100, 100)).strong());
                }
            }

            if ui.button("Gerar Casos de Teste Automaticos").clicked() {
                populate_default_tests_if_empty(project);
                run_all_tests(project, state);
            }
        });
    });

    ui.add_space(10.0);

    // Form to add new test case
    ui.collapsing("Adicionar Novo Caso de Teste", |ui| {
        ui.horizontal(|ui| {
            ui.label("Nome:");
            ui.add(egui::TextEdit::singleline(&mut state.new_test_name).desired_width(140.0));

            ui.label("Letra / Entrada:");
            ui.add(egui::TextEdit::singleline(&mut state.new_test_lyrics).desired_width(180.0));

            ui.label("Aliases Esperados (separados por virgula):");
            ui.add(egui::TextEdit::singleline(&mut state.new_test_expected).desired_width(200.0));

            if ui.button(egui::RichText::new(" Adicionar").strong()).clicked() {
                let expected: Vec<String> = state
                    .new_test_expected
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                project.test_cases.push(TestCase {
                    id: format!("{:x}", project.test_cases.len() + 1),
                    name: state.new_test_name.clone(),
                    lyrics_input: state.new_test_lyrics.clone(),
                    tone_input: "C4".to_string(),
                    bpm: 120.0,
                    expected_phonemes: expected,
                    expected_durations_ms: None,
                });
            }
        });
    });

    ui.add_space(10.0);

    if project.test_cases.is_empty() {
        ui.label("Nenhum caso de teste cadastrado. Clique em 'Gerar Casos de Teste Automaticos' ou crie novos acima.");
        return;
    }

    let mut to_remove = None;

    egui::ScrollArea::vertical()
        .id_salt("test_suite_list_scroll")
        .show(ui, |ui| {
            for (idx, tc) in project.test_cases.iter_mut().enumerate() {
                let res = state.execution_results.iter().find(|r| r.test_case_id == tc.id);

                let bg_color = match res {
                    Some(r) if r.passed => egui::Color32::from_rgb(18, 35, 25),
                    Some(_) => egui::Color32::from_rgb(45, 18, 20),
                    None => egui::Color32::from_rgb(22, 26, 35),
                };

                let stroke_color = match res {
                    Some(r) if r.passed => egui::Color32::from_rgb(40, 140, 80),
                    Some(_) => egui::Color32::from_rgb(180, 50, 60),
                    None => egui::Color32::from_rgb(45, 55, 75),
                };

                egui::Frame::group(ui.style())
                    .fill(bg_color)
                    .stroke(egui::Stroke::new(1.0, stroke_color))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let status_icon = match res {
                                Some(r) if r.passed => "[PASS]",
                                Some(_) => "[FAIL]",
                                None => "[PENDING]",
                            };

                            let status_color = match res {
                                Some(r) if r.passed => egui::Color32::from_rgb(100, 240, 160),
                                Some(_) => egui::Color32::from_rgb(255, 100, 100),
                                None => egui::Color32::from_rgb(180, 180, 180),
                            };

                            ui.label(egui::RichText::new(status_icon).color(status_color).strong());
                            ui.label(egui::RichText::new(&tc.name).strong());
                            ui.label(format!("Entrada: \"{}\"", tc.lyrics_input));
                            ui.label(format!("Esperado: [{}]", tc.expected_phonemes.join(", ")));

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button(egui::RichText::new("").color(egui::Color32::from_rgb(255, 100, 100))).clicked() {
                                    to_remove = Some(idx);
                                }
                            });
                        });

                        if let Some(r) = res {
                            if !r.passed {
                                ui.add_space(4.0);
                                ui.label(egui::RichText::new(format!("Diferenca: {}", r.diff_message)).color(egui::Color32::from_rgb(255, 160, 160)));
                                ui.label(egui::RichText::new(format!("Obtido: [{}]", r.actual_phonemes.join(", "))).color(egui::Color32::from_rgb(200, 200, 200)));
                            }
                        }
                    });
                ui.add_space(6.0);
            }
        });

    if let Some(idx) = to_remove {
        if idx < project.test_cases.len() {
            project.test_cases.remove(idx);
        }
    }
}

fn run_all_tests(project: &PhonemizerProject, state: &mut TestSuiteViewState) {
    let simulator = PhonemizerSimulator::new(project);
    let mut results = Vec::new();

    for tc in &project.test_cases {
        let sim_res = simulator.simulate_phrase(&tc.lyrics_input);
        let mut actual_aliases = Vec::new();
        for n in &sim_res {
            for ph in &n.generated_phonemes {
                actual_aliases.push(ph.final_alias.clone());
            }
        }

        let passed = actual_aliases == tc.expected_phonemes;
        let diff_message = if !passed {
            format!("Esperado: {:?}, Obtido: {:?}", tc.expected_phonemes, actual_aliases)
        } else {
            String::new()
        };

        results.push(TestExecutionResult {
            test_case_id: tc.id.clone(),
            passed,
            actual_phonemes: actual_aliases,
            diff_message,
            duration_ms: 1,
        });
    }

    state.execution_results = results;
}

fn populate_default_tests_if_empty(project: &mut PhonemizerProject) {
    let sample_phrases = ["la", "ka sa", "na no", "ta te ti"];
    let mut new_cases = Vec::new();

    {
        let simulator = PhonemizerSimulator::new(project);
        for (i, phrase) in sample_phrases.iter().enumerate() {
            let res = simulator.simulate_phrase(phrase);
            let mut aliases = Vec::new();
            for n in &res {
                for ph in &n.generated_phonemes {
                    aliases.push(ph.final_alias.clone());
                }
            }

            if !aliases.is_empty() {
                new_cases.push(TestCase {
                    id: format!("auto_{}", i + 1),
                    name: format!("Teste Automatico: {}", phrase),
                    lyrics_input: phrase.to_string(),
                    tone_input: "C4".to_string(),
                    bpm: 120.0,
                    expected_phonemes: aliases,
                    expected_durations_ms: None,
                });
            }
        }
    }

    project.test_cases.extend(new_cases);
}
