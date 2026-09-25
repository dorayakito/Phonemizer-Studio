use crate::models::{Consonant, ConsonantType, PhonemizerProject, PlaceOfArticulation};
use crate::ui::helpers::{help_marker, section_header};
use egui::Ui;

#[derive(Default)]
pub struct ConsonantsViewState {
    pub search_filter: String,
    pub batch_import_text: String,
    pub show_batch_modal: bool,
}

pub fn render_consonants_view(ui: &mut Ui, project: &mut PhonemizerProject, state: &mut ConsonantsViewState) {
    section_header(
        ui,
        "Editor Avancado de Consoantes & Fonetica Articulatoria",
        Some("Configure o inventario consonantal, modo e ponto de articulacao, sonoridade, aspiracao, onset/coda e multiplicadores de timing."),
    );

    ui.horizontal(|ui| {
        if ui.button("Adicionar Consoante").clicked() {
            project.consonants.push(Consonant::new("c", ConsonantType::Stop));
        }

        if ui.button("Importacao em Lote...").clicked() {
            state.show_batch_modal = !state.show_batch_modal;
        }

        ui.label("Filtrar:");
        ui.text_edit_singleline(&mut state.search_filter);

        ui.label(format!("Total: {}", project.consonants.len()));
        help_marker(ui, "Filtre consoantes por simbolo ou alias em tempo real.");
    });

    if state.show_batch_modal {
        ui.group(|ui| {
            ui.label("Digite os simbolos das consoantes separados por espaco ou virgula (ex: k s t n m p l r f v z):");
            ui.text_edit_singleline(&mut state.batch_import_text);
            ui.horizontal(|ui| {
                if ui.button("Adicionar Todos").clicked() {
                    let tokens: Vec<&str> = state.batch_import_text
                        .split(|c: char| c.is_whitespace() || c == ',' || c == ';')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect();

                    for tok in tokens {
                        if !project.consonants.iter().any(|c| c.symbol == tok) {
                            project.consonants.push(Consonant::new(tok, ConsonantType::Stop));
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
        egui::Grid::new("consonants_advanced_table")
            .striped(true)
            .min_col_width(65.0)
            .spacing([10.0, 8.0])
            .show(ui, |ui| {
                ui.horizontal(|ui| { ui.strong("Simbolo"); help_marker(ui, "Simbolo principal da consoante."); });
                ui.horizontal(|ui| { ui.strong("IPA"); help_marker(ui, "Simbolo correspondente no Alfabeto Fonetico Internacional."); });
                ui.horizontal(|ui| { ui.strong("Modo"); help_marker(ui, "Modo de articulacao (Oclusiva, Fricativa, Nasal, Liquida, etc.)."); });
                ui.horizontal(|ui| { ui.strong("Ponto"); help_marker(ui, "Ponto de articulacao anatomico (Bilabial, Alveolar, Velar, etc.)."); });
                ui.horizontal(|ui| { ui.strong("Sonora"); help_marker(ui, "Vibracao das pregas vocais (Sonora vs Surda)."); });
                ui.horizontal(|ui| { ui.strong("Aspirada"); help_marker(ui, "Consoante aspirada com sopro audivel (ex: kh, th, ph)."); });
                ui.horizontal(|ui| { ui.strong("Onset"); help_marker(ui, "Permitir que a consoante atue como ataque inicial de silaba."); });
                ui.horizontal(|ui| { ui.strong("Coda"); help_marker(ui, "Permitir que a consoante atue como finalizacao de silaba."); });
                ui.horizontal(|ui| { ui.strong("Tempo"); help_marker(ui, "Multiplicador relativo da duracao da consoante."); });
                ui.horizontal(|ui| { ui.strong("Pre-ut."); help_marker(ui, "Multiplicador de pre-emissao (antecipacao de transicao)."); });
                ui.strong("Acoes");
                ui.end_row();

                for (idx, consonant) in project.consonants.iter_mut().enumerate() {
                    if !query.is_empty() && !consonant.symbol.to_lowercase().contains(&query) && !consonant.aliases.iter().any(|a| a.to_lowercase().contains(&query)) {
                        continue;
                    }

                    ui.text_edit_singleline(&mut consonant.symbol);
                    ui.text_edit_singleline(&mut consonant.ipa);

                    egui::ComboBox::from_id_salt(format!("adv_ctype_combo_{}", idx))
                        .selected_text(format!("{}", consonant.consonant_type))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut consonant.consonant_type, ConsonantType::Stop, "Stop (Oclusiva)");
                            ui.selectable_value(&mut consonant.consonant_type, ConsonantType::Fricative, "Fricative (Fricativa)");
                            ui.selectable_value(&mut consonant.consonant_type, ConsonantType::Nasal, "Nasal (Nasal)");
                            ui.selectable_value(&mut consonant.consonant_type, ConsonantType::Liquid, "Liquid (Liquida/Vibrante)");
                            ui.selectable_value(&mut consonant.consonant_type, ConsonantType::Affricate, "Affricate (Africada)");
                            ui.selectable_value(&mut consonant.consonant_type, ConsonantType::Semivowel, "Semivowel (Semivogal)");
                            ui.selectable_value(&mut consonant.consonant_type, ConsonantType::Glottal, "Glottal (Glotal)");
                            ui.selectable_value(&mut consonant.consonant_type, ConsonantType::ClickOrEjective, "Click / Ejetiva");
                            ui.selectable_value(&mut consonant.consonant_type, ConsonantType::Special, "Special (Especial)");
                        });

                    egui::ComboBox::from_id_salt(format!("adv_cplace_combo_{}", idx))
                        .selected_text(format!("{}", consonant.place_of_articulation))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut consonant.place_of_articulation, PlaceOfArticulation::Bilabial, "Bilabial");
                            ui.selectable_value(&mut consonant.place_of_articulation, PlaceOfArticulation::Labiodental, "Labiodental");
                            ui.selectable_value(&mut consonant.place_of_articulation, PlaceOfArticulation::Dental, "Dental");
                            ui.selectable_value(&mut consonant.place_of_articulation, PlaceOfArticulation::Alveolar, "Alveolar");
                            ui.selectable_value(&mut consonant.place_of_articulation, PlaceOfArticulation::Postalveolar, "Postalveolar");
                            ui.selectable_value(&mut consonant.place_of_articulation, PlaceOfArticulation::Retroflex, "Retroflexo");
                            ui.selectable_value(&mut consonant.place_of_articulation, PlaceOfArticulation::Palatal, "Palatal");
                            ui.selectable_value(&mut consonant.place_of_articulation, PlaceOfArticulation::Velar, "Velar");
                            ui.selectable_value(&mut consonant.place_of_articulation, PlaceOfArticulation::Uvular, "Uvular");
                            ui.selectable_value(&mut consonant.place_of_articulation, PlaceOfArticulation::Pharyngeal, "Faringeo");
                            ui.selectable_value(&mut consonant.place_of_articulation, PlaceOfArticulation::Glottal, "Glotal");
                        });

                    ui.checkbox(&mut consonant.is_voiced, "");
                    ui.checkbox(&mut consonant.is_aspirated, "");
                    ui.checkbox(&mut consonant.can_be_onset, "");
                    ui.checkbox(&mut consonant.can_be_coda, "");

                    ui.add(egui::Slider::new(&mut consonant.timing_ratio, 0.5..=2.5).text("x"));
                    ui.add(egui::Slider::new(&mut consonant.pre_utterance_multiplier, 0.5..=2.0).text("x"));

                    if ui.button("Excluir").clicked() {
                        remove_idx = Some(idx);
                    }
                    ui.end_row();
                }
            });
    });

    if let Some(idx) = remove_idx {
        if idx < project.consonants.len() {
            project.consonants.remove(idx);
        }
    }
}
