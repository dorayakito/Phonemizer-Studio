use crate::models::{AppSettings, PhonemizerProject, SystemSpecs};
use crate::presets::*;
use crate::ui::clusters_view::render_clusters_view;
use crate::ui::code_view::render_code_view;
use crate::ui::compiler_view::{render_compiler_view, CompilerViewState};
use crate::ui::consonants_view::{render_consonants_view, ConsonantsViewState};
use crate::ui::docs_view::render_docs_view;
use crate::ui::exceptions_view::{render_exceptions_view, ExceptionsViewState};
use crate::ui::github_import_view::{render_github_import_view, GitHubImportState};
use crate::ui::matrix_view::render_matrix_view;
use crate::ui::metadata_view::render_metadata_view;
use crate::ui::settings_view::{render_settings_view, SettingsViewState};
use crate::ui::simulator_view::{render_simulator_view, SimulatorState};
use crate::ui::test_suite_view::{render_test_suite_view, TestSuiteViewState};
use crate::ui::vowels_view::{render_vowels_view, VowelsViewState};
use eframe::egui;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    Metadata,
    Vowels,
    Consonants,
    Clusters,
    Matrix,
    Exceptions,
    Simulator,
    TestSuite,
    GitHubImport,
    CSharpCode,
    Compiler,
    Settings,
    Documentation,
}

pub struct PhonemizerStudioApp {
    pub project: PhonemizerProject,
    pub settings: AppSettings,
    pub system_specs: SystemSpecs,
    pub active_tab: ActiveTab,
    pub current_file_path: Option<String>,
    pub status_message: String,
    pub last_auto_save: Instant,
    pub simulator_state: SimulatorState,
    pub test_suite_state: TestSuiteViewState,
    pub compiler_state: CompilerViewState,
    pub vowels_state: VowelsViewState,
    pub consonants_state: ConsonantsViewState,
    pub exceptions_state: ExceptionsViewState,
    pub github_import_state: GitHubImportState,
    pub settings_state: SettingsViewState,
}

impl Default for PhonemizerStudioApp {
    fn default() -> Self {
        let loaded_settings = AppSettings::load();
        let specs = SystemSpecs::detect(if loaded_settings.custom_dotnet_path.is_empty() {
            None
        } else {
            Some(&loaded_settings.custom_dotnet_path)
        });

        Self {
            project: create_portuguese_preset(),
            settings: loaded_settings,
            system_specs: specs,
            active_tab: ActiveTab::Metadata,
            current_file_path: None,
            status_message: "Pronto. Projeto padrao carregado.".to_string(),
            last_auto_save: Instant::now(),
            simulator_state: SimulatorState::new(),
            test_suite_state: TestSuiteViewState::default(),
            compiler_state: CompilerViewState::default(),
            vowels_state: VowelsViewState::default(),
            consonants_state: ConsonantsViewState::default(),
            exceptions_state: ExceptionsViewState::default(),
            github_import_state: GitHubImportState::default(),
            settings_state: SettingsViewState::default(),
        }
    }
}

impl PhonemizerStudioApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Setup comprehensive Unicode & IPA phonetic fonts
        crate::ui::fonts::setup_custom_fonts(&cc.egui_ctx);

        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = egui::Color32::from_rgb(18, 20, 26);
        visuals.panel_fill = egui::Color32::from_rgb(22, 24, 30);
        visuals.override_text_color = Some(egui::Color32::from_rgb(232, 238, 248));
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(28, 31, 38);
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(32, 36, 46);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(45, 52, 68);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(55, 65, 85);
        cc.egui_ctx.set_visuals(visuals);

        Self::default()
    }

    fn save_project_to_disk(&mut self, path: &str) {
        match serde_json::to_string_pretty(&self.project) {
            Ok(json) => match std::fs::write(path, json) {
                Ok(_) => {
                    self.current_file_path = Some(path.to_string());
                    self.status_message = format!("Projeto salvo com sucesso em: {}", path);
                }
                Err(e) => {
                    self.status_message = format!("Erro ao salvar arquivo: {}", e);
                }
            },
            Err(e) => {
                self.status_message = format!("Erro ao serializar projeto: {}", e);
            }
        }
    }

    fn load_project_from_disk(&mut self, path: &str) {
        match std::fs::read_to_string(path) {
            Ok(json) => match serde_json::from_str::<PhonemizerProject>(&json) {
                Ok(proj) => {
                    self.project = proj;
                    self.current_file_path = Some(path.to_string());
                    self.status_message = format!("Projeto carregado com sucesso de: {}", path);
                }
                Err(e) => {
                    self.status_message = format!("Erro ao parsear arquivo: {}", e);
                }
            },
            Err(e) => {
                self.status_message = format!("Erro ao ler arquivo: {}", e);
            }
        }
    }
}

impl eframe::App for PhonemizerStudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply custom UI zoom scale
        if (self.settings.ui_scale - 1.0).abs() > 0.01 {
            ctx.set_zoom_factor(self.settings.ui_scale);
        }

        // Periodic auto-save if enabled and file has a path
        if self.settings.auto_save_enabled {
            let interval_secs = (self.settings.auto_save_interval_minutes as u64).max(1) * 60;
            if self.last_auto_save.elapsed().as_secs() >= interval_secs {
                if let Some(ref path) = self.current_file_path.clone() {
                    self.save_project_to_disk(path);
                    self.status_message = format!("Auto-salvamento automatico realizado: {}", path);
                }
                self.last_auto_save = Instant::now();
            }
        }

        // Repaint timer for compilation / network jobs
        if self.compiler_state.is_compiling.load(std::sync::atomic::Ordering::Relaxed)
            || self.github_import_state.is_loading.load(std::sync::atomic::Ordering::Relaxed)
        {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }

        // Global keyboard shortcut for settings (Ctrl+, or Cmd+,)
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::Comma)) {
            self.active_tab = ActiveTab::Settings;
        }

        // Top Menu Bar
        egui::TopBottomPanel::top("top_menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Arquivo", |ui| {
                    if ui.button("Novo Projeto (Universal / Do Zero)").clicked() {
                        self.project = create_universal_scratch_preset();
                        self.current_file_path = None;
                        self.status_message = "Novo projeto universal criado do zero.".to_string();
                        ui.close_menu();
                    }

                    if ui.button("Abrir Projeto (.json)...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().add_filter("Phonemizer Project", &["json"]).pick_file() {
                            let path_str = path.to_string_lossy().to_string();
                            self.load_project_from_disk(&path_str);
                        }
                        ui.close_menu();
                    }

                    if ui.button("Salvar").clicked() {
                        if let Some(ref path) = self.current_file_path.clone() {
                            self.save_project_to_disk(path);
                        } else if let Some(path) = rfd::FileDialog::new().add_filter("Phonemizer Project", &["json"]).set_file_name(&format!("{}.json", self.project.name)).save_file() {
                            let path_str = path.to_string_lossy().to_string();
                            self.save_project_to_disk(&path_str);
                        }
                        ui.close_menu();
                    }

                    if ui.button("Salvar Como...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().add_filter("Phonemizer Project", &["json"]).set_file_name(&format!("{}.json", self.project.name)).save_file() {
                            let path_str = path.to_string_lossy().to_string();
                            self.save_project_to_disk(&path_str);
                        }
                        ui.close_menu();
                    }

                    ui.separator();

                    if ui.button("Importar do GitHub / OpenUtau Hub...").clicked() {
                        self.active_tab = ActiveTab::GitHubImport;
                        ui.close_menu();
                    }

                    ui.separator();

                    if ui.button("Configuracoes & Hardware (Ctrl+,)").clicked() {
                        self.active_tab = ActiveTab::Settings;
                        ui.close_menu();
                    }

                    ui.separator();

                    if ui.button("Sair").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Presets Oficiais OpenUtau", |ui| {
                    if ui.button("Universal / Em Branco (Scratch do Zero)").clicked() {
                        self.project = create_universal_scratch_preset();
                        self.status_message = "Preset Universal em branco carregado.".to_string();
                        ui.close_menu();
                    }

                    ui.separator();

                    if ui.button("Portugues Brasileiro CVC (PT-BR CVC: HAI-D / BRAPA)").clicked() {
                        self.project = create_portuguese_preset();
                        self.status_message = "Preset Oficial de Portugues Brasileiro CVC carregado.".to_string();
                        ui.close_menu();
                    }

                    if ui.button("Japones VCV (JA VCV: Oficial stakira)").clicked() {
                        self.project = create_japanese_vcv_preset();
                        self.status_message = "Preset Oficial de Japones VCV carregado.".to_string();
                        ui.close_menu();
                    }

                    if ui.button("Japones CVVC (JA CVVC: TUBS)").clicked() {
                        self.project = create_japanese_cvvc_preset();
                        self.status_message = "Preset Oficial de Japones CVVC carregado.".to_string();
                        ui.close_menu();
                    }

                    if ui.button("Ingles Arpasing (EN ARPA: CMUdict)").clicked() {
                        self.project = create_english_arpasing_preset();
                        self.status_message = "Preset Oficial de Ingles Arpasing carregado.".to_string();
                        ui.close_menu();
                    }

                    if ui.button("Ingles VCCV (EN VCCV: Cz / cubialpha & Mim)").clicked() {
                        self.project = create_english_vccv_preset();
                        self.status_message = "Preset Oficial de Ingles VCCV carregado.".to_string();
                        ui.close_menu();
                    }

                    if ui.button("Espanhol Silabico (ES SYL: Lotte V)").clicked() {
                        self.project = create_spanish_preset();
                        self.status_message = "Preset Oficial de Espanhol Silabico carregado.".to_string();
                        ui.close_menu();
                    }

                    if ui.button("Frances CVVC (FR CVVC: Mim)").clicked() {
                        self.project = create_french_preset();
                        self.status_message = "Preset Oficial de Frances CVVC carregado.".to_string();
                        ui.close_menu();
                    }

                    if ui.button("Russo CVC (RU CVC: Heiden.BZR)").clicked() {
                        self.project = create_russian_cvc_preset();
                        self.status_message = "Preset Oficial de Russo CVC carregado.".to_string();
                        ui.close_menu();
                    }

                    if ui.button("Italiano Silabico (IT SYL: Lotte V)").clicked() {
                        self.project = create_italian_preset();
                        self.status_message = "Preset Oficial de Italiano carregado.".to_string();
                        ui.close_menu();
                    }

                    if ui.button("Alemao VCCV (DE VCCV: Lotte V)").clicked() {
                        self.project = create_german_vccv_preset();
                        self.status_message = "Preset Oficial de Alemao VCCV carregado.".to_string();
                        ui.close_menu();
                    }

                    if ui.button("Coreano CVC (KO CVC: NANA)").clicked() {
                        self.project = create_korean_hangul_preset();
                        self.status_message = "Preset Oficial de Coreano CVC carregado.".to_string();
                        ui.close_menu();
                    }

                    if ui.button("Chines CVV Mandarim (ZH CVV: stakira)").clicked() {
                        self.project = create_chinese_cvv_preset();
                        self.status_message = "Preset Oficial de Chines CVV carregado.".to_string();
                        ui.close_menu();
                    }

                    if ui.button("Cantones CVVC (YUE CVVC: stakira)").clicked() {
                        self.project = create_cantonese_preset();
                        self.status_message = "Preset Oficial de Cantones CVVC carregado.".to_string();
                        ui.close_menu();
                    }

                    if ui.button("Polones CVC (PL CVC: Heiden.BZR)").clicked() {
                        self.project = create_polish_preset();
                        self.status_message = "Preset Oficial de Polones CVC carregado.".to_string();
                        ui.close_menu();
                    }

                    if ui.button("Tailandes VCCV (TH VCCV: Kuro & mim)").clicked() {
                        self.project = create_thai_preset();
                        self.status_message = "Preset Oficial de Tailandes VCCV carregado.".to_string();
                        ui.close_menu();
                    }

                    if ui.button("Turco CVVC (TR CVVC: mim & Berke)").clicked() {
                        self.project = create_turkish_preset();
                        self.status_message = "Preset Oficial de Turco CVVC carregado.".to_string();
                        ui.close_menu();
                    }

                    if ui.button("Vietnamita CVVC (VI CVVC: mim & kuro)").clicked() {
                        self.project = create_vietnamese_preset();
                        self.status_message = "Preset Oficial de Vietnamita CVVC carregado.".to_string();
                        ui.close_menu();
                    }

                    if ui.button("Latim Difone (LA DIPHONE: stakira)").clicked() {
                        self.project = create_latin_preset();
                        self.status_message = "Preset Oficial de Latim Difone carregado.".to_string();
                        ui.close_menu();
                    }
                });

                ui.menu_button("Ajuda & Documentacao", |ui| {
                    if ui.button("Abrir Guia e Downloads").clicked() {
                        self.active_tab = ActiveTab::Documentation;
                        ui.close_menu();
                    }
                    if ui.button("Configuracoes & Hardware").clicked() {
                        self.active_tab = ActiveTab::Settings;
                        ui.close_menu();
                    }
                    if ui.button("Site Oficial OpenUtau").clicked() {
                        let _ = open::that("https://github.com/openutau/OpenUtau");
                    }
                    if ui.button("Baixar .NET SDK").clicked() {
                        let _ = open::that("https://dotnet.microsoft.com/download/dotnet/8.0");
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("OpenUtau Phonemizer Studio v1.0").color(egui::Color32::from_rgb(140, 170, 220)).strong());
                });
            });
        });

        // Tab Navigation Bar
        egui::TopBottomPanel::top("tab_navigation_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.selectable_value(&mut self.active_tab, ActiveTab::Metadata, "Metadados");
                ui.selectable_value(&mut self.active_tab, ActiveTab::Vowels, format!("Vogais ({})", self.project.vowels.len()));
                ui.selectable_value(&mut self.active_tab, ActiveTab::Consonants, format!("Consoantes ({})", self.project.consonants.len()));
                ui.selectable_value(&mut self.active_tab, ActiveTab::Clusters, format!("Clusters ({})", self.project.onset_clusters.len() + self.project.coda_clusters.len()));
                ui.selectable_value(&mut self.active_tab, ActiveTab::Matrix, format!("Matriz ({})", self.project.pattern_rules.len()));
                ui.selectable_value(&mut self.active_tab, ActiveTab::Exceptions, format!("Excecoes ({})", self.project.dictionary_overrides.len() + self.project.regex_rules.len()));
                ui.selectable_value(&mut self.active_tab, ActiveTab::Simulator, "Simulador & Audio");
                ui.selectable_value(&mut self.active_tab, ActiveTab::TestSuite, format!("Testes ({})", self.project.test_cases.len()));
                ui.selectable_value(&mut self.active_tab, ActiveTab::GitHubImport, "OpenUtau GitHub Hub");
                ui.selectable_value(&mut self.active_tab, ActiveTab::CSharpCode, "Codigo C#");
                ui.selectable_value(&mut self.active_tab, ActiveTab::Compiler, "Compilar .DLL");
                ui.selectable_value(&mut self.active_tab, ActiveTab::Settings, "Configuracoes & Hardware");
                ui.selectable_value(&mut self.active_tab, ActiveTab::Documentation, "Documentacao & Downloads");
            });
        });

        // Bottom Status Bar
        egui::TopBottomPanel::bottom("bottom_status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(&self.status_message).size(12.0).color(egui::Color32::from_rgb(180, 200, 230)));
                if let Some(ref path) = self.current_file_path {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new(format!("Arquivo: {}", path)).size(11.0));
                    });
                }
            });
        });

        // Main Central Content Panel
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.active_tab {
                ActiveTab::Metadata => render_metadata_view(ui, &mut self.project),
                ActiveTab::Vowels => render_vowels_view(ui, &mut self.project, &mut self.vowels_state),
                ActiveTab::Consonants => render_consonants_view(ui, &mut self.project, &mut self.consonants_state),
                ActiveTab::Clusters => render_clusters_view(ui, &mut self.project),
                ActiveTab::Matrix => render_matrix_view(ui, &mut self.project),
                ActiveTab::Exceptions => render_exceptions_view(ui, &mut self.project, &mut self.exceptions_state),
                ActiveTab::Simulator => render_simulator_view(ui, &self.project, &mut self.simulator_state),
                ActiveTab::TestSuite => render_test_suite_view(ui, &mut self.project, &mut self.test_suite_state),
                ActiveTab::GitHubImport => render_github_import_view(ui, &mut self.project, &mut self.github_import_state),
                ActiveTab::CSharpCode => render_code_view(ui, &mut self.project),
                ActiveTab::Compiler => render_compiler_view(ui, &mut self.project, &mut self.compiler_state),
                ActiveTab::Settings => render_settings_view(ui, &mut self.settings, &mut self.system_specs, &mut self.settings_state),
                ActiveTab::Documentation => render_docs_view(ui),
            }
        });
    }
}
