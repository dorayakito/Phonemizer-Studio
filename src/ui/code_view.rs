use crate::generator::generate_csharp_code;
use crate::models::{PhonemeSystemType, PhonemizerProject};
use crate::ui::helpers::{help_marker, section_header};
use egui::Ui;

pub fn render_code_view(ui: &mut Ui, project: &mut PhonemizerProject) {
    section_header(
        ui,
        "Codigo Fonte C# & Editor Direto",
        Some("Visualize o codigo C# gerado automaticamente pelo Phonemizer Studio ou edite diretamente em Modo C# Puro."),
    );

    let is_raw_mode = project.base_class_type == PhonemeSystemType::RawCSharpDirect;

    ui.horizontal(|ui| {
        if is_raw_mode {
            ui.label(egui::RichText::new("Modo C# Puro Ativo (Edicao Livre de Codigo)").color(egui::Color32::from_rgb(255, 200, 80)).strong());
            if ui.button("Voltar para Modo Visual (Gerador Automatico)").clicked() {
                project.base_class_type = PhonemeSystemType::SyllableBased;
            }
        } else {
            ui.label("Modo Visual Ativo.");
            if ui.button("Converter para Modo C# Puro (Edicao Livre)").clicked() {
                let code = generate_csharp_code(project);
                project.raw_csharp_source = Some(code);
                project.base_class_type = PhonemeSystemType::RawCSharpDirect;
            }
        }
        help_marker(ui, "Alterne entre o gerador visual de regras e a edicao livre de codigo fonte C#.");

        let code_to_copy = if is_raw_mode {
            project.raw_csharp_source.clone().unwrap_or_default()
        } else {
            generate_csharp_code(project)
        };

        if ui.button("Copiar Codigo C#").clicked() {
            ui.output_mut(|o| o.copied_text = code_to_copy);
        }

        ui.separator();
        if ui.button("Exportar DiffSinger (phonemes.txt)").clicked() {
            let txt = crate::generator::DiffSingerExporter::export_phonemes_txt(project);
            if let Some(path) = rfd::FileDialog::new().add_filter("Text", &["txt"]).set_file_name("phonemes.txt").save_file() {
                let _ = std::fs::write(path, txt);
            }
        }

        if ui.button("Exportar DiffSinger (lexicon.txt)").clicked() {
            let txt = crate::generator::DiffSingerExporter::export_lexicon_txt(project);
            if let Some(path) = rfd::FileDialog::new().add_filter("Text", &["txt"]).set_file_name("lexicon.txt").save_file() {
                let _ = std::fs::write(path, txt);
            }
        }
    });

    ui.add_space(8.0);

    if !is_raw_mode {
        ui.collapsing("Injecao de Codigo C# Customizado (Hooks Avancados)", |ui| {
            ui.label("Campos Customizados da Classe:");
            ui.text_edit_multiline(&mut project.custom_class_fields);
            help_marker(ui, "Campos de classe adicionais, caches ou estruturas de dados C#.");

            ui.label("Codigo Customizado no PreProcess:");
            ui.text_edit_multiline(&mut project.custom_preprocess_code);
            help_marker(ui, "Codigo C# executado no inicio do metodo Process.");

            ui.label("Codigo Customizado no PostProcess:");
            ui.text_edit_multiline(&mut project.custom_postprocess_code);
            help_marker(ui, "Codigo C# executado antes do retorno final dos fonemas.");

            ui.label("Metodos Auxiliares C# adicionais:");
            ui.text_edit_multiline(&mut project.custom_helper_methods);
            help_marker(ui, "Funcoes auxiliares privadas ou protegidas.");
        });
        ui.add_space(8.0);
    }

    let avail_width = ui.available_width().max(400.0);
    egui::ScrollArea::vertical().id_salt("csharp_code_editor_scroll").show(ui, |ui| {
        if is_raw_mode {
            let mut raw_code = project.raw_csharp_source.clone().unwrap_or_default();
            if ui.add(
                egui::TextEdit::multiline(&mut raw_code)
                    .code_editor()
                    .desired_width(avail_width)
                    .desired_rows(35)
                    .interactive(true),
            ).changed() {
                project.raw_csharp_source = Some(raw_code);
            }
        } else {
            let mut display_code = generate_csharp_code(project);
            ui.add(
                egui::TextEdit::multiline(&mut display_code)
                    .code_editor()
                    .desired_width(avail_width)
                    .desired_rows(35)
                    .interactive(false),
            );
        }
    });
}
