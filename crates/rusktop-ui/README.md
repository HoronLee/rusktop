# Rusktop UI 层代码组织

## 目录结构

```
crates/rusktop-ui/src/
├── lib.rs              # 模块导出和公共 API
├── views/              # 视图组件
│   ├── mod.rs
│   └── web_service.rs  # Web 服务控制视图
└── windows/            # 窗口管理
    ├── mod.rs
    └── root.rs         # 根窗口配置
```

## 设计原则

### 封装 GPUI 框架细节

`rusktop-ui` 完全封装了 GPUI 框架的实现细节，使得 `rusktop-app` 层：
- **不需要**直接依赖 `gpui` 或 `gpui-component`
- **不需要**了解 GPUI 的 API（如 `Application::new()`、`gpui_component::init()` 等）
- **只需要**调用 `rusktop_ui::run()` 即可启动 UI 应用

这遵循了严格的分层架构原则：**App → UI → GPUI**

### 1. **Views（视图层）**
- **职责**：可复用的 UI 组件
- **特点**：
  - 实现 `Render` trait
  - 包含业务逻辑
  - 可以在不同窗口中使用
- **示例**：`WebServiceView` - Web 服务控制面板

### 2. **Windows（窗口层）**
- **职责**：窗口创建和配置
- **特点**：
  - 定义窗口选项（大小、标题等）
  - 组装视图到窗口
  - 配置 `Root` 包装器
- **示例**：`root.rs` - 主窗口配置

## 公共 API

### `run()` - 启动 UI 应用

```rust
pub fn run()
```

这是主要的入口函数，封装了：
- `gpui::Application::new()` - 创建 GPUI 应用
- `gpui_component::init()` - 初始化组件库
- 窗口创建和配置

**使用示例（在 rusktop-app 中）**：

```rust
#[cfg(feature = "ui")]
fn start_ui() {
    rusktop_ui::run();  // 简单调用，无需了解 GPUI
}
```

### 其他导出（高级用法）

如果需要更细粒度的控制，也可以使用：
- `create_root_window()` - 创建根窗口
- `default_window_options()` - 默认窗口选项

## 模块说明

### `views/web_service.rs`
Web 服务控制视图，提供：
- 端口输入
- 启动/停止按钮
- 状态显示
- Web 服务器生命周期管理

### `windows/root.rs`
根窗口配置，提供：
- `create_root_window()` - 创建根窗口
- `default_window_options()` - 默认窗口选项

## 扩展指南

### 添加新视图
1. 在 `views/` 目录创建新文件
2. 实现 `Render` trait
3. 在 `views/mod.rs` 中导出
4. 在窗口中使用

### 添加新窗口
1. 在 `windows/` 目录创建新文件
2. 定义窗口选项和创建函数
3. 在 `windows/mod.rs` 中导出
4. 在 `run()` 或自定义启动函数中调用

## 设计优势

1. **清晰的分层**：视图与窗口分离，框架细节完全封装
2. **易于扩展**：添加新组件或窗口很简单
3. **可复用性**：视图可以在多个窗口中使用
4. **易于测试**：各层可以独立测试
5. **架构一致性**：与 `rusktop_core::run()` 对称，统一的启动模式
