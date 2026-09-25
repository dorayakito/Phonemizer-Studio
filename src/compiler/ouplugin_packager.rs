use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OuPluginManifest {
    pub name: String,
    pub version: String,
    pub author: String,
    pub email: String,
    pub website: String,
    pub description: String,
    pub encoding: String,
}

pub struct OuPluginPackager;

impl OuPluginPackager {
    pub fn package_ouplugin<P: AsRef<Path>>(
        output_ouplugin_path: P,
        plugin_name: &str,
        version: &str,
        author: &str,
        description: &str,
        dll_path: &Path,
    ) -> Result<String, String> {
        let file = File::create(output_ouplugin_path.as_ref())
            .map_err(|e| format!("Falha ao criar arquivo .ouplugin: {}", e))?;

        let mut zip = ZipWriter::new(file);
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        let manifest = OuPluginManifest {
            name: plugin_name.to_string(),
            version: version.to_string(),
            author: author.to_string(),
            email: String::new(),
            website: "https://github.com/openutau/OpenUtau".to_string(),
            description: description.to_string(),
            encoding: "UTF-8".to_string(),
        };

        let manifest_json = serde_json::to_string_pretty(&manifest)
            .map_err(|e| format!("Erro ao serializar plugin.json: {}", e))?;

        zip.start_file("plugin.json", options)
            .map_err(|e| format!("Erro ao adicionar plugin.json no zip: {}", e))?;
        zip.write_all(manifest_json.as_bytes())
            .map_err(|e| format!("Erro ao escrever plugin.json: {}", e))?;

        if dll_path.exists() {
            let dll_name = dll_path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| format!("{}.dll", plugin_name));

            let dll_bytes = std::fs::read(dll_path)
                .map_err(|e| format!("Falha ao ler DLL ({:?}): {}", dll_path, e))?;

            zip.start_file(&dll_name, options)
                .map_err(|e| format!("Erro ao adicionar DLL no zip: {}", e))?;
            zip.write_all(&dll_bytes)
                .map_err(|e| format!("Erro ao gravar DLL no zip: {}", e))?;
        } else {
            return Err(format!("O arquivo DLL indicado nao existe: {:?}", dll_path));
        }

        zip.finish()
            .map_err(|e| format!("Falha ao finalizar empacotamento zip: {}", e))?;

        Ok(output_ouplugin_path.as_ref().to_string_lossy().to_string())
    }
}
