use crate::models::PhonemizerProject;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct SimulationNoteInput {
    pub lyric: String,
    pub tone: String,
    pub color: String,
}

#[derive(Debug, Clone)]
pub struct SimulatedPhonemeOutput {
    pub raw_phoneme: String,
    pub final_alias: String,
    pub rule_applied: String,
}

#[derive(Debug, Clone)]
pub struct NoteSimulationResult {
    pub input_lyric: String,
    pub normalized_phonemes: Vec<String>,
    pub generated_phonemes: Vec<SimulatedPhonemeOutput>,
}

#[derive(Debug, Clone)]
pub struct SyllableUnit {
    pub onset: Vec<String>,
    pub vowel: String,
    pub coda: Vec<String>,
}

pub struct PhonemizerSimulator<'a> {
    project: &'a PhonemizerProject,
}

impl<'a> PhonemizerSimulator<'a> {
    pub fn new(project: &'a PhonemizerProject) -> Self {
        Self { project }
    }

    /// Simulates a plain text phrase composed of multiple words/notes
    pub fn simulate_phrase(&self, phrase: &str) -> Vec<NoteSimulationResult> {
        let words: Vec<&str> = phrase.split_whitespace().collect();
        let notes: Vec<SimulationNoteInput> = words
            .into_iter()
            .map(|w| SimulationNoteInput {
                lyric: w.to_string(),
                tone: String::new(),
                color: String::new(),
            })
            .collect();
        self.simulate_sequence(&notes)
    }

    /// Strips accents from a string for fuzzy dictionary lookup
    fn strip_diacritics(s: &str) -> String {
        s.chars()
            .map(|c| match c {
                'á' | 'à' | 'ã' | 'â' | 'ä' => 'a',
                'é' | 'è' | 'ê' | 'ë' => 'e',
                'í' | 'ì' | 'î' | 'ï' => 'i',
                'ó' | 'ò' | 'õ' | 'ô' | 'ö' => 'o',
                'ú' | 'ù' | 'û' | 'ü' => 'u',
                'ç' => 'c',
                'Á' | 'À' | 'Ã' | 'Â' | 'Ä' => 'A',
                'É' | 'È' | 'Ê' | 'Ë' => 'E',
                'Í' | 'Ì' | 'Î' | 'Ï' => 'I',
                'Ó' | 'Ò' | 'Õ' | 'Ô' | 'Ö' => 'O',
                'Ú' | 'Ù' | 'Û' | 'Ü' => 'U',
                'Ç' => 'C',
                _ => c,
            })
            .collect()
    }

    /// Normalizes a lyric using dictionary overrides, regex rules, and automatic syllabification
    pub fn normalize_lyric(&self, lyric: &str) -> Vec<String> {
        let trimmed = lyric.trim();
        if trimmed.is_empty() {
            return Vec::new();
        }

        // 1. Direct dictionary override check (exact and diacritic-insensitive)
        let trimmed_clean = Self::strip_diacritics(trimmed).to_lowercase();
        for dict in &self.project.dictionary_overrides {
            if dict.is_case_sensitive {
                if dict.word == trimmed {
                    return dict.phonemes.clone();
                }
            } else {
                if dict.word.eq_ignore_ascii_case(trimmed) {
                    return dict.phonemes.clone();
                }
                let dict_clean = Self::strip_diacritics(&dict.word).to_lowercase();
                if dict_clean == trimmed_clean {
                    return dict.phonemes.clone();
                }
            }
        }

        // 2. Apply regex rules
        let mut processed = trimmed.to_string();
        for rule in &self.project.regex_rules {
            if rule.enabled {
                if let Ok(re) = Regex::new(&rule.pattern) {
                    processed = re.replace_all(&processed, &rule.replacement).to_string();
                }
            }
        }

        // If regex produced space-separated phonemes, return them directly
        if processed.contains(' ') {
            return processed.split_whitespace().map(|s| s.to_string()).collect();
        }

        // 3. Check if processed word is directly in dictionary after regex
        let proc_clean = Self::strip_diacritics(&processed).to_lowercase();
        for dict in &self.project.dictionary_overrides {
            let dict_clean = Self::strip_diacritics(&dict.word).to_lowercase();
            if dict_clean == proc_clean {
                return dict.phonemes.clone();
            }
        }

        // 4. Fallback: Automatic Syllable & Phoneme Decomposition
        self.decompose_word(&processed)
    }

    /// Decomposes an unknown word into phonemes based on known vowels, consonants and digraphs
    fn decompose_word(&self, word: &str) -> Vec<String> {
        let lower = word.to_lowercase();
        let mut result = Vec::new();
        let chars: Vec<char> = lower.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            // Check 3-char clusters / phonemes
            if i + 3 <= chars.len() {
                let sub3: String = chars[i..i + 3].iter().collect();
                if self.is_known_phoneme(&sub3) {
                    result.push(sub3);
                    i += 3;
                    continue;
                }
            }

            // Check 2-char clusters / phonemes (e.g. "nh", "lh", "ch", "rr", "ão", "õe")
            if i + 2 <= chars.len() {
                let sub2: String = chars[i..i + 2].iter().collect();
                if self.is_known_phoneme(&sub2) {
                    result.push(sub2);
                    i += 2;
                    continue;
                }
                match sub2.as_str() {
                    "ch" => { result.push("ch".to_string()); i += 2; continue; }
                    "lh" => { result.push("lh".to_string()); i += 2; continue; }
                    "nh" => { result.push("nh".to_string()); i += 2; continue; }
                    "rr" => { result.push("rr".to_string()); i += 2; continue; }
                    "ss" => { result.push("s".to_string()); i += 2; continue; }
                    "ão" | "ao" => { result.push("an".to_string()); result.push("u".to_string()); i += 2; continue; }
                    "õe" | "oe" => { result.push("on".to_string()); result.push("i".to_string()); i += 2; continue; }
                    "am" => { result.push("an".to_string()); i += 2; continue; }
                    "em" => { result.push("en".to_string()); result.push("i".to_string()); i += 2; continue; }
                    "im" => { result.push("in".to_string()); i += 2; continue; }
                    "om" => { result.push("on".to_string()); i += 2; continue; }
                    "um" => { result.push("un".to_string()); i += 2; continue; }
                    _ => {}
                }
            }

            // Check 1-char phoneme
            let ch = chars[i];
            let s1 = ch.to_string();
            if self.is_known_phoneme(&s1) {
                result.push(s1);
            } else {
                // Map special and accented characters
                match ch {
                    'á' | 'à' | 'â' | 'ä' => result.push("a".to_string()),
                    'ã' => result.push("an".to_string()),
                    'é' => result.push("eh".to_string()),
                    'ê' | 'ë' => result.push("e".to_string()),
                    'í' | 'ì' | 'î' | 'ï' => result.push("i".to_string()),
                    'ó' => result.push("oh".to_string()),
                    'ô' | 'ö' => result.push("o".to_string()),
                    'õ' => result.push("on".to_string()),
                    'ú' | 'ù' | 'û' | 'ü' => result.push("u".to_string()),
                    'ç' => result.push("s".to_string()),
                    'c' => {
                        if i + 1 < chars.len() && matches!(chars[i + 1], 'e' | 'i' | 'é' | 'ê' | 'í') {
                            result.push("s".to_string());
                        } else {
                            result.push("k".to_string());
                        }
                    }
                    'g' => {
                        if i + 1 < chars.len() && matches!(chars[i + 1], 'e' | 'i' | 'é' | 'ê' | 'í') {
                            result.push("j".to_string());
                        } else {
                            result.push("g".to_string());
                        }
                    }
                    'x' => result.push("sh".to_string()),
                    'z' => result.push("z".to_string()),
                    _ => result.push(s1),
                }
            }
            i += 1;
        }

        if result.is_empty() {
            vec![word.to_string()]
        } else {
            result
        }
    }

    fn is_known_phoneme(&self, sym: &str) -> bool {
        self.project.vowels.iter().any(|v| v.symbol == sym || v.aliases.iter().any(|a| a == sym))
            || self.project.consonants.iter().any(|c| c.symbol == sym || c.aliases.iter().any(|a| a == sym))
            || self.project.onset_clusters.iter().any(|cl| cl.cluster == sym)
    }

    fn is_vowel(&self, sym: &str) -> bool {
        self.project.vowels.iter().any(|v| v.symbol == sym || v.aliases.iter().any(|a| a == sym))
            || [
                "a", "e", "i", "o", "u", "an", "en", "in", "on", "un",
                "eh", "oh", "ah", "ax", "ae", "oa", "aen", "ehn", "ohn",
                "i0", "u0", "ai", "ei", "oi", "au", "eu", "ou", "ui",
                "aon", "oen", "3", "0", "6", "E", "O", "I", "U", "Y", "N", "M", "y"
            ].contains(&sym)
    }

    fn is_onset_cluster(&self, c1: &str, c2: &str) -> bool {
        let combined = format!("{}{}", c1, c2);
        self.project.onset_clusters.iter().any(|cl| {
            cl.cluster == combined || (cl.components.len() == 2 && cl.components[0] == c1 && cl.components[1] == c2)
        }) || [
            "br", "bl", "cr", "cl", "dr", "fr", "fl", "gr", "gl",
            "pr", "pl", "tr", "tl", "vr", "vl", "kr", "kl"
        ].contains(&combined.as_str())
    }

    /// Splits a list of phonemes for a single word/note into standard phonetic syllables (OpenUtau SyllableBased architecture)
    pub fn split_into_syllables(&self, phonemes: &[String]) -> Vec<SyllableUnit> {
        if phonemes.is_empty() {
            return Vec::new();
        }

        // Find indices of all vowels
        let mut vowel_indices = Vec::new();
        for (idx, ph) in phonemes.iter().enumerate() {
            if self.is_vowel(ph) {
                vowel_indices.push(idx);
            }
        }

        // If no vowels found (pure consonants or pauses), return as a single unit
        if vowel_indices.is_empty() {
            return vec![SyllableUnit {
                onset: phonemes.to_vec(),
                vowel: String::new(),
                coda: Vec::new(),
            }];
        }

        let mut syllables = Vec::new();
        let num_vowels = vowel_indices.len();

        for (v_idx, &v_pos) in vowel_indices.iter().enumerate() {
            let vowel = phonemes[v_pos].clone();

            if v_idx == 0 {
                // First syllable: onset is all phonemes before first vowel
                let onset = phonemes[0..v_pos].to_vec();
                let coda = if num_vowels == 1 {
                    phonemes[v_pos + 1..].to_vec()
                } else {
                    Vec::new()
                };

                syllables.push(SyllableUnit { onset, vowel, coda });
            } else {
                let prev_v_pos = vowel_indices[v_idx - 1];
                let between = &phonemes[prev_v_pos + 1..v_pos];

                let (coda_prev, onset_curr) = match between.len() {
                    0 => (Vec::new(), Vec::new()), // VV (diphthong or adjacent vowels)
                    1 => (Vec::new(), vec![between[0].clone()]), // Single consonant goes to onset (Maximal Onset)
                    2 => {
                        if self.is_onset_cluster(&between[0], &between[1]) {
                            (Vec::new(), vec![between[0].clone(), between[1].clone()])
                        } else {
                            (vec![between[0].clone()], vec![between[1].clone()])
                        }
                    }
                    _ => {
                        // 3+ consonants: check if last two form an onset cluster
                        let last2_start = between.len() - 2;
                        if self.is_onset_cluster(&between[last2_start], &between[last2_start + 1]) {
                            (between[0..last2_start].to_vec(), between[last2_start..].to_vec())
                        } else {
                            (between[0..between.len() - 1].to_vec(), vec![between.last().unwrap().clone()])
                        }
                    }
                };

                // Attach coda to previous syllable if needed
                if !coda_prev.is_empty() {
                    if let Some(prev_syl) = syllables.last_mut() {
                        prev_syl.coda.extend(coda_prev);
                    }
                }

                let coda_curr = if v_idx == num_vowels - 1 {
                    // Trailing consonants after the last vowel
                    phonemes[v_pos + 1..].to_vec()
                } else {
                    Vec::new()
                };

                syllables.push(SyllableUnit {
                    onset: onset_curr,
                    vowel,
                    coda: coda_curr,
                });
            }
        }

        syllables
    }

    /// Simulates a sequence of musical notes into OpenUtau transitions with 100% fidelity to SyllableBasedPhonemizer
    pub fn simulate_sequence(&self, notes: &[SimulationNoteInput]) -> Vec<NoteSimulationResult> {
        let mut results = Vec::new();
        let mut prev_vowel: Option<String> = None;

        for note in notes {
            let normalized = self.normalize_lyric(&note.lyric);
            let mut generated = Vec::new();

            if note.lyric == "R" || note.lyric == "pau" || note.lyric == "AP" || note.lyric == "SP" {
                prev_vowel = None;
                results.push(NoteSimulationResult {
                    input_lyric: note.lyric.clone(),
                    normalized_phonemes: vec!["R".to_string()],
                    generated_phonemes: vec![SimulatedPhonemeOutput {
                        raw_phoneme: "R".to_string(),
                        final_alias: "R".to_string(),
                        rule_applied: "Rest Pause".to_string(),
                    }],
                });
                continue;
            }

            let syllables = self.split_into_syllables(&normalized);

            for syl in &syllables {
                if syl.vowel.is_empty() {
                    // Pure consonant note
                    for c in &syl.onset {
                        let alias = format!("- {}", c);
                        generated.push(SimulatedPhonemeOutput {
                            raw_phoneme: c.clone(),
                            final_alias: self.apply_attributes(&alias, note),
                            rule_applied: "Consonant Solo (- C)".to_string(),
                        });
                    }
                    continue;
                }

                // 1. Starting vs Transition Syllable
                match &prev_vowel {
                    None => {
                        // Starting Syllable (- V or - C + CV)
                        if syl.onset.is_empty() {
                            // Starting Vowel: "- v"
                            let alias = format!("- {}", syl.vowel);
                            generated.push(SimulatedPhonemeOutput {
                                raw_phoneme: syl.vowel.clone(),
                                final_alias: self.apply_attributes(&alias, note),
                                rule_applied: "Resting Start (- V)".to_string(),
                            });
                        } else {
                            // Resting Consonant: "- c"
                            let first_c = &syl.onset[0];
                            let start_c_alias = format!("- {}", first_c);
                            generated.push(SimulatedPhonemeOutput {
                                raw_phoneme: first_c.clone(),
                                final_alias: self.apply_attributes(&start_c_alias, note),
                                rule_applied: "Resting Consonant (- C)".to_string(),
                            });

                            // Onset clusters if any: "c1 c2"
                            for c_idx in 1..syl.onset.len() {
                                let c = &syl.onset[c_idx];
                                let cc_alias = format!("{} {}", syl.onset[c_idx - 1], c);
                                generated.push(SimulatedPhonemeOutput {
                                    raw_phoneme: c.clone(),
                                    final_alias: self.apply_attributes(&cc_alias, note),
                                    rule_applied: "Onset Cluster (C C)".to_string(),
                                });
                            }

                            // CV Syllable: "c v"
                            let last_c = syl.onset.last().unwrap();
                            let cv_alias = format!("{} {}", last_c, syl.vowel);
                            generated.push(SimulatedPhonemeOutput {
                                raw_phoneme: syl.vowel.clone(),
                                final_alias: self.apply_attributes(&cv_alias, note),
                                rule_applied: "CV Syllable (C V)".to_string(),
                            });
                        }
                    }
                    Some(pv) => {
                        // Middle Transition Syllable (VV or VC + CV)
                        if syl.onset.is_empty() {
                            // VV Transition: "prev_v v"
                            let vv_alias = format!("{} {}", pv, syl.vowel);
                            generated.push(SimulatedPhonemeOutput {
                                raw_phoneme: syl.vowel.clone(),
                                final_alias: self.apply_attributes(&vv_alias, note),
                                rule_applied: format!("VV Transition ({} {})", pv, syl.vowel),
                            });
                        } else {
                            // VC Transition from previous vowel: "prev_v c"
                            let first_c = &syl.onset[0];
                            let vc_alias = format!("{} {}", pv, first_c);
                            generated.push(SimulatedPhonemeOutput {
                                raw_phoneme: first_c.clone(),
                                final_alias: self.apply_attributes(&vc_alias, note),
                                rule_applied: format!("VC Transition ({} {})", pv, first_c),
                            });

                            // Consonant cluster if any
                            for c_idx in 1..syl.onset.len() {
                                let c = &syl.onset[c_idx];
                                let cc_alias = format!("{} {}", syl.onset[c_idx - 1], c);
                                generated.push(SimulatedPhonemeOutput {
                                    raw_phoneme: c.clone(),
                                    final_alias: self.apply_attributes(&cc_alias, note),
                                    rule_applied: "Consonant Cluster (C C)".to_string(),
                                });
                            }

                            // CV Syllable: "c v"
                            let last_c = syl.onset.last().unwrap();
                            let cv_alias = format!("{} {}", last_c, syl.vowel);
                            generated.push(SimulatedPhonemeOutput {
                                raw_phoneme: syl.vowel.clone(),
                                final_alias: self.apply_attributes(&cv_alias, note),
                                rule_applied: "CV Syllable (C V)".to_string(),
                            });
                        }
                    }
                }

                // 2. Process coda consonants (VC-, C-)
                for coda in &syl.coda {
                    let coda_alias = format!("{} {}-", syl.vowel, coda);
                    generated.push(SimulatedPhonemeOutput {
                        raw_phoneme: coda.clone(),
                        final_alias: self.apply_attributes(&coda_alias, note),
                        rule_applied: format!("Coda Ending ({} {}-)", syl.vowel, coda),
                    });
                }

                prev_vowel = Some(syl.vowel.clone());
            }

            results.push(NoteSimulationResult {
                input_lyric: note.lyric.clone(),
                normalized_phonemes: normalized,
                generated_phonemes: generated,
            });
        }

        results
    }

    fn apply_attributes(&self, alias: &str, note: &SimulationNoteInput) -> String {
        let mut result = alias.to_string();
        if !note.color.is_empty() {
            result = format!("{}_{}", result, note.color);
        }
        if !note.tone.is_empty() && self.project.voice_color_and_pitch.use_pitch_suffixes {
            result = format!("{}{}", result, note.tone);
        }
        result
    }
}
