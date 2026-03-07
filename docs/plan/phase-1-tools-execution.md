# 阶段 1：工具与执行 (Tools & Execution)

## 阶段目标

Agent 循环，工具，安全机制，格式化显示

## 主要内容

1. **Agent 循环** - 使用状态机模式实现持续运行的交互循环
2. **内置工具**（硬编码）：
   - `read` - 读取文件（含路径安全检查）
   - `write` - 写入文件（含路径安全检查）
   - `edit` - 编辑文件（含路径安全检查）
   - `bash` - 执行命令（含命令安全限制）
3. **安全机制**：
   - 路径检查：防止访问工作目录外的文件（不允许 `../` 向上穿越）
   - 工作目录隔离：限定操作范围
4. **用户界面**：
   - 工具返回结果的格式化显示
   - 清晰的错误提示

## 实施步骤

### 1.1 项目初始化

**步骤 1.1.1**: 创建项目结构

- 创建 `Cargo.toml` 配置文件
- 依赖：
  - `clap` - CLI 参数解析
  - `tokio` - 异步运行时
  - `reqwest` - HTTP 客户端
  - `ratatui` - TUI 框架
  - `thiserror` - 自定义错误类型
  - `anyhow` - 灵活错误处理
  - `tracing` - 日志系统
  - `serde` / `serde_json` - 序列化
  - `toml` - TOML 配置解析

**步骤 1.1.2**: 创建基础目录结构

```
src/
├── main.rs
├── lib.rs
├── cli.rs       # CLI 参数解析
├── config.rs    # 配置管理
├── error.rs     # 错误类型定义
├── state.rs     # Agent 状态机
├── tools/       # 工具模块
│   ├── mod.rs
│   ├── read.rs
│   ├── write.rs
│   ├── edit.rs
│   └── bash.rs
├── agent.rs     # Agent 核心逻辑
└── ui.rs        # 用户界面
```

**步骤 1.1.3**: 配置加载实现

- 从 `~/.config/kloud/config.toml` 读取配置
- 支持环境变量覆盖（优先）
- 配置项：
  - `api_key` - LLM API 密钥
  - `api_base_url` - API 基础 URL
  - `model` - 使用的模型
  - `workdir` - 工作目录
  - `max_concurrent` - 最大并发数

### 1.2 Agent 状态机实现

**步骤 1.2.1**: 定义状态类型

```rust
enum AgentState {
    Idle,              // 等待用户输入
    Thinking,           // 正在思考（调用 LLM）
    ExecutingTool,      // 执行工具中
    WaitingConfirm,    // 等待用户确认
    Error,             // 错误状态
}
```

**步骤 1.2.2**: 实现状态转换逻辑

- Idle → Thinking: 用户输入
- Thinking → ExecutingTool: LLM 返回工具调用
- ExecutingTool → Thinking: 工具执行完成
- Thinking → Idle: LLM 返回最终响应
- 任意 → Error: 发生错误
- Error → Idle: 用户取消或重试

### 1.3 工具实现

**步骤 1.3.1**: 路径安全检查

```rust
fn is_path_safe(path: &Path, workdir: &Path) -> bool {
    // 解析绝对路径，检查是否在工作目录内
    // 不允许 ../ 向上穿越
}
```

**步骤 1.3.2**: 实现 read 工具

- 读取指定文件内容
- 支持行号范围
- 路径安全检查

**步骤 1.3.3**: 实现 write 工具

- 创建新文件或覆盖现有文件
- 路径安全检查

**步骤 1.3.4**: 实现 edit 工具

- 支持行号指定编辑
- 路径安全检查

**步骤 1.3.5**: 实现 bash 工具

- 执行 shell 命令
- 安全限制（可选：命令白名单）
- 超时处理
- 输出捕获

### 1.4 用户界面

**步骤 1.4.1**: 格式化输出

- 工具结果使用格式化的文本显示
- 错误信息清晰明确

**步骤 1.4.2**: 进度指示

- 执行工具时显示等待状态
- 可选：使用 ratatui 实现 TUI

## 验收标准

- [ ] 项目可以成功编译
- [ ] Agent 可以持续运行，等待用户输入
- [ ] read/write/edit/bash 工具可以正常工作
- [ ] 路径安全检查可以阻止目录穿越攻击
- [ ] 错误信息清晰可读
- [ ] 单元测试通过

## 技术细节

### 配置文件格式 (TOML)

```toml
[llm]
api_base_url = "https://api.openai.com/v1"
model = "gpt-4"

[agent]
workdir = "."
max_concurrent = 5

[tools]
# 可选：命令白名单
allowed_commands = ["git", "cargo", "npm", "node"]
```

### 环境变量

- `KLOUD_API_KEY` - API 密钥（优先于配置文件）
- `KLOUD_API_BASE_URL` - API 基础 URL
- `KLOUD_MODEL` - 使用的模型
- `KLOUD_WORKDIR` - 工作目录
