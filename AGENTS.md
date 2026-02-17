# AGENTS.md - Rusktop 开发指南

本指南为在 Rusktop 项目上工作的智能编码助手提供必要信息。

## 项目概览

Rusktop 是一个双模式 Rust 应用程序：
- **UI 模式**：使用 GPUI 框架的桌面应用程序
- **Web 模式**：使用 Axum/Tonic 的 REST/gRPC API 服务器
- **架构**：三层设计（Core → UI/Web → App）

### 工作空间结构

```
crates/
├── rusktop-app      # 应用程序入口点（CLI，双模式调度器）
├── rusktop-core     # 纯业务逻辑（entity、data、biz 层）
├── rusktop-ui       # GPUI 桌面 UI 组件
│   ├── views/       # UI 视图组件（可复用）
│   └── windows/     # 窗口配置和管理
├── rusktop-web      # Axum/Tonic Web 服务
├── konfig           # 配置管理库
└── lug              # 结构化日志包装器
```

#### rusktop-ui 目录结构

```
rusktop-ui/src/
├── lib.rs           # 模块导出
├── views/           # 视图组件（实现 Render trait）
│   ├── mod.rs
│   └── web_service.rs  # Web 服务控制视图
└── windows/         # 窗口管理
    ├── mod.rs
    └── root.rs      # 根窗口配置
```

- **views/**：可复用的 UI 组件，包含业务逻辑
- **windows/**：窗口创建、配置和 Root 包装

## 构建与测试命令

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
cargo test -p rusktop-web

# 运行单个测试
cargo test -p rusktop-core test_name

# 运行测试并显示输出
cargo test -- --nocapture
```

### 代码检查与格式化

```bash
# 检查 Rust 代码
cargo clippy
cargo clippy --all-targets --all-features

# 格式化 Rust 代码
cargo fmt

# 检查格式（不应用更改）
cargo fmt -- --check

# 检查 protobuf 文件
make lint-proto

# 格式化 protobuf 文件
make fmt-proto
```

### Proto/OpenAPI 工作流

```bash
# 从 protos 生成 OpenAPI 文档
make openapi

# 检查 proto 工作流一致性
make check-proto-workflow

# 创建新的数据库迁移
make new-migration name=add_email_to_users
```

**注意**：Proto 代码生成通过 `build.rs` 自动完成，无需手动运行生成命令。

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
use rusktop_core::entity::user;

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
pub mod counter;
pub mod entity;
pub mod data;
pub mod biz;

pub use counter::Counter;  // 扁平化常用项
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

1. **Core 层**（`rusktop-core`）：纯业务逻辑，无框架依赖
   - `entity/`：Sea-ORM 模型
   - `data/`：仓储 trait 和实现
   - `biz/`：用例 trait 和业务逻辑

2. **UI/Web 层**：框架特定实现
   - `rusktop-ui`：GPUI 组件，封装所有 GPUI 框架细节
   - `rusktop-web`：Axum 路由、Tonic 服务

3. **App 层**（`rusktop-app`）：入口点、依赖装配
   - **重要**：App 层不直接依赖 GPUI 或其他 UI 框架
   - UI 模式通过 `rusktop_ui::run()` 启动
   - Web 模式通过 `rusktop_web::run()` 启动

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

1. 在 `rusktop-core/src/entity/` 中创建实体
2. 在 `rusktop-web/src/migration/` 中创建迁移
3. 在 `rusktop-core/src/data/` 中添加仓储 trait
4. 在 `rusktop-core/src/biz/` 中添加用例 trait
5. 在 `rusktop-web/src/service/` 中实现服务
6. 在 `rusktop-web/src/di/mod.rs` 中装配

### 添加新 API 端点

1. 在 `api/protos/` 中定义 proto
2. 运行 `make openapi` 重新生成 OpenAPI 文档
3. 运行 `make check-proto-workflow` 进行验证
4. 在 `rusktop-web/src/service/` 中实现服务
5. 构建以触发 proto 代码生成：`cargo build -p rusktop-web`

### 调试

```bash
# 使用 debug 日志运行
RUST_LOG=debug cargo run

# 为特定模块使用 trace 日志
RUST_LOG=rusktop_web=trace cargo run

# 使用 lug 日志配置
# 编辑 config.toml：
[log]
level = "debug"  # trace, debug, info, warn, error
```

## 关键依赖

- **UI**：`gpui`、`gpui-component`
- **Web**：`axum`、`tonic`、`tonic-rest`
- **数据库**：`sea-orm`、`sea-orm-migration`
- **异步**：`tokio`、`async-trait`
- **配置**：`konfig`（workspace）、`serde`
- **日志**：`lug`（workspace）、`tracing`
- **Protobuf**：`prost`、`prost-types`

## 版本要求

- Rust：1.85+
- Edition：2024
- Resolver：3
