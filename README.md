# OpenUtau Phonemizer Studio & C# Compiler (Rust)

A comprehensive, professional, native, and high-performance software built with **Rust** to design, configure, simulate, and compile complete **C# Phonemizers (.dll and .cs)** for [OpenUtau](https://github.com/openutau/OpenUtau).

---

## Key Features

1. **OpenUtau GitHub Hub & C# Importer**:
   - Connects to GitHub API and lists all official phonemizers from the OpenUtau repository (`OpenUtau.Plugin.Builtin`).
   - Imports C# source code from any GitHub repository, direct URL, or local `.cs` file.
   - **Parser and Reverse Engineering**: Automatically recognizes `[Phonemizer]` attributes, vowel and consonant lists, clusters, dictionaries, and regex rules, converting everything into an editable project.
   - **Raw C# Direct Mode**: Allows pasting and editing arbitrary C# code directly in the studio with immediate DLL compilation.

2. **Full Phonetic & Phonological Editor**:
   - **Vowels and IPA**: Full IPA symbol support, oral, nasal, diphthongs, and triphthongs with custom percentage splits (e.g. 65%/35%), ending glides (`a -`, `a R`, `a h`, `a AP`), and glottal attacks (`' a`, `? a`).
   - **Consonants and Place/Manner of Articulation**:
     - *Manners*: Plosives, Fricatives, Nasals, Liquids, Affricates, Semivowels, Glottals, Clicks/Ejectives, Specials.
     - *Places*: Bilabial, Labiodental, Dental, Alveolar, Postalveolar, Retroflex, Palatal, Velar, Uvular, Pharyngeal, Glottal.
     - *Properties*: Voiced/Voiceless, Aspirated, Onset, Coda, Intervocalic, Timing multiplier, and Pre-utterance.
   - **Consonant Clusters**: Support for onset and coda clusters, proportional duration splitting, BPM threshold limits, and automatic fast-tempo elision.
   - **Dictionary & Exceptions**: Import and export phonetic dictionaries in **CMUdict**, **TXT**, **TSV**, and **CSV** formats.
   - **Batch Import & Search**: Real-time filtering and search across all phonetic tables, plus bulk import wizards.

3. **Phonetic Matrix, Voice Colors & Phonotactics**:
   - Supported concatenations: `- V`, `- CV`, `VCV`, `CV`, `VC`, `VV`, `CC`, `CVC`, `V -`, `C -`, `' V`, `V ɾ V`, `C_V` (Liaison), breath marks (`br`), and rests.
   - Pitch range filters (note ranges) and Voice Color filters (`Power`, `Soft`, etc.).
   - Phonotactic rules with cross-word liaison and linking transitions.

4. **Real-time Phonetic Simulator & Audio Preview**:
   - Real-time lyric and phrase decomposition into notes and phoneme sequences with acoustic tone player and BPM control.
   - **Voicebank Cross-Check (`oto.ini`)**: Load any voicebank's `oto.ini` to verify alias coverage percentages and identify missing aliases before deployment.

5. **Automated Unit Testing Suite for Phonetics**:
   - Define custom test cases with input lyrics and expected phoneme sequences.
   - 1-click regression testing with instant pass/fail metrics and detailed diff reports.

6. **Integrated .DLL Compiler & .ouplugin Packager**:
   - Fast background compilation via .NET SDK (`dotnet build`).
   - 1-click automatic deployment directly to your OpenUtau `Plugins/` folder.
   - Official `.ouplugin` packager for easy drag-and-drop distribution.
   - DiffSinger neural dictionary export (`phonemes.txt`, `lexicon.txt`, `dsdict.yaml`).

7. **Official Presets + Universal From Scratch**:
   - **Universal From Scratch (Custom)**: Create any language or phonetic system from scratch.
   - **Brazilian Portuguese (PT-BR CVC)**
   - **Japanese VCV & CVVC (JA-VCV / JA-CVVC)**
   - **Spanish Syllable & VCCV (ES-SYL / ES-VCCV)**
   - **French CVVC (FR-CVVC)**
   - **Russian CVC (RU-CVC)**
   - **German VCCV (DE-VCCV)**
   - **Italian Syllable (IT-SYL)**
   - **English Arpasing & VCCV (EN-ARPA / EN-VCCV)**
   - **Korean CVC & Hangul (KO-CVC / KO-KR)**
   - **Chinese Mandarim & Cantonese (ZH-CVV / YUE-CVVC)**
   - **Polish, Turkish, Vietnamese, Thai, Latin Diphone**

---

## How to Run

### 1. Launch the Desktop GUI (Native GPU via egui/eframe)
```bash
cargo run --release
```

### 2. Command Line Interface (CLI) Build
```bash
./target/release/phonemizer-studio --build presets_json/universal_scratch.json ./test_plugins_output
```

### 3. Run Automated Tests
```bash
cargo test
```
