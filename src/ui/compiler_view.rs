use crate::compiler::{check_dotnet_installed, compile_phonemizer_project, CompilationResult};
use crate::models::PhonemizerProject;
use crate::ui::helpers::{help_marker, section_header};
use egui::Ui;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct CompilerViewState {
    pub is_compiling: Arc<AtomicBool>,
    pub live_logs: Vec<String>,
    pub last_result: Option<CompilationResult>,
    pub log_receiver: Option<mpsc::UnboundedReceiver<String>>,
}

impl Default for CompilerViewState {
    fn default() -> Self {
        Self {
            is_compiling: Arc::new(AtomicBool::new(false)),
            live_logs: Vec::new(),
            last_result: None,
            log_receiver: None,
        }
    }
}

pub fn render_compiler_view(ui: &mut Ui, project: &mut PhonemizerProject, state: &mut CompilerViewState) {
    if let Some(ref mut rx) = state.log_receiver {
        while let Ok(log) = rx.try_recv() {
            state.live_logs.push(log);
        }
    }

    section_header(
        ui,
        "Compilacao de .DLL & Instalacao no OpenUtau",
        Some("Compile o phonemizer C# diretamente para uma biblioteca .DLL compativel com o OpenUtau e instale automaticamente."),
    );

    match check_dotnet_installed(None) {
        Ok(v) => {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(".NET SDK Detectado:").color(egui::Color32::from_rgb(80, 220, 80)).strong());
                ui.label(format!("v{}", v));
                help_marker(ui, "Ambiente Microsoft .NET SDK pronto para compilacao de DLLs.");
            });
        }
        Err(e) => {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(".NET SDK nao encontrado:").color(egui::Color32::from_rgb(240, 80, 80)).strong());
                ui.label(e);
                help_marker(ui, "Consulte a aba 'Documentacao & Downloads' para baixar o .NET SDK.");
            });
        }
    }

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label("Destino da DLL (Pasta Plugins do OpenUtau):");
            ui.text_edit_singleline(&mut project.openutau_plugins_dir);
            if ui.button("Selecionar Pasta...").clicked() {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    project.openutau_plugins_dir = folder.to_string_lossy().to_string();
                }
            }
            help_marker(ui, "Pasta onde a DLL compilada sera salva automaticamente.");
        });

        ui.horizontal(|ui| {
            ui.label("Nome da DLL de Saida:");
            ui.text_edit_singleline(&mut project.output_dll_name);
            help_marker(ui, "Nome do arquivo compilado (ex: MeuPhonemizer.dll).");
        });
    });

    ui.add_space(10.0);

    let is_running = state.is_compiling.load(Ordering::Relaxed);

    ui.horizontal(|ui| {
        if is_running {
            ui.spinner();
            ui.label(egui::RichText::new("Compilando projeto C# via dotnet build...").strong());
        } else if ui.button(egui::RichText::new("COMPILAR .DLL E INSTALAR").size(15.0).strong()).clicked() {
            state.live_logs.clear();
            state.last_result = None;
            state.is_compiling.store(true, Ordering::Relaxed);

            let (tx, rx) = mpsc::unbounded_channel();
            state.log_receiver = Some(rx);

            let project_clone = project.clone();
            let is_compiling_flag = state.is_compiling.clone();

            tokio::spawn(async move {
                let _res = compile_phonemizer_project(&project_clone, None, Some(tx)).await;
                is_compiling_flag.store(false, Ordering::Relaxed);
            });
        }

        if ui.button("Limpar Logs").clicked() {
            state.live_logs.clear();
        }

        if ui.button("Abrir Pasta de Plugins").clicked() {
            let _ = open::that(&project.openutau_plugins_dir);
        }

        if ui.button(egui::RichText::new(" Gerar Instalador .ouplugin").color(egui::Color32::from_rgb(120, 200, 255)).strong()).clicked() {
            let default_name = format!("{}.ouplugin", project.name);
            if let Some(save_path) = rfd::FileDialog::new().add_filter("OpenUtau Plugin", &["ouplugin"]).set_file_name(&default_name).save_file() {
                let dll_path = std::path::Path::new(&project.openutau_plugins_dir).join(&project.output_dll_name);
                match crate::compiler::OuPluginPackager::package_ouplugin(
                    &save_path,
                    &project.name,
                    &project.version,
                    &project.author,
                    &project.description,
                    &dll_path,
                ) {
                    Ok(p) => {
                        state.live_logs.push(format!("[SUCESSO] Pacote .ouplugin gerado com sucesso: {}", p));
                    }
                    Err(e) => {
                        state.live_logs.push(format!("[ERRO OUPLUGIN] {}", e));
                    }
                }
            }
        }
    });

    ui.add_space(15.0);
    section_header(
        ui,
        "Console de Compilacao & Logs",
        Some("Saida em tempo real do compilador dotnet build."),
    );

    let avail_width = ui.available_width().max(300.0);
    egui::ScrollArea::vertical()
        .id_salt("compiler_console_scroll")
        .stick_to_bottom(true)
        .max_height(220.0)
        .show(ui, |ui| {
            egui::Frame::canvas(ui.style())
                .fill(egui::Color32::from_rgb(15, 17, 22))
                .show(ui, |ui| {
                    ui.set_min_height(180.0);
                    ui.set_min_width(avail_width);

                    if state.live_logs.is_empty() {
                        ui.label("Nenhum log de compilacao no momento. Clique em 'COMPILAR .DLL' para iniciar.");
                    } else {
                        for log in &state.live_logs {
                            if log.contains("error") || log.contains("Error") || log.contains("FALHA") || log.contains("Failed") {
                                ui.label(egui::RichText::new(log).color(egui::Color32::from_rgb(255, 90, 90)));
                            } else if log.contains("SUCCEEDED") || log.contains("sucesso") || log.contains("restaurado") {
                                ui.label(egui::RichText::new(log).color(egui::Color32::from_rgb(90, 240, 130)).strong());
                            } else if log.contains("warning") || log.contains("Warning") || log.contains("Aviso") {
                                ui.label(egui::RichText::new(log).color(egui::Color32::from_rgb(255, 200, 60)));
                            } else {
                                ui.label(egui::RichText::new(log).color(egui::Color32::from_rgb(220, 225, 235)));
                            }
                        }
                    }
                });
        });
}
