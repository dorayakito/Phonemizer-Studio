use phonemizer_studio::compiler::compile_phonemizer_project;
use phonemizer_studio::generator::{generate_csharp_code, generate_csproj};
use phonemizer_studio::importer::parse_csharp_to_project;
use phonemizer_studio::presets::*;
use phonemizer_studio::simulator::{PhonemizerSimulator, SimulationNoteInput};

#[test]
fn test_all_official_presets_creation() {
    let pt = create_portuguese_preset();
    assert_eq!(pt.tag, "PT-BR CVC");
    assert_eq!(pt.author, "HAI-D");
    assert_eq!(pt.vowels.len(), 22);
    assert_eq!(pt.consonants.len(), 31);
    assert!(pt.raw_csharp_source.is_some());

    let ja_vcv = create_japanese_vcv_preset();
    assert_eq!(ja_vcv.tag, "JA VCV");
    assert_eq!(ja_vcv.author, "stakira");

    let ja_cvvc = create_japanese_cvvc_preset();
    assert_eq!(ja_cvvc.tag, "JA CVVC");
    assert_eq!(ja_cvvc.author, "TUBS");

    let es = create_spanish_preset();
    assert_eq!(es.tag, "ES SYL");
    assert_eq!(es.author, "Lotte V");

    let fr = create_french_preset();
    assert_eq!(fr.tag, "FR CVVC");
    assert_eq!(fr.author, "Mim");

    let ru = create_russian_cvc_preset();
    assert_eq!(ru.tag, "RU CVC");
    assert_eq!(ru.author, "Heiden.BZR");

    let it = create_italian_preset();
    assert_eq!(it.tag, "IT SYL");
    assert_eq!(it.author, "Lotte V");

    let de = create_german_vccv_preset();
    assert_eq!(de.tag, "DE VCCV");
    assert_eq!(de.author, "Lotte V");

    let ko = create_korean_hangul_preset();
    assert_eq!(ko.tag, "KO CVC");
    assert_eq!(ko.author, "NANA");

    let zh = create_chinese_cvv_preset();
    assert_eq!(zh.tag, "ZH CVV");

    let yue = create_cantonese_preset();
    assert_eq!(yue.tag, "YUE CVVC");

    let en_arpa = create_english_arpasing_preset();
    assert_eq!(en_arpa.tag, "EN ARPA");

    let en_vccv = create_english_vccv_preset();
    assert_eq!(en_vccv.tag, "EN VCCV");

    let pl = create_polish_preset();
    assert_eq!(pl.tag, "PL CVC");

    let th = create_thai_preset();
    assert_eq!(th.tag, "TH VCCV");

    let tr = create_turkish_preset();
    assert_eq!(tr.tag, "TR CVVC");

    let vi = create_vietnamese_preset();
    assert_eq!(vi.tag, "VI CVVC");

    let la = create_latin_preset();
    assert_eq!(la.tag, "LA DIPHONE");

    let universal = create_universal_scratch_preset();
    assert_eq!(universal.tag, "CUSTOM");
}

#[test]
fn test_csharp_parser_official_files() {
    let raw_cs = include_str!("../official_openutau_sources/BrazilianPortugueseCVCPhonemizer.cs");
    let parsed = parse_csharp_to_project(raw_cs, Some("BrazilianPortugueseCVCPhonemizer.cs"));
    assert_eq!(parsed.name, "Brazilian Portuguese CVC Phonemizer");
    assert_eq!(parsed.tag, "PT-BR CVC");
    assert_eq!(parsed.author, "HAI-D");
    assert_eq!(parsed.language_code, "PT");
    assert_eq!(parsed.vowels.len(), 22);
    assert_eq!(parsed.consonants.len(), 31);
    assert!(!parsed.regex_rules.is_empty());
}

#[test]
fn test_csharp_code_generation() {
    let pt = create_portuguese_preset();
    let cs_code = generate_csharp_code(&pt);

    assert!(cs_code.contains("BrazilianPortugueseCVCPhonemizer"));
}

#[test]
fn test_csproj_generation() {
    let pt = create_portuguese_preset();
    let csproj = generate_csproj(&pt, None);

    assert!(csproj.contains("<TargetFramework>net8.0</TargetFramework>"));
    assert!(csproj.contains("<AssemblyName>BrazilianPortugueseCVCPhonemizer</AssemblyName>"));
}

#[test]
fn test_phonetic_simulator() {
    let pt = create_portuguese_preset();
    let simulator = PhonemizerSimulator::new(&pt);

    let voce_results = simulator.simulate_phrase("você");
    assert_eq!(voce_results.len(), 1);
    assert_eq!(voce_results[0].normalized_phonemes, vec!["v", "o", "s", "e"]);
    let aliases: Vec<String> = voce_results[0]
        .generated_phonemes
        .iter()
        .map(|p| p.final_alias.clone())
        .collect();
    assert_eq!(aliases, vec!["- v", "v o", "o s", "s e"]);

    let phrase_words = vec!["voce", "me", "deu", "amor", "no", "coracao"];
    let notes: Vec<SimulationNoteInput> = phrase_words
        .into_iter()
        .map(|w| SimulationNoteInput {
            lyric: w.to_string(),
            tone: String::new(),
            color: String::new(),
        })
        .collect();

    let results = simulator.simulate_sequence(&notes);
    assert_eq!(results.len(), 6);

    assert_eq!(results[0].input_lyric, "voce");
    assert_eq!(results[0].normalized_phonemes, vec!["v", "o", "s", "e"]);
    let voce_aliases: Vec<String> = results[0].generated_phonemes.iter().map(|p| p.final_alias.clone()).collect();
    assert_eq!(voce_aliases, vec!["- v", "v o", "o s", "s e"]);

    assert_eq!(results[1].input_lyric, "me");
    assert_eq!(results[1].normalized_phonemes, vec!["m", "i"]);
    let me_aliases: Vec<String> = results[1].generated_phonemes.iter().map(|p| p.final_alias.clone()).collect();
    assert_eq!(me_aliases, vec!["e m", "m i"]);

    assert_eq!(results[2].input_lyric, "deu");
    let deu_aliases: Vec<String> = results[2].generated_phonemes.iter().map(|p| p.final_alias.clone()).collect();
    assert_eq!(deu_aliases, vec!["i d", "d e", "e u"]);

    assert_eq!(results[3].input_lyric, "amor");
    let amor_aliases: Vec<String> = results[3].generated_phonemes.iter().map(|p| p.final_alias.clone()).collect();
    assert_eq!(amor_aliases, vec!["u a", "a m", "m o", "o r-"]);

    assert_eq!(results[4].input_lyric, "no");
    let no_aliases: Vec<String> = results[4].generated_phonemes.iter().map(|p| p.final_alias.clone()).collect();
    assert_eq!(no_aliases, vec!["o n", "n u"]);

    assert_eq!(results[5].input_lyric, "coracao");
    let coracao_aliases: Vec<String> = results[5].generated_phonemes.iter().map(|p| p.final_alias.clone()).collect();
    assert_eq!(coracao_aliases, vec!["u k", "k o", "o r", "r a", "a s", "s an", "an u"]);
}

#[tokio::test]
async fn test_dotnet_dll_compilation_e2e() {
    let universal = create_universal_scratch_preset();
    let temp_out = tempfile::tempdir().expect("tempdir");
    let out_path = temp_out.path().to_string_lossy().to_string();

    let result = compile_phonemizer_project(&universal, Some(&out_path), None).await;
    assert!(result.success, "Universal scratch preset must compile to DLL successfully");
    assert!(result.output_dll_path.is_some());
    let dll_path = result.output_dll_path.unwrap();
    assert!(dll_path.exists());
}


#[test]
fn test_app_settings_and_system_specs() {
    use phonemizer_studio::models::{AppSettings, DotnetTarget, RoslynOptimizationLevel, SystemSpecs};
    use phonemizer_studio::generator::generate_csproj_with_settings;

    let mut settings = AppSettings::default();
    assert!(settings.hardware_acceleration);
    assert_eq!(settings.dotnet_target_framework, DotnetTarget::Net8_0);
    assert_eq!(settings.roslyn_optimization, RoslynOptimizationLevel::Release);

    let specs = SystemSpecs::detect(None);
    assert!(!specs.os_name.is_empty());
    assert!(!specs.os_arch.is_empty());
    assert!(specs.logical_cores >= 1);

    let pt = create_portuguese_preset();
    settings.dotnet_target_framework = DotnetTarget::Net8_0;
    settings.ready_to_run_aot = true;
    settings.allow_unsafe_code = true;

    let csproj = generate_csproj_with_settings(&pt, None, Some(&settings));
    assert!(csproj.contains("<TargetFramework>net8.0</TargetFramework>"));
    assert!(csproj.contains("<PublishReadyToRun>true</PublishReadyToRun>"));
    assert!(csproj.contains("<AllowUnsafeBlocks>true</AllowUnsafeBlocks>"));
    assert!(csproj.contains("<Deterministic>true</Deterministic>"));
}

#[test]
fn test_diffsinger_and_test_suite_and_oto_checker() {
    use phonemizer_studio::generator::DiffSingerExporter;
    use phonemizer_studio::simulator::VoicebankOto;

    let pt = create_portuguese_preset();
    let phonemes_txt = DiffSingerExporter::export_phonemes_txt(&pt);
    assert!(phonemes_txt.contains("a\n"));
    assert!(phonemes_txt.contains("AP\n"));
    assert!(phonemes_txt.contains("SP\n"));

    let lexicon_txt = DiffSingerExporter::export_lexicon_txt(&pt);
    assert!(lexicon_txt.contains("a\ta\n"));

    let dsdict = DiffSingerExporter::export_dsdict_yaml(&pt);
    assert!(dsdict.contains("language: \"PT\""));

    let mut mock_vb = VoicebankOto::default();
    mock_vb.alias_map.insert("- a".to_string(), phonemizer_studio::simulator::OtoEntry {
        wav_file: "a.wav".to_string(),
        alias: "- a".to_string(),
        offset: 0.0,
        consonant: 100.0,
        cutoff: -100.0,
        preutterance: 50.0,
        overlap: 25.0,
    });
    mock_vb.alias_map.insert("a -".to_string(), phonemizer_studio::simulator::OtoEntry {
        wav_file: "a_end.wav".to_string(),
        alias: "a -".to_string(),
        offset: 0.0,
        consonant: 100.0,
        cutoff: -100.0,
        preutterance: 50.0,
        overlap: 25.0,
    });

    let coverage = mock_vb.calculate_coverage(&["- a".to_string(), "k a".to_string()]);
    assert_eq!(coverage.total_bank_aliases, 2);
    assert_eq!(coverage.covered_by_phonemizer, 1);
    assert_eq!(coverage.coverage_percentage, 50.0);
    assert_eq!(coverage.phonemizer_extra_aliases, vec!["k a".to_string()]);
}
