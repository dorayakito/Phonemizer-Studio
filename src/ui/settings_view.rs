use crate::models::{AppSettings, DotnetTarget, LogVerbosity, RenderBackend, RoslynOptimizationLevel, SystemSpecs};
use crate::ui::helpers::{help_marker, section_header};
use egui::Ui;

pub struct SettingsViewState {
    pub show_api_key: bool,
    pub api_test_status: Option<(bool, String)>,
    pub is_testing_api: bool,
    pub status_feedback: Option<String>,
}

impl Default for SettingsViewState {
    fn default() -> Self {
        Self {
            show_api_key: false,
            api_test_status: None,
            is_testing_api: false,
            status_feedback: None,
        }
    }
}

pub fn render_settings_view(
    ui: &mut Ui,
    settings: &mut AppSettings,
    system_specs: &mut SystemSpecs,
    state: &mut SettingsViewState,
) {
    section_header(
        ui,
        "Especificacoes do Sistema, Hardware & Configuracoes Globais",
        Some("Gerencie a aceleracao de hardware GPU, multi-threading, especificacoes do compilador Roslyn .NET e caminhos do OpenUtau."),
    );

    if let Some(ref feedback) = state.status_feedback {
        egui::Frame::group(ui.style())
            .fill(egui::Color32::from_rgb(20, 45, 35))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 180, 110)))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(feedback).color(egui::Color32::from_rgb(120, 255, 180)).strong());
                });
            });
        ui.add_space(8.0);
    }

    egui::ScrollArea::vertical()
        .id_salt("settings_scroll_area")
        .show(ui, |ui| {

            egui::Frame::group(ui.style())
                .fill(egui::Color32::from_rgb(22, 26, 36))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 60, 85)))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Especificacoes de Hardware & Diagnostico do Sistema").strong().size(15.0).color(egui::Color32::from_rgb(100, 200, 255)));
                        help_marker(ui, "Informacoes detalhadas do hardware detectado e estado do ambiente .NET.");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Re-verificar Hardware & .NET").clicked() {
                                *system_specs = SystemSpecs::detect(if settings.custom_dotnet_path.is_empty() { None } else { Some(&settings.custom_dotnet_path) });
                                state.status_feedback = Some("Diagnostico de hardware e .NET SDK atualizado com sucesso!".to_string());
                            }
                        });
                    });

                    ui.add_space(6.0);
                    egui::Grid::new("specs_grid")
                        .num_columns(2)
                        .spacing([25.0, 7.0])
                        .show(ui, |ui| {
                            ui.label("Sistema Operacional & Arquitetura:");
                            ui.label(egui::RichText::new(format!("{} ({})", system_specs.os_name, system_specs.os_arch)).strong().color(egui::Color32::from_rgb(230, 240, 255)));
                            ui.end_row();

                            ui.label("Processador & Cores:");
                            ui.label(egui::RichText::new(format!("{} Cores Logicos (Threads) | ~{} Cores Fisicos", system_specs.logical_cores, system_specs.physical_cores_estimate)).strong());
                            ui.end_row();

                            ui.label("Suporte SIMD / Vetorizacao:");
                            let simd_str = if system_specs.simd_neon {
                                "ARM NEON (Ativo)"
                            } else if system_specs.simd_avx2 {
                                "x86_64 AVX2 (Ativo)"
                            } else {
                                "SSE/Standard (Compativel)"
                            };
                            ui.label(egui::RichText::new(simd_str).color(egui::Color32::from_rgb(80, 230, 160)));
                            ui.end_row();

                            ui.label("Motor Grafico / Renderizador:");
                            ui.label(egui::RichText::new(&system_specs.renderer_info).color(egui::Color32::from_rgb(255, 215, 100)));
                            ui.end_row();

                            ui.label("Microsoft .NET SDK:");
                            if system_specs.dotnet_installed {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(format!("Instalado (v{})", system_specs.dotnet_version)).color(egui::Color32::from_rgb(80, 240, 140)).strong());
                                    if !system_specs.dotnet_path.is_empty() {
                                        ui.label(egui::RichText::new(format!("em {}", system_specs.dotnet_path)).size(11.0).color(egui::Color32::from_rgb(160, 180, 200)));
                                    }
                                });
                            } else {
                                ui.label(egui::RichText::new("Nao detectado no PATH (Clique em 'Baixar .NET' na aba de Documentacao)").color(egui::Color32::from_rgb(255, 100, 100)).strong());
                            }
                            ui.end_row();
                        });
                });

            ui.add_space(14.0);

            egui::Frame::group(ui.style())
                .fill(egui::Color32::from_rgb(20, 24, 32))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 55, 75)))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Aceleracao de Hardware, GPU & Multi-Threading").strong().size(15.0).color(egui::Color32::from_rgb(130, 220, 180)));
                        help_marker(ui, "Controle de desempenho grafico por GPU, multi-threading e taxa de atualizacao.");
                    });

                    ui.add_space(6.0);

                    ui.checkbox(&mut settings.hardware_acceleration, "Ativar Aceleracao de Hardware GPU");
                    help_marker(ui, "Utiliza aceleracao por GPU via WGPU / Metal / Vulkan / DirectX para renderizacao de alta fluidez.");

                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label("Backend de Renderizacao:");
                        egui::ComboBox::from_id_salt("render_backend_combo")
                            .selected_text(format!("{}", settings.render_backend))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut settings.render_backend, RenderBackend::Auto, format!("{}", RenderBackend::Auto));
                                ui.selectable_value(&mut settings.render_backend, RenderBackend::Metal, format!("{}", RenderBackend::Metal));
                                ui.selectable_value(&mut settings.render_backend, RenderBackend::Vulkan, format!("{}", RenderBackend::Vulkan));
                                ui.selectable_value(&mut settings.render_backend, RenderBackend::DirectX12, format!("{}", RenderBackend::DirectX12));
                                ui.selectable_value(&mut settings.render_backend, RenderBackend::OpenGL, format!("{}", RenderBackend::OpenGL));
                                ui.selectable_value(&mut settings.render_backend, RenderBackend::Software, format!("{}", RenderBackend::Software));
                            });
                        help_marker(ui, "Selecione a API grafica preferida ou mantenha Automatico para melhor compatibilidade com seu SO.");
                    });

                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label("Limite de Taxa de Quadros (FPS):");
                        let fps_text = if settings.max_fps == 0 { "Ilimitado".to_string() } else { format!("{} FPS", settings.max_fps) };
                        egui::ComboBox::from_id_salt("max_fps_combo")
                            .selected_text(fps_text)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut settings.max_fps, 30, "30 FPS (Economia de Energia)");
                                ui.selectable_value(&mut settings.max_fps, 60, "60 FPS (Padrao)");
                                ui.selectable_value(&mut settings.max_fps, 120, "120 FPS (Alta Fluidez / ProMotion)");
                                ui.selectable_value(&mut settings.max_fps, 144, "144 FPS (Monitores Gamer)");
                                ui.selectable_value(&mut settings.max_fps, 240, "240 FPS (Ultra Rapido)");
                                ui.selectable_value(&mut settings.max_fps, 0, "Ilimitado");
                            });

                        ui.add_space(15.0);
                        ui.checkbox(&mut settings.vsync, "Sincronizacao Vertical (V-Sync)");
                    });

                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label(format!("Threads de Processamento Paralelo: {} threads", settings.parallel_threads));
                        let max_threads = (system_specs.logical_cores * 2).max(16);
                        ui.add(egui::Slider::new(&mut settings.parallel_threads, 1..=max_threads));
                        help_marker(ui, "Controle de threads concorrentes para o compilador Roslyn e simulador fonetico em lote.");
                    });

                    ui.add_space(4.0);
                    ui.checkbox(&mut settings.enable_simd_phonetics, "Ativar Vetorizacao SIMD (AVX2 / ARM NEON) para busca fonetica ultra-rapida");

                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut settings.font_subpixel_antialiasing, "Anti-aliasing Subpixel e Fontes Nitidas");
                        ui.add_space(20.0);
                        ui.label(format!("Escala da Interface: {:.1}x", settings.ui_scale));
                        ui.add(egui::Slider::new(&mut settings.ui_scale, 0.8..=1.5).step_by(0.05));
                        if ui.button("Redefinir (1.0x)").clicked() {
                            settings.ui_scale = 1.0;
                        }
                    });
                });

            ui.add_space(14.0);

            egui::Frame::group(ui.style())
                .fill(egui::Color32::from_rgb(20, 24, 32))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 55, 75)))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Especificacoes de Compilacao C# (.NET & Roslyn)").strong().size(15.0).color(egui::Color32::from_rgb(255, 185, 100)));
                        help_marker(ui, "Configuracoes tecnicas de geracao do arquivo .csproj e compilação do plugin .dll.");
                    });

                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.label("Target Framework (.NET):");
                        egui::ComboBox::from_id_salt("dotnet_target_combo")
                            .selected_text(format!("{}", settings.dotnet_target_framework))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut settings.dotnet_target_framework, DotnetTarget::Net8_0, format!("{}", DotnetTarget::Net8_0));
                                ui.selectable_value(&mut settings.dotnet_target_framework, DotnetTarget::Net7_0, format!("{}", DotnetTarget::Net7_0));
                                ui.selectable_value(&mut settings.dotnet_target_framework, DotnetTarget::Net9_0, format!("{}", DotnetTarget::Net9_0));
                                ui.selectable_value(&mut settings.dotnet_target_framework, DotnetTarget::NetStandard2_1, format!("{}", DotnetTarget::NetStandard2_1));
                            });
                        help_marker(ui, "Versao do runtime do .NET compilada na tag <TargetFramework> do arquivo .csproj.");
                    });

                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label("Nivel de Otimizacao Roslyn:");
                        egui::ComboBox::from_id_salt("roslyn_opt_combo")
                            .selected_text(format!("{}", settings.roslyn_optimization))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut settings.roslyn_optimization, RoslynOptimizationLevel::Release, format!("{}", RoslynOptimizationLevel::Release));
                                ui.selectable_value(&mut settings.roslyn_optimization, RoslynOptimizationLevel::Debug, format!("{}", RoslynOptimizationLevel::Debug));
                            });
                    });

                    ui.add_space(6.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.checkbox(&mut settings.ready_to_run_aot, "Pre-compilacao Ahead-of-Time (PublishReadyToRun)");
                        help_marker(ui, "Compila codigo de maquina nativo com antecedencia para reducao de latencia em tempo real.");

                        ui.add_space(15.0);
                        ui.checkbox(&mut settings.nullable_context, "Contexto de Tipos Nulos (Nullable Context)");

                        ui.add_space(15.0);
                        ui.checkbox(&mut settings.allow_unsafe_code, "Permitir Codigo Inseguro (AllowUnsafeBlocks)");

                        ui.add_space(15.0);
                        ui.checkbox(&mut settings.deterministic_build, "Compilacao Deterministica (Deterministic Build)");
                    });
                });

            ui.add_space(14.0);

            egui::Frame::group(ui.style())
                .fill(egui::Color32::from_rgb(20, 24, 32))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 55, 75)))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Caminhos de Ambiente, Plugins & Integracoes").strong().size(15.0).color(egui::Color32::from_rgb(180, 160, 255)));
                        help_marker(ui, "Configure onde os plugins compilados sao instalados e o executavel do .NET SDK.");
                    });

                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.label("Caminho Customizado do .NET SDK (dotnet):");
                        let edit_width = (ui.available_width() - 190.0).max(180.0);
                        ui.add(egui::TextEdit::singleline(&mut settings.custom_dotnet_path).desired_width(edit_width).hint_text("Deixe vazio para usar o 'dotnet' do PATH"));
                        if ui.button("Procurar...").clicked() {
                            if let Some(file) = rfd::FileDialog::new().pick_file() {
                                settings.custom_dotnet_path = file.to_string_lossy().to_string();
                            }
                        }
                        if ui.button("Limpar").clicked() {
                            settings.custom_dotnet_path.clear();
                        }
                    });

                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.label("Pasta de Plugins do OpenUtau:");
                        let edit_width = (ui.available_width() - 240.0).max(180.0);
                        ui.add(egui::TextEdit::singleline(&mut settings.openutau_plugins_dir).desired_width(edit_width));
                        if ui.button("Escolher Pasta...").clicked() {
                            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                settings.openutau_plugins_dir = folder.to_string_lossy().to_string();
                            }
                        }
                        if ui.button("Auto-detectar").clicked() {
                            settings.openutau_plugins_dir = crate::models::default_openutau_plugins_path();
                        }
                    });
                });

            ui.add_space(14.0);

            egui::Frame::group(ui.style())
                .fill(egui::Color32::from_rgb(20, 24, 32))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 55, 75)))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Auto-Salvamento & Preferencias Gerais").strong().size(15.0).color(egui::Color32::from_rgb(160, 210, 255)));
                        help_marker(ui, "Preferencias de persistencia e nivel de verbosidade.");
                    });

                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.checkbox(&mut settings.auto_save_enabled, "Salvar projeto automaticamente a cada:");
                        ui.add(egui::Slider::new(&mut settings.auto_save_interval_minutes, 1..=30).suffix(" min"));
                    });

                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label("Nivel de Verbosidade de Logs:");
                        egui::ComboBox::from_id_salt("log_verbosity_combo")
                            .selected_text(format!("{}", settings.log_verbosity))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut settings.log_verbosity, LogVerbosity::Verbose, format!("{}", LogVerbosity::Verbose));
                                ui.selectable_value(&mut settings.log_verbosity, LogVerbosity::Normal, format!("{}", LogVerbosity::Normal));
                                ui.selectable_value(&mut settings.log_verbosity, LogVerbosity::ErrorsOnly, format!("{}", LogVerbosity::ErrorsOnly));
                            });
                    });
                });

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                if ui.button(egui::RichText::new("Salvar Configuracoes").strong().size(14.0)).clicked() {
                    match settings.save() {
                        Ok(_) => {
                            state.status_feedback = Some("Todas as configuracoes e especificacoes foram salvas no disco com sucesso!".to_string());
                        }
                        Err(e) => {
                            state.status_feedback = Some(format!("Erro ao salvar configuracoes: {}", e));
                        }
                    }
                }

                if ui.button("Redefinir para Padroes Recomendados").clicked() {
                    *settings = AppSettings::default();
                    state.status_feedback = Some("Configuracoes redefinidas para os padroes de fabrica.".to_string());
                }

                if ui.button("Exportar Configuracoes (.json)...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("JSON Settings", &["json"]).set_file_name("phonemizer_studio_settings.json").save_file() {
                        if let Ok(json) = serde_json::to_string_pretty(settings) {
                            let _ = std::fs::write(path, json);
                            state.status_feedback = Some("Configuracoes exportadas com sucesso!".to_string());
                        }
                    }
                }

                if ui.button("Importar Configuracoes (.json)...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("JSON Settings", &["json"]).pick_file() {
                        if let Ok(data) = std::fs::read_to_string(path) {
                            if let Ok(imported) = serde_json::from_str::<AppSettings>(&data) {
                                *settings = imported;
                                state.status_feedback = Some("Configuracoes importadas com sucesso!".to_string());
                            }
                        }
                    }
                }
            });

            ui.add_space(20.0);
        });
}
