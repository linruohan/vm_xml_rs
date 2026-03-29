//! 面板宏定义 - 用于减少样板代码

/// 通用字段行宏 - 用于创建标签 + 单行文本编辑框的标准布局
#[macro_export]
macro_rules! field_row {
    ($ui:expr, $label:expr, $value:expr) => {{
        $ui.label($label);
        $ui.text_edit_singleline($value);
        $ui.end_row();
    }};
}

/// 通用字段行宏（带按钮） - 用于创建标签 + 文本框 + 按钮的布局
#[macro_export]
macro_rules! field_row_with_button {
    ($ui:expr, $label:expr, $value:expr, $button_label:expr, $button_action:expr) => {{
        $ui.label($label);
        $ui.horizontal(|ui| {
            ui.text_edit_singleline($value);
            if ui.button($button_label).clicked() {
                $button_action;
            }
        });
        $ui.end_row();
    }};
}

/// 通用字段行宏（带验证） - 用于创建带验证提示的字段
#[macro_export]
macro_rules! field_row_with_validation {
    ($ui:expr, $label:expr, $value:expr, $is_valid:expr, $error_message:expr) => {{
        $ui.label($label);
        $ui.horizontal(|ui| {
            let response = ui.text_edit_singleline($value);
            if !$is_valid {
                response.on_hover_text($error_message);
                ui.label(egui::RichText::new("⚠").color(egui::Color32::from_rgb(255, 100, 100)));
            }
        });
        $ui.end_row();
    }};
}

/// 通用字段行宏（带单位和下拉框） - 用于创建带单位的字段
#[macro_export]
macro_rules! field_row_with_unit {
    ($ui:expr, $label:expr, $value:expr, $unit:expr, $unit_options:expr) => {{
        $ui.label($label);
        $ui.horizontal(|ui| {
            ui.text_edit_singleline($value);
            egui::ComboBox::from_id_source(format!("unit_{}", $label))
                .selected_text($unit.as_str())
                .show_ui(ui, |ui| {
                    for (val, label) in $unit_options {
                        ui.selectable_value($unit, val.to_string(), label);
                    }
                });
        });
        $ui.end_row();
    }};
}

/// 通用滑块字段宏 - 用于创建带滑块的字段
#[macro_export]
macro_rules! slider_row {
    ($ui:expr, $label:expr, $value:expr, $range:expr, $text:expr) => {{
        $ui.label($label);
        $ui.add(egui::Slider::new($value, $range).text($text));
        $ui.end_row();
    }};
}

/// 通用下拉框字段宏 - 用于创建下拉框字段
#[macro_export]
macro_rules! combo_row {
    ($ui:expr, $label:expr, $value:expr, $source:expr, $options:expr) => {{
        $ui.label($label);
        egui::ComboBox::from_id_source($source).selected_text($value.as_str()).show_ui($ui, |ui| {
            for (val, label) in $options {
                ui.selectable_value($value, val.to_string(), label);
            }
        });
        $ui.end_row();
    }};
}

/// 通用复选框字段宏 - 用于创建复选框字段
#[macro_export]
macro_rules! checkbox_row {
    ($ui:expr, $value:expr, $label:expr) => {{
        $ui.horizontal(|ui| {
            egui::Checkbox::new($value, $label).ui(ui);
        });
        $ui.end_row();
    }};
}

/// 通用字段行宏（Option<String> 类型） - 用于创建可空字符串字段
#[macro_export]
macro_rules! field_row_opt {
    ($ui:expr, $label:expr, $value:expr) => {{
        $ui.label($label);
        let mut val = $value.clone().unwrap_or_default();
        if $ui.text_edit_singleline(&mut val).changed() {
            *$value = if val.is_empty() { None } else { Some(val) };
        }
        $ui.end_row();
    }};
}

/// 通用字段行宏（数字类型，带 DragValue） - 用于创建数字输入字段
#[macro_export]
macro_rules! field_row_drag {
    ($ui:expr, $label:expr, $value:expr) => {{
        $ui.label($label);
        let mut val = $value.unwrap_or(0);
        if $ui.add(egui::DragValue::new(&mut val).speed(0.1)).changed() {
            *$value = if val == 0 { None } else { Some(val) };
        }
        $ui.end_row();
    }};
}

/// 通用下拉框宏（简化版） - 用于字符串选项下拉框
#[macro_export]
macro_rules! combo_row_str {
    ($ui:expr, $label:expr, $value:expr, $id:expr, $options:expr) => {{
        $ui.label($label);
        let val = $value.get_or_insert_with(|| $options[0].to_string());
        egui::ComboBox::from_id_source($id).selected_text(val.as_str()).show_ui($ui, |ui| {
            for opt in $options {
                ui.selectable_value(val, opt.to_string(), *opt);
            }
        });
        $ui.end_row();
    }};
}

/// 通用滑块宏（简化版） - 用于创建滑块输入
#[macro_export]
macro_rules! slider_row_simple {
    ($ui:expr, $label:expr, $value:expr, $range:expr, $text:expr) => {{
        $ui.label($label);
        $ui.add(egui::Slider::new($value, $range).text($text));
        $ui.end_row();
    }};
}
