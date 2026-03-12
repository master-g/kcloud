# Phase 1: 工具与执行 (Tools & Execution)

**阶段目标**: Agent 循环，工具，安全机制，格式化显示

## 调整说明

本阶段基于 pi-mono 架构对原有 plan 进行了以下调整：

### 步骤调整

| 原步骤 | 新步骤 | 调整内容 |
|--------|--------|----------|
| 1.5 状态机定义 | **1.5 Agent 核心类型** | 改为 struct-based 设计，参考 pi-mono |
| 1.14 Agent 核心逻辑 | **1.14 Agent 结构体与消息队列** | 拆分并增加 steering/follow-up 队列 |
| - | **1.15 Agent Loop 双循环实现** | 新增，核心难点 |
| - | **1.16 事件系统** | 新增，16+ 事件类型 |
| 1.15 CLI + 主循环 | **1.17 CLI + 主循环** | 调整编号 |
| 1.16 UI 格式化 | **1.18 UI 格式化** | 调整编号 |
| 1.17 测试 | **1.19 测试** | 调整编号 |

### 关键变化

1. **AgentState 从 enum 改为 struct**
   - 原有：简单状态机 (Idle/Thinking/ExecutingTool/...)
   - 新设计：包含 messages/tools/is_streaming/pending_tool_calls 的完整状态

2. **新增双循环架构** (Step 1.15)
   - Outer loop: 处理 follow-up 消息
   - Inner loop: 处理 tool calls + steering

3. **新增消息队列** (Step 1.14)
   - `steering_queue`: 中断 agent mid-run
   - `follow_up_queue`: agent 完成后继续

4. **新增事件系统** (Step 1.16)
   - 四层生命周期：Agent/Turn/Message/Tool
   - 支持流式更新的 MessageUpdate 事件

## 实施顺序

```
基础阶段：
  1.1 → 1.2 → 1.3 → 1.4

类型定义阶段：
  1.5 (核心类型) → 1.6 (工具 trait)

工具实现阶段 (可并行)：
  1.7 → 1.8 → 1.9 → 1.10 → 1.11 → 1.12 → 1.13

Agent 核心阶段 (核心难点)：
  1.14 (结构体) → 1.15 (双循环) → 1.16 (事件)

集成阶段：
  1.17 (CLI) → 1.18 (UI) → 1.19 (测试)
```

## 目录结构

```
src/
├── main.rs              # 入口文件
├── lib.rs               # 库入口
├── cli.rs               # CLI 参数解析
├── config.rs            # 配置管理
├── error.rs             # 错误类型定义
├── ui.rs                # 用户界面
├── llm/                 # LLM 客户端模块
│   ├── mod.rs
│   ├── client.rs
│   └── models.rs
├── tools/               # 工具模块
│   ├── mod.rs           # 模块入口 + trait 定义
│   ├── read.rs
│   ├── write.rs
│   ├── edit.rs
│   ├── bash.rs
│   └── git.rs
└── agent/               # Agent 模块 (新增)
    ├── mod.rs           # 模块入口
    ├── types.rs         # 核心类型 (Step 1.5)
    ├── agent.rs         # Agent 结构体 + 队列 (Step 1.14)
    ├── loop.rs          # 双循环实现 (Step 1.15)
    ├── events.rs        # 事件系统 (Step 1.16)
    └── stream.rs        # 流式响应支持
```

## 参考资源

- **pi-mono 源码**:
  - `packages/agent/src/agent-loop.ts` - 双循环实现
  - `packages/agent/src/types.ts` - 类型定义
  - `packages/agent/src/agent.ts` - Agent 类

## 验收标准

- [ ] 项目可以成功编译
- [ ] Agent 可以持续运行，等待用户输入
- [ ] read/write/edit/bash/git 工具可以正常工作
- [ ] 路径安全检查可以阻止目录穿越攻击
- [ ] 支持 steering 中断工具执行
- [ ] 支持 follow-up 连续对话
- [ ] 事件系统能正确发射 16+ 种事件
- [ ] 错误信息清晰可读
- [ ] 单元测试通过
