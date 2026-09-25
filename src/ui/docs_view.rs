use crate::ui::helpers::{card_frame, section_header};
use egui::{Color32, RichText, Ui};

pub fn render_docs_view(ui: &mut Ui) {
    section_header(
        ui,
        "Documentacao Interna, Guias & Downloads Essenciais",
        Some("Aprenda a arquitetura de phonemizers do OpenUtau e baixe os compiladores e ferramentas necessarias para Windows, macOS e Linux."),
    );

    egui::ScrollArea::vertical().show(ui, |ui| {

        card_frame(ui, |ui| {
            ui.label(RichText::new("Downloads Importantes para Compilar DLLs (.NET SDK & OpenUtau)").size(16.0).strong().color(Color32::from_rgb(120, 180, 255)));
            ui.add_space(4.0);
            ui.label("Para compilar arquivos .DLL de phonemizers, e necessario ter o Microsoft .NET SDK instalado em sua maquina.");
            ui.add_space(8.0);

            egui::Grid::new("downloads_grid")
                .num_columns(3)
                .spacing([20.0, 10.0])
                .show(ui, |ui| {
                    ui.strong("Plataforma / Ferramenta");
                    ui.strong("Descricao");
                    ui.strong("Link de Download / Instalacao");
                    ui.end_row();

                    ui.label("Microsoft .NET 8.0 SDK (Windows)");
                    ui.label("Instalador oficial para Windows x64 e ARM64");
                    if ui.button("Baixar para Windows (x64)").clicked() {
                        let _ = open::that("https://dotnet.microsoft.com/download/dotnet/8.0");
                    }
                    ui.end_row();

                    ui.label("Microsoft .NET 8.0 SDK (macOS)");
                    ui.label("Instalador PKG para Apple Silicon (M1/M2/M3/M4) e Intel x64");
                    if ui.button("Baixar para macOS").clicked() {
                        let _ = open::that("https://dotnet.microsoft.com/download/dotnet/8.0");
                    }
                    ui.end_row();

                    ui.label("Microsoft .NET 8.0 SDK (Linux)");
                    ui.label("Pacotes para Ubuntu, Debian, Fedora, Arch Linux");
                    if ui.button("Instrucoes Linux").clicked() {
                        let _ = open::that("https://learn.microsoft.com/dotnet/core/install/linux");
                    }
                    ui.end_row();

                    ui.label("OpenUtau (Ultima Versao Oficial)");
                    ui.label("Sintetizador de canto Open-Source oficial");
                    if ui.button("Acessar GitHub Releases").clicked() {
                        let _ = open::that("https://github.com/openutau/OpenUtau/releases");
                    }
                    ui.end_row();

                    ui.label("Visual Studio Code & C# Dev Kit");
                    ui.label("Editor de codigo recomendado para desenvolvimento em C#");
                    if ui.button("Baixar VS Code").clicked() {
                        let _ = open::that("https://code.visualstudio.com/");
                    }
                    ui.end_row();
                });
        });

        ui.add_space(14.0);

        card_frame(ui, |ui| {
            ui.label(RichText::new("O que e um Phonemizer no OpenUtau?").size(16.0).strong().color(Color32::from_rgb(120, 180, 255)));
            ui.add_space(4.0);
            ui.label(
                "Um Phonemizer e o componente do OpenUtau responsavel por converter o texto inserido nas notas (letras/silabas) \
                em uma sequencia de simbolos foneticos compativeis com o voicebank do cantor (ex: arquivos oto.ini do UTAU).",
            );
            ui.add_space(6.0);
            ui.label("Principais estilos de fonemizacao:");
            ui.label("1. Syllable-Based: Divide palavras em silabas (Ataque, Nucleo Vocálico, Coda) e resolve transicoes automaticamente (CV, VCV, VC, etc.).");
            ui.label("2. VCV (Vowel-Consonant-Vowel): Conecta a vogal da nota anterior com a consoante e vogal da nota atual (ex: 'a ka', 'o te').");
            ui.label("3. CVVC / CVC: Separa transicoes em difonemas (Consoante-Vogal, Vogal-Consoante e Consoante-Consoante).");
            ui.label("4. Arpasing: Utiliza difonemas baseados no alfabeto fonetico CMU ARPAbet com dicionario fonetico de palavras completas.");
        });

        ui.add_space(14.0);

        card_frame(ui, |ui| {
            ui.label(RichText::new("Guia Passo a Passo para Criar seu Phonemizer").size(16.0).strong().color(Color32::from_rgb(120, 180, 255)));
            ui.add_space(4.0);

            ui.label("Passo 1: Configuracao de Metadados");
            ui.label("Defina o nome da classe C#, a tag curta (ex: PT-BR, ES-ES), o autor e o idioma (ISO).");
            ui.add_space(4.0);

            ui.label("Passo 2: Cadastro de Vogais e Nasais");
            ui.label("Adicione todas as vogais da sua lingua. Se houver vogais nasais ou ditongos (ex: 'ai', 'au'), marque as opcoes correspondentes.");
            ui.add_space(4.0);

            ui.label("Passo 3: Cadastro de Consoantes e Encontros Consonantais");
            ui.label("Defina as consoantes, informando se podem aparecer no inicio (onset) ou no final da silaba (coda). Em seguida, configure os clusters (ex: 'br', 'tr', 'pl').");
            ui.add_space(4.0);

            ui.label("Passo 4: Matriz Fonetica e Padroes de Alias");
            ui.label("Configure como os aliases do oto.ini sao formatados (ex: '{prev_vowel} {consonant}{vowel}' para VCV ou '{consonant}{vowel}' para CV simples).");
            ui.add_space(4.0);

            ui.label("Passo 5: Simulacao e Testes em Tempo Real");
            ui.label("Va para a aba 'Simulador', digite uma frase de teste e verifique a decomposicao das notas e aliases gerados.");
            ui.add_space(4.0);

            ui.label("Passo 6: Compilacao da DLL");
            ui.label("Na aba 'Compilar .DLL', confirme o caminho da pasta de Plugins do OpenUtau e clique em 'Compilar .DLL e Instalar'.");
        });

        ui.add_space(14.0);

        card_frame(ui, |ui| {
            ui.label(RichText::new("Caminhos Padrao da Pasta de Plugins do OpenUtau").size(16.0).strong().color(Color32::from_rgb(120, 180, 255)));
            ui.add_space(4.0);
            ui.label("Windows: %APPDATA%\\OpenUtau\\Plugins (ex: C:\\Users\\Nome\\AppData\\Roaming\\OpenUtau\\Plugins)");
            ui.label("macOS: ~/Library/Application Support/OpenUtau/Plugins ou ~/OpenUtau/Plugins");
            ui.label("Linux: ~/.config/OpenUtau/Plugins");
            ui.add_space(4.0);
            ui.label("Qualquer arquivo .dll colocado nessa pasta sera carregado automaticamente pelo OpenUtau ao reiniciar o programa.");
        });
    });
}
