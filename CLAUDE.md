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

### 核心模块

- **`src/main.rs`** - 应用入口，配置 egui 字体（MapleMono）和窗口参数，加载应用图标
- **`src/app.rs`** - 主应用结构 `VMConfigApp`，管理 UI 状态、菜单栏、标签页导航
- **`src/model/vm_config.rs`** - VM 配置数据模型根结构，组合所有子配置模块
- **`src/xml_gen/mod.rs`** - XML 生成器入口，使用 `quick-xml` 库手动构建 XML 事件流
- **`src/panels/`** - UI 面板模块，每个面板对应一个配置类别

### XML 生成器结构

`src/xml_gen/` 采用模块化设计，每个子模块负责特定类别的 XML 生成：

- `general.rs` - 基础配置（name, UUID, metadata）
- `os.rs` - 引导配置（type, arch, boot, loader）
- `cpu.rs` - CPU/vCPU 配置
- `memory.rs` - 内存配置
- `devices/` - 设备配置（disk, network, input, sound, graphics, serial, tpm, controller, filesystem, rng, watchdog）
- `tuning.rs` - CPU 调优（cputune）
- `misc.rs` - 杂项（clock, features, perf, iothreads, events, numatune）
- `advanced/` - 高级配置（power management, sysinfo/SMBIOS 等）

### 面板结构

基础面板：
- `general_panel.rs` - 基础配置（VM 名称、UUID、vCPU、内存）
- `os_panel.rs` - 系统引导配置
- `cpu_panel.rs` - CPU 配置
- `memory_panel.rs` - 内存配置
- `devices_panel.rs` - 设备配置（磁盘、网络、串口等）

高级面板（位于 `src/panels/advanced/`）：
- SMBIOS、IO 线程、CPU 调优、内存调优、NUMA、块 IO 调优、资源分区、光纤通道 VMID、事件处理、电源管理、磁盘限流组、虚拟机监控特性、时间同步、性能监控、安全标签、密钥封装、启动安全

### 数据流

1. `VMConfigApp` 持有 `VMConfig` 状态
2. 各面板通过 `show(ui, &mut config)` 编辑配置
3. `XMLGenerator::generate(&config)` 将配置序列化为 libvirt XML 格式
4. 支持导出到文件、复制到剪贴板、预览

## 关键依赖

- `eframe` / `egui` (0.27) - GUI 框架
- `quick-xml` (0.31) - XML 序列化（使用手写事件流而非派生宏）
- `serde` (1.0) - 数据模型序列化支持
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
