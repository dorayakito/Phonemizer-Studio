use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Vowel {
    pub symbol: String,
    pub ipa: String,
    pub aliases: Vec<String>,
    pub is_nasal: bool,
    pub is_diphthong: bool,
    pub is_triphthong: bool,
    pub second_vowel: Option<String>,
    pub third_vowel: Option<String>,
    pub diphthong_split_ratio: f32,
    pub end_glide: Option<String>,
    pub glottal_start_alias: Option<String>,
    pub elongation_symbol: Option<String>,
    pub stress_variants: Vec<String>,
    pub color_overrides: Vec<String>,
}

impl Vowel {
    pub fn new(symbol: impl Into<String>) -> Self {
        let sym = symbol.into();
        Self {
            symbol: sym.clone(),
            ipa: sym,
            aliases: Vec::new(),
            is_nasal: false,
            is_diphthong: false,
            is_triphthong: false,
            second_vowel: None,
            third_vowel: None,
            diphthong_split_ratio: 0.65,
            end_glide: None,
            glottal_start_alias: None,
            elongation_symbol: None,
            stress_variants: Vec::new(),
            color_overrides: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Consonant {
    pub symbol: String,
    pub ipa: String,
    pub aliases: Vec<String>,
    pub consonant_type: ConsonantType,
    pub place_of_articulation: PlaceOfArticulation,
    pub is_voiced: bool,
    pub is_aspirated: bool,
    pub is_syllabic: bool,
    pub can_be_onset: bool,
    pub can_be_coda: bool,
    pub can_be_intervocalic: bool,
    pub timing_ratio: f32,
    pub default_offset_ms: i32,
    pub pre_utterance_multiplier: f32,
    pub silence_gap_before_ms: i32,
}

impl Consonant {
    pub fn new(symbol: impl Into<String>, ctype: ConsonantType) -> Self {
        let sym = symbol.into();
        let is_voiced = matches!(ctype, ConsonantType::Nasal | ConsonantType::Liquid | ConsonantType::Semivowel);
        Self {
            symbol: sym.clone(),
            ipa: sym,
            aliases: Vec::new(),
            consonant_type: ctype,
            place_of_articulation: PlaceOfArticulation::Alveolar,
            is_voiced,
            is_aspirated: false,
            is_syllabic: false,
            can_be_onset: true,
            can_be_coda: false,
            can_be_intervocalic: true,
            timing_ratio: 1.0,
            default_offset_ms: 0,
            pre_utterance_multiplier: 1.0,
            silence_gap_before_ms: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsonantType {
    Stop,
    Fricative,
    Nasal,
    Liquid,
    Affricate,
    Semivowel,
    Glottal,
    ClickOrEjective,
    Special,
}

impl std::fmt::Display for ConsonantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConsonantType::Stop => write!(f, "Stop (Oclusiva)"),
            ConsonantType::Fricative => write!(f, "Fricative (Fricativa)"),
            ConsonantType::Nasal => write!(f, "Nasal (Nasal)"),
            ConsonantType::Liquid => write!(f, "Liquid (Líquida/Vibrante)"),
            ConsonantType::Affricate => write!(f, "Affricate (Africada)"),
            ConsonantType::Semivowel => write!(f, "Semivowel (Semivogal/Glide)"),
            ConsonantType::Glottal => write!(f, "Glottal (Oclusiva/Fricativa Glotal)"),
            ConsonantType::ClickOrEjective => write!(f, "Click / Ejetiva"),
            ConsonantType::Special => write!(f, "Special (Respiração/Pausa/Especial)"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlaceOfArticulation {
    Bilabial, // p, b, m, w
    Labiodental, // f, v
    Dental, // th, dh, d, t
    Alveolar, // t, d, s, z, n, l, r
    Postalveolar, // sh, zh, ch, dj
    Retroflex, // ʈ, ɖ, ʂ, ʐ, ɻ
    Palatal, // c, ɟ, ɲ, j, lh, nh
    Velar, // k, g, ng
    Uvular, // q, ɢ, ɴ, ʁ, χ
    Pharyngeal, // ħ, ʕ
    Glottal, // ʔ, h, ɦ
}

impl std::fmt::Display for PlaceOfArticulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaceOfArticulation::Bilabial => write!(f, "Bilabial (p, b, m)"),
            PlaceOfArticulation::Labiodental => write!(f, "Labiodental (f, v)"),
            PlaceOfArticulation::Dental => write!(f, "Dental (th, dh)"),
            PlaceOfArticulation::Alveolar => write!(f, "Alveolar (t, d, s, z, n, l, r)"),
            PlaceOfArticulation::Postalveolar => write!(f, "Postalveolar (sh, zh, ch, j)"),
            PlaceOfArticulation::Retroflex => write!(f, "Retroflexo (ʈ, ɖ, ɻ)"),
            PlaceOfArticulation::Palatal => write!(f, "Palatal (nh, lh, j, ɲ)"),
            PlaceOfArticulation::Velar => write!(f, "Velar (k, g, ng)"),
            PlaceOfArticulation::Uvular => write!(f, "Uvular (q, ʁ, χ)"),
            PlaceOfArticulation::Pharyngeal => write!(f, "Faríngeo (ħ, ʕ)"),
            PlaceOfArticulation::Glottal => write!(f, "Glotal (?, h)"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConsonantCluster {
    pub cluster: String,
    pub components: Vec<String>,
    pub is_onset: bool,
    pub split_timing_percentages: Vec<f32>,
    pub bpm_shortening_threshold: i32,
    pub allow_elision_if_fast: bool,
    pub custom_oto_alias: Option<String>,
}

impl ConsonantCluster {
    pub fn new(cluster: impl Into<String>, components: Vec<String>, is_onset: bool) -> Self {
        let count = components.len().max(1);
        let equal_pct = 100.0 / count as f32;
        let split_timing_percentages = vec![equal_pct; count];
        Self {
            cluster: cluster.into(),
            components,
            is_onset,
            split_timing_percentages,
            bpm_shortening_threshold: 160,
            allow_elision_if_fast: false,
            custom_oto_alias: None,
        }
    }
}
