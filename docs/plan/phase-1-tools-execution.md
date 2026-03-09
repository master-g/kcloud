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
   - `git` - Git 操作（封装常用 Git 命令）
3. **安全机制**：
   - 路径检查：防止访问工作目录外的文件（不允许 `../` 向上穿越）
   - 工作目录隔离：限定操作范围
4. **用户界面**：
   - 工具返回结果的格式化显示
   - 清晰的错误提示

---

## 子步骤规划（可独立提交）

### Step 1.1: 项目初始化 - Cargo.toml

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

---

### Step 1.2: 基础目录结构

**目标**: 创建基础源代码文件结构

**涉及文件**:
- `src/main.rs`
- `src/lib.rs`

**具体改动**:
- 创建 src/main.rs (入口文件)
- 创建 src/lib.rs (库入口)
- 创建基础模块结构

**验收标准**: 项目可以编译

---

### Step 1.3: 错误类型定义

**目标**: 定义项目使用的错误类型

**涉及文件**:
- `src/error.rs`

**具体改动**:
- 创建 error 模块
- 定义 Error enum，使用 thiserror
- 实现 std::error::Error
- 添加常见错误类型：
  - ConfigError - 配置错误
  - ToolError - 工具执行错误
  - AgentError - Agent 错误
  - LLMError - LLM API 错误

**验收标准**: cargo check 通过

---

### Step 1.4: 配置管理

**目标**: 实现配置加载逻辑

**涉及文件**:
- `src/config.rs`

**具体改动**:
- 定义 Config 结构体
- 实现配置加载（从 ~/.config/kloud/config.toml）
- 支持环境变量覆盖
- 配置项：
  - LlmConfig (api_key, api_base_url, model)
  - AgentConfig (workdir, max_concurrent)
  - ToolsConfig (allowed_commands)

**验收标准**: cargo check 通过

---

### Step 1.5: 状态机定义

**目标**: 定义 Agent 状态机

**涉及文件**:
- `src/state.rs`

**具体改动**:
- 定义 AgentState enum:
  - Idle - 等待用户输入
  - Thinking - 正在思考（调用 LLM）
  - ExecutingTool - 执行工具中
  - WaitingConfirm - 等待用户确认
  - Error - 错误状态
- 定义 AgentEvent enum
- 实现状态转换逻辑

**验收标准**: cargo check 通过

---

### Step 1.6: 工具 trait 定义

**目标**: 定义工具的基础 trait

**涉及文件**:
- `src/tools/mod.rs`

**具体改动**:
- 创建 tools 模块目录
- 定义 Tool trait:
  - name() - 工具名称
  - description() - 工具描述
  - execute() - 执行工具
- 定义 ToolResult 结构体
- 定义 ToolCall 结构体
- 实现路径安全检查函数

**验收标准**: cargo check 通过

---

### Step 1.7: LLM 客户端

**目标**: 实现 LLM API 调用封装

**涉及文件**:
- `src/llm/mod.rs`
- `src/llm/client.rs`
- `src/llm/models.rs`

**具体改动**:
- 定义 LLM 客户端 trait
- 实现 OpenAI 兼容 API 客户端
- 定义请求/响应结构体
- 实现流式响应支持（如需要）
- 实现重试和错误处理

**验收标准**: cargo check 通过

---

### Step 1.8: read 工具实现

**目标**: 实现文件读取工具

**涉及文件**:
- `src/tools/read.rs`

**具体改动**:
- 实现 ReadTool 结构体
- 实现 Tool trait
- 功能：
  - 读取指定文件内容
  - 支持行号范围参数
  - 路径安全检查

**验收标准**: cargo check 通过 + 单元测试

---

### Step 1.9: write 工具实现

**目标**: 实现文件写入工具

**涉及文件**:
- `src/tools/write.rs`

**具体改动**:
- 实现 WriteTool 结构体
- 实现 Tool trait
- 功能：
  - 创建新文件
  - 覆盖现有文件
  - 路径安全检查

**验收标准**: cargo check 通过 + 单元测试

---

### Step 1.10: edit 工具实现

**目标**: 实现文件编辑工具

**涉及文件**:
- `src/tools/edit.rs`

**具体改动**:
- 实现 EditTool 结构体
- 实现 Tool trait
- 功能：
  - 按行号替换
  - 按内容替换
  - 路径安全检查

**验收标准**: cargo check 通过 + 单元测试

---

### Step 1.11: bash 工具实现

**目标**: 实现命令执行工具

**涉及文件**:
- `src/tools/bash.rs`

**具体改动**:
- 实现 BashTool 结构体
- 实现 Tool trait
- 功能：
  - 执行 shell 命令
  - 命令白名单（可选）
  - 超时处理
  - 输出捕获（stdout/stderr）

**验收标准**: cargo check 通过 + 单元测试

---

### Step 1.12: git 工具实现

**目标**: 实现 Git 操作工具

**涉及文件**:
- `src/tools/git.rs`

**具体改动**:
- 实现 GitTool 结构体
- 实现 Tool trait
- 功能：
  - git status - 查看状态
  - git add / git commit - 提交更改
  - git branch / git checkout - 分支操作
  - git diff - 查看差异
  - 封装常用 Git 操作（作为 bash 的特化版本）

**验收标准**: cargo check 通过 + 单元测试

---

### Step 1.13: 工具注册

**目标**: 注册所有内置工具

**涉及文件**:
- `src/tools/mod.rs`

**具体改动**:
- 创建工具注册表
- 实现工具查找
- 实现工具列表

**验收标准**: cargo check 通过

---

### Step 1.14: Agent 核心逻辑

**目标**: 实现 Agent 的核心处理逻辑

**涉及文件**:
- `src/agent.rs`

**具体改动**:
- 定义 Agent 结构体
- 实现消息处理
- 实现 LLM 调用
- 实现工具调用循环

**验收标准**: cargo check 通过

---

### Step 1.15: CLI + 主循环

**目标**: 实现 CLI 参数解析和主循环

**涉及文件**:
- `src/cli.rs`
- `src/main.rs`

**具体改动**:
- 实现 CLI 参数解析（clap）
- 实现 REPL 主循环
- 集成状态机
- 处理退出命令

**验收标准**: cargo check 通过 + 可以运行

---

### Step 1.16: UI 格式化

**目标**: 实现输出格式化显示

**涉及文件**:
- `src/ui.rs`

**具体改动**:
- 实现工具结果格式化
- 实现错误信息显示
- 实现进度指示

**验收标准**: cargo check 通过

---

### Step 1.17: 测试

**目标**: 完善测试

**涉及文件**:
- 各个模块的测试

**具体改动**:
- 添加更多单元测试
- 添加集成测试

**验收标准**: cargo test 通过

---

## 目录结构

```
src/
├── main.rs          # 入口文件
├── lib.rs           # 库入口
├── cli.rs           # CLI 参数解析
├── config.rs        # 配置管理
├── error.rs         # 错误类型定义
├── state.rs         # Agent 状态机
├── agent.rs         # Agent 核心逻辑
├── ui.rs            # 用户界面
├── llm/             # LLM 客户端模块
│   ├── mod.rs
│   ├── client.rs
│   └── models.rs
└── tools/           # 工具模块
    ├── mod.rs       # 模块入口 + trait 定义
    ├── read.rs      # 读文件工具
    ├── write.rs     # 写文件工具
    ├── edit.rs      # 编辑工具
    ├── bash.rs      # 命令执行工具
    └── git.rs       # Git 操作工具
```

---

## 提交记录模板

| Step | 提交信息 |
|------|----------|
| 1.1 | init: add dependencies to Cargo.toml |
| 1.2 | init: create basic project structure |
| 1.3 | feat: add error types with thiserror |
| 1.4 | feat: add configuration management |
| 1.5 | feat: add agent state machine |
| 1.6 | feat: add tool trait definition |
| 1.7 | feat: add LLM client implementation |
| 1.8 | feat: implement read tool |
| 1.9 | feat: implement write tool |
| 1.10 | feat: implement edit tool |
| 1.11 | feat: implement bash tool |
| 1.12 | feat: implement git tool |
| 1.13 | feat: register all built-in tools |
| 1.14 | feat: implement agent core logic |
| 1.15 | feat: add CLI and main loop |
| 1.16 | feat: add UI formatting |
| 1.17 | test: add unit and integration tests |

---

## 验收标准

- [ ] 项目可以成功编译
- [ ] Agent 可以持续运行，等待用户输入
- [ ] read/write/edit/bash/git 工具可以正常工作
- [ ] 路径安全检查可以阻止目录穿越攻击
- [ ] 错误信息清晰可读
- [ ] 单元测试通过

---

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

| 变量名 | 说明 | 默认值 |
|--------|------|--------|
| `KLOUD_API_KEY` | API 密钥 | - |
| `KLOUD_API_BASE_URL` | API 基础 URL | `https://api.openai.com/v1` |
| `KLOUD_MODEL` | 使用的模型 | `gpt-4` |
| `KLOUD_WORKDIR` | 工作目录 | 当前目录 |
| `KLOUD_MAX_CONCURRENT` | 最大并发数 | `5` |
| `KLOUD_TASK_TIMEOUT` | 任务超时(秒) | `300` |
| `KLOUD_CONFIG_FILE` | 配置文件路径 | `~/.config/kloud/config.toml` |