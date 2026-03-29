use egui::{Color32, RichText, Ui};

/// 主题枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
    Blue,
    // 新增主题
    Midnight,    // 午夜蓝深色主题
    Forest,      // 森林绿主题
    Aurora,      // 极光紫主题
}

impl Theme {
    pub const fn name(&self) -> &'static str {
        match self {
            Theme::Light => "浅色主题",
            Theme::Dark => "深色主题",
            Theme::Blue => "蓝色主题",
            Theme::Midnight => "午夜主题",
            Theme::Forest => "森林主题",
            Theme::Aurora => "极光主题",
        }
    }
}

/// 主题颜色配置
#[allow(dead_code)]
pub struct ThemeColors {
    pub window_fill: Color32,
    pub panel_fill: Color32,
    pub card_background: Color32,
    pub card_background_hover: Color32,
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
    pub button_bg_hover: Color32,
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
    pub accent_gradient_start: Color32,
    pub accent_gradient_end: Color32,
}

impl ThemeColors {
    pub fn from_theme(theme: Theme) -> Self {
        match theme {
            Theme::Light => Self {
                window_fill: Color32::from_rgb(240, 242, 245),
                panel_fill: Color32::from_rgb(240, 242, 245),
                card_background: Color32::from_rgb(255, 255, 255),
                card_background_hover: Color32::from_rgb(250, 252, 255),
                border_color: Color32::from_rgb(210, 215, 220),
                text_primary: Color32::from_rgb(25, 30, 35),
                text_secondary: Color32::from_rgb(70, 80, 90),
                tab_active_bg: Color32::from_rgb(255, 120, 50),
                tab_inactive_bg: Color32::from_rgb(235, 238, 242),
                tab_active_text: Color32::WHITE,
                tab_inactive_text: Color32::from_rgb(60, 70, 80),
                advanced_tab_active_bg: Color32::from_rgb(235, 245, 255),
                advanced_tab_active_text: Color32::from_rgb(25, 118, 210),
                button_bg: Color32::from_rgb(255, 140, 40),
                button_bg_hover: Color32::from_rgb(255, 160, 80),
                header_color: Color32::from_rgb(255, 120, 50),
                success: Color32::from_rgb(34, 139, 34),
                error: Color32::from_rgb(220, 53, 69),
                warning: Color32::from_rgb(255, 193, 7),
                info: Color32::from_rgb(23, 162, 184),
                xml_bg: Color32::from_rgb(25, 28, 35),
                xml_text: Color32::from_rgb(212, 212, 212),
                xml_border: Color32::from_rgb(55, 60, 70),
                btn_copy: Color32::from_rgb(52, 152, 219),
                btn_save: Color32::from_rgb(46, 204, 113),
                btn_format: Color32::from_rgb(155, 89, 182),
                status_ready: Color32::from_rgb(120, 130, 140),
                input_text: Color32::from_rgb(25, 30, 35),
                input_background: Color32::from_rgb(255, 255, 255),
                input_border: Color32::from_rgb(200, 205, 210),
                accent_gradient_start: Color32::from_rgb(255, 140, 50),
                accent_gradient_end: Color32::from_rgb(255, 100, 80),
            },
            Theme::Dark => Self {
                window_fill: Color32::from_rgb(22, 24, 28),
                panel_fill: Color32::from_rgb(28, 30, 35),
                card_background: Color32::from_rgb(38, 40, 48),
                card_background_hover: Color32::from_rgb(45, 48, 56),
                border_color: Color32::from_rgb(55, 58, 68),
                text_primary: Color32::from_rgb(235, 235, 240),
                text_secondary: Color32::from_rgb(165, 170, 180),
                tab_active_bg: Color32::from_rgb(255, 130, 60),
                tab_inactive_bg: Color32::from_rgb(38, 40, 48),
                tab_active_text: Color32::WHITE,
                tab_inactive_text: Color32::from_rgb(170, 175, 180),
                advanced_tab_active_bg: Color32::from_rgb(35, 45, 55),
                advanced_tab_active_text: Color32::from_rgb(100, 200, 255),
                button_bg: Color32::from_rgb(255, 145, 70),
                button_bg_hover: Color32::from_rgb(255, 165, 100),
                header_color: Color32::from_rgb(255, 150, 80),
                success: Color32::from_rgb(76, 175, 80),
                error: Color32::from_rgb(239, 83, 80),
                warning: Color32::from_rgb(255, 183, 77),
                info: Color32::from_rgb(66, 165, 245),
                xml_bg: Color32::from_rgb(18, 20, 25),
                xml_text: Color32::from_rgb(200, 205, 210),
                xml_border: Color32::from_rgb(50, 55, 65),
                btn_copy: Color32::from_rgb(41, 128, 185),
                btn_save: Color32::from_rgb(39, 174, 96),
                btn_format: Color32::from_rgb(142, 68, 173),
                status_ready: Color32::from_rgb(110, 115, 120),
                input_text: Color32::from_rgb(230, 232, 235),
                input_background: Color32::from_rgb(32, 35, 42),
                input_border: Color32::from_rgb(60, 65, 75),
                accent_gradient_start: Color32::from_rgb(255, 150, 80),
                accent_gradient_end: Color32::from_rgb(255, 100, 100),
            },
            Theme::Blue => Self {
                window_fill: Color32::from_rgb(225, 235, 242),
                panel_fill: Color32::from_rgb(225, 235, 242),
                card_background: Color32::from_rgb(250, 252, 255),
                card_background_hover: Color32::from_rgb(245, 250, 255),
                border_color: Color32::from_rgb(180, 200, 215),
                text_primary: Color32::from_rgb(15, 35, 50),
                text_secondary: Color32::from_rgb(45, 70, 90),
                tab_active_bg: Color32::from_rgb(21, 130, 220),
                tab_inactive_bg: Color32::from_rgb(215, 230, 242),
                tab_active_text: Color32::WHITE,
                tab_inactive_text: Color32::from_rgb(35, 60, 80),
                advanced_tab_active_bg: Color32::from_rgb(195, 225, 250),
                advanced_tab_active_text: Color32::from_rgb(10, 100, 180),
                button_bg: Color32::from_rgb(33, 150, 243),
                button_bg_hover: Color32::from_rgb(66, 165, 245),
                header_color: Color32::from_rgb(21, 130, 220),
                success: Color32::from_rgb(56, 142, 60),
                error: Color32::from_rgb(198, 40, 40),
                warning: Color32::from_rgb(245, 127, 23),
                info: Color32::from_rgb(25, 118, 210),
                xml_bg: Color32::from_rgb(20, 32, 45),
                xml_text: Color32::from_rgb(190, 210, 230),
                xml_border: Color32::from_rgb(50, 70, 90),
                btn_copy: Color32::from_rgb(25, 118, 210),
                btn_save: Color32::from_rgb(56, 142, 60),
                btn_format: Color32::from_rgb(142, 36, 177),
                status_ready: Color32::from_rgb(90, 110, 130),
                input_text: Color32::from_rgb(15, 35, 50),
                input_background: Color32::from_rgb(255, 255, 255),
                input_border: Color32::from_rgb(160, 185, 205),
                accent_gradient_start: Color32::from_rgb(33, 150, 243),
                accent_gradient_end: Color32::from_rgb(25, 118, 210),
            },
            Theme::Midnight => Self {
                window_fill: Color32::from_rgb(15, 18, 25),
                panel_fill: Color32::from_rgb(20, 25, 35),
                card_background: Color32::from_rgb(30, 35, 50),
                card_background_hover: Color32::from_rgb(40, 45, 60),
                border_color: Color32::from_rgb(50, 60, 80),
                text_primary: Color32::from_rgb(220, 225, 235),
                text_secondary: Color32::from_rgb(150, 160, 180),
                tab_active_bg: Color32::from_rgb(99, 179, 237),
                tab_inactive_bg: Color32::from_rgb(35, 40, 55),
                tab_active_text: Color32::WHITE,
                tab_inactive_text: Color32::from_rgb(150, 160, 180),
                advanced_tab_active_bg: Color32::from_rgb(25, 35, 50),
                advanced_tab_active_text: Color32::from_rgb(100, 180, 255),
                button_bg: Color32::from_rgb(60, 90, 140),
                button_bg_hover: Color32::from_rgb(80, 120, 180),
                header_color: Color32::from_rgb(100, 180, 255),
                success: Color32::from_rgb(80, 200, 120),
                error: Color32::from_rgb(240, 100, 100),
                warning: Color32::from_rgb(255, 200, 100),
                info: Color32::from_rgb(80, 170, 250),
                xml_bg: Color32::from_rgb(12, 15, 22),
                xml_text: Color32::from_rgb(180, 200, 220),
                xml_border: Color32::from_rgb(40, 50, 70),
                btn_copy: Color32::from_rgb(50, 120, 200),
                btn_save: Color32::from_rgb(60, 160, 100),
                btn_format: Color32::from_rgb(150, 80, 200),
                status_ready: Color32::from_rgb(100, 120, 150),
                input_text: Color32::from_rgb(220, 225, 235),
                input_background: Color32::from_rgb(25, 30, 45),
                input_border: Color32::from_rgb(55, 65, 85),
                accent_gradient_start: Color32::from_rgb(60, 110, 180),
                accent_gradient_end: Color32::from_rgb(100, 160, 255),
            },
            Theme::Forest => Self {
                window_fill: Color32::from_rgb(230, 235, 230),
                panel_fill: Color32::from_rgb(235, 240, 235),
                card_background: Color32::from_rgb(245, 250, 245),
                card_background_hover: Color32::from_rgb(240, 245, 240),
                border_color: Color32::from_rgb(180, 200, 180),
                text_primary: Color32::from_rgb(20, 35, 25),
                text_secondary: Color32::from_rgb(50, 75, 55),
                tab_active_bg: Color32::from_rgb(76, 175, 80),
                tab_inactive_bg: Color32::from_rgb(220, 230, 220),
                tab_active_text: Color32::WHITE,
                tab_inactive_text: Color32::from_rgb(40, 65, 45),
                advanced_tab_active_bg: Color32::from_rgb(200, 230, 200),
                advanced_tab_active_text: Color32::from_rgb(30, 100, 40),
                button_bg: Color32::from_rgb(100, 180, 100),
                button_bg_hover: Color32::from_rgb(120, 200, 120),
                header_color: Color32::from_rgb(56, 142, 60),
                success: Color32::from_rgb(46, 125, 50),
                error: Color32::from_rgb(198, 40, 40),
                warning: Color32::from_rgb(245, 150, 23),
                info: Color32::from_rgb(30, 120, 100),
                xml_bg: Color32::from_rgb(20, 30, 25),
                xml_text: Color32::from_rgb(180, 200, 185),
                xml_border: Color32::from_rgb(45, 65, 50),
                btn_copy: Color32::from_rgb(30, 130, 120),
                btn_save: Color32::from_rgb(56, 142, 60),
                btn_format: Color32::from_rgb(120, 80, 140),
                status_ready: Color32::from_rgb(80, 110, 85),
                input_text: Color32::from_rgb(20, 35, 25),
                input_background: Color32::from_rgb(255, 255, 255),
                input_border: Color32::from_rgb(160, 185, 160),
                accent_gradient_start: Color32::from_rgb(76, 175, 80),
                accent_gradient_end: Color32::from_rgb(46, 125, 50),
            },
            Theme::Aurora => Self {
                window_fill: Color32::from_rgb(28, 25, 35),
                panel_fill: Color32::from_rgb(33, 30, 42),
                card_background: Color32::from_rgb(42, 38, 55),
                card_background_hover: Color32::from_rgb(52, 48, 68),
                border_color: Color32::from_rgb(70, 60, 90),
                text_primary: Color32::from_rgb(235, 230, 245),
                text_secondary: Color32::from_rgb(170, 160, 190),
                tab_active_bg: Color32::from_rgb(150, 100, 200),
                tab_inactive_bg: Color32::from_rgb(50, 45, 65),
                tab_active_text: Color32::WHITE,
                tab_inactive_text: Color32::from_rgb(160, 150, 180),
                advanced_tab_active_bg: Color32::from_rgb(55, 45, 75),
                advanced_tab_active_text: Color32::from_rgb(200, 160, 255),
                button_bg: Color32::from_rgb(130, 80, 180),
                button_bg_hover: Color32::from_rgb(160, 110, 210),
                header_color: Color32::from_rgb(180, 130, 230),
                success: Color32::from_rgb(100, 200, 130),
                error: Color32::from_rgb(240, 100, 130),
                warning: Color32::from_rgb(255, 200, 100),
                info: Color32::from_rgb(120, 160, 255),
                xml_bg: Color32::from_rgb(20, 18, 28),
                xml_text: Color32::from_rgb(200, 195, 220),
                xml_border: Color32::from_rgb(55, 50, 75),
                btn_copy: Color32::from_rgb(100, 150, 220),
                btn_save: Color32::from_rgb(80, 180, 120),
                btn_format: Color32::from_rgb(180, 100, 200),
                status_ready: Color32::from_rgb(130, 120, 160),
                input_text: Color32::from_rgb(230, 225, 240),
                input_background: Color32::from_rgb(38, 35, 50),
                input_border: Color32::from_rgb(70, 65, 90),
                accent_gradient_start: Color32::from_rgb(150, 100, 200),
                accent_gradient_end: Color32::from_rgb(200, 100, 180),
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
    let style = style.unwrap_or_else(|| HeadingStyle::with_theme(Theme::Dark));
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
            color: Color32::from_black_alpha(10),
        })
        .show(ui, |ui| {
            ui.label(RichText::new(full_title).strong().size(15.0).color(colors.header_color));
            ui.add_space(8.0);
            add_contents(ui)
        })
}

/// 标准内部组（用于子项分组）
pub fn inner_group<R>(
    ui: &mut Ui,
    colors: &ThemeColors,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> egui::InnerResponse<R> {
    egui::Frame::group(ui.style())
        .fill(colors.card_background)
        .stroke(egui::Stroke::new(1.0, colors.border_color))
        .rounding(egui::Rounding::same(6.0))
        .inner_margin(egui::Margin::same(8.0))
        .show(ui, add_contents)
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

/// 增强版卡片组 - 带阴影和渐变效果
pub fn enhanced_card_group<R>(
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
        .stroke(egui::Stroke::new(1.5, colors.border_color))
        .rounding(egui::Rounding::same(12.0))
        .inner_margin(egui::Margin::same(14.0))
        .shadow(egui::epaint::Shadow {
            offset: egui::vec2(0.0, 3.0),
            blur: 8.0,
            spread: 0.0,
            color: Color32::from_black_alpha(20),
        })
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(full_title)
                        .strong()
                        .size(15.0)
                        .color(colors.header_color),
                );
            });
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(5.0);
            add_contents(ui)
        })
}

/// 带渐变边框的卡片
pub fn gradient_border_card<R>(
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

    let response = egui::Frame::group(ui.style())
        .fill(colors.card_background)
        .stroke(egui::Stroke::new(2.0, colors.accent_gradient_start))
        .rounding(egui::Rounding::same(10.0))
        .inner_margin(egui::Margin::same(12.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(full_title)
                        .strong()
                        .size(14.0)
                        .color(colors.accent_gradient_start),
                );
            });
            ui.add_space(6.0);
            add_contents(ui)
        });

    response
}

/// 高亮卡片（用于重要信息）
pub fn highlight_card<R>(
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

    // 手动混合颜色
    let highlight_bg = Color32::from_rgb(
        ((colors.card_background.r() as u16 * 19 + colors.info.r() as u16) / 20) as u8,
        ((colors.card_background.g() as u16 * 19 + colors.info.g() as u16) / 20) as u8,
        ((colors.card_background.b() as u16 * 19 + colors.info.b() as u16) / 20) as u8,
    );

    egui::Frame::group(ui.style())
        .fill(highlight_bg)
        .stroke(egui::Stroke::new(2.0, colors.info))
        .rounding(egui::Rounding::same(10.0))
        .inner_margin(egui::Margin::same(14.0))
        .show(ui, |ui| {
            ui.label(
                RichText::new(full_title)
                    .strong()
                    .size(15.0)
                    .color(colors.info),
            );
            ui.add_space(8.0);
            add_contents(ui)
        })
}

/// 带渐变背景的卡片头部
pub fn card_header_with_gradient(ui: &mut Ui, title: &str, icon: Option<&str>, colors: &ThemeColors) {
    let full_title = match icon {
        Some(i) => format!("{} {}", i, title),
        None => title.to_string(),
    };

    let rect = ui.available_rect_before_wrap();
    let header_height = 32.0;
    let header_rect = egui::Rect::from_min_size(
        rect.min,
        egui::vec2(rect.width(), header_height),
    );

    // 绘制渐变背景
    let gradient_start = colors.accent_gradient_start;
    let gradient_end = colors.accent_gradient_end;

    ui.painter().rect_filled(
        header_rect,
        egui::Rounding::same(8.0),
        egui::Color32::from_rgb(
            ((gradient_start.r() as u16 + gradient_end.r() as u16) / 2) as u8,
            ((gradient_start.g() as u16 + gradient_end.g() as u16) / 2) as u8,
            ((gradient_start.b() as u16 + gradient_end.b() as u16) / 2) as u8,
        ),
    );

    ui.label(
        RichText::new(full_title)
            .strong()
            .size(14.0)
            .color(Color32::WHITE),
    );
    ui.add_space(8.0);
}

/// 分隔条（带颜色）
pub fn colored_separator(ui: &mut Ui, color: Color32) {
    let rect = ui.available_rect_before_wrap();
    let separator_rect = egui::Rect::from_min_size(
        rect.min,
        egui::vec2(rect.width(), 2.0),
    );
    ui.painter().rect_filled(
        separator_rect,
        egui::Rounding::same(1.0),
        color,
    );
    ui.add_space(4.0);
}

/// 徽章/标签组件
pub fn badge(ui: &mut Ui, text: &str, color: Color32) -> egui::Response {
    let text_color = if color.r() > 128 || color.g() > 128 || color.b() > 128 {
        Color32::BLACK
    } else {
        Color32::WHITE
    };

    let font_id = egui::TextStyle::Button.resolve(ui.style());
    let text_width = ui.fonts(|fonts| {
        text.chars().map(|c| fonts.glyph_width(&font_id, c)).sum::<f32>()
    });
    let desired_size = egui::vec2(text_width + 12.0, 20.0);

    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

    ui.painter().rect_filled(
        rect,
        egui::Rounding::same(10.0),
        color,
    );

    let text_width_total = ui.fonts(|fonts| {
        text.chars().map(|c| fonts.glyph_width(&font_id, c)).sum::<f32>()
    });
    let text_pos = rect.center() - egui::vec2(text_width_total / 2.0, 7.0);

    ui.painter().text(
        text_pos,
        egui::Align2::LEFT_TOP,
        text,
        font_id,
        text_color,
    );

    response
}

/// 带图标的按钮（现代化样式）
pub fn icon_button(ui: &mut Ui, icon: &str, text: &str, color: Color32) -> bool {
    let btn_text = format!("{} {}", icon, text);
    let btn = egui::Button::new(
        RichText::new(btn_text)
            .strong()
            .size(13.0)
            .color(Color32::WHITE),
    )
    .fill(color)
    .rounding(8.0);

    ui.add(btn).clicked()
}

/// 输入框行（带标签）
pub fn input_row(ui: &mut Ui, label: &str, widget: impl FnOnce(&mut Ui) -> bool, colors: &ThemeColors) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label(RichText::new(label).size(13.0).color(colors.text_secondary));
        ui.add_space(5.0);
        changed = widget(ui);
    });

    changed
}

/// 带阴影的按钮
pub fn shadow_button(ui: &mut Ui, text: &str, color: Color32) -> bool {
    let btn = egui::Button::new(
        RichText::new(text)
            .strong()
            .size(13.0)
            .color(Color32::WHITE),
    )
    .fill(color)
    .rounding(8.0);

    ui.add(btn).clicked()
}
