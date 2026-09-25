use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RenderBackend {
    Auto,
    Metal,
    Vulkan,
    DirectX12,
    OpenGL,
    Software,
}

impl std::fmt::Display for RenderBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderBackend::Auto => write!(f, "Automatico (Recomendado)"),
            RenderBackend::Metal => write!(f, "Apple Metal (macOS / Apple Silicon)"),
            RenderBackend::Vulkan => write!(f, "Vulkan (Linux / Windows)"),
            RenderBackend::DirectX12 => write!(f, "DirectX 12 (Windows)"),
            RenderBackend::OpenGL => write!(f, "OpenGL / Glow (Universal)"),
            RenderBackend::Software => write!(f, "Software Rendering (CPU Fallback)"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DotnetTarget {
    Net8_0,
    Net7_0,
    Net9_0,
    NetStandard2_1,
}

impl std::fmt::Display for DotnetTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DotnetTarget::Net8_0 => write!(f, ".NET 8.0 (Recomendado / OpenUtau Moderno)"),
            DotnetTarget::Net7_0 => write!(f, ".NET 7.0 (Legado)"),
            DotnetTarget::Net9_0 => write!(f, ".NET 9.0 (Preview / Cutting-Edge)"),
            DotnetTarget::NetStandard2_1 => write!(f, ".NET Standard 2.1 (Universal)"),
        }
    }
}

impl DotnetTarget {
    pub fn tfm_string(&self) -> &'static str {
        match self {
            DotnetTarget::Net8_0 => "net8.0",
            DotnetTarget::Net7_0 => "net7.0",
            DotnetTarget::Net9_0 => "net9.0",
            DotnetTarget::NetStandard2_1 => "netstandard2.1",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RoslynOptimizationLevel {
    Release,
    Debug,
}

impl std::fmt::Display for RoslynOptimizationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoslynOptimizationLevel::Release => write!(f, "Release (-O3 Otimizacao Maxima)"),
            RoslynOptimizationLevel::Debug => write!(f, "Debug (Com Simbolos de Depuracao)"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogVerbosity {
    Verbose,
    Normal,
    ErrorsOnly,
}

impl std::fmt::Display for LogVerbosity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogVerbosity::Verbose => write!(f, "Detalhado / Verbose (Todos os logs e passos)"),
            LogVerbosity::Normal => write!(f, "Normal (Avisos e Conclusoes)"),
            LogVerbosity::ErrorsOnly => write!(f, "Apenas Erros"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppSettings {
    // Hardware & GPU Acceleration
    pub hardware_acceleration: bool,
    pub render_backend: RenderBackend,
    pub max_fps: u32,
    pub vsync: bool,
    pub parallel_threads: usize,
    pub enable_simd_phonetics: bool,
    pub font_subpixel_antialiasing: bool,
    pub ui_scale: f32,

    // Environment Paths & SDK
    pub custom_dotnet_path: String,
    pub openutau_plugins_dir: String,

    // Compilation & Roslyn Engine Specs
    pub dotnet_target_framework: DotnetTarget,
    pub roslyn_optimization: RoslynOptimizationLevel,
    pub ready_to_run_aot: bool,
    pub nullable_context: bool,
    pub allow_unsafe_code: bool,
    pub deterministic_build: bool,

    // Auto-Save & Telemetry
    pub auto_save_enabled: bool,
    pub auto_save_interval_minutes: u32,
    pub log_verbosity: LogVerbosity,
}

impl Default for AppSettings {
    fn default() -> Self {
        let threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);

        Self {
            hardware_acceleration: true,
            render_backend: RenderBackend::Auto,
            max_fps: 60,
            vsync: true,
            parallel_threads: threads,
            enable_simd_phonetics: true,
            font_subpixel_antialiasing: true,
            ui_scale: 1.0,

            custom_dotnet_path: String::new(),
            openutau_plugins_dir: crate::models::default_openutau_plugins_path(),

            dotnet_target_framework: DotnetTarget::Net8_0,
            roslyn_optimization: RoslynOptimizationLevel::Release,
            ready_to_run_aot: false,
            nullable_context: true,
            allow_unsafe_code: true,
            deterministic_build: true,

            auto_save_enabled: true,
            auto_save_interval_minutes: 5,
            log_verbosity: LogVerbosity::Normal,
        }
    }
}

impl AppSettings {
    fn get_config_path() -> Option<PathBuf> {
        #[cfg(target_os = "macos")]
        {
            if let Ok(home) = std::env::var("HOME") {
                let p = PathBuf::from(home).join("Library/Application Support/PhonemizerStudio/settings.json");
                return Some(p);
            }
        }
        #[cfg(target_os = "windows")]
        {
            if let Ok(appdata) = std::env::var("APPDATA") {
                let p = PathBuf::from(appdata).join("PhonemizerStudio\\settings.json");
                return Some(p);
            }
        }
        #[cfg(target_os = "linux")]
        {
            if let Ok(home) = std::env::var("HOME") {
                let p = PathBuf::from(home).join(".config/phonemizer_studio/settings.json");
                return Some(p);
            }
        }
        None
    }

    pub fn load() -> Self {
        if let Some(path) = Self::get_config_path() {
            if path.exists() {
                if let Ok(data) = fs::read_to_string(&path) {
                    if let Ok(settings) = serde_json::from_str::<AppSettings>(&data) {
                        return settings;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), String> {
        if let Some(path) = Self::get_config_path() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let json = serde_json::to_string_pretty(self)
                .map_err(|e| format!("Erro ao serializar configuracoes: {}", e))?;
            fs::write(&path, json)
                .map_err(|e| format!("Erro ao gravar arquivo de configuracao: {}", e))?;
            Ok(())
        } else {
            Err("Nao foi possivel determinar o diretorio de configuracoes do sistema.".to_string())
        }
    }
}
