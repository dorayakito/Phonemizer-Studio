use serde::{Deserialize, Serialize};
use super::phonemes::{Consonant, ConsonantCluster, Vowel};
use super::rules::{PhonemeSystemType, PhoneticPatternRule, SyllabificationConfig, VoiceColorAndPitchConfig};
use super::exceptions::{DictionaryOverride, RegexRule};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PhonemizerProject {

    pub name: String,
    pub tag: String,
    pub author: String,
    pub version: String,
    pub language_code: String,
    pub description: String,
    pub namespace: String,
    pub base_class_type: PhonemeSystemType,

    pub openutau_plugins_dir: String,
    pub output_dll_name: String,

    pub vowels: Vec<Vowel>,
    pub consonants: Vec<Consonant>,
    pub onset_clusters: Vec<ConsonantCluster>,
    pub coda_clusters: Vec<ConsonantCluster>,
    pub pattern_rules: Vec<PhoneticPatternRule>,
    pub voice_color_and_pitch: VoiceColorAndPitchConfig,
    pub syllabification_config: SyllabificationConfig,

    pub dictionary_overrides: Vec<DictionaryOverride>,
    pub regex_rules: Vec<RegexRule>,

    pub allow_fallback_cv: bool,
    pub allow_fallback_vv: bool,
    pub split_ratio_onset: f32,
    pub split_ratio_coda: f32,
    pub max_consonant_duration_ms: i32,

    pub raw_csharp_source: Option<String>,
    pub source_github_url: Option<String>,

    pub custom_usings: Vec<String>,
    pub custom_class_fields: String,
    pub custom_preprocess_code: String,
    pub custom_postprocess_code: String,
    pub custom_helper_methods: String,

    #[serde(default)]
    pub test_cases: Vec<super::test_suite::TestCase>,
}

impl Default for PhonemizerProject {
    fn default() -> Self {
        Self {
            name: "MyCustomPhonemizer".to_string(),
            tag: "CUSTOM".to_string(),
            author: "Author".to_string(),
            version: "1.0.0".to_string(),
            language_code: "pt".to_string(),
            description: "Custom Phonemizer created with Phonemizer Studio".to_string(),
            namespace: "OpenUtau.Plugin.Builtin".to_string(),
            base_class_type: PhonemeSystemType::SyllableBased,
            openutau_plugins_dir: default_openutau_plugins_path(),
            output_dll_name: "MyCustomPhonemizer.dll".to_string(),
            vowels: Vec::new(),
            consonants: Vec::new(),
            onset_clusters: Vec::new(),
            coda_clusters: Vec::new(),
            pattern_rules: Vec::new(),
            voice_color_and_pitch: VoiceColorAndPitchConfig::default(),
            syllabification_config: SyllabificationConfig::default(),
            dictionary_overrides: Vec::new(),
            regex_rules: Vec::new(),
            allow_fallback_cv: true,
            allow_fallback_vv: true,
            split_ratio_onset: 0.5,
            split_ratio_coda: 0.5,
            max_consonant_duration_ms: 120,
            raw_csharp_source: None,
            source_github_url: None,
            custom_usings: vec![
                "using System;".to_string(),
                "using System.Collections.Generic;".to_string(),
                "using System.Linq;".to_string(),
                "using OpenUtau.Api;".to_string(),
                "using OpenUtau.Core.Ustx;".to_string(),
            ],
            custom_class_fields: String::new(),
            custom_preprocess_code: String::new(),
            custom_postprocess_code: String::new(),
            custom_helper_methods: String::new(),
            test_cases: Vec::new(),
        }
    }
}

pub fn default_openutau_plugins_path() -> String {
    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            let path = format!("{}/Library/Application Support/OpenUtau/Plugins", home);
            if std::path::Path::new(&path).exists() {
                return path;
            }
            return format!("{}/OpenUtau/Plugins", home);
        }
    }
    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            return format!("{}\\OpenUtau\\Plugins", appdata);
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(home) = std::env::var("HOME") {
            return format!("{}/.config/OpenUtau/Plugins", home);
        }
    }
    "./Plugins".to_string()
}
