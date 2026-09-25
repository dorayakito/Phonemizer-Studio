use crate::importer::{get_curated_openutau_phonemizers, parse_csharp_to_project, GitHubClient, GitHubFileItem};
use crate::models::PhonemizerProject;
use crate::ui::helpers::{help_marker, section_header};
use egui::Ui;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct GitHubImportState {
    pub repo_owner: String,
    pub repo_name: String,
    pub repo_path: String,
    pub filter_query: String,
    pub custom_url: String,
    pub items: Vec<GitHubFileItem>,
    pub is_loading: Arc<AtomicBool>,
    pub status_message: String,
    pub preview_code: Option<String>,
    pub downloaded_item_receiver: Option<mpsc::UnboundedReceiver<(String, String)>>,
    pub item_list_receiver: Option<mpsc::UnboundedReceiver<Result<Vec<GitHubFileItem>, String>>>,
}

impl Default for GitHubImportState {
    fn default() -> Self {
        Self {
            repo_owner: "openutau".to_string(),
            repo_name: "OpenUtau".to_string(),
            repo_path: "OpenUtau.Plugin.Builtin".to_string(),
            filter_query: String::new(),
            custom_url: String::new(),
            items: get_curated_openutau_phonemizers(),
            is_loading: Arc::new(AtomicBool::new(false)),
            status_message: "Pronto. Lista de phonemizers oficiais carregada.".to_string(),
            preview_code: None,
            downloaded_item_receiver: None,
            item_list_receiver: None,
        }
    }
}

pub fn render_github_import_view(
    ui: &mut Ui,
    project: &mut PhonemizerProject,
    state: &mut GitHubImportState,
) {
    if let Some(ref mut rx) = state.item_list_receiver {
        if let Ok(res) = rx.try_recv() {
            state.is_loading.store(false, Ordering::Relaxed);
            match res {
                Ok(items) => {
                    state.status_message = format!("Sucesso: {} phonemizers encontrados no repositorio.", items.len());
                    state.items = items;
                }
                Err(err) => {
                    state.status_message = format!("Aviso: {}, usando lista pré-carregada.", err);
                    state.items = get_curated_openutau_phonemizers();
                }
            }
        }
    }

    if let Some(ref mut rx) = state.downloaded_item_receiver {
        if let Ok((file_name, code)) = rx.try_recv() {
            state.is_loading.store(false, Ordering::Relaxed);
            state.preview_code = Some(code.clone());

            let imported_project = parse_csharp_to_project(&code, Some(&file_name));
            *project = imported_project;
            state.status_message = format!("Phonemizer '{}' importado com sucesso! (Tag: {})", project.name, project.tag);
        }
    }

    section_header(
        ui,
        "Importar Phonemizers C# do OpenUtau (GitHub API & Hub)",
        Some("Puxe phonemizers oficiais ou da comunidade diretamente do repositorio OpenUtau ou qualquer repositorio GitHub via API."),
    );

    let is_busy = state.is_loading.load(Ordering::Relaxed);

    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label("Repositorio (Owner / Repo):");
            ui.text_edit_singleline(&mut state.repo_owner);
            ui.label("/");
            ui.text_edit_singleline(&mut state.repo_name);

            ui.label("Caminho:");
            ui.text_edit_singleline(&mut state.repo_path);
            help_marker(ui, "Coordenadas do repositorio GitHub contendo o codigo C# dos phonemizers.");

            if is_busy {
                ui.spinner();
                ui.label("Buscando no GitHub...");
            } else if ui.button("Atualizar Lista via GitHub API").clicked() {
                state.is_loading.store(true, Ordering::Relaxed);
                state.status_message = "Conectando a API do GitHub...".to_string();

                let (tx, rx) = mpsc::unbounded_channel();
                state.item_list_receiver = Some(rx);

                let owner = state.repo_owner.clone();
                let repo = state.repo_name.clone();
                let path = state.repo_path.clone();

                tokio::spawn(async move {
                    let client = GitHubClient::new();
                    let res = client.list_openutau_builtin_phonemizers(Some(&owner), Some(&repo), Some(&path)).await;
                    let _ = tx.send(res);
                });
            }
        });

        ui.add_space(6.0);

        ui.horizontal(|ui| {
            ui.label("Ou importar por URL Direta / Arquivo Local:");
            ui.text_edit_singleline(&mut state.custom_url);
            help_marker(ui, "Insira um link direto de arquivo raw do GitHub (ex: raw.githubusercontent.com/...).");

            if ui.button("Baixar URL").clicked() && !state.custom_url.is_empty() {
                state.is_loading.store(true, Ordering::Relaxed);
                let (tx, rx) = mpsc::unbounded_channel();
                state.downloaded_item_receiver = Some(rx);
                let url = state.custom_url.clone();

                tokio::spawn(async move {
                    let client = GitHubClient::new();
                    if let Ok(code) = client.fetch_raw_csharp_file(&url).await {
                        let _ = tx.send(("CustomUrlPhonemizer.cs".to_string(), code));
                    }
                });
            }

            if ui.button("Abrir Arquivo .cs Local...").clicked() {
                if let Some(file_path) = rfd::FileDialog::new().add_filter("C# Source", &["cs"]).pick_file() {
                    if let Ok(code) = std::fs::read_to_string(&file_path) {
                        let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("LocalPhonemizer.cs");
                        let imported = parse_csharp_to_project(&code, Some(file_name));
                        *project = imported;
                        state.preview_code = Some(code);
                        state.status_message = format!("Arquivo local '{}' importado!", file_name);
                    }
                }
            }
        });
    });

    ui.add_space(10.0);

    ui.horizontal(|ui| {
        ui.label("Filtrar Phonemizers:");
        ui.text_edit_singleline(&mut state.filter_query);
        ui.label(format!("(Total: {})", state.items.len()));
        help_marker(ui, "Filtre phonemizers disponiveis por nome ou idioma.");
    });

    ui.add_space(5.0);

    ui.columns(2, |columns| {

        columns[0].vertical(|ui| {
            ui.label(egui::RichText::new("Phonemizers Disponiveis no Repositorio").strong().size(15.0));

            let q = state.filter_query.to_lowercase();
            let filtered_items: Vec<&GitHubFileItem> = state.items.iter().filter(|i| {
                if q.is_empty() {
                    true
                } else {
                    i.name.to_lowercase().contains(&q)
                }
            }).collect();

            egui::ScrollArea::vertical().id_salt("github_items_list").show(ui, |ui| {
                for item in filtered_items {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(&item.name).strong().color(egui::Color32::from_rgb(100, 200, 255)));
                            if let Some(ref html_url) = item.html_url {
                                if ui.small_button("Abrir no GitHub").clicked() {
                                    let _ = open::that(html_url);
                                }
                            }
                        });

                        ui.horizontal(|ui| {
                            if ui.button("Importar e Converter").clicked() {
                                if let Some(ref dl_url) = item.download_url {
                                    state.is_loading.store(true, Ordering::Relaxed);
                                    let (tx, rx) = mpsc::unbounded_channel();
                                    state.downloaded_item_receiver = Some(rx);
                                    let dl = dl_url.clone();
                                    let fname = item.name.clone();

                                    tokio::spawn(async move {
                                        let client = GitHubClient::new();
                                        if let Ok(code) = client.fetch_raw_csharp_file(&dl).await {
                                            let _ = tx.send((fname, code));
                                        }
                                    });
                                }
                            }
                            help_marker(ui, "Baixa e converte automaticamente a classe C# para o formato do projeto editavel.");

                            if ui.button("Modo C# Puro").clicked() {
                                if let Some(ref dl_url) = item.download_url {
                                    state.is_loading.store(true, Ordering::Relaxed);
                                    let (tx, rx) = mpsc::unbounded_channel();
                                    state.downloaded_item_receiver = Some(rx);
                                    let dl = dl_url.clone();
                                    let fname = item.name.clone();

                                    tokio::spawn(async move {
                                        let client = GitHubClient::new();
                                        if let Ok(code) = client.fetch_raw_csharp_file(&dl).await {
                                            let _ = tx.send((fname, code));
                                        }
                                    });
                                }
                            }
                            help_marker(ui, "Importa o codigo C# bruto para edicao livre e compilacao direta.");
                        });
                    });
                    ui.add_space(4.0);
                }
            });
        });

        columns[1].vertical(|ui| {
            ui.label(egui::RichText::new("Preview do Codigo Fonte Importado").strong().size(15.0));

            if let Some(ref code) = state.preview_code {
                ui.label(format!("Linhas baixadas: {}", code.lines().count()));
                egui::ScrollArea::vertical().id_salt("preview_code_scroll").show(ui, |ui| {
                    let mut display_code = code.clone();
                    ui.add(
                        egui::TextEdit::multiline(&mut display_code)
                            .code_editor()
                            .desired_width(ui.available_width().max(300.0))
                            .desired_rows(25)
                            .interactive(false),
                    );
                });
            } else {
                ui.label("Selecione um phonemizer na lista a esquerda e clique em 'Importar e Converter' para visualizar o codigo fonte.");
            }
        });
    });
}
