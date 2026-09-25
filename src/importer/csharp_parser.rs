use crate::models::*;
use regex::Regex;

pub fn parse_csharp_to_project(csharp_code: &str, file_name: Option<&str>) -> PhonemizerProject {
    let mut project = PhonemizerProject::default();
    project.raw_csharp_source = Some(csharp_code.to_string());

    let default_name = file_name
        .map(|f| f.trim_end_matches(".cs").to_string())
        .unwrap_or_else(|| "ImportedPhonemizer".to_string());

    project.name = default_name.clone();
    project.output_dll_name = format!("{}.dll", default_name);

    let summary_re = Regex::new(r#"(?s)///\s*<summary>(.*?)///\s*</summary>"#).unwrap();
    if let Some(caps) = summary_re.captures(csharp_code) {
        if let Some(m) = caps.get(1) {
            let clean_desc: Vec<String> = m.as_str()
                .lines()
                .map(|l| l.trim().trim_start_matches("///").trim().to_string())
                .filter(|l| !l.is_empty())
                .collect();
            if !clean_desc.is_empty() {
                project.description = clean_desc.join(" ");
            }
        }
    }

    let attr_re = Regex::new(r#"\[Phonemizer\s*\(\s*"([^"]+)"\s*,\s*"([^"]+)"(?:\s*,\s*"([^"]+)")?(?:\s*,\s*(?:language\s*:\s*)?"([^"]*)")?\s*\)\]"#).unwrap();
    if let Some(caps) = attr_re.captures(csharp_code) {
        if let Some(m) = caps.get(1) {
            project.name = m.as_str().to_string();
            project.output_dll_name = format!("{}.dll", project.name);
        }
        if let Some(m) = caps.get(2) {
            project.tag = m.as_str().to_string();
        }
        if let Some(m) = caps.get(3) {
            project.author = m.as_str().to_string();
        }
        if let Some(m) = caps.get(4) {
            let lang = m.as_str().trim();
            if !lang.is_empty() {
                project.language_code = lang.to_string();
            }
        }
    }

    let ns_re = Regex::new(r#"namespace\s+([a-zA-Z0-9_.]+)"#).unwrap();
    if let Some(caps) = ns_re.captures(csharp_code) {
        if let Some(m) = caps.get(1) {
            project.namespace = m.as_str().to_string();
        }
    }

    let class_re = Regex::new(r#"public\s+class\s+([a-zA-Z0-9_]+)\s*:\s*([a-zA-Z0-9_]+)"#).unwrap();
    if let Some(caps) = class_re.captures(csharp_code) {
        if let Some(base_cls) = caps.get(2) {
            let base_str = base_cls.as_str();
            project.base_class_type = match base_str {
                "SyllableBasedPhonemizer" => PhonemeSystemType::SyllableBased,
                "JapaneseVCVPhonemizer" | "JapaneseVCV" => PhonemeSystemType::JapaneseVCV,
                "JapaneseCVVCPhonemizer" | "JapaneseCVVC" => PhonemeSystemType::JapaneseCVVC,
                "JapanesePresampPhonemizer" => PhonemeSystemType::JapanesePresamp,
                "ArpasingPhonemizer" => PhonemeSystemType::Arpasing,
                "RussianCVCPhonemizer" => PhonemeSystemType::RussianCVC,
                "FrenchSyllableBasedPhonemizer" => PhonemeSystemType::FrenchSyllable,
                _ => PhonemeSystemType::CustomDirect,
            };
        }
    }

    let vowels_split_re = Regex::new(r#"(?:PlainVowels|vowels|Vowels)\s*=\s*"([^"]+)"\.Split\("#).unwrap();
    if let Some(caps) = vowels_split_re.captures(csharp_code) {
        if let Some(s) = caps.get(1) {
            for sym in s.as_str().split([',', ';', ' ']) {
                let sym = sym.trim();
                if !sym.is_empty() && !project.vowels.iter().any(|v| v.symbol == sym) {
                    project.vowels.push(Vowel::new(sym));
                }
            }
        }
    }

    let vowels_arr_re = Regex::new(r#"(?:PlainVowels|vowels|Vowels)\s*=\s*(?:new\s+string\[\])?\s*\{([^}]+)\}"#).unwrap();
    if let Some(caps) = vowels_arr_re.captures(csharp_code) {
        if let Some(items_str) = caps.get(1) {
            let item_re = Regex::new(r#""([^"]+)""#).unwrap();
            for cap in item_re.captures_iter(items_str.as_str()) {
                let sym_entry = &cap[1];
                let sym = if sym_entry.contains('=') {
                    sym_entry.split('=').next().unwrap_or(sym_entry).trim()
                } else {
                    sym_entry.trim()
                };
                if !sym.is_empty() && !project.vowels.iter().any(|v| v.symbol == sym) {
                    project.vowels.push(Vowel::new(sym));
                }
            }
        }
    }

    let consonants_split_re = Regex::new(r#"(?:PlainConsonants|consonants|Consonants)\s*=\s*"([^"]+)"\.Split\("#).unwrap();
    if let Some(caps) = consonants_split_re.captures(csharp_code) {
        if let Some(s) = caps.get(1) {
            for sym in s.as_str().split([',', ';', ' ']) {
                let sym = sym.trim();
                if !sym.is_empty() && !project.consonants.iter().any(|c| c.symbol == sym) {
                    project.consonants.push(Consonant::new(sym, ConsonantType::Stop));
                }
            }
        }
    }

    let consonants_arr_re = Regex::new(r#"(?:PlainConsonants|consonants|Consonants)\s*=\s*(?:new\s+string\[\])?\s*\{([^}]+)\}"#).unwrap();
    if let Some(caps) = consonants_arr_re.captures(csharp_code) {
        if let Some(items_str) = caps.get(1) {
            let item_re = Regex::new(r#""([^"]+)""#).unwrap();
            for cap in item_re.captures_iter(items_str.as_str()) {
                let sym = cap[1].trim();
                if !sym.is_empty() && !project.consonants.iter().any(|c| c.symbol == sym) {
                    project.consonants.push(Consonant::new(sym, ConsonantType::Stop));
                }
            }
        }
    }

    let clusters_split_re = Regex::new(r#"(?:initialCC|clusters|Clusters)\s*=\s*"([^"]+)"\.Split\("#).unwrap();
    if let Some(caps) = clusters_split_re.captures(csharp_code) {
        if let Some(s) = caps.get(1) {
            for cluster_sym in s.as_str().split([',', ';', ' ']) {
                let cluster_sym = cluster_sym.trim();
                if !cluster_sym.is_empty() && !project.onset_clusters.iter().any(|cl| cl.cluster == cluster_sym) {
                    let parts: Vec<String> = cluster_sym.chars().map(|c| c.to_string()).collect();
                    project.onset_clusters.push(ConsonantCluster::new(cluster_sym, parts, true));
                }
            }
        }
    }

    let burst_re = Regex::new(r#"burstConsonants\s*=\s*"([^"]+)"\.Split\("#).unwrap();
    if let Some(caps) = burst_re.captures(csharp_code) {
        if let Some(s) = caps.get(1) {
            for burst_sym in s.as_str().split([',', ';', ' ']) {
                let burst_sym = burst_sym.trim();
                for c in &mut project.consonants {
                    if c.symbol == burst_sym {
                        c.consonant_type = ConsonantType::Stop;
                    }
                }
            }
        }
    }

    let dict_re = Regex::new(r#"\{\s*"([^"]+)"\s*,\s*(?:new\s+string\[\]\s*)?\{\s*([^}]+)\s*\}\s*\}"#).unwrap();
    for cap in dict_re.captures_iter(csharp_code) {
        let word = cap[1].to_string();
        let phs_str = &cap[2];
        let item_re = Regex::new(r#""([^"]+)""#).unwrap();
        let phs: Vec<String> = item_re.captures_iter(phs_str).map(|c| c[1].to_string()).collect();

        if !phs.is_empty() && !project.dictionary_overrides.iter().any(|d| d.word == word) {
            project.dictionary_overrides.push(DictionaryOverride::new(word, phs));
        }
    }

    let replacements_re = Regex::new(r#"(?:dictionaryReplacements|seseo|fallBacks|semiVowelFallback|eñeFallback)\s*=\s*(?:\("([^"]+)"|"([^"]+)")"#).unwrap();
    let mut order = 1;
    for cap in replacements_re.captures_iter(csharp_code) {
        let raw_pairs = cap.get(1).or_else(|| cap.get(2)).map(|m| m.as_str()).unwrap_or("");
        for pair in raw_pairs.split([';', ',']) {
            let parts: Vec<&str> = pair.split('=').map(|p| p.trim()).collect();
            if parts.len() == 2 && parts[0] != parts[1] && !parts[0].is_empty() {
                project.regex_rules.push(RegexRule::new(
                    &format!("G2P {} -> {}", parts[0], parts[1]),
                    parts[0],
                    parts[1],
                    order,
                ));
                order += 1;
            }
        }
    }

    if project.vowels.is_empty() && project.consonants.is_empty() {
        project.base_class_type = PhonemeSystemType::RawCSharpDirect;
    }

    project
}
