mod app;
mod model;
mod panels;
mod xml_gen;

use app::VMConfigApp;
use eframe::icon_data;

/// 从 PNG 文件加载图标
fn load_icon() -> egui::IconData {
    icon_data::from_png_bytes(include_bytes!("../resources/mytool.png")).unwrap()
}

fn main() -> eframe::Result<()> {
    // 配置字体以支持中文
    let mut fonts = egui::FontDefinitions::default();

    // 添加 MapleMono 字体
    fonts.font_data.insert(
        "maple-mono".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../resources/fonts/MapleMonoNormal-NF-CN-Regular.ttf"
        )),
    );

    // 将 MapleMono 添加到默认字体家族
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "maple-mono".to_owned());
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, "maple-mono".to_owned());

    // 加载应用图标
    let icon = load_icon();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("VM XML 配置生成器")
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "VM XML 配置生成器",
        options,
        Box::new(|cc| {
            // 设置字体
            cc.egui_ctx.set_fonts(fonts);
            Box::new(VMConfigApp::new(cc))
        }),
    )
}
