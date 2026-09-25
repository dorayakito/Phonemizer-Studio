use crate::models::PhonemizerProject;
use crate::simulator::{AudioTonePlayer, CoverageReport, NoteSimulationResult, PhonemizerSimulator, VoicebankOto};
use crate::ui::helpers::section_header;
use egui::Ui;

pub struct SimulatorState {
    pub input_text: String,
    pub input_tone: String,
    pub bpm: f64,
    pub results: Vec<NoteSimulationResult>,
    pub audio_player: AudioTonePlayer,

    pub voicebank_oto: Option<VoicebankOto>,
    pub coverage_report: Option<CoverageReport>,
    pub voicebank_path_input: String,
}

impl Default for SimulatorState {
    fn default() -> Self {
        Self {
            input_text: "voce me deu amor no coracao".to_string(),
            input_tone: "C4".to_string(),
            bpm: 120.0,
            results: Vec::new(),
            audio_player: AudioTonePlayer::new(),
            voicebank_oto: None,
            coverage_report: None,
            voicebank_path_input: String::new(),
        }
    }
}

impl SimulatorState {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn render_simulator_view(ui: &mut Ui, project: &PhonemizerProject, state: &mut SimulatorState) {
    section_header(
        ui,
        "Simulador Fonetico, Audio & Verificador Oto.ini",
        Some("Decomposicao de notas musicais, reproducao de tons em tempo real e analise de cobertura de voicebanks."),
    );

    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Frase:").strong());
            let edit_width = (ui.available_width() - 320.0).max(180.0);
            let text_response = ui.add(egui::TextEdit::singleline(&mut state.input_text).desired_width(edit_width));
            
            ui.label("Tom:");
            ui.add(egui::TextEdit::singleline(&mut state.input_tone).desired_width(45.0));

            ui.label("BPM:");
            ui.add(egui::DragValue::new(&mut state.bpm).range(40.0..=300.0).speed(1.0));

            let run_clicked = ui.button(egui::RichText::new("Simular Frase").strong()).clicked();

            if run_clicked || (text_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) || state.results.is_empty() {
                let simulator = PhonemizerSimulator::new(project);
                state.results = simulator.simulate_phrase(&state.input_text);
                
                if let Some(ref vb) = state.voicebank_oto {
                    let mut aliases = Vec::new();
                    for r in &state.results {
                        for ph in &r.generated_phonemes {
                            aliases.push(ph.final_alias.clone());
                        }
                    }
                    state.coverage_report = Some(vb.calculate_coverage(&aliases));
                }
            }
        });

        ui.add_space(6.0);
        ui.horizontal(|ui| {
            let is_playing = state.audio_player.is_playing();
            if is_playing {
                if ui.button(egui::RichText::new(" Parar Audio").color(egui::Color32::from_rgb(255, 100, 100)).strong()).clicked() {
                    state.audio_player.stop();
                }
            } else {
                if ui.button(egui::RichText::new(" Tocar Sequencia de Audio").color(egui::Color32::from_rgb(100, 220, 150)).strong()).clicked() {
                    let freq = AudioTonePlayer::note_name_to_freq(&state.input_tone);
                    let ms_per_beat = (60000.0 / state.bpm.max(20.0)) as u64;
                    let mut notes = Vec::new();
                    for _ in &state.results {
                        notes.push((freq, ms_per_beat));
                    }
                    if !notes.is_empty() {
                        state.audio_player.play_phoneme_sequence(notes);
                    }
                }
            }

            ui.separator();
            ui.label(egui::RichText::new("Exemplos:").color(egui::Color32::from_rgb(160, 175, 195)).size(12.0));

            if ui.button("Exemplo 1: voce me deu amor no coracao").clicked() {
                state.input_text = "voce me deu amor no coracao".to_string();
                let simulator = PhonemizerSimulator::new(project);
                state.results = simulator.simulate_phrase(&state.input_text);
            }

            if ui.button("Exemplo 2: nao me deixe aqui na solidao").clicked() {
                state.input_text = "nao me deixe aqui na solidao".to_string();
                let simulator = PhonemizerSimulator::new(project);
                state.results = simulator.simulate_phrase(&state.input_text);
            }
        });
    });

    ui.add_space(10.0);

    // Voicebank oto.ini Cross-Check Group
    ui.collapsing("Verificacao Cruzada de Voicebank (oto.ini / Cobertura de Aliases)", |ui| {
        ui.horizontal(|ui| {
            ui.label("Arquivo oto.ini:");
            ui.add(egui::TextEdit::singleline(&mut state.voicebank_path_input).desired_width(320.0));
            
            if ui.button("Procurar oto.ini...").clicked() {
                if let Some(path) = rfd::FileDialog::new().add_filter("Oto ini", &["ini"]).pick_file() {
                    let path_str = path.to_string_lossy().to_string();
                    state.voicebank_path_input = path_str.clone();
                    match VoicebankOto::parse_oto_ini(&path_str) {
                        Ok(vb) => {
                            let mut aliases = Vec::new();
                            for r in &state.results {
                                for ph in &r.generated_phonemes {
                                    aliases.push(ph.final_alias.clone());
                                }
                            }
                            state.coverage_report = Some(vb.calculate_coverage(&aliases));
                            state.voicebank_oto = Some(vb);
                        }
                        Err(e) => {
                            eprintln!("Erro oto.ini: {}", e);
                        }
                    }
                }
            }

            if ui.button("Recalcular Cobertura").clicked() {
                if let Some(ref vb) = state.voicebank_oto {
                    let mut aliases = Vec::new();
                    for r in &state.results {
                        for ph in &r.generated_phonemes {
                            aliases.push(ph.final_alias.clone());
                        }
                    }
                    state.coverage_report = Some(vb.calculate_coverage(&aliases));
                }
            }
        });

        if let Some(ref rep) = state.coverage_report {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("Total de Aliases no Voicebank: {}", rep.total_bank_aliases)).strong());
                ui.label(egui::RichText::new(format!("Cobertos pela Frase: {}", rep.covered_by_phonemizer)).color(egui::Color32::from_rgb(0, 220, 180)).strong());
                if !rep.phonemizer_extra_aliases.is_empty() {
                    ui.label(egui::RichText::new(format!("Aliases Gerados Nao Encontrados no Banco: {}", rep.phonemizer_extra_aliases.len())).color(egui::Color32::from_rgb(255, 120, 120)).strong());
                }
            });
            if !rep.phonemizer_extra_aliases.is_empty() {
                ui.label(egui::RichText::new(format!("Aliases Faltando no Banco: [{}]", rep.phonemizer_extra_aliases.join(", "))).color(egui::Color32::from_rgb(255, 150, 150)).size(11.5));
            }
        }
    });

    ui.add_space(10.0);
    section_header(
        ui,
        "Resultados da Decomposicao Fonetica & Aliases",
        Some("Visualizacao detalhada de cada nota com os fonemas identificados, alias final gerado e regra de transicao aplicada."),
    );

    if state.results.is_empty() {
        ui.label("Digite uma frase e clique em 'Simular Frase' para visualizar os resultados.");
    } else {
        let avail_width = ui.available_width().max(300.0);
        egui::ScrollArea::vertical()
            .id_salt("simulator_results_scroll")
            .show(ui, |ui| {
                for (note_idx, note_res) in state.results.iter().enumerate() {
                    egui::Frame::group(ui.style())
                        .fill(egui::Color32::from_rgb(22, 26, 35))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 55, 75)))
                        .show(ui, |ui| {
                            ui.set_min_width(avail_width);

                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(format!("Nota #{}: \"{}\"", note_idx + 1, note_res.input_lyric)).strong().size(14.5).color(egui::Color32::from_rgb(255, 220, 100)));
                                ui.label(egui::RichText::new(format!("-> Fonemas brutos: [{}]", note_res.normalized_phonemes.join(", "))).color(egui::Color32::from_rgb(180, 195, 220)));
                            });

                            ui.add_space(4.0);
                            ui.indent("note_phonemes_indent", |ui| {
                                for ph_out in &note_res.generated_phonemes {
                                    ui.horizontal(|ui| {
                                        ui.label(egui::RichText::new(&ph_out.final_alias).color(egui::Color32::from_rgb(0, 225, 185)).strong().size(14.0));
                                        ui.label(egui::RichText::new(format!("(Regra: {})", ph_out.rule_applied)).color(egui::Color32::from_rgb(150, 165, 190)));
                                    });
                                }
                            });
                        });
                    ui.add_space(6.0);
                }
            });
    }
}
