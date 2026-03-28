# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

基于 egui 的 VM XML 配置生成器，用于生成 libvirt 虚拟机配置 XML 文件。

## 构建与运行

```bash
# 开发模式运行
cargo run

# 发布模式构建（启用 LTO 和优化级别 2）
cargo build --release

# 运行所有测试
cargo test

# 运行单个测试
cargo test <test_name>

# Clippy 检查
cargo clippy
```

## 代码架构

### 模块层次结构

```
src/
├── main.rs          # 入口：字体配置、窗口参数、图标加载
├── app.rs           # VMConfigApp: UI 状态管理、菜单栏、标签页导航
├── model/           # 数据模型（Serde 序列化）
│   ├── vm_config.rs # 根结构，组合所有配置子模块
│   ├── general.rs, os.rs, cpu.rs, memory.rs
│   ├── devices/     # 设备配置（disk, network, graphics 等）
│   └── *.rs         # 高级配置（tuning, numa, smbios 等）
├── xml_gen/         # XML 生成器（quick-xml 手写事件流）
│   ├── mod.rs       # 生成入口，定义 domain 根元素顺序
│   ├── general.rs, os.rs, cpu.rs, memory.rs
│   ├── devices/     # 设备 XML 生成
│   ├── tuning.rs    # CPU 调优
│   ├── misc.rs      # 杂项（clock, features, events, numatune）
│   └── advanced/    # 高级配置（sysinfo, power_mgmt）
└── panels/          # UI 面板（与 model 一一对应）
    ├── utils.rs     # 公共 UI 工具函数和样式常量
    ├── general_panel.rs, os_panel.rs, cpu_panel.rs, memory_panel.rs
    ├── devices_panel.rs
    └── advanced/    # 高级配置面板
```

### UI 工具模块 (panels/utils.rs)

提供公共 UI 组件和样式常量，避免重复代码：

**样式常量**：
- `colors::PRIMARY` - 主色调（橙色）
- `colors::INFO` - 信息色（蓝色）
- `colors::CARD_BACKGROUND` - 卡片背景色
- `colors::BORDER_COLOR` - 边框颜色

**公共组件**：
- `panel_header(ui, icon, title)` - 标准面板头部（图标 + 标题 + 分隔线）
- `card_group(ui, title, icon, |ui| { ... })` - 卡片式分组容器（圆角边框 + 背景色）
- `grid(ui, id, columns, |ui| { ... })` - 统一 Grid 配置（spacing [12.0, 10.0], min_col_width 80.0）
- `checkbox(ui, &mut bool, label)` - 复选框包装器，返回 bool 表示是否变化
- `add_button(ui, text)` / `delete_button(ui, tooltip)` - 标准操作按钮
- `heading()` / `heading_with_icon()` - 标题样式

**使用模式**：
```rust
pub fn show(ui: &mut egui::Ui, config: &mut VMConfig) {
    panel_header(ui, "📊", "配置标题");

    card_group(ui, "分组标题", Some("🔧"), |ui| {
        // 使用 grid, checkbox 等组件
    });
}
```

### XML 生成顺序

`xml_gen/mod.rs:generate()` 定义 XML 输出顺序：
1. domain 根元素（type 属性）
2. general（name, UUID, metadata）
3. os（引导配置）
4. cpu
5. memory
6. events / features / clock / perf / iothreads
7. cputune
8. devices（所有设备）
9. advanced（sysinfo, power 等）
10. numatune

### 数据流

1. `VMConfigApp` 持有 `VMConfig` 状态
2. 各面板通过 `show(ui, &mut config)` 编辑配置
3. `XMLGenerator::generate(&config)` 序列化为 libvirt XML
4. 支持导出到文件、复制到剪贴板、预览

## 关键依赖

- `eframe` / `egui` (0.27) - GUI 框架
- `quick-xml` (0.38) - XML 序列化（手写事件流）
- `serde` (1.0) - 数据模型序列化
- `uuid` (1.7) - UUID 生成
- `rfd` (0.14) - 文件对话框

## 代码规范

- 格式化：`rustfmt`（配置见 `rustfmt.toml`），最大行宽 100，4 空格缩进
- Lint: `clippy`（配置见 `clippy.toml`）
- 导入分组：`StdExternalCrate` 方式分组
- 代码风格：使用 `?` 操作符处理错误，字段初始化使用简写语法

## 注意事项

- 项目使用中文 UI 和 MapleMono 字体，字体文件位于 `resources/fonts/MapleMonoNormal-NF-CN-Regular.ttf`
- XML 生成器采用手写方式而非 `serde_xml` 派生，以便更精确控制输出格式
- `libvirtd_xml.md` 包含 libvirt XML 规范的详细参考文档
- 新增配置项时需同步修改三处：`model/`、`xml_gen/`、`panels/`
