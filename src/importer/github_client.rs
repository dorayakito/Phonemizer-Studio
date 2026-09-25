use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubFileItem {
    pub name: String,
    pub path: String,
    pub download_url: Option<String>,
    pub html_url: Option<String>,
    pub size: Option<usize>,
    #[serde(rename = "type")]
    pub item_type: String,
}

pub struct GitHubClient {
    client: reqwest::Client,
}

impl GitHubClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(15))
            .user_agent("OpenUtau-Phonemizer-Studio-Rust")
            .build()
            .unwrap_or_default();
        Self { client }
    }

    pub async fn list_openutau_builtin_phonemizers(
        &self,
        owner: Option<&str>,
        repo: Option<&str>,
        path: Option<&str>,
    ) -> Result<Vec<GitHubFileItem>, String> {
        let owner_str = owner.unwrap_or("openutau");
        let repo_str = repo.unwrap_or("OpenUtau");
        let path_str = path.unwrap_or("OpenUtau.Plugin.Builtin");

        let api_url = format!("https://api.github.com/repos/{}/{}/contents/{}", owner_str, repo_str, path_str);

        match self.client.get(&api_url).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<Vec<GitHubFileItem>>().await {
                        Ok(items) => {
                            let cs_files: Vec<GitHubFileItem> = items
                                .into_iter()
                                .filter(|item| item.name.ends_with(".cs"))
                                .collect();
                            Ok(cs_files)
                        }
                        Err(e) => Err(format!("Erro ao desserializar resposta do GitHub: {}", e)),
                    }
                } else if resp.status().as_u16() == 403 {

                    Ok(get_curated_openutau_phonemizers())
                } else {
                    Err(format!("GitHub API retornou status HTTP {}: {}", resp.status(), resp.text().await.unwrap_or_default()))
                }
            }
            Err(_e) => {

                Ok(get_curated_openutau_phonemizers())
            }
        }
    }

    pub async fn fetch_raw_csharp_file(&self, download_url: &str) -> Result<String, String> {
        match self.client.get(download_url).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp.text().await.map_err(|e| format!("Erro ao ler conteúdo: {}", e))
                } else {
                    Err(format!("Falha ao baixar arquivo (HTTP {}).", resp.status()))
                }
            }
            Err(e) => Err(format!("Erro de rede ao baixar C# do GitHub: {}", e)),
        }
    }
}

pub fn get_curated_openutau_phonemizers() -> Vec<GitHubFileItem> {
    let files = [
        "PortugueseSyllablePhonemizer.cs",
        "SpanishSyllableBasedPhonemizer.cs",
        "FrenchSyllableBasedPhonemizer.cs",
        "ItalianSyllableBasedPhonemizer.cs",
        "GermanSyllablePhonemizer.cs",
        "RussianCVCPhonemizer.cs",
        "RussianSyllableBasedPhonemizer.cs",
        "UkrainianCVCPhonemizer.cs",
        "PolishCVCPhonemizer.cs",
        "JapaneseVCVPhonemizer.cs",
        "JapaneseCVVCPhonemizer.cs",
        "JapanesePresampPhonemizer.cs",
        "EnglishArpasingPhonemizer.cs",
        "EnglishDeltaPhonemizer.cs",
        "EnglishVCCVPhonemizer.cs",
        "ChineseCVVPhonemizer.cs",
        "ChineseSyllablePhonemizer.cs",
        "CantonesePhonemizer.cs",
        "KoreanSyllablePhonemizer.cs",
        "VietnameseCVPhonemizer.cs",
        "DefaultPhonemizer.cs",
    ];

    files
        .iter()
        .map(|f| GitHubFileItem {
            name: f.to_string(),
            path: format!("OpenUtau.Plugin.Builtin/{}", f),
            download_url: Some(format!(
                "https://raw.githubusercontent.com/openutau/OpenUtau/master/OpenUtau.Plugin.Builtin/{}",
                f
            )),
            html_url: Some(format!(
                "https://github.com/openutau/OpenUtau/blob/master/OpenUtau.Plugin.Builtin/{}",
                f
            )),
            size: Some(10240),
            item_type: "file".to_string(),
        })
        .collect()
}
