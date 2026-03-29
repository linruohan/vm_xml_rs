# VM XML 配置生成器

## 项目概述

VM XML 配置生成器是一个基于 Rust 和 egui 的图形化工具，用于生成和管理虚拟机的 XML 配置文件。它提供了直观的用户界面，帮助用户轻松配置虚拟机的各种参数，无需手动编辑 XML 文件。

## 功能特点

- **直观的图形界面**：基于 egui 构建的现代化界面，支持深色/浅色主题切换
- **全面的配置选项**：
  - 基础配置：虚拟机名称、类型、UUID等
  - 系统引导：操作系统类型、内核参数等
  - CPU 配置：核心数、线程数、模式等
  - 内存配置：内存大小、内存限制等
  - 设备管理：网络、磁盘、控制器、输入设备等
  - 高级配置：SMBIOS、IO 线程、CPU 调优、内存调优等
- **实时 XML 预览**：配置修改后可立即查看生成的 XML 内容
- **XML 导出功能**：支持将配置导出为 XML 文件
- **剪贴板支持**：可将生成的 XML 复制到剪贴板
- **中文支持**：内置中文字体，支持中文界面

## 系统要求

- Rust 1.60+ （推荐使用最新稳定版）
- 支持的操作系统：
  - Windows
  - Linux
  - macOS

## 安装说明

### 从源码构建

1. 克隆项目仓库：

```bash
git clone https://github.com/linruohan/vm_xml_rs.git
cd vm_xml_rs
```

2. 构建项目：

```bash
cargo build --release
```

3. 运行应用：

```bash
cargo run --release
```

### 直接运行

如果您不想从源码构建，可以下载预编译的可执行文件（如果有提供）。

## 使用指南

1. **启动应用**：运行 `vm_xml_tool` 可执行文件

2. **配置虚拟机**：
   - 在左侧标签页中选择要配置的类别
   - 填写或修改相应的配置参数
   - 实时预览生成的 XML 内容

3. **导出配置**：
   - 点击底部的 "💾 导出 XML" 按钮
   - 选择保存位置和文件名
   - 点击 "保存" 完成导出

4. **复制到剪贴板**：
   - 点击底部的 "📋 预览 XML" 按钮
   - 在预览窗口中点击 "📋 复制" 按钮

5. **主题切换**：
   - 点击顶部菜单 "视图" -> "主题切换"
   - 选择 "浅色"、"深色" 或 "蓝色" 主题

## 项目结构

```
vm_xml_rs/
├── src/
│   ├── model/           # 数据模型定义
│   │   ├── devices/     # 设备相关模型
│   │   └── vm_config.rs # 虚拟机配置主模型
│   ├── panels/          # UI 面板
│   │   ├── advanced/    # 高级配置面板
│   │   └── utils.rs     # 工具函数
│   ├── xml_gen/         # XML 生成器
│   │   ├── devices/     # 设备 XML 生成
│   │   └── mod.rs       # XML 生成器主模块
│   ├── app.rs           # 应用主逻辑
│   └── main.rs          # 程序入口
├── resources/           # 资源文件
│   ├── fonts/           # 字体文件
│   └── mytool.png       # 应用图标
├── Cargo.toml           # 项目配置文件
└── README.md            # 项目说明文档
```

## 技术栈

- **前端**：egui (Rust GUI 库)
- **XML 处理**：quick-xml, serde-xml-rs
- **数据序列化**：serde
- **文件对话框**：rfd
- **唯一标识符**：uuid

## 许可证

本项目采用 MIT 许可证，详情请查看 [LICENSE](LICENSE) 文件。

## 贡献

欢迎贡献到本项目！如果您有任何建议或问题，请提交 Issue 或 Pull Request。

## 联系信息

- 作者：linruohan
- 项目地址：[https://github.com/linruohan/vm_xml_rs](https://github.com/linruohan/vm_xml_rs)

## 致谢

- 感谢 [egui](https://github.com/emilk/egui) 提供的优秀 GUI 库
- 感谢 [libvirt](https://libvirt.org/) 提供的虚拟机配置标准

---

祝您使用愉快！ 🎉
