use egui::{Color32, RichText, Ui};

/// 主题枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
    Blue,
}

impl Theme {
    pub const fn name(&self) -> &'static str {
        match self {
            Theme::Light => "浅色主题",
            Theme::Dark => "深色主题",
            Theme::Blue => "蓝色主题",
        }
    }
}

/// 主题颜色配置
#[allow(dead_code)]
pub struct ThemeColors {
    pub window_fill: Color32,
    pub panel_fill: Color32,
    pub card_background: Color32,
    pub border_color: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub tab_active_bg: Color32,
    pub tab_inactive_bg: Color32,
    pub tab_active_text: Color32,
    pub tab_inactive_text: Color32,
    pub advanced_tab_active_bg: Color32,
    pub advanced_tab_active_text: Color32,
    pub button_bg: Color32,
    pub header_color: Color32,
    pub success: Color32,
    pub error: Color32,
    pub warning: Color32,
    pub info: Color32,
    pub xml_bg: Color32,
    pub xml_text: Color32,
    pub xml_border: Color32,
    pub btn_copy: Color32,
    pub btn_save: Color32,
    pub btn_format: Color32,
    pub status_ready: Color32,
    pub input_text: Color32,
    pub input_background: Color32,
    pub input_border: Color32,
}

impl ThemeColors {
    pub fn from_theme(theme: Theme) -> Self {
        match theme {
            Theme::Light => Self {
                window_fill: Color32::from_rgb(245, 245, 240),
                panel_fill: Color32::from_rgb(245, 245, 240),
                card_background: Color32::from_rgb(252, 252, 250),
                border_color: Color32::from_rgb(220, 220, 220),
                text_primary: Color32::from_rgb(30, 30, 30),
                text_secondary: Color32::from_rgb(80, 80, 80),
                tab_active_bg: Color32::from_rgb(255, 140, 0),
                tab_inactive_bg: Color32::from_rgb(245, 245, 240),
                tab_active_text: Color32::WHITE,
                tab_inactive_text: Color32::BLACK,
                advanced_tab_active_bg: Color32::from_rgb(230, 240, 250),
                advanced_tab_active_text: Color32::from_rgb(100, 149, 237),
                button_bg: Color32::from_rgb(255, 165, 0),
                header_color: Color32::from_rgb(255, 140, 0),
                success: Color32::from_rgb(76, 175, 80),
                error: Color32::from_rgb(244, 67, 54),
                warning: Color32::from_rgb(255, 193, 7),
                info: Color32::from_rgb(33, 150, 243),
                xml_bg: Color32::from_rgb(28, 30, 36),
                xml_text: Color32::from_rgb(200, 200, 200),
                xml_border: Color32::from_rgb(60, 60, 70),
                btn_copy: Color32::from_rgb(50, 150, 200),
                btn_save: Color32::from_rgb(76, 175, 80),
                btn_format: Color32::from_rgb(156, 39, 176),
                status_ready: Color32::GRAY,
                input_text: Color32::BLACK,
                input_background: Color32::WHITE,
                input_border: Color32::from_rgb(180, 180, 180),
            },
            Theme::Dark => Self {
                window_fill: Color32::from_rgb(30, 30, 30),
                panel_fill: Color32::from_rgb(35, 35, 35),
                card_background: Color32::from_rgb(45, 45, 45),
                border_color: Color32::from_rgb(60, 60, 60),
                text_primary: Color32::from_rgb(230, 230, 230),
                text_secondary: Color32::from_rgb(180, 180, 180),
                tab_active_bg: Color32::from_rgb(255, 140, 0),
                tab_inactive_bg: Color32::from_rgb(50, 50, 50),
                tab_active_text: Color32::WHITE,
                tab_inactive_text: Color32::from_rgb(200, 200, 200),
                advanced_tab_active_bg: Color32::from_rgb(40, 50, 60),
                advanced_tab_active_text: Color32::from_rgb(100, 180, 237),
                button_bg: Color32::from_rgb(60, 60, 60),
                header_color: Color32::from_rgb(255, 160, 50),
                success: Color32::from_rgb(100, 200, 100),
                error: Color32::from_rgb(240, 100, 100),
                warning: Color32::from_rgb(255, 200, 100),
                info: Color32::from_rgb(100, 180, 243),
                xml_bg: Color32::from_rgb(15, 15, 20),
                xml_text: Color32::from_rgb(180, 180, 180),
                xml_border: Color32::from_rgb(50, 50, 60),
                btn_copy: Color32::from_rgb(40, 130, 180),
                btn_save: Color32::from_rgb(60, 155, 70),
                btn_format: Color32::from_rgb(136, 29, 156),
                status_ready: Color32::from_rgb(120, 120, 120),
                input_text: Color32::from_rgb(230, 230, 230),
                input_background: Color32::from_rgb(50, 50, 50),
                input_border: Color32::from_rgb(80, 80, 80),
            },
            Theme::Blue => Self {
                window_fill: Color32::from_rgb(230, 240, 245),
                panel_fill: Color32::from_rgb(235, 242, 248),
                card_background: Color32::from_rgb(245, 250, 252),
                border_color: Color32::from_rgb(180, 200, 220),
                text_primary: Color32::from_rgb(20, 40, 60),
                text_secondary: Color32::from_rgb(50, 80, 110),
                tab_active_bg: Color32::from_rgb(33, 150, 243),
                tab_inactive_bg: Color32::from_rgb(220, 235, 245),
                tab_active_text: Color32::WHITE,
                tab_inactive_text: Color32::from_rgb(30, 60, 90),
                advanced_tab_active_bg: Color32::from_rgb(200, 230, 250),
                advanced_tab_active_text: Color32::from_rgb(25, 118, 210),
                button_bg: Color32::from_rgb(100, 181, 246),
                header_color: Color32::from_rgb(33, 150, 243),
                success: Color32::from_rgb(56, 142, 60),
                error: Color32::from_rgb(198, 40, 40),
                warning: Color32::from_rgb(245, 127, 23),
                info: Color32::from_rgb(25, 118, 210),
                xml_bg: Color32::from_rgb(20, 30, 40),
                xml_text: Color32::from_rgb(180, 200, 220),
                xml_border: Color32::from_rgb(50, 70, 90),
                btn_copy: Color32::from_rgb(25, 118, 210),
                btn_save: Color32::from_rgb(56, 142, 60),
                btn_format: Color32::from_rgb(142, 36, 177),
                status_ready: Color32::from_rgb(100, 120, 140),
                input_text: Color32::from_rgb(20, 40, 60),
                input_background: Color32::from_rgb(255, 255, 255),
                input_border: Color32::from_rgb(150, 170, 190),
            },
        }
    }
}

/// 获取当前主题颜色
pub fn get_theme_colors(theme: Theme) -> ThemeColors {
    ThemeColors::from_theme(theme)
}

/// 标题样式配置
pub struct HeadingStyle {
    pub size: f32,
    pub color: Color32,
    pub bold: bool,
}

impl HeadingStyle {
    pub fn with_theme(theme: Theme) -> Self {
        let colors = get_theme_colors(theme);
        Self { size: 18.0, color: colors.header_color, bold: true }
    }
}

/// 显示带样式的标题 - 支持主题
pub fn heading(ui: &mut Ui, text: &str, style: Option<HeadingStyle>) {
    let style = style.unwrap_or(HeadingStyle::with_theme(Theme::Light));
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

/// 添加按钮 - 支持主题颜色
pub fn add_button(ui: &mut Ui, text: &str, colors: &ThemeColors) -> bool {
    let btn = ui.button(RichText::new(text).color(colors.success));
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
pub fn card_group_with_theme<R>(
    ui: &mut Ui,
    title: &str,
    icon: Option<&str>,
    colors: &ThemeColors,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> egui::InnerResponse<R> {
    let full_title = match icon {
        Some(i) => format!("{} {}", i, title),
        None => title.to_string(),
    };

    egui::Frame::group(ui.style())
        .fill(colors.card_background)
        .stroke(egui::Stroke::new(1.0, colors.border_color))
        .rounding(egui::Rounding::same(10.0))
        .inner_margin(egui::Margin::same(12.0))
        .shadow(egui::epaint::Shadow {
            offset: egui::vec2(0.0, 2.0),
            blur: 4.0,
            spread: 0.0,
            color: Color32::from_black_alpha(20),
        })
        .show(ui, |ui| {
            ui.label(RichText::new(full_title).strong().size(15.0).color(colors.header_color));
            ui.add_space(8.0);
            add_contents(ui)
        })
}

/// 标准卡片组（带标题）- 支持主题
pub fn card_group<R>(
    ui: &mut Ui,
    title: &str,
    icon: Option<&str>,
    colors: &ThemeColors,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> egui::InnerResponse<R> {
    card_group_with_theme(ui, title, icon, colors, add_contents)
}
