# Rusktop - GPUI 桌面应用示例

基于 `gpui` 和 `gpui-component` 的桌面应用程序框架示例。

## 架构设计

本项目采用三层架构，实现了清晰的职责分离：

```
┌─────────────────────────────────────┐
│         rusktop-app                 │  应用入口层
│  - 应用启动                         │
│  - 窗口管理                         │
│  - 全局配置                         │
│  依赖: gpui, gpui-component,        │
│        rusktop-ui, rusktop-core     │
└─────────────────────────────────────┘
              ↓
┌─────────────────────────────────────┐
│         rusktop-ui                  │  UI 组件层
│  - 视图组件定义                     │
│  - UI 状态管理                      │
│  - 事件处理                         │
│  依赖: gpui, gpui-component, core   │
└─────────────────────────────────────┘
              ↓
┌─────────────────────────────────────┐
│         rusktop-core                │  业务逻辑层
│  - 纯业务逻辑                       │
│  - 数据模型                         │
│  - 算法实现                         │
│  依赖: 无 (完全独立)                │
└─────────────────────────────────────┘
```

### 各层职责

#### 1. `rusktop-core` (核心层)
- **职责**: 纯业务逻辑，不依赖任何 UI 框架
- **包含**: 数据模型、业务规则、算法
- **优势**: 可独立测试、可复用于其他前端（如 CLI、Web）
- **示例**: `Counter` 结构体及其方法

#### 2. `rusktop-ui` (UI 层)
- **职责**: UI 组件定义和渲染逻辑
- **包含**: `Render` trait 实现、gpui-component 组件使用
- **依赖**: `rusktop-core` (使用业务模型)、`gpui`、`gpui-component`
- **示例**: `CounterView` 实现 `Render` trait

#### 3. `rusktop-app` (应用层)
- **职责**: 应用程序入口和生命周期管理
- **包含**: `main` 函数、窗口配置、全局初始化
- **依赖**: `rusktop-ui`、`rusktop-core`、`gpui`、`gpui-component`
- **示例**: 创建 `Application`、配置窗口、挂载根视图

## 运行示例

```bash
cargo run -p rusktop-app
```

## 项目结构

```
rusktop/
├── Cargo.toml                   # Workspace 配置
└── crates/
    ├── rusktop-core/            # 核心业务逻辑
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs
    │       └── counter.rs       # 计数器业务逻辑
    ├── rusktop-ui/              # UI 组件
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs
    │       └── counter_view.rs  # 计数器视图组件
    └── rusktop-app/             # 应用入口
        ├── Cargo.toml
        └── src/
            └── main.rs          # 应用启动入口
```

## 关键技术点

### 1. GPUI 应用启动

```rust
let app = Application::new();
app.run(|cx: &mut App| {
    // 必须先初始化 gpui-component
    gpui_component::init(cx);
    
    // 打开窗口
    cx.open_window(window_options, |_, cx| {
        cx.new(|_| MyView::new())
    });
});
```

### 2. 视图定义

```rust
use gpui_component::button::{Button, ButtonVariants};  // 需要导入 ButtonVariants trait

pub struct CounterView {
    counter: Counter,  // 来自 core 层
}

impl Render for CounterView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 使用 gpui-component 的组件
        v_flex()
            .child(Button::new("btn")
                .primary()  // 使用 ButtonVariants trait 的方法
                .on_click(cx.listener(|view, _, _, _| {
                    view.counter.increment();
                })))
    }
}
```

### 3. 事件处理

使用 `cx.listener()` 创建事件处理器：

```rust
.on_click(cx.listener(|view, _event, _window, _cx| {
    view.counter.increment();  // 修改状态会自动触发重新渲染
}))
```

## 扩展建议

### 添加新功能

1. **Core 层**: 定义业务模型和逻辑
   ```rust
   // crates/rusktop-core/src/todo.rs
   pub struct TodoList { ... }
   ```

2. **UI 层**: 创建对应的视图组件
   ```rust
   // crates/rusktop-ui/src/todo_view.rs
   pub struct TodoView { ... }
   impl Render for TodoView { ... }
   ```

3. **App 层**: 在 main.rs 中挂载新视图
   ```rust
   cx.new(|_| TodoView::new())
   ```

### 状态管理进阶

对于跨组件共享的状态，可以使用 `Model`:

```rust
// Core 层定义 Model
let counter_model = cx.new_model(|_| Counter::new());

// UI 层使用 Model
pub struct CounterView {
    counter: Entity<Counter>,
}

// 监听 Model 变化
counter.read(cx).value()
```

## 参考资源

- [GPUI 官方文档](https://github.com/zed-industries/zed)
- [gpui-component 文档](https://longbridge.github.io/gpui-component/)
- [Zed 源码示例](https://github.com/zed-industries/zed/tree/main/crates/gpui/examples)
