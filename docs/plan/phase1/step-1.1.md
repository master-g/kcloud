# Step 1.1: 项目初始化 - Cargo.toml

**目标**: 创建项目配置文件，添加所有依赖

**涉及文件**:
- `Cargo.toml`

**具体改动**:
- 创建 Cargo.toml
- 添加依赖：
  - `clap` - CLI 参数解析
  - `tokio` - 异步运行时 (full features)
  - `reqwest` - HTTP 客户端
  - `ratatui` - TUI 框架
  - `thiserror` - 自定义错误类型
  - `anyhow` - 灵活错误处理
  - `tracing` - 日志系统
  - `tracing-subscriber` - 日志输出
  - `serde` / `serde_json` - 序列化
  - `toml` - TOML 配置解析

**验收标准**: `cargo check` 通过

**提交信息**: `init: add dependencies to Cargo.toml`
