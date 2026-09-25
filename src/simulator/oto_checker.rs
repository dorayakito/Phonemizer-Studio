use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct OtoEntry {
    pub wav_file: String,
    pub alias: String,
    pub offset: f64,
    pub consonant: f64,
    pub cutoff: f64,
    pub preutterance: f64,
    pub overlap: f64,
}

#[derive(Debug, Clone, Default)]
pub struct VoicebankOto {
    pub name: String,
    pub folder_path: String,
    pub entries: Vec<OtoEntry>,
    pub alias_map: HashMap<String, OtoEntry>,
}

#[derive(Debug, Clone)]
pub struct CoverageReport {
    pub total_bank_aliases: usize,
    pub covered_by_phonemizer: usize,
    pub uncovered_aliases: Vec<String>,
    pub phonemizer_extra_aliases: Vec<String>,
    pub coverage_percentage: f32,
}

impl VoicebankOto {
    pub fn parse_oto_ini<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path_ref = path.as_ref();
        let file = File::open(path_ref).map_err(|e| format!("Erro ao abrir oto.ini: {}", e))?;
        let reader = BufReader::new(file);

        let mut entries = Vec::new();
        let mut alias_map = HashMap::new();

        for (_line_idx, line_res) in reader.lines().enumerate() {
            let line = match line_res {
                Ok(l) => l,
                Err(_) => continue,
            };

            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }

            if let Some((wav_part, rest)) = line.split_once('=') {
                let parts: Vec<&str> = rest.split(',').collect();
                let wav_file = wav_part.trim().to_string();
                let alias = if !parts.is_empty() && !parts[0].trim().is_empty() {
                    parts[0].trim().to_string()
                } else {
                    wav_file.trim_end_matches(".wav").to_string()
                };

                let offset = parts.get(1).and_then(|s| s.trim().parse::<f64>().ok()).unwrap_or(0.0);
                let consonant = parts.get(2).and_then(|s| s.trim().parse::<f64>().ok()).unwrap_or(0.0);
                let cutoff = parts.get(3).and_then(|s| s.trim().parse::<f64>().ok()).unwrap_or(0.0);
                let preutterance = parts.get(4).and_then(|s| s.trim().parse::<f64>().ok()).unwrap_or(0.0);
                let overlap = parts.get(5).and_then(|s| s.trim().parse::<f64>().ok()).unwrap_or(0.0);

                let entry = OtoEntry {
                    wav_file,
                    alias: alias.clone(),
                    offset,
                    consonant,
                    cutoff,
                    preutterance,
                    overlap,
                };

                alias_map.insert(alias, entry.clone());
                entries.push(entry);
            }
        }

        let folder_path = path_ref
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        let name = path_ref
            .parent()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Voicebank".to_string());

        Ok(Self {
            name,
            folder_path,
            entries,
            alias_map,
        })
    }

    pub fn calculate_coverage(&self, generated_aliases: &[String]) -> CoverageReport {
        let total_bank = self.alias_map.len();
        if total_bank == 0 {
            return CoverageReport {
                total_bank_aliases: 0,
                covered_by_phonemizer: 0,
                uncovered_aliases: Vec::new(),
                phonemizer_extra_aliases: generated_aliases.to_vec(),
                coverage_percentage: 0.0,
            };
        }

        let mut gen_set: std::collections::HashSet<&str> = std::collections::HashSet::new();
        for a in generated_aliases {
            gen_set.insert(a.as_str());
        }

        let mut covered = 0;
        let mut uncovered = Vec::new();

        for alias in self.alias_map.keys() {
            if gen_set.contains(alias.as_str()) {
                covered += 1;
            } else {
                uncovered.push(alias.clone());
            }
        }

        let mut extra = Vec::new();
        for alias in generated_aliases {
            if !self.alias_map.contains_key(alias) {
                extra.push(alias.clone());
            }
        }

        uncovered.sort();
        extra.sort();
        extra.dedup();

        let pct = (covered as f32 / total_bank as f32) * 100.0;

        CoverageReport {
            total_bank_aliases: total_bank,
            covered_by_phonemizer: covered,
            uncovered_aliases: uncovered,
            phonemizer_extra_aliases: extra,
            coverage_percentage: pct,
        }
    }
}
