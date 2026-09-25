use crate::models::PhonemizerProject;
use std::collections::BTreeSet;

pub struct DiffSingerExporter;

impl DiffSingerExporter {
    pub fn export_phonemes_txt(project: &PhonemizerProject) -> String {
        let mut set = BTreeSet::new();
        set.insert("AP".to_string());
        set.insert("SP".to_string());
        set.insert("pau".to_string());
        set.insert("br".to_string());

        for v in &project.vowels {
            if !v.symbol.trim().is_empty() {
                set.insert(v.symbol.trim().to_string());
            }
        }

        for c in &project.consonants {
            if !c.symbol.trim().is_empty() {
                set.insert(c.symbol.trim().to_string());
            }
        }

        set.into_iter().collect::<Vec<String>>().join("\n") + "\n"
    }

    pub fn export_lexicon_txt(project: &PhonemizerProject) -> String {
        let mut lines = Vec::new();

        for d in &project.dictionary_overrides {
            let ph_str = d.phonemes.join(" ");
            lines.push(format!("{}\t{}", d.word, ph_str));
        }

        for v in &project.vowels {
            lines.push(format!("{}\t{}", v.symbol, v.symbol));
        }

        lines.sort();
        lines.dedup();
        lines.join("\n") + "\n"
    }

    pub fn export_dsdict_yaml(project: &PhonemizerProject) -> String {
        let mut out = String::new();
        out.push_str(&format!("# DiffSinger Dictionary for {}\n", project.name));
        out.push_str(&format!("language: \"{}\"\n", project.language_code));
        out.push_str("entries:\n");

        for d in &project.dictionary_overrides {
            out.push_str(&format!(" - word: \"{}\"\n", d.word));
            out.push_str(&format!(" phonemes: [{}]\n", d.phonemes.iter().map(|p| format!("\"{}\"", p)).collect::<Vec<_>>().join(", ")));
        }

        out
    }
}
