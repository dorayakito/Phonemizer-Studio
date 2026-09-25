use crate::models::{PhonemeSystemType, PhonemizerProject};
use crate::ui::helpers::{help_marker, section_header};
use egui::Ui;

pub fn render_metadata_view(ui: &mut Ui, project: &mut PhonemizerProject) {
    section_header(
        ui,
        "Metadados e Configuracoes Principais",
        Some("Configure os detalhes de identificacao, arquitetura da classe base e caminho de instalacao da DLL para o OpenUtau."),
    );

    egui::Grid::new("metadata_grid")
        .num_columns(3)
        .spacing([15.0, 12.0])
        .show(ui, |ui| {
            ui.label("Nome da Classe / Phonemizer:");
            ui.text_edit_singleline(&mut project.name);
            help_marker(ui, "Nome da classe C# que sera gerada e compilada na DLL (ex: PortugueseSyllablePhonemizer).");
            ui.end_row();

            ui.label("Tag Curta de Identificacao:");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut project.tag);
                ui.label("(Ex: PT-BR, JA-VCV, EN-ARPA, ES-ES)");
            });
            help_marker(ui, "Identificador unico curto exibido no menu de selecao de phonemizer do OpenUtau.");
            ui.end_row();

            ui.label("Autor / Criador:");
            ui.text_edit_singleline(&mut project.author);
            help_marker(ui, "Nome do autor ou comunidade responsavel pela criacao do phonemizer.");
            ui.end_row();

            ui.label("Versao:");
            ui.text_edit_singleline(&mut project.version);
            help_marker(ui, "Numero de versao semantica do plugin (ex: 1.0.0).");
            ui.end_row();

            ui.label("Codigo do Idioma (ISO):");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut project.language_code);
                ui.label("(Ex: pt, ja, es, en, fr, ru, ko, zh)");
            });
            help_marker(ui, "Codigo de duas letras ISO 639-1 do idioma suportado.");
            ui.end_row();

            ui.label("Namespace C#:");
            ui.text_edit_singleline(&mut project.namespace);
            help_marker(ui, "Namespace C# da biblioteca (padrao recomendado: OpenUtau.Plugin.Builtin).");
            ui.end_row();

            ui.label("Descricao do Plugin:");
            ui.text_edit_multiline(&mut project.description);
            help_marker(ui, "Breve descricao explicativa sobre o funcionamento e caracteristicas do phonemizer.");
            ui.end_row();

            ui.label("Arquitetura / Classe Base:");
            egui::ComboBox::from_id_salt("base_class_combo")
                .selected_text(format!("{}", project.base_class_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut project.base_class_type, PhonemeSystemType::SyllableBased, "Syllable Based (Multilingue: PT, ES, EN, FR, IT, etc.)");
                    ui.selectable_value(&mut project.base_class_type, PhonemeSystemType::JapaneseVCV, "Japanese VCV (- a, a ka, i ka)");
                    ui.selectable_value(&mut project.base_class_type, PhonemeSystemType::JapaneseCVVC, "Japanese CVVC (ka, a k, k a)");
                    ui.selectable_value(&mut project.base_class_type, PhonemeSystemType::JapanesePresamp, "Japanese Presamp / Romaji-Kana");
                    ui.selectable_value(&mut project.base_class_type, PhonemeSystemType::Arpasing, "Arpasing (Ingles com CMUdict / difonemas)");
                    ui.selectable_value(&mut project.base_class_type, PhonemeSystemType::RussianCVC, "Russian / Slavic CVC (CVC e Palatalizadas)");
                    ui.selectable_value(&mut project.base_class_type, PhonemeSystemType::FrenchSyllable, "French Syllable com Liaisons");
                    ui.selectable_value(&mut project.base_class_type, PhonemeSystemType::KoreanHangul, "Korean Hangul");
                    ui.selectable_value(&mut project.base_class_type, PhonemeSystemType::ChineseCVV, "Chinese Mandarin (Pinyin / CVV)");
                    ui.selectable_value(&mut project.base_class_type, PhonemeSystemType::CustomDirect, "Custom Low-Level OpenUtau.Api.Phonemizer");
                    ui.selectable_value(&mut project.base_class_type, PhonemeSystemType::RawCSharpDirect, "Modo C# Puro Direto (Edicao Livre de Codigo)");
                });
            help_marker(ui, "Define a classe base de heranca do OpenUtau para estruturar a logica de processamento.");
            ui.end_row();

            ui.label("Nome do Arquivo .DLL:");
            ui.text_edit_singleline(&mut project.output_dll_name);
            help_marker(ui, "Nome do arquivo binario compilado gerado na pasta de saida (ex: MeuPhonemizer.dll).");
            ui.end_row();

            ui.label("Pasta de Plugins do OpenUtau:");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut project.openutau_plugins_dir);
                if ui.button("Procurar...").clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        project.openutau_plugins_dir = folder.to_string_lossy().to_string();
                    }
                }
            });
            help_marker(ui, "Caminho absoluto para a pasta de Plugins do OpenUtau onde a DLL sera instalada diretamente.");
            ui.end_row();
        });

    ui.add_space(20.0);
    section_header(
        ui,
        "Configuracoes Gerais de Transicao e Fallbacks",
        Some("Ajuste as tolerancias e proporcoes temporais padrao para transicoes consonantais."),
    );

    ui.horizontal(|ui| {
        ui.checkbox(&mut project.allow_fallback_cv, "Permitir Fallback para CV caso VCV nao exista no voicebank");
        help_marker(ui, "Se habilitado, quando um alias VCV (ex: 'a ka') nao for encontrado no oto.ini do cantor, o sistema utilizara o CV simples ('ka').");
    });

    ui.horizontal(|ui| {
        ui.checkbox(&mut project.allow_fallback_vv, "Permitir Fallback para V simples caso transicao VV nao exista");
        help_marker(ui, "Se habilitado, quando uma transicao de ditongo (ex: 'a i') nao existir, usara a segunda vogal simples ('i').");
    });

    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.label("Proporcao de Divisao Consonantal no Ataque (Onset Split):");
        ui.add(egui::Slider::new(&mut project.split_ratio_onset, 0.1..=0.9).text("%"));
        help_marker(ui, "Percentual do tempo de antecipacao da consoante antes do inicio da nota principal.");
    });

    ui.horizontal(|ui| {
        ui.label("Proporcao de Divisao Consonantal na Coda (Coda Split):");
        ui.add(egui::Slider::new(&mut project.split_ratio_coda, 0.1..=0.9).text("%"));
        help_marker(ui, "Percentual do tempo dedicado a consoante final no encerramento da silaba.");
    });

    ui.horizontal(|ui| {
        ui.label("Duracao Maxima de Transicao Consonantal (ms):");
        ui.add(egui::Slider::new(&mut project.max_consonant_duration_ms, 30..=300).suffix(" ms"));
        help_marker(ui, "Limite maximo em milissegundos para transicoes de consoantes rapidas para evitar lentidao.");
    });
}
