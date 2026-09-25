use eframe::egui;
use phonemizer_studio::compiler::compile_phonemizer_project;
use phonemizer_studio::generator::generate_csharp_code;
use phonemizer_studio::models::PhonemizerProject;
use phonemizer_studio::ui::PhonemizerStudioApp;
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--build" => {
                if args.len() < 3 {
                    eprintln!("Uso: phonemizer-studio --build <arquivo_projeto.json> [pasta_destino]");
                    std::process::exit(1);
                }
                let project_path = &args[2];
                let out_dir = args.get(3).map(|s| s.as_str());

                println!("[INFO] Carregando projeto de: {}", project_path);
                let json_data = fs::read_to_string(project_path)?;
                let project: PhonemizerProject = serde_json::from_str(&json_data)?;

                println!("[INFO] Compilando phonemizer: {} (Tag: {})", project.name, project.tag);
                let result = compile_phonemizer_project(&project, out_dir, None).await;

                if result.success {
                    println!("[SUCESSO] DLL gerada em: {:?}", result.output_dll_path);
                    return Ok(());
                } else {
                    eprintln!("[ERRO] Falha na compilacao:");
                    eprintln!("{}", result.logs);
                    std::process::exit(1);
                }
            }
            "--export-cs" => {
                if args.len() < 4 {
                    eprintln!("Uso: phonemizer-studio --export-cs <arquivo_projeto.json> <saida.cs>");
                    std::process::exit(1);
                }
                let project_path = &args[2];
                let out_path = &args[3];

                let json_data = fs::read_to_string(project_path)?;
                let project: PhonemizerProject = serde_json::from_str(&json_data)?;

                let cs_code = generate_csharp_code(&project);
                fs::write(out_path, cs_code)?;
                println!("[SUCESSO] Codigo C# exportado com sucesso para: {}", out_path);
                return Ok(());
            }
            "--help" | "-h" => {
                println!("OpenUtau Phonemizer Studio & C# Compiler v1.0.0");
                println!("Uso:");
                println!(" phonemizer-studio (Abre a interface grafica Desktop)");
                println!(" phonemizer-studio --build <proj.json> [dir] (Compila a DLL via linha de comando)");
                println!(" phonemizer-studio --export-cs <proj.json> <cs> (Exporta o codigo fonte C#)");
                return Ok(());
            }
            _ => {}
        }
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1220.0, 830.0])
            .with_min_inner_size([850.0, 600.0])
            .with_title("OpenUtau Phonemizer Studio & C# Compiler"),
        ..Default::default()
    };

    eframe::run_native(
        "OpenUtau Phonemizer Studio",
        options,
        Box::new(|cc| Ok(Box::new(PhonemizerStudioApp::new(cc)))),
    )?;

    Ok(())
}
