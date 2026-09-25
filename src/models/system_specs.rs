use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub struct SystemSpecs {
    pub os_name: String,
    pub os_arch: String,
    pub logical_cores: usize,
    pub physical_cores_estimate: usize,
    pub dotnet_installed: bool,
    pub dotnet_version: String,
    pub dotnet_path: String,
    pub simd_avx2: bool,
    pub simd_neon: bool,
    pub renderer_info: String,
}

impl SystemSpecs {
    pub fn detect(custom_dotnet_path: Option<&str>) -> Self {
        let logical_cores = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        let physical_cores_estimate = (logical_cores / 2).max(1);

        let os_name = std::env::consts::OS.to_string();
        let os_arch = std::env::consts::ARCH.to_string();

        let simd_avx2 = cfg!(target_feature = "avx2") || is_x86_feature_detected_safe();
        let simd_neon = os_arch == "aarch64" || cfg!(target_feature = "neon");

        let dotnet_cmd = if let Some(p) = custom_dotnet_path {
            if !p.trim().is_empty() {
                p.trim()
            } else {
                "dotnet"
            }
        } else {
            "dotnet"
        };

        let mut dotnet_installed = false;
        let mut dotnet_version = "Nao detectado".to_string();
        let mut dotnet_path = String::new();

        if let Ok(output) = Command::new(dotnet_cmd).arg("--version").output() {
            if output.status.success() {
                dotnet_installed = true;
                dotnet_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                dotnet_path = dotnet_cmd.to_string();
            }
        }

        if !dotnet_installed {
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
                            dotnet_installed = true;
                            dotnet_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                            dotnet_path = p.to_string();
                            break;
                        }
                    }
                }
            }
        }

        let renderer_info = match os_name.as_str() {
            "macos" => "Apple Metal (GPU Acelerado)",
            "windows" => "DirectX 12 / Vulkan / WGPU (GPU Acelerado)",
            "linux" => "Vulkan / OpenGL Glow (GPU Acelerado)",
            _ => "Hardware Renderer (Universal)",
        }.to_string();

        Self {
            os_name,
            os_arch,
            logical_cores,
            physical_cores_estimate,
            dotnet_installed,
            dotnet_version,
            dotnet_path,
            simd_avx2,
            simd_neon,
            renderer_info,
        }
    }
}

fn is_x86_feature_detected_safe() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        is_x86_feature_detected!("avx2")
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}
