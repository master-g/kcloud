# Step 1.5: Agent 核心类型

**目标**: 定义 Agent 系统的核心类型（参考 pi-mono 架构）

**涉及文件**:
- `src/agent/mod.rs`
- `src/agent/types.rs`

**具体改动**:
- 创建 agent 模块目录
- 定义 MessageRole enum (User/Assistant/ToolResult)
- 定义 ContentBlock enum (Text/ToolCall/Thinking/Image)
- 定义 AgentMessage 结构体 (role/content/timestamp)
- 定义 ToolResultMessage 结构体
- 定义 AgentContext 结构体 (system_prompt/messages/tools)
- 定义 AgentState 结构体 (完整运行时状态)
- 定义 StopReason enum (Complete/Length/ToolCalls/Error/Aborted)
- 定义 AssistantMessage 结构体

**参考实现** (pi-mono/packages/agent/src/types.ts):
- AgentMessage 支持多模态内容块
- AgentState 包含 streaming 状态和 pending_tool_calls
- 类型设计支持事件驱动架构

**验收标准**:
- cargo check 通过
- 所有类型支持 JSON 序列化
- 类型与 pi-mono 概念对齐

**提交信息**: `feat: add agent core types`
