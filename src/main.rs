mod app;
mod model;
mod panels;
mod xml_gen;

use app::VMConfigApp;

fn main() -> eframe::Result<()> {
    // 配置字体以支持中文
    let mut fonts = egui::FontDefinitions::default();

    // 添加文楷GB字体
    fonts.font_data.insert(
        "wenkai".to_owned(),
        egui::FontData::from_static(include_bytes!("../resources/fonts/LXGWWenKaiGB-Regular.ttf")),
    );

    // 添加文楷GB等宽字体
    fonts.font_data.insert(
        "wenkai-mono".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../resources/fonts/LXGWWenKaiMonoGB-Regular.ttf"
        )),
    );

    // 将中文字体添加到默认字体家族
    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "wenkai".to_owned());
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, "wenkai-mono".to_owned());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("VM XML 配置生成器"),
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
