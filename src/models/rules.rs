use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PhonemeSystemType {
    SyllableBased,
    JapaneseVCV,
    JapaneseCVVC,
    JapanesePresamp,
    Arpasing,
    RussianCVC,
    FrenchSyllable,
    KoreanHangul,
    ChineseCVV,
    CustomDirect,
    RawCSharpDirect,
}

impl std::fmt::Display for PhonemeSystemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhonemeSystemType::SyllableBased => write!(f, "Syllable Based (Multilíngue: PT, ES, EN, FR, IT, etc.)"),
            PhonemeSystemType::JapaneseVCV => write!(f, "Japanese VCV (- a, a ka, i ka)"),
            PhonemeSystemType::JapaneseCVVC => write!(f, "Japanese CVVC (ka, a k, k a)"),
            PhonemeSystemType::JapanesePresamp => write!(f, "Japanese Presamp / Hiragana-Romaji"),
            PhonemeSystemType::Arpasing => write!(f, "Arpasing (Inglês com CMUdict / difonemas)"),
            PhonemeSystemType::RussianCVC => write!(f, "Russian / Slavic CVC (CVC & Palatalizadas)"),
            PhonemeSystemType::FrenchSyllable => write!(f, "French Syllable com Liaisons e Enchaînement"),
            PhonemeSystemType::KoreanHangul => write!(f, "Korean Hangul (, , )"),
            PhonemeSystemType::ChineseCVV => write!(f, "Chinese Mandarin (Pinyin / CVV)"),
            PhonemeSystemType::CustomDirect => write!(f, "Custom Low-Level OpenUtau.Api.Phonemizer"),
            PhonemeSystemType::RawCSharpDirect => write!(f, "Modo C# Puro Direto (Edição Livre de Código Fonte)"),
        }
    }
}

/// Rule for generating a specific phonetic alias pattern
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PhoneticPatternRule {
    pub pattern_type: PatternType,
    pub template: String, // e.g. "{prev_vowel} {consonant}{vowel}", "{vowel} {consonant}", "{consonant}{vowel}"
    pub enabled: bool,
    pub priority: i32,
    pub fallback_template: Option<String>,
    pub min_pitch_note: Option<String>, // Apply rule only above this note (e.g. "C4")
    pub max_pitch_note: Option<String>, // Apply rule only below this note (e.g. "G5")
    pub color_filter: Option<String>, // Apply rule only for specific Voice Color
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PatternType {
    RestingStartVowel, // "- a", "_a"
    RestingStartConsonant, // "- ka", "_ka", "- k"
    CV, // "ka", "te"
    VCV, // "a ka", "o te"
    VC, // "a k", "e s"
    VV, // "a i", "o u"
    CC, // "s k", "n d", "p l"
    CVC, // "k a t" (Single oto)
    EndingVowelGlide, // "a -", "a R", "a h", "a AP"
    EndingConsonantCoda, // "k -", "s -", "n R"
    GlottalStop, // "' a", "? a"
    IntervocalicFlap, // "a rh", "o d"
    CrossWordLiaison, // "s_a", "z_e" (linking consonants across words)
    BreathAndPause, // "br", "inhale", "pau", "AP", "SP"
    CustomPattern, // Custom user-defined pattern
}

impl std::fmt::Display for PatternType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternType::RestingStartVowel => write!(f, "Início c/ Vogal (- V / _V)"),
            PatternType::RestingStartConsonant => write!(f, "Início c/ Consoante (- CV / - C)"),
            PatternType::CV => write!(f, "Ataque Simples (CV)"),
            PatternType::VCV => write!(f, "Transição VCV (V CV)"),
            PatternType::VC => write!(f, "Transição Vogal-Consoante (VC)"),
            PatternType::VV => write!(f, "Transição Vogal-Vogal (VV / Ditongo)"),
            PatternType::CC => write!(f, "Encontro Consonantal (CC)"),
            PatternType::CVC => write!(f, "Tri-fone Direto (CVC)"),
            PatternType::EndingVowelGlide => write!(f, "Finalização Vocálica (V - / V R / V h)"),
            PatternType::EndingConsonantCoda => write!(f, "Finalização Consonantal (C - / C R)"),
            PatternType::GlottalStop => write!(f, "Oclusiva Glotal (' V / ? V)"),
            PatternType::IntervocalicFlap => write!(f, "Tepe Intervocálico (V ɾ V)"),
            PatternType::CrossWordLiaison => write!(f, "Liaison / Ligação entre Palavras (C_V)"),
            PatternType::BreathAndPause => write!(f, "Respiração e Pausas (br, AP, SP)"),
            PatternType::CustomPattern => write!(f, "Padrão Customizado"),
        }
    }
}

/// Pitch and Voice Color Configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoiceColorAndPitchConfig {
    pub use_pitch_suffixes: bool, // e.g. _C4, _G4
    pub pitch_suffixes: Vec<String>, // ["_C3", "_F3", "_A3", "_C4", "_E4", "_G4", "_B4", "_C5"]
    pub voice_colors: Vec<String>, // ["", "Power", "Soft", "Whisper", "Falsetto", "Clear"]
    pub fallback_to_default_color: bool, // fallback if color oto doesn't exist
    pub auto_color_prefix_suffix: bool,
    pub color_separator: String,
    pub custom_resampler_flags: String,
    pub pitch_shift_steps: Vec<i32>,
}

impl Default for VoiceColorAndPitchConfig {
    fn default() -> Self {
        Self {
            use_pitch_suffixes: false,
            pitch_suffixes: vec![
                "_C3".to_string(),
                "_F3".to_string(),
                "_A3".to_string(),
                "_C4".to_string(),
                "_E4".to_string(),
                "_G4".to_string(),
                "_C5".to_string(),
            ],
            voice_colors: vec![
                "".to_string(),
                "Power".to_string(),
                "Soft".to_string(),
                "Whisper".to_string(),
                "Falsetto".to_string(),
                "Clear".to_string(),
            ],
            fallback_to_default_color: true,
            auto_color_prefix_suffix: false,
            color_separator: "_".to_string(),
            custom_resampler_flags: String::new(),
            pitch_shift_steps: vec![0, 12, -12, 24, -24],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SyllabificationConfig {
    pub enable_cross_word_liaison: bool,
    pub word_boundary_rest_threshold_ticks: i32,
    pub auto_detect_diphthongs: bool,
    pub syllable_separator_chars: Vec<String>,
    pub allow_glottal_stop_insertion: bool,
}

impl Default for SyllabificationConfig {
    fn default() -> Self {
        Self {
            enable_cross_word_liaison: true,
            word_boundary_rest_threshold_ticks: 60,
            auto_detect_diphthongs: true,
            syllable_separator_chars: vec![
                ".".to_string(),
                "-".to_string(),
                "_".to_string(),
                " ".to_string(),
            ],
            allow_glottal_stop_insertion: true,
        }
    }
}
