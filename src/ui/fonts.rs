use eframe::egui;
use std::fs;
use std::path::Path;

pub const NOTO_SANS_TTF: &[u8] = include_bytes!("../../assets/fonts/NotoSans-Regular.ttf");

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "noto_sans".to_owned(),
        egui::FontData::from_static(NOTO_SANS_TTF),
    );

    let system_candidates = [

        "/System/Library/Fonts/Supplemental/Arial Unicode.ttf",
        "/Library/Fonts/Arial Unicode.ttf",
        "/System/Library/Fonts/PingFang.ttc",
        "/System/Library/Fonts/Hiragino Sans GB.ttc",
        "/System/Library/Fonts/Supplemental/AppleGothic.ttf",
        "/System/Library/Fonts/Geneva.ttf",

        "C:\\Windows\\Fonts\\arialuni.ttf",
        "C:\\Windows\\Fonts\\msyh.ttc",
        "C:\\Windows\\Fonts\\msgothic.ttc",
        "C:\\Windows\\Fonts\\malgun.ttf",
        "C:\\Windows\\Fonts\\segoeui.ttf",

        "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
        "/usr/share/fonts/truetype/freefont/FreeSans.ttf",
    ];

    let mut loaded_system_fonts = Vec::new();
    for (i, path_str) in system_candidates.iter().enumerate() {
        if Path::new(path_str).exists() {
            if let Ok(font_bytes) = fs::read(path_str) {
                let font_key = format!("system_fallback_{}", i);
                fonts.font_data.insert(
                    font_key.clone(),
                    egui::FontData::from_owned(font_bytes),
                );
                loaded_system_fonts.push(font_key);

                if loaded_system_fonts.len() >= 3 {
                    break;
                }
            }
        }
    }

    if let Some(prop) = fonts.families.get_mut(&egui::FontFamily::Proportional) {

        prop.insert(0, "noto_sans".to_owned());

        for (idx, sys_font) in loaded_system_fonts.iter().enumerate() {
            prop.insert(1 + idx, sys_font.clone());
        }
    }

    if let Some(mono) = fonts.families.get_mut(&egui::FontFamily::Monospace) {

        mono.push("noto_sans".to_owned());
        for sys_font in &loaded_system_fonts {
            mono.push(sys_font.clone());
        }
    }

    ctx.set_fonts(fonts);
}
