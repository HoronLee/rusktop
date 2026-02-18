# Rusktop

Rusktop 是一个**双模式 Rust 应用程序**：
- **UI 模式**：使用 GPUI 框架的桌面应用程序
- **Web 模式**：使用 Axum/Tonic 的 REST/gRPC API 服务器
- **架构**：三层设计（Core → UI → App）

## 快速开始

### 运行应用程序

```bash
# 启动 UI 应用（默认）
make run.ui
# 或
cargo run

# 启动 Web 服务器
make run.web
# 或
cargo run --no-default-features -- web

# 使用自定义 host/port 启动 Web
cargo run --no-default-features -- web --host 0.0.0.0 --port 3000
```

### 构建

```bash
# 构建完整二进制（UI + Web）
make build.ui
# 输出：target/release/rusktop-app

# 仅构建 Web 二进制（更小，无 UI 依赖）
make build.web
# 输出：target/release/rusktop-app

# 开发构建
cargo build
```

### 测试

```bash
# 运行所有测试
cargo test

# 测试特定 crate
cargo test -p rusktop-core

# 运行单个测试
cargo test -p rusktop-core test_name

# 运行测试并显示输出
cargo test -- --nocapture
```

## 架构设计

本项目采用三层架构，实现了清晰的职责分离：

```
┌─────────────────────────────────────┐
│         rusktop-app                 │  应用入口层
│  - CLI 入口                         │
│  - 双模式调度器                     │
│  - 窗口管理                         │
│  - 依赖装配                         │
└─────────────────────────────────────┘
        ↓                     ↓
┌──────────────────┐          │
│   rusktop-ui     │          │  UI 层
│  - GPUI 组件     │          │
│  - 视图渲染      │          │
│  - 事件处理      │          │
└──────────────────┘          │
        ↓                     ↓
┌─────────────────────────────────────┐
│         rusktop-core                │  业务逻辑 + Web 服务层
│  - data/entity: Sea-ORM 模型       │
│  - data: 仓储 trait 和实现          │
│  - biz: 用例 trait 和业务逻辑       │
│  - service: gRPC/REST 服务实现      │
│  - server: HTTP 服务器              │
│  - di: 依赖注入装配                 │
└─────────────────────────────────────┘
```

### 工作空间结构

```
crates/
├── rusktop-app      # 应用程序入口点（CLI，双模式调度器）
├── rusktop-core     # 业务逻辑 + Web 服务（data、biz、service、server）
├── rusktop-ui       # GPUI 桌面 UI 组件
├── konfig           # 配置管理库
└── lug              # 结构化日志包装器
```

### 各层职责

#### 1. Core 层（`rusktop-core`）
- **职责**: 业务逻辑 + Web 服务
- **包含**: 
  - `data/entity/`：Sea-ORM 模型（Persistent Objects）
  - `data/`：仓储 trait 和实现
  - `biz/`：用例 trait 和业务逻辑
  - `service/`：gRPC/REST 服务实现
  - `server/`：HTTP 服务器（Axum 路由 + Swagger）
  - `di/`：依赖注入装配
- **优势**: biz 和 service 紧密结合，符合 DDD 设计

#### 2. UI 层（`rusktop-ui`）
- GPUI 桌面 UI 组件
  - `Render` trait 实现
  - GPUI 组件使用
  - 事件处理
- 封装所有 GPUI 框架细节

#### 3. App 层（`rusktop-app`）
- **职责**: 入口点、依赖装配、模式调度
- **包含**: `main` 函数、窗口配置、全局初始化
- **重要**: App 层不直接依赖 GPUI 或其他 UI 框架

## 代码风格指南

### 导入组织

按以下顺序组织导入：
1. 标准库（`std`、`core`）
2. 外部 crate（按字母顺序）
3. 内部工作空间 crate
4. 相对模块路径（`crate::`、`super::`、`self::`）

```rust
// ✅ 好的示例
use std::net::SocketAddr;
use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{DbErr, EntityTrait};
use tokio::runtime::Runtime;

use rusktop_core::biz::UserUseCase;
use rusktop_core::data::entity::user;

use crate::data::UserRepository;
use super::config::AppConfig;

// ❌ 不好的示例 - 顺序混乱
use crate::data::UserRepository;
use std::sync::Arc;
use async_trait::async_trait;
```

### 错误处理

**库**：使用 `thiserror` 定义自定义错误枚举，并提供 `Result` 类型别名。

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileRead(#[from] std::io::Error),
    
    #[error("Key not found: {0}")]
    KeyNotFound(String),
}

pub type Result<T> = std::result::Result<T, ConfigError>;
```

**应用程序**：使用 `Box<dyn std::error::Error>` 以获得灵活性。

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ...
    Ok(())
}
```

**服务**：将错误映射到适当的类型（例如 gRPC 的 `Status`）。

```rust
let user = self.use_case
    .get_user(req.id)
    .await
    .map_err(|e| Status::internal(e.to_string()))?
    .ok_or(Status::not_found("User not found"))?;
```

### 命名约定

- **类型/Trait**：`PascalCase`（`UserService`、`ConfigBuilder`、`UserRepository`）
- **函数/变量**：`snake_case`（`create_user`、`db_url`、`init_db`）
- **常量**：`SCREAMING_SNAKE_CASE`（`MAX_CONNECTIONS`、`DEFAULT_PORT`）
- **生命周期**：单个小写字母（`'a`、`'static`）

### 模块组织

目录使用 `mod.rs`，在 `lib.rs` 中重新导出公共项：

```rust
// lib.rs
pub mod data;
pub mod biz;

pub use web_service::{ServiceStatus, WebServiceConfig};  // 扁平化常用项
```

模块在父模块中声明一次，然后通过路径访问：

```rust
// ✅ 好的示例 - 在 lib.rs 中声明
pub mod service;

// ❌ 不好的示例 - 不要在其他文件中重新声明
mod service;  // 错误：模块已声明
```

### 异步模式

- 应用程序入口点使用 `#[tokio::main]`
- 异步 trait 方法使用 `async_trait` 宏
- 依赖注入优先使用 `Arc<dyn Trait>`

```rust
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, name: String, age: i32) -> Result<user::Model, DbErr>;
}

pub struct UserUseCaseImpl {
    repo: Arc<dyn UserRepository>,  // 通过 trait 对象实现 DI
}

#[async_trait]
impl UserUseCase for UserUseCaseImpl {
    async fn create_user(&self, name: String, age: i32) -> Result<user::Model, DbErr> {
        self.repo.create(name, age).await
    }
}
```

### 文档

项文档使用 `///`，模块文档使用 `//!`：

```rust
//! 此模块提供用户仓储实现。

/// UserRepositoryImpl 处理用户的数据库操作。
///
/// 它封装了数据库连接，并提供遵循仓储模式的 CRUD 操作。
pub struct UserRepositoryImpl {
    data: Arc<dyn Data>,
}
```

## 架构模式

### 分层架构

严格遵循三层模式：

1. **Core 层**（`rusktop-core`）：业务逻辑 + Web 服务
- `data/entity/`：Sea-ORM 模型（Persistent Objects）
   - `data/`：仓储 trait 和实现
   - `biz/`：用例 trait 和业务逻辑
   - `service/`：gRPC/REST 服务实现
   - `server/`：HTTP 服务器（Axum 路由）
   - `di/`：依赖注入装配

2. **UI 层**（`rusktop-ui`）：GPUI 桌面 UI 组件，封装所有 GPUI 框架细节

3. **App 层**（`rusktop-app`）：入口点、依赖装配

### 依赖注入

使用基于 trait 的 DI，配合 `Arc<dyn Trait>`：

```rust
// 在 core 层定义 trait
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, name: String) -> Result<User>;
}

// 在 data 层实现
pub struct UserRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

// 在 service 中注入
pub struct UserService {
    repo: Arc<dyn UserRepository>,
}
```

## 常见任务

### 添加新实体

1. 在 `rusktop-core/src/data/entity/` 中创建 PO 模型
2. 在 `rusktop-core/src/data/` 中添加仓储 trait
3. 在 `rusktop-core/src/biz/` 中添加用例 trait
4. 在 `rusktop-core/src/service/` 中实现服务
5. 在 `rusktop-core/src/di/mod.rs` 中装配

### 添加新 API 端点

1. 在 `api/protos/` 中定义 proto
2. 运行 `make openapi` 重新生成 OpenAPI 文档
3. 运行 `make check-proto-workflow` 进行验证
4. 在 `rusktop-core/src/service/` 中实现服务
5. 构建以触发 proto 代码生成：`cargo build -p rusktop-core`

### 添加 UI 功能

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

## Proto/OpenAPI 开发规范

### 目录约定

- Proto 源文件：`api/protos/**`
- 文档型 Proto：`*_doc.proto`（仅用于文档注解，不参与 Rust 业务代码生成）
- OpenAPI 输出：`api/openapi/v1/openapi.yaml`
- 构建脚本：`crates/rusktop-core/build.rs`
- Proto workflow 检查脚本：`scripts/check-proto-workflow.sh`

### build.rs 生成规则

`crates/rusktop-core/build.rs` 采用自动扫描模式：
- 扫描 `api/protos` 下所有 `.proto`
- 排除 `*_doc.proto`
- 对结果做稳定排序（避免不同机器顺序不一致）

同时保留：
- `RestCodegenConfig::package(...)` 的 package→Rust module 映射（新增 package 时需要补）
- `cargo:rerun-if-changed=../../api/protos`（proto 变更自动触发重建）

### 新增 Proto 功能的标准流程

假设新增 `order.service.v1`：

1. 在 `api/protos/...` 新增/修改 `.proto`
2. 实现对应业务代码（service/usecase/repo）
3. 补全 package 映射与 module 引入（如新增 package）
4. 执行检查与生成：
   ```bash
   make check-proto-workflow
   make openapi
   cargo build -p rusktop-core
   ```
5. 启动并验证接口：
   ```bash
   make run.web
   ```

### 何时需要手工改动

以下场景需要手工更新：
- 新增了**新的 proto package**：
  - `crates/rusktop-core/build.rs` 中 `RestCodegenConfig::package(...)`
  - `crates/rusktop-core/src/proto/mod.rs` 中 `include_proto!(...)`
- 新增 HTTP 注解服务但 OpenAPI 路径未覆盖：
  - `api/buf.openapi.gen.yaml` 的 `inputs.paths`

### 团队约束

- 禁止手工修改 `target/**/out` 下生成文件
- `*_doc.proto` 只用于文档配置，不进入业务生成链
- 合并代码前至少执行一次：
  ```bash
  make check-proto-workflow
  make openapi
  cargo build -p rusktop-core
  ```

## GPUI 关键技术点

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

### 4. 状态管理进阶

对于跨组件共享的状态，可以使用 `Model`：

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

## 技术栈

### 关键依赖

- **UI**：`gpui`、`gpui-component`
- **Web**：`axum`、`tonic`、`tonic-rest`
- **数据库**：`sea-orm`
- **异步**：`tokio`、`async-trait`
- **配置**：`konfig`（workspace）、`serde`
- **日志**：`lug`（workspace）、`tracing`
- **Protobuf**：`prost`、`prost-types`

### 版本要求

- Rust：1.85+
- Edition：2024
- Resolver：3

## 参考资源

- [GPUI 官方文档](https://github.com/zed-industries/zed)
- [gpui-component 文档](https://longbridge.github.io/gpui-component/)
- [Zed 源码示例](https://github.com/zed-industries/zed/tree/main/crates/gpui/examples)
- [AGENTS.md](./AGENTS.md) - 完整开发指南
- [开发规范](./docs/开发规范.md) - Proto/OpenAPI 工作流详细说明
