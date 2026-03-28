use egui::{Color32, RichText, Style, Ui};

/// UI 样式常量
pub mod colors {
    use egui::Color32;

    pub const PRIMARY: Color32 = Color32::from_rgb(255, 140, 0);
    pub const INFO: Color32 = Color32::from_rgb(33, 150, 243);
    pub const SUCCESS: Color32 = Color32::from_rgb(76, 175, 80);

    pub const CARD_BACKGROUND: Color32 = Color32::from_rgb(252, 252, 250);
    pub const BORDER_COLOR: Color32 = Color32::from_rgb(220, 220, 220);
}

/// 应用全局样式
pub fn apply_global_style(style: &mut Style) {
    style.visuals.window_fill = Color32::from_rgb(245, 245, 240);
    style.visuals.panel_fill = Color32::from_rgb(245, 245, 240);
}

/// 标题样式配置
pub struct HeadingStyle {
    pub size: f32,
    pub color: Color32,
    pub bold: bool,
}

impl Default for HeadingStyle {
    fn default() -> Self {
        Self { size: 18.0, color: Color32::BLACK, bold: true }
    }
}

/// 显示带样式的标题
pub fn heading(ui: &mut Ui, text: &str, style: Option<HeadingStyle>) {
    let style = style.unwrap_or_default();
    let mut rich_text = RichText::new(text).size(style.size);
    if style.bold {
        rich_text = rich_text.strong();
    }
    rich_text = rich_text.color(style.color);
    ui.heading(rich_text);
}

/// 显示带图标的标题
pub fn heading_with_icon(ui: &mut Ui, icon: &str, text: &str) {
    heading(ui, &format!("{} {}", icon, text), None);
}

/// 标准 Grid 配置
pub fn grid<R>(
    ui: &mut Ui,
    id: impl std::hash::Hash,
    columns: usize,
    add_contents: impl FnOnce(&mut Ui) -> R,
) {
    egui::Grid::new(id)
        .num_columns(columns)
        .spacing([12.0, 10.0])
        .min_col_width(80.0)
        .show(ui, add_contents);
}

/// 复选框包装器，返回 bool
pub fn checkbox(ui: &mut Ui, is_set: &mut bool, label: &str) -> bool {
    ui.checkbox(is_set, label).changed()
}

/// 删除按钮
pub fn delete_button(ui: &mut Ui, tooltip: Option<&str>) -> bool {
    let mut btn = ui.small_button("🗑️ 删除");
    if let Some(tip) = tooltip {
        btn = btn.on_hover_text(tip);
    }
    btn.clicked()
}

/// 添加按钮
pub fn add_button(ui: &mut Ui, text: &str) -> bool {
    let btn = ui.button(RichText::new(text).color(colors::SUCCESS));
    btn.clicked()
}

/// 标准面板头部
pub fn panel_header(ui: &mut Ui, icon: &str, title: &str) {
    ui.add_space(5.0);
    heading_with_icon(ui, icon, title);
    ui.add_space(5.0);
    ui.separator();
    ui.add_space(10.0);
}

/// 标准卡片组（带标题）
pub fn card_group<R>(
    ui: &mut Ui,
    title: &str,
    icon: Option<&str>,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> egui::InnerResponse<R> {
    let full_title = match icon {
        Some(i) => format!("{} {}", i, title),
        None => title.to_string(),
    };

    egui::Frame::group(ui.style())
        .fill(colors::CARD_BACKGROUND)
        .stroke(egui::Stroke::new(1.0, colors::BORDER_COLOR))
        .rounding(egui::Rounding::same(10.0))
        .inner_margin(egui::Margin::same(12.0))
        .shadow(egui::epaint::Shadow {
            offset: egui::vec2(0.0, 2.0),
            blur: 4.0,
            spread: 0.0,
            color: Color32::from_black_alpha(20),
        })
        .show(ui, |ui| {
            ui.label(RichText::new(full_title).strong().size(15.0).color(colors::PRIMARY));
            ui.add_space(8.0);
            add_contents(ui)
        })
}
