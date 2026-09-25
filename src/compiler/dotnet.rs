use crate::generator::{generate_csharp_code, generate_csproj_with_settings};
use crate::models::{AppSettings, PhonemizerProject, RoslynOptimizationLevel};
use super::openutau_api_stub::generate_openutau_api_contract;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct CompilationResult {
    pub success: bool,
    pub output_dll_path: Option<PathBuf>,
    pub logs: String,
    pub error_message: Option<String>,
}

pub fn check_dotnet_installed(custom_path: Option<&str>) -> Result<String, String> {
    let cmd = if let Some(p) = custom_path {
        if !p.trim().is_empty() {
            p.trim()
        } else {
            "dotnet"
        }
    } else {
        "dotnet"
    };

    match Command::new(cmd).arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                let ver = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Ok(ver)
            } else {
                let err = String::from_utf8_lossy(&output.stderr).to_string();
                Err(format!("dotnet retornou erro: {}", err))
            }
        }
        Err(e) => {

            let standard_paths = [
                "/usr/local/share/dotnet/dotnet",
                "/usr/share/dotnet/dotnet",
                "/opt/dotnet/dotnet",
                "/usr/bin/dotnet",
                "C:\\Program Files\\dotnet\\dotnet.exe",
            ];
            for p in &standard_paths {
                if std::path::Path::new(p).exists() {
                    if let Ok(output) = Command::new(p).arg("--version").output() {
                        if output.status.success() {
                            let ver = String::from_utf8_lossy(&output.stdout).trim().to_string();
                            return Ok(ver);
                        }
                    }
                }
            }
            Err(format!("Comando dotnet nao encontrado: {}", e))
        }
    }
}

pub async fn compile_phonemizer_project_with_settings(
    project: &PhonemizerProject,
    custom_plugins_dir: Option<&str>,
    settings: Option<&AppSettings>,
    log_sender: Option<mpsc::UnboundedSender<String>>,
) -> CompilationResult {
    let send_log = |msg: &str| {
        if let Some(ref sender) = log_sender {
            let _ = sender.send(msg.to_string());
        }
    };

    send_log("[INFO] Iniciando processo de compilacao do phonemizer...");

    let custom_dotnet = settings.and_then(|s| {
        if !s.custom_dotnet_path.trim().is_empty() {
            Some(s.custom_dotnet_path.as_str())
        } else {
            None
        }
    });

    let _dotnet_ver = match check_dotnet_installed(custom_dotnet) {
        Ok(v) => {
            send_log(&format!("[OK] Versao do .NET SDK detectada: {}", v));
            v
        }
        Err(e) => {
            send_log(&format!("[ERRO] Falha ao verificar .NET SDK: {}", e));
            return CompilationResult {
                success: false,
                output_dll_path: None,
                logs: format!("Falha ao localizar dotnet: {}", e),
                error_message: Some("Microsoft .NET SDK (dotnet) e necessario para compilar plugins .dll.".to_string()),
            };
        }
    };

    let temp_dir = match tempfile::Builder::new().prefix("phonemizer_build_").tempdir() {
        Ok(d) => d,
        Err(e) => {
            send_log(&format!("[ERRO] Falha ao criar diretorio temporario de build: {}", e));
            return CompilationResult {
                success: false,
                output_dll_path: None,
                logs: format!("Tempdir error: {}", e),
                error_message: Some(e.to_string()),
            };
        }
    };
    let build_dir = temp_dir.path();
    send_log(&format!("[INFO] Diretorio temporario de staging: {}", build_dir.display()));

    let csharp_code = generate_csharp_code(project);
    let csproj_content = generate_csproj_with_settings(project, None, settings);
    let api_contract = generate_openutau_api_contract();

    let csproj_path = build_dir.join(format!("{}.csproj", project.name));
    let cs_path = build_dir.join(format!("{}.cs", project.name));
    let api_path = build_dir.join("OpenUtauApi.cs");

    if let Err(e) = fs::write(&csproj_path, csproj_content) {
        return CompilationResult {
            success: false,
            output_dll_path: None,
            logs: format!("Falha ao gravar .csproj: {}", e),
            error_message: Some(e.to_string()),
        };
    }

    if let Err(e) = fs::write(&cs_path, csharp_code) {
        return CompilationResult {
            success: false,
            output_dll_path: None,
            logs: format!("Falha ao gravar arquivo .cs: {}", e),
            error_message: Some(e.to_string()),
        };
    }

    if let Err(e) = fs::write(&api_path, api_contract) {
        return CompilationResult {
            success: false,
            output_dll_path: None,
            logs: format!("Falha ao gravar OpenUtauApi.cs: {}", e),
            error_message: Some(e.to_string()),
        };
    }

    send_log("[INFO] Arquivos fonte C# e definicao de projeto gerados.");

    let target_plugins_dir = custom_plugins_dir
        .map(|p| p.to_string())
        .or_else(|| settings.map(|s| s.openutau_plugins_dir.clone()))
        .unwrap_or_else(|| project.openutau_plugins_dir.clone());

    let raw_path = PathBuf::from(&target_plugins_dir);
    let final_output_dir = if raw_path.is_absolute() {
        raw_path
    } else if let Ok(cwd) = std::env::current_dir() {
        cwd.join(raw_path)
    } else {
        raw_path
    };

    if !final_output_dir.exists() {
        send_log(&format!("[INFO] Criando pasta de destino: {}", final_output_dir.display()));
        let _ = fs::create_dir_all(&final_output_dir);
    }

    let out_dir_arg = final_output_dir.to_string_lossy().to_string();

    let config_arg = match settings.map(|s| s.roslyn_optimization).unwrap_or(RoslynOptimizationLevel::Release) {
        RoslynOptimizationLevel::Release => "Release",
        RoslynOptimizationLevel::Debug => "Debug",
    };

    let dotnet_bin = custom_dotnet.unwrap_or("dotnet");
    send_log(&format!("[INFO] Executando: {} build -c {} -o \"{}\"...", dotnet_bin, config_arg, out_dir_arg));

    let build_output = Command::new(dotnet_bin)
        .current_dir(build_dir)
        .arg("build")
        .arg("-c")
        .arg(config_arg)
        .arg("-o")
        .arg(&out_dir_arg)
        .output();

    match build_output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let combined_logs = format!("{}\n{}", stdout, stderr);

            for line in stdout.lines() {
                if !line.trim().is_empty() {
                    send_log(line);
                }
            }
            if !stderr.is_empty() {
                for line in stderr.lines() {
                    send_log(&format!("[AVISO] {}", line));
                }
            }

            if output.status.success() {
                let dll_file_name = if project.output_dll_name.ends_with(".dll") {
                    project.output_dll_name.clone()
                } else {
                    format!("{}.dll", project.name)
                };

                let expected_dll_path = final_output_dir.join(&dll_file_name);
                send_log(&format!("[SUCESSO] Build concluido! DLL gerada em: {}", expected_dll_path.display()));

                CompilationResult {
                    success: true,
                    output_dll_path: Some(expected_dll_path),
                    logs: combined_logs,
                    error_message: None,
                }
            } else {
                send_log("[ERRO] Falha na compilacao da biblioteca DLL.");
                CompilationResult {
                    success: false,
                    output_dll_path: None,
                    logs: combined_logs,
                    error_message: Some("dotnet build retornou codigo de saida diferente de zero.".to_string()),
                }
            }
        }
        Err(e) => {
            send_log(&format!("[ERRO] Falha ao executar comando dotnet: {}", e));
            CompilationResult {
                success: false,
                output_dll_path: None,
                logs: format!("Erro de execucao: {}", e),
                error_message: Some(e.to_string()),
            }
        }
    }
}

pub async fn compile_phonemizer_project(
    project: &PhonemizerProject,
    custom_plugins_dir: Option<&str>,
    log_sender: Option<mpsc::UnboundedSender<String>>,
) -> CompilationResult {
    compile_phonemizer_project_with_settings(project, custom_plugins_dir, None, log_sender).await
}
