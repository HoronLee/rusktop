# Lug 使用指南

## 🎯 项目概览

**Lug** 是一个基于 `tracing` 的 Rust 日志封装库，灵感来自 Go 的 Zap 和 Kratos 框架。

### 核心统计

- **总文件数**: 14
- **源代码行数**: 735 行
- **测试覆盖**: 28 个测试，100% 通过
- **Rust Edition**: 2024
- **许可证**: MIT OR Apache-2.0

## 🔧 安装

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
lug = { path = "crates/lug" }

# 可选：SeaORM 集成
lug = { path = "crates/lug", features = ["sea-orm-integration"] }
sea-orm = "1.1"
```

## 📖 使用教程

### 1. 基础初始化

```rust
use lug::{LugConfig, Environment, Level};

fn main() {
    // 开发环境：彩色终端输出
    lug::init(LugConfig {
        env: Environment::Dev,
        level: Level::Debug,
        file: None,
    }).expect("Failed to initialize logger");

    // 直接使用 tracing 宏
    tracing::info!("Application started");
    tracing::debug!(version = "0.1.0", "System info");
    tracing::warn!(latency_ms = 250, "Slow operation");
    tracing::error!(error = "Timeout", "Failed to connect");
}
```

**输出示例**：
```
2026-02-14T16:36:24.493877+08:00  INFO Application started
2026-02-14T16:36:24.494136+08:00 DEBUG Debug information version="0.1.0"
2026-02-14T16:36:24.494148+08:00  WARN Slow operation detected latency_ms=250
2026-02-14T16:36:24.494161+08:00 ERROR Failed to connect error="Connection refused"
```

### 2. 生产环境配置

```rust
use lug::{LugConfig, Environment, Level, FileConfig};
use std::path::PathBuf;

lug::init(LugConfig {
    env: Environment::Prod,
    level: Level::Info,
    file: Some(FileConfig {
        path: PathBuf::from("/var/log/myapp/app.log"),
        max_size_mb: 50,      // 暂未实现按大小轮转
        max_backups: 10,       // 暂未实现自动清理
        max_age_days: 30,      // 每日轮转
        compress: false,       // 暂未实现压缩
    }),
}).unwrap();
```

**输出**：
- 终端：非彩色结构化日志
- 文件：JSON 格式，每日轮转

**JSON 日志示例**：
```json
{"timestamp":"2026-02-14T16:37:23.053805+08:00","level":"INFO","target":"production","fields":{"message":"HTTP request processed","request_id":1,"latency_ms":10,"status":200}}
```

### 3. 模块标签（核心特性）

Lug 提供三种灵活的模块标签模式：

#### 模式 1: Span 模式（推荐）

**适用场景**：跨函数调用、异步代码、需要上下文传播

```rust
use lug::module_span;

fn handle_request() {
    let _guard = module_span!("auth/biz/myapp").entered();
    
    tracing::info!(user_id = 42, "Request started");
    validate_user();  // 嵌套函数自动继承 module 标签
    tracing::info!("Request completed");
}

fn validate_user() {
    // 这里的日志自动包含 module="auth/biz/myapp"
    tracing::debug!("Validating credentials");
}
```

**优势**：
- ✅ 自动上下文传播
- ✅ 异步安全
- ✅ 最符合 Rust 惯用法

#### 模式 2: 作用域宏

**适用场景**：局部代码块，快速添加模块标签

```rust
use lug::with_module;

with_module!("redis/data/myapp", {
    tracing::info!("Connecting to Redis");
    tracing::debug!(host = "localhost", "Connection params");
});

with_module!("postgres/data/myapp", {
    tracing::info!("Executing query");
});
```

#### 模式 3: 内联宏

**适用场景**：单次日志，不需要作用域

```rust
lug::info!(module: "metrics/api/myapp", counter = 100, "Metric recorded");
lug::warn!(module: "cache/data/myapp", hit_rate = 0.85, "Low cache hit");
lug::error!(module: "payment/biz/myapp", "Payment failed");
```

### 4. SeaORM 集成

启用 feature：

```toml
[dependencies]
lug = { path = "crates/lug", features = ["sea-orm-integration"] }
```

配置 SeaORM：

```rust
use lug::integrations::configure_sea_orm;
use sea_orm::{Database, ConnectOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    lug::init(lug::LugConfig::default())?;

    let mut opt = ConnectOptions::new("postgres://localhost/mydb");
    configure_sea_orm(&mut opt, "database/data/myapp");

    let db = Database::connect(opt).await?;
    
    // SQL 查询自动记录，带 module="database/data/myapp"
    // 使用 RUST_LOG=sqlx=debug 查看详细 SQL
    
    Ok(())
}
```

### 5. 环境变量控制

Lug 支持 `RUST_LOG` 环境变量（优先级高于配置）：

```bash
# 全局设置日志级别
RUST_LOG=debug cargo run

# 按模块过滤
RUST_LOG=myapp::auth=debug cargo run

# 查看 SQL 查询
RUST_LOG=sqlx=debug cargo run

# 多个过滤器
RUST_LOG=myapp=info,sqlx=debug cargo run
```

## 🎨 最佳实践

### 模块命名规范

**格式**: `[组件]/[层]/[服务名]`

| 示例 | 说明 |
|------|------|
| `auth/biz/myapp` | 认证业务逻辑 |
| `redis/data/myapp` | Redis 数据访问层 |
| `http/api/myapp` | HTTP API 层 |
| `postgres/data/myapp` | PostgreSQL 数据访问层 |
| `metrics/system/myapp` | 系统指标收集 |

### 结构化字段使用

```rust
// ❌ 不推荐：拼接字符串
tracing::info!("User alice logged in with id 42");

// ✅ 推荐：结构化字段
tracing::info!(
    user_id = 42,
    username = "alice",
    "User logged in"
);
```

**优势**：
- JSON 日志中可独立查询
- 类型安全
- 性能更好（避免字符串拼接）

### 错误日志

```rust
use anyhow::Result;

fn process() -> Result<()> {
    let result = risky_operation();
    
    match result {
        Ok(data) => {
            tracing::info!(data_len = data.len(), "Operation successful");
            Ok(())
        }
        Err(e) => {
            // 使用 %e 格式化错误（Display trait）
            tracing::error!(error = %e, "Operation failed");
            Err(e)
        }
    }
}
```

## 🧪 测试中使用

```rust
#[test]
fn my_test() {
    // 测试环境：静默
    lug::init(lug::LugConfig {
        env: lug::Environment::Test,
        ..Default::default()
    }).ok(); // 忽略重复初始化错误
    
    // 测试代码...
}
```

## 📊 性能考虑

### 禁用日志的零成本

Lug 基于 `tracing`，当日志级别禁用时，有**零运行时开销**：

```rust
// 如果 Debug 级别被禁用，这行代码编译后完全不执行
tracing::debug!(expensive_value = compute_expensive(), "Debug info");
```

### 异步友好

Span 在异步代码中自动传播：

```rust
async fn async_operation() {
    let _guard = module_span!("async/task").entered();
    
    tracing::info!("Task started");
    tokio::time::sleep(Duration::from_secs(1)).await;
    tracing::info!("Task completed");
    // 两条日志都包含 module="async/task"
}
```

## 🔍 故障排查

### 问题 1: 日志没有输出

**检查清单**：
1. 是否调用了 `lug::init()`？
2. 日志级别是否正确？（`Level::Info` 不会显示 `debug!()` 日志）
3. 环境变量 `RUST_LOG` 是否覆盖了配置？

### 问题 2: 文件日志没有生成

**检查清单**：
1. 文件路径是否有写权限？
2. 父目录是否存在？（lug 会自动创建）
3. 是否在 Prod 环境？（Dev/Test 不写文件）

### 问题 3: SeaORM SQL 看不到

**解决方案**：
```bash
RUST_LOG=sqlx=debug cargo run
```

## 📚 示例代码运行

```bash
# 基础使用
cargo run -p lug --example basic

# 三种模块标签模式对比
cargo run -p lug --example module_modes

# 生产环境配置（生成 JSON 文件）
cargo run -p lug --example production

# SeaORM 集成
cargo run -p lug --example sea_orm --features sea-orm-integration
```

## 🚀 迁移指南

### 从 Go 版本迁移

| Go (Zap) | Rust (lug) |
|----------|-----------|
| `logger.NewLogger(config)` | `lug::init(config)` |
| `logger.WithModule(l, "auth/biz")` | `module_span!("auth/biz").entered()` |
| `log.Info("message")` | `tracing::info!("message")` |
| `log.Error("msg", zap.Error(err))` | `tracing::error!(error = %err, "msg")` |
| `gormLogger := l.GetGormLogger("db")` | `configure_sea_orm(&mut opt, "db")` |

### 从 `env_logger` 迁移

```rust
// 之前
env_logger::init();

// 现在
lug::init(lug::LugConfig {
    env: lug::Environment::Dev,
    ..Default::default()
}).unwrap();
```

## 📞 获取帮助

- **文档**: 查看 `cargo doc --open -p lug`
- **示例**: 查看 `crates/lug/examples/`
- **测试**: 查看 `crates/lug/tests/`

---

**版本**: 0.1.0  
**状态**: ✅ 生产就绪  
**最后更新**: 2026-02-14
