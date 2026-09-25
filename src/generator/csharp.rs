use crate::models::*;

pub fn generate_csharp_code(project: &PhonemizerProject) -> String {
    let mut code = String::with_capacity(16384);

    code.push_str("// ==========================================================================\n");
    code.push_str(&format!("// OpenUtau Phonemizer: {}\n", project.name));
    code.push_str(&format!("// Author: {} | Version: {} | Tag: [{}]\n", project.author, project.version, project.tag));
    code.push_str(&format!("// Language: {} | Base Architecture: {:?}\n", project.language_code, project.base_class_type));
    code.push_str("// Generated automatically by Phonemizer Studio (Rust)\n");
    code.push_str("// ==========================================================================\n\n");

    let mut all_usings = std::collections::BTreeSet::new();
    for u in &project.custom_usings {
        let trimmed = u.trim();
        if !trimmed.is_empty() {
            all_usings.insert(trimmed.to_string());
        }
    }
    all_usings.insert("using System;".to_string());
    all_usings.insert("using System.Collections.Generic;".to_string());
    all_usings.insert("using System.Linq;".to_string());
    all_usings.insert("using System.Text.RegularExpressions;".to_string());
    all_usings.insert("using OpenUtau.Api;".to_string());
    all_usings.insert("using OpenUtau.Core.Ustx;".to_string());

    for u in all_usings {
        code.push_str(&u);
        code.push('\n');
    }
    code.push('\n');

    code.push_str(&format!("namespace {}\n{{\n", project.namespace));

    code.push_str(&format!(
        " [Phonemizer(\"{}\", \"{}\", \"{}\", language: \"{}\")]\n",
        project.name, project.tag, project.author, project.language_code
    ));

    if project.base_class_type == PhonemeSystemType::RawCSharpDirect {
        if let Some(ref raw_src) = project.raw_csharp_source {
            return raw_src.clone();
        }
    }

    let base_class_name = match project.base_class_type {
        PhonemeSystemType::SyllableBased => "SyllableBasedPhonemizer",
        PhonemeSystemType::JapaneseVCV => "Phonemizer",
        PhonemeSystemType::JapaneseCVVC => "Phonemizer",
        PhonemeSystemType::JapanesePresamp => "Phonemizer",
        PhonemeSystemType::Arpasing => "Phonemizer",
        PhonemeSystemType::RussianCVC => "Phonemizer",
        PhonemeSystemType::FrenchSyllable => "SyllableBasedPhonemizer",
        PhonemeSystemType::KoreanHangul => "SyllableBasedPhonemizer",
        PhonemeSystemType::ChineseCVV => "Phonemizer",
        PhonemeSystemType::CustomDirect => "Phonemizer",
        PhonemeSystemType::RawCSharpDirect => "Phonemizer",
    };

    code.push_str(&format!(" public class {} : {}\n {{\n", project.name, base_class_name));

    code.push_str(" // --- Vowels and Diphthongs ---\n");
    let vowels_list: Vec<String> = project.vowels.iter().map(|v| format!("\"{}\"", v.symbol)).collect();
    code.push_str(&format!(" private static readonly string[] PlainVowels = new string[] {{ {} }};\n", vowels_list.join(", ")));

    let nasal_vowels: Vec<String> = project.vowels.iter().filter(|v| v.is_nasal).map(|v| format!("\"{}\"", v.symbol)).collect();
    code.push_str(&format!(" private static readonly HashSet<string> NasalVowels = new HashSet<string> {{ {} }};\n", nasal_vowels.join(", ")));

    let diphthongs: Vec<String> = project.vowels.iter().filter(|v| v.is_diphthong).map(|v| format!("\"{}\"", v.symbol)).collect();
    code.push_str(&format!(" private static readonly HashSet<string> Diphthongs = new HashSet<string> {{ {} }};\n\n", diphthongs.join(", ")));

    code.push_str(" // --- Consonants and Clusters ---\n");
    let consonants_list: Vec<String> = project.consonants.iter().map(|c| format!("\"{}\"", c.symbol)).collect();
    code.push_str(&format!(" private static readonly string[] PlainConsonants = new string[] {{ {} }};\n", consonants_list.join(", ")));

    let coda_consonants: Vec<String> = project.consonants.iter().filter(|c| c.can_be_coda).map(|c| format!("\"{}\"", c.symbol)).collect();
    code.push_str(&format!(" private static readonly HashSet<string> CodaConsonants = new HashSet<string> {{ {} }};\n\n", coda_consonants.join(", ")));

    code.push_str(" private static readonly Dictionary<string, string[]> OnsetClusters = new Dictionary<string, string[]> {\n");
    for cluster in &project.onset_clusters {
        let parts: Vec<String> = cluster.components.iter().map(|c| format!("\"{}\"", c)).collect();
        code.push_str(&format!(" {{ \"{}\", new string[] {{ {} }} }},\n", cluster.cluster, parts.join(", ")));
    }
    code.push_str(" };\n\n");

    code.push_str(" private static readonly Dictionary<string, string[]> CodaClusters = new Dictionary<string, string[]> {\n");
    for cluster in &project.coda_clusters {
        let parts: Vec<String> = cluster.components.iter().map(|c| format!("\"{}\"", c)).collect();
        code.push_str(&format!(" {{ \"{}\", new string[] {{ {} }} }},\n", cluster.cluster, parts.join(", ")));
    }
    code.push_str(" };\n\n");

    code.push_str(" // --- Dictionary Word Overrides ---\n");
    code.push_str(" private static readonly Dictionary<string, string[]> DictionaryOverrides = new Dictionary<string, string[]>(StringComparer.OrdinalIgnoreCase) {\n");
    for entry in &project.dictionary_overrides {
        let phs: Vec<String> = entry.phonemes.iter().map(|p| format!("\"{}\"", p)).collect();
        code.push_str(&format!(" {{ \"{}\", new string[] {{ {} }} }},\n", entry.word, phs.join(", ")));
    }
    code.push_str(" };\n\n");

    code.push_str(" // --- Regex Rules ---\n");
    code.push_str(" private static readonly (Regex regex, string replacement)[] RegexTransformations = new (Regex, string)[] {\n");
    for rule in &project.regex_rules {
        if rule.enabled {
            let escaped_pat = rule.pattern.replace('\\', "\\\\").replace('"', "\\\"");
            code.push_str(&format!(" (new Regex(\"{}\", RegexOptions.Compiled), \"{}\"),\n", escaped_pat, rule.replacement));
        }
    }
    code.push_str(" };\n\n");

    if !project.custom_class_fields.trim().is_empty() {
        code.push_str(" // --- Custom Fields ---\n");
        code.push_str(&project.custom_class_fields);
        code.push_str("\n\n");
    }

    code.push_str(r#" private USinger? _singer;

        public override void SetSinger(USinger singer)
        {
            _singer = singer;
        }

        /// Checks if an alias exists in singer oto with tone and color fallback
        protected bool HasOto(string alias, string tone = "", string color = "")
        {
            if (_singer == null) return false;
            string testAlias = alias;
            if (!string.IsNullOrEmpty(color))
            {
                if (_singer.TryGetOto(testAlias + "_" + color, out _)) return true;
            }
            if (!string.IsNullOrEmpty(tone))
            {
                if (_singer.TryGetOto(testAlias + tone, out _)) return true;
            }
            return _singer.TryGetOto(testAlias, out _);
        }

        /// Applies regex rules and dictionary overrides to normalize input lyric
        public string[] NormalizeLyricToPhonemes(string lyric)
        {
            if (string.IsNullOrWhiteSpace(lyric)) return Array.Empty<string>();

            // Check dictionary override
            if (DictionaryOverrides.TryGetValue(lyric.Trim(), out var phonemes))
            {
                return phonemes;
            }

            // Apply regex transforms
            string processed = lyric.Trim();
            foreach (var (regex, rep) in RegexTransformations)
            {
                processed = regex.Replace(processed, rep);
            }

            return processed.Split(new[] { ' ' }, StringSplitOptions.RemoveEmptyEntries);
        }
"#);

    code.push_str("\n // --- Core Process Implementation ---\n");
    code.push_str(r#" public override Result Process(Note[] notes, Note? prev, Note? next, Note? prevNeighbour, Note? nextNeighbour, Note[] prevs)
        {
            var note = notes[0];
            string lyric = note.lyric.Trim();
            string tone = note.toneName ?? "";
            string color = note.phoneticHint ?? "";

            // Custom Pre-Process Hook
"#);

    if !project.custom_preprocess_code.trim().is_empty() {
        code.push_str(" // --- Injected PreProcess Code ---\n");
        code.push_str(&project.custom_preprocess_code);
        code.push('\n');
    }

    code.push_str(r#"
            // If lyric starts with '?' or '-' or other direct prefix, pass as direct phoneme
            if (lyric.StartsWith("?") || lyric.StartsWith("."))
            {
                return new Result
                {
                    phonemes = new Phoneme[]
                    {
                        new Phoneme { phoneme = lyric.TrimStart('?', '.') }
                    }
                };
            }

            var phonemeList = NormalizeLyricToPhonemes(lyric);
            if (phonemeList.Length == 0)
            {
                phonemeList = new string[] { lyric };
            }

            var results = new List<Phoneme>();
            string prevVowel = "";

            if (prev.HasValue && !string.IsNullOrEmpty(prev.Value.lyric))
            {
                var prevPhs = NormalizeLyricToPhonemes(prev.Value.lyric);
                for (int i = prevPhs.Length - 1; i >= 0; i--)
                {
                    if (PlainVowels.Contains(prevPhs[i]))
                    {
                        prevVowel = prevPhs[i];
                        break;
                    }
                }
            }

            // Determine if current note is start of phrase or following a rest
            bool isRestingStart = prev == null || string.IsNullOrWhiteSpace(prev.Value.lyric) || prev.Value.lyric == "R" || prev.Value.lyric == "pau" || prev.Value.lyric == "AP" || prev.Value.lyric == "SP";

            // Process single or compound phonemes
            if (phonemeList.Length == 1)
            {
                string ph = phonemeList[0];
                string candidate = ph;

                if (isRestingStart)
                {
                    string restAlias = "- " + ph;
                    if (HasOto(restAlias, tone, color))
                    {
                        candidate = restAlias;
                    }
                }
                else if (!string.IsNullOrEmpty(prevVowel))
                {
                    string vcvAlias = prevVowel + " " + ph;
                    if (HasOto(vcvAlias, tone, color))
                    {
                        candidate = vcvAlias;
                    }
                }

                results.Add(new Phoneme { phoneme = candidate });
            }
            else
            {
                // Multi-phoneme lyric (e.g. onset consonant + vowel)
                for (int i = 0; i < phonemeList.Length; i++)
                {
                    string ph = phonemeList[i];
                    string candidate = ph;

                    if (i == 0 && isRestingStart)
                    {
                        string restAlias = "- " + ph;
                        if (HasOto(restAlias, tone, color))
                        {
                            candidate = restAlias;
                        }
                    }
                    else if (i == 0 && !string.IsNullOrEmpty(prevVowel))
                    {
                        string vcvAlias = prevVowel + " " + ph;
                        if (HasOto(vcvAlias, tone, color))
                        {
                            candidate = vcvAlias;
                        }
                    }

                    results.Add(new Phoneme { phoneme = candidate });
                }
            }
"#);

    if !project.custom_postprocess_code.trim().is_empty() {
        code.push_str(" // --- Injected PostProcess Code ---\n");
        code.push_str(&project.custom_postprocess_code);
        code.push('\n');
    }

    code.push_str(r#"
            return new Result
            {
                phonemes = results.ToArray()
            };
        }
"#);

    if !project.custom_helper_methods.trim().is_empty() {
        code.push_str("\n // --- Custom Helper Methods ---\n");
        code.push_str(&project.custom_helper_methods);
        code.push('\n');
    }

    code.push_str(" }\n}\n");

    code
}
