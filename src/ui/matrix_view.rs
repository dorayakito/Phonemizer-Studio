use crate::models::{PatternType, PhoneticPatternRule, PhonemizerProject};
use crate::ui::helpers::{help_marker, section_header};
use egui::Ui;

pub fn render_matrix_view(ui: &mut Ui, project: &mut PhonemizerProject) {
    section_header(
        ui,
        "Matriz Fonetica, Transicoes, Voice Colors & Fonotatica",
        Some("Configure as regras de geracao de aliases (CV, VCV, VC, VV, CC, CVC), Voice Colors, Pitch limits e limites de silaba."),
    );

    ui.collapsing("Voice Colors, Flags & Pitch Suffixes", |ui| {
        ui.horizontal(|ui| {
            ui.checkbox(&mut project.voice_color_and_pitch.use_pitch_suffixes, "Habilitar sufixos de tom (_C4, _G4, etc.)");
            help_marker(ui, "Verifica se o voicebank contem sufixos de tom por oitava no oto.ini e faz fallback automatico.");
        });

        ui.horizontal(|ui| {
            ui.checkbox(&mut project.voice_color_and_pitch.fallback_to_default_color, "Fallback automatico caso Voice Color nao exista no oto");
            help_marker(ui, "Se o cantor nao possuir a amostra na cor solicitada (ex: Power), utiliza a amostra padrao.");
        });

        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.label("Separador de Cor/Prefixo:");
            ui.text_edit_singleline(&mut project.voice_color_and_pitch.color_separator);
            help_marker(ui, "Caractere separador entre o alias e a cor de voz (padrao: '_').");

            ui.label("Flags Customizadas do Resampler:");
            ui.text_edit_singleline(&mut project.voice_color_and_pitch.custom_resampler_flags);
            help_marker(ui, "Flags adicionais passadas para o motor de sintese/resampler (ex: 'g+5 B50').");
        });

        ui.add_space(5.0);
        ui.horizontal(|ui| {
            ui.label("Voice Colors cadastradas (separadas por virgula):");
            help_marker(ui, "Lista de cores de voz suportadas pelo phonemizer (ex: Power, Soft, Whisper, Falsetto).");
        });
        let mut colors_str = project.voice_color_and_pitch.voice_colors.join(", ");
        if ui.text_edit_singleline(&mut colors_str).changed() {
            project.voice_color_and_pitch.voice_colors = colors_str
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }

        ui.horizontal(|ui| {
            ui.label("Sufixos de Pitch suportados:");
            help_marker(ui, "Lista de sufixos de tom configurados no voicebank (ex: _C3, _F3, _A3, _C4, _G4).");
        });
        let mut pitches_str = project.voice_color_and_pitch.pitch_suffixes.join(", ");
        if ui.text_edit_singleline(&mut pitches_str).changed() {
            project.voice_color_and_pitch.pitch_suffixes = pitches_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }
    });

    ui.collapsing("Fonotatica & Fronteiras de Silaba (Liaison & Enchainement)", |ui| {
        ui.horizontal(|ui| {
            ui.checkbox(&mut project.syllabification_config.enable_cross_word_liaison, "Habilitar Liaison / Ligacao de Consoante Coda com Vogal da proxima palavra");
            help_marker(ui, "Liga a consoante final de uma palavra com a vogal inicial da palavra seguinte sem pausa.");
        });

        ui.horizontal(|ui| {
            ui.checkbox(&mut project.syllabification_config.auto_detect_diphthongs, "Auto-detectar ditongos em vogais consecutivas na mesma silaba");
            help_marker(ui, "Identifica automaticamente encontros vocalicos e aplica a regra de ditongo correspondente.");
        });

        ui.horizontal(|ui| {
            ui.checkbox(&mut project.syllabification_config.allow_glottal_stop_insertion, "Inserir Oclusiva Glotal (?) apos pausas antes de vogal inicial");
            help_marker(ui, "Adiciona ataque glotal suave em notas com inicio vocalico apos pausas ou respiracoes.");
        });

        ui.horizontal(|ui| {
            ui.label("Limite de Pausa para Inicio em Repouso (Ticks):");
            ui.add(egui::DragValue::new(&mut project.syllabification_config.word_boundary_rest_threshold_ticks).range(10..=480));
            help_marker(ui, "Intervalo minimo de ticks musicais de silencio para considerar o inicio como repouso (- V).");
        });
    });

    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Regras de Padroes Foneticos (Alias Templates)").heading().size(16.0));
        help_marker(ui, "Defina os templates de string para formatar cada tipo de alias pesquisado no oto.ini.");
    });

    ui.horizontal(|ui| {
        if ui.button("Adicionar Regra de Padrao").clicked() {
            project.pattern_rules.push(PhoneticPatternRule {
                pattern_type: PatternType::CV,
                template: "{consonant}{vowel}".to_string(),
                enabled: true,
                priority: 5,
                fallback_template: None,
                min_pitch_note: None,
                max_pitch_note: None,
                color_filter: None,
            });
        }
    });

    ui.add_space(10.0);

    let mut remove_idx = None;

    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("patterns_grid_adv")
            .striped(true)
            .min_col_width(85.0)
            .spacing([10.0, 8.0])
            .show(ui, |ui| {
                ui.strong("Ativo");
                ui.horizontal(|ui| { ui.strong("Tipo de Transicao"); help_marker(ui, "Contexto fonetico em que o padrao se aplica."); });
                ui.horizontal(|ui| { ui.strong("Template de Alias"); help_marker(ui, "Variaveis aceitas: {vowel}, {consonant}, {prev_vowel}, {prev_consonant}."); });
                ui.horizontal(|ui| { ui.strong("Prioridade"); help_marker(ui, "Regras de maior prioridade sao avaliadas primeiro."); });
                ui.horizontal(|ui| { ui.strong("Fallback"); help_marker(ui, "Template alternativo se o alias principal nao existir."); });
                ui.horizontal(|ui| { ui.strong("Filtro de Cor"); help_marker(ui, "Aplica este padrao apenas quando a nota tiver esta cor de voz."); });
                ui.strong("Acoes");
                ui.end_row();

                for (idx, rule) in project.pattern_rules.iter_mut().enumerate() {
                    ui.checkbox(&mut rule.enabled, "");

                    egui::ComboBox::from_id_salt(format!("adv_ptype_combo_{}", idx))
                        .selected_text(format!("{}", rule.pattern_type))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut rule.pattern_type, PatternType::RestingStartVowel, "Inicio c/ Vogal (- V)");
                            ui.selectable_value(&mut rule.pattern_type, PatternType::RestingStartConsonant, "Inicio c/ Consoante (- CV)");
                            ui.selectable_value(&mut rule.pattern_type, PatternType::CV, "Ataque Simples (CV)");
                            ui.selectable_value(&mut rule.pattern_type, PatternType::VCV, "Transicao VCV (V CV)");
                            ui.selectable_value(&mut rule.pattern_type, PatternType::VC, "Transicao Vogal-Consoante (VC)");
                            ui.selectable_value(&mut rule.pattern_type, PatternType::VV, "Transicao Vogal-Vogal (VV)");
                            ui.selectable_value(&mut rule.pattern_type, PatternType::CC, "Encontro Consonantal (CC)");
                            ui.selectable_value(&mut rule.pattern_type, PatternType::CVC, "Tri-fone Direto (CVC)");
                            ui.selectable_value(&mut rule.pattern_type, PatternType::EndingVowelGlide, "Finalizacao Vocalica (V -)");
                            ui.selectable_value(&mut rule.pattern_type, PatternType::EndingConsonantCoda, "Finalizacao Consonantal (C -)");
                            ui.selectable_value(&mut rule.pattern_type, PatternType::GlottalStop, "Oclusiva Glotal (' V)");
                            ui.selectable_value(&mut rule.pattern_type, PatternType::IntervocalicFlap, "Tepe Intervocalico (V r V)");
                            ui.selectable_value(&mut rule.pattern_type, PatternType::CrossWordLiaison, "Liaison (C_V)");
                            ui.selectable_value(&mut rule.pattern_type, PatternType::BreathAndPause, "Respiracao/Pausa (br)");
                            ui.selectable_value(&mut rule.pattern_type, PatternType::CustomPattern, "Customizado");
                        });

                    ui.text_edit_singleline(&mut rule.template);
                    ui.add(egui::DragValue::new(&mut rule.priority).range(1..=100));

                    let mut fb = rule.fallback_template.clone().unwrap_or_default();
                    if ui.text_edit_singleline(&mut fb).changed() {
                        rule.fallback_template = if fb.trim().is_empty() { None } else { Some(fb.trim().to_string()) };
                    }

                    let mut col_filt = rule.color_filter.clone().unwrap_or_default();
                    if ui.text_edit_singleline(&mut col_filt).changed() {
                        rule.color_filter = if col_filt.trim().is_empty() { None } else { Some(col_filt.trim().to_string()) };
                    }

                    if ui.button("Excluir").clicked() {
                        remove_idx = Some(idx);
                    }
                    ui.end_row();
                }
            });
    });

    if let Some(idx) = remove_idx {
        if idx < project.pattern_rules.len() {
            project.pattern_rules.remove(idx);
        }
    }
}
