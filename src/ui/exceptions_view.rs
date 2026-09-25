use crate::models::{DictionaryOverride, PhonemizerProject, RegexRule};
use crate::ui::helpers::{help_marker, section_header};
use egui::Ui;

#[derive(Default)]
pub struct ExceptionsViewState {
    pub search_filter: String,
    pub import_export_msg: String,
}

pub fn render_exceptions_view(ui: &mut Ui, project: &mut PhonemizerProject, state: &mut ExceptionsViewState) {
    section_header(
        ui,
        "Dicionario de Excecoes & Regras Regex",
        Some("Cadastre pronuncias personalizadas para palavras irregulares, importe dicionarios (CMUdict/TSV) e configure regras de transformacao via Regex."),
    );

    ui.horizontal(|ui| {
        ui.label("Filtrar:");
        ui.text_edit_singleline(&mut state.search_filter);
        help_marker(ui, "Filtre palavras ou regras regex em tempo real.");

        if ui.button("Importar Dicionario (CMUdict / TXT / TSV)...").clicked() {
            if let Some(file_path) = rfd::FileDialog::new().add_filter("Dictionary Files", &["dict", "txt", "tsv", "csv"]).pick_file() {
                if let Ok(content) = std::fs::read_to_string(&file_path) {
                    let mut imported_count = 0;
                    for line in content.lines() {
                        let line = line.trim();
                        if line.is_empty() || line.starts_with(";;;") || line.starts_with('#') {
                            continue;
                        }
                        let parts: Vec<&str> = if line.contains('\t') {
                            line.split('\t').collect()
                        } else {
                            line.split_whitespace().collect()
                        };

                        if parts.len() >= 2 {
                            let word = parts[0].trim().to_string();
                            let phonemes: Vec<String> = parts[1..].iter().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                            if !word.is_empty() && !phonemes.is_empty() {
                                if !project.dictionary_overrides.iter().any(|d| d.word.eq_ignore_ascii_case(&word)) {
                                    project.dictionary_overrides.push(DictionaryOverride::new(word, phonemes));
                                    imported_count += 1;
                                }
                            }
                        }
                    }
                    state.import_export_msg = format!("{} palavras importadas com sucesso!", imported_count);
                }
            }
        }

        if ui.button("Exportar Dicionario (TSV)...").clicked() {
            if let Some(save_path) = rfd::FileDialog::new().add_filter("TSV Dictionary", &["tsv"]).set_file_name("dictionary.tsv").save_file() {
                let mut out = String::new();
                for entry in &project.dictionary_overrides {
                    out.push_str(&format!("{}\t{}\n", entry.word, entry.phonemes.join(" ")));
                }
                if let Ok(_) = std::fs::write(save_path, out) {
                    state.import_export_msg = "Dicionario exportado com sucesso!".to_string();
                }
            }
        }
    });

    if !state.import_export_msg.is_empty() {
        ui.label(egui::RichText::new(&state.import_export_msg).color(egui::Color32::from_rgb(100, 230, 150)));
    }

    ui.add_space(6.0);

    let query = state.search_filter.to_lowercase();

    ui.columns(2, |columns| {

        columns[0].vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("Palavras Irregulares ({})", project.dictionary_overrides.len())).heading().size(15.0));
                help_marker(ui, "Mapeamento direto de palavras completas para uma sequencia explicita de fonemas.");
            });

            if ui.button("Adicionar Excecao de Palavra").clicked() {
                project.dictionary_overrides.push(DictionaryOverride::new("palavra", vec!["p".to_string(), "a".to_string()]));
            }
            ui.add_space(5.0);

            let mut remove_dict = None;
            egui::ScrollArea::vertical().id_salt("dict_scroll_adv").show(ui, |ui| {
                for (idx, entry) in project.dictionary_overrides.iter_mut().enumerate() {
                    if !query.is_empty() && !entry.word.to_lowercase().contains(&query) && !entry.phonemes.iter().any(|p| p.to_lowercase().contains(&query)) {
                        continue;
                    }

                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Palavra:");
                            ui.text_edit_singleline(&mut entry.word);
                            ui.checkbox(&mut entry.is_case_sensitive, "Case sensitive");
                            if ui.button("Excluir").clicked() {
                                remove_dict = Some(idx);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Fonemas:");
                            let mut phs = entry.phonemes.join(" ");
                            if ui.text_edit_singleline(&mut phs).changed() {
                                entry.phonemes = phs.split_whitespace().map(|s| s.to_string()).collect();
                            }
                            help_marker(ui, "Fonemas separados por espaco.");
                        });
                    });
                }
            });
            if let Some(idx) = remove_dict {
                if idx < project.dictionary_overrides.len() {
                    project.dictionary_overrides.remove(idx);
                }
            }
        });

        columns[1].vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("Regras Regex ({})", project.regex_rules.len())).heading().size(15.0));
                help_marker(ui, "Substituicoes ortograficas ou foneticas baseadas em expressoes regulares.");
            });

            if ui.button("Adicionar Regra Regex").clicked() {
                let order = project.regex_rules.len() as i32 + 1;
                project.regex_rules.push(RegexRule::new("Nova Regra", "padrao", "substituicao", order));
            }
            ui.add_space(5.0);

            let mut remove_regex = None;
            egui::ScrollArea::vertical().id_salt("regex_scroll_adv").show(ui, |ui| {
                for (idx, rule) in project.regex_rules.iter_mut().enumerate() {
                    if !query.is_empty() && !rule.name.to_lowercase().contains(&query) && !rule.pattern.to_lowercase().contains(&query) {
                        continue;
                    }

                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut rule.enabled, "");
                            ui.text_edit_singleline(&mut rule.name);
                            if ui.button("Excluir").clicked() {
                                remove_regex = Some(idx);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Padrao:");
                            ui.text_edit_singleline(&mut rule.pattern);
                            help_marker(ui, "Expressao regular (ex: 'qu([ei])').");
                        });
                        ui.horizontal(|ui| {
                            ui.label("Substituir por:");
                            ui.text_edit_singleline(&mut rule.replacement);
                            help_marker(ui, "Texto de substituicao (ex: 'k$1').");
                        });
                    });
                }
            });
            if let Some(idx) = remove_regex {
                if idx < project.regex_rules.len() {
                    project.regex_rules.remove(idx);
                }
            }
        });
    });
}
