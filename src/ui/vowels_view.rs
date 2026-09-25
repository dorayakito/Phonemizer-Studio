use crate::models::{PhonemizerProject, Vowel};
use crate::ui::helpers::{help_marker, section_header};
use egui::Ui;

#[derive(Default)]
pub struct VowelsViewState {
    pub search_filter: String,
    pub batch_import_text: String,
    pub show_batch_modal: bool,
}

pub fn render_vowels_view(ui: &mut Ui, project: &mut PhonemizerProject, state: &mut VowelsViewState) {
    section_header(
        ui,
        "Editor Avancado de Vogais, Ditongos, Tritongos & IPA",
        Some("Configure o inventario vocalico, simbolos IPA, aliases alternativos, nasalidade, ditongos e finalizacoes."),
    );

    ui.horizontal(|ui| {
        if ui.button("Adicionar Vogal").clicked() {
            project.vowels.push(Vowel::new("v"));
        }

        if ui.button("Importacao em Lote...").clicked() {
            state.show_batch_modal = !state.show_batch_modal;
        }

        ui.label("Filtrar:");
        ui.text_edit_singleline(&mut state.search_filter);

        ui.label(format!("Total: {}", project.vowels.len()));
        help_marker(ui, "Filtre vogais por simbolo ou alias em tempo real.");
    });

    if state.show_batch_modal {
        ui.group(|ui| {
            ui.label("Digite os simbolos das vogais separados por espaco ou virgula (ex: a e i o u an en in on un):");
            ui.text_edit_singleline(&mut state.batch_import_text);
            ui.horizontal(|ui| {
                if ui.button("Adicionar Todos").clicked() {
                    let tokens: Vec<&str> = state.batch_import_text
                        .split(|c: char| c.is_whitespace() || c == ',' || c == ';')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect();

                    for tok in tokens {
                        if !project.vowels.iter().any(|v| v.symbol == tok) {
                            project.vowels.push(Vowel::new(tok));
                        }
                    }
                    state.batch_import_text.clear();
                    state.show_batch_modal = false;
                }
                if ui.button("Cancelar").clicked() {
                    state.show_batch_modal = false;
                }
            });
        });
    }

    ui.add_space(8.0);

    let mut remove_idx = None;
    let query = state.search_filter.to_lowercase();

    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("vowels_advanced_table")
            .striped(true)
            .min_col_width(70.0)
            .spacing([10.0, 8.0])
            .show(ui, |ui| {
                ui.horizontal(|ui| { ui.strong("Simbolo"); help_marker(ui, "Simbolo principal da vogal utilizado nas regras."); });
                ui.horizontal(|ui| { ui.strong("IPA"); help_marker(ui, "Representacao no Alfabeto Fonetico Internacional."); });
                ui.horizontal(|ui| { ui.strong("Aliases (virgula)"); help_marker(ui, "Nomes alternativos aceitos como a mesma vogal."); });
                ui.horizontal(|ui| { ui.strong("Nasal"); help_marker(ui, "Marca se e uma vogal nasal (ex: an, en, ã)."); });
                ui.horizontal(|ui| { ui.strong("Ditongo"); help_marker(ui, "Marca se e um ditongo composto por duas vogais."); });
                ui.horizontal(|ui| { ui.strong("2a Vogal"); help_marker(ui, "Segunda vogal do ditongo (ex: 'i' em 'ai')."); });
                ui.horizontal(|ui| { ui.strong("Divisao %"); help_marker(ui, "Proporcao de tempo entre a primeira e segunda vogal do ditongo."); });
                ui.horizontal(|ui| { ui.strong("Finalizacao"); help_marker(ui, "Alias de glide de finalizacao (ex: 'a -', 'a R')."); });
                ui.horizontal(|ui| { ui.strong("Ataque Glotal"); help_marker(ui, "Alias de ataque glotal (ex: '? a', '_a')."); });
                ui.strong("Acoes");
                ui.end_row();

                for (idx, vowel) in project.vowels.iter_mut().enumerate() {
                    if !query.is_empty() && !vowel.symbol.to_lowercase().contains(&query) && !vowel.aliases.iter().any(|a| a.to_lowercase().contains(&query)) {
                        continue;
                    }

                    ui.text_edit_singleline(&mut vowel.symbol);
                    ui.text_edit_singleline(&mut vowel.ipa);

                    let mut aliases_str = vowel.aliases.join(", ");
                    if ui.text_edit_singleline(&mut aliases_str).changed() {
                        vowel.aliases = aliases_str
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }

                    ui.checkbox(&mut vowel.is_nasal, "");
                    ui.checkbox(&mut vowel.is_diphthong, "");

                    if vowel.is_diphthong {
                        let mut sec = vowel.second_vowel.clone().unwrap_or_default();
                        if ui.text_edit_singleline(&mut sec).changed() {
                            vowel.second_vowel = if sec.trim().is_empty() { None } else { Some(sec.trim().to_string()) };
                        }
                        ui.add(egui::Slider::new(&mut vowel.diphthong_split_ratio, 0.1..=0.9).text("%"));
                    } else {
                        ui.label("-");
                        ui.label("-");
                    }

                    let mut glide = vowel.end_glide.clone().unwrap_or_default();
                    if ui.text_edit_singleline(&mut glide).changed() {
                        vowel.end_glide = if glide.trim().is_empty() { None } else { Some(glide.trim().to_string()) };
                    }

                    let mut glottal = vowel.glottal_start_alias.clone().unwrap_or_default();
                    if ui.text_edit_singleline(&mut glottal).changed() {
                        vowel.glottal_start_alias = if glottal.trim().is_empty() { None } else { Some(glottal.trim().to_string()) };
                    }

                    if ui.button("Excluir").clicked() {
                        remove_idx = Some(idx);
                    }
                    ui.end_row();
                }
            });
    });

    if let Some(idx) = remove_idx {
        if idx < project.vowels.len() {
            project.vowels.remove(idx);
        }
    }
}
