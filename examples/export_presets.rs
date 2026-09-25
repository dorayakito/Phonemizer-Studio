use phonemizer_studio::presets::*;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("presets_json")?;

    let pt = create_portuguese_preset();
    fs::write("presets_json/brazilian_portuguese_cvc.json", serde_json::to_string_pretty(&pt)?)?;

    let ja_vcv = create_japanese_vcv_preset();
    fs::write("presets_json/japanese_vcv.json", serde_json::to_string_pretty(&ja_vcv)?)?;

    let ja_cvvc = create_japanese_cvvc_preset();
    fs::write("presets_json/japanese_cvvc.json", serde_json::to_string_pretty(&ja_cvvc)?)?;

    let en_arpa = create_english_arpasing_preset();
    fs::write("presets_json/english_arpasing.json", serde_json::to_string_pretty(&en_arpa)?)?;

    let en_vccv = create_english_vccv_preset();
    fs::write("presets_json/english_vccv.json", serde_json::to_string_pretty(&en_vccv)?)?;

    let es = create_spanish_preset();
    fs::write("presets_json/spanish_syllable.json", serde_json::to_string_pretty(&es)?)?;

    let fr = create_french_preset();
    fs::write("presets_json/french_cvvc.json", serde_json::to_string_pretty(&fr)?)?;

    let ru = create_russian_cvc_preset();
    fs::write("presets_json/russian_cvc.json", serde_json::to_string_pretty(&ru)?)?;

    let it = create_italian_preset();
    fs::write("presets_json/italian_syllable.json", serde_json::to_string_pretty(&it)?)?;

    let de = create_german_vccv_preset();
    fs::write("presets_json/german_vccv.json", serde_json::to_string_pretty(&de)?)?;

    let ko = create_korean_hangul_preset();
    fs::write("presets_json/korean_cvc.json", serde_json::to_string_pretty(&ko)?)?;

    let zh = create_chinese_cvv_preset();
    fs::write("presets_json/chinese_cvv.json", serde_json::to_string_pretty(&zh)?)?;

    let yue = create_cantonese_preset();
    fs::write("presets_json/cantonese_cvvc.json", serde_json::to_string_pretty(&yue)?)?;

    let pl = create_polish_preset();
    fs::write("presets_json/polish_cvc.json", serde_json::to_string_pretty(&pl)?)?;

    let th = create_thai_preset();
    fs::write("presets_json/thai_vccv.json", serde_json::to_string_pretty(&th)?)?;

    let tr = create_turkish_preset();
    fs::write("presets_json/turkish_cvvc.json", serde_json::to_string_pretty(&tr)?)?;

    let vi = create_vietnamese_preset();
    fs::write("presets_json/vietnamese_cvvc.json", serde_json::to_string_pretty(&vi)?)?;

    let la = create_latin_preset();
    fs::write("presets_json/latin_diphone.json", serde_json::to_string_pretty(&la)?)?;

    let univ = create_universal_scratch_preset();
    fs::write("presets_json/universal_scratch.json", serde_json::to_string_pretty(&univ)?)?;

    println!("[OK] Todos os presets oficiais salvos em formato .json na pasta presets_json/");
    Ok(())
}
