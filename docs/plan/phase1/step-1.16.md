# Step 1.16: 事件系统

**目标**: 实现细粒度的 Agent 事件系统（pi-mono 风格）

**涉及文件**:
- `src/agent/events.rs`

**具体改动**:
- 定义 AgentEvent enum，包含四层生命周期：
  - Agent 生命周期: AgentStart, AgentEnd
  - Turn 生命周期: TurnStart, TurnEnd { message, tool_results }
  - Message 生命周期: MessageStart, MessageUpdate { delta }, MessageEnd
  - Tool 生命周期: ToolExecutionStart, ToolExecutionUpdate, ToolExecutionEnd

- 实现 EventSender trait 和 ChannelEventSender
- 实现辅助方法：
  - type_name() -> 事件类型字符串
  - is_agent_event() / is_turn_event() / is_message_event() / is_tool_event()

**参考实现** (pi-mono/packages/agent/src/types.ts):
- AgentEvent 是 tagged union，16+ 种事件
- 事件用于驱动 UI 更新
- 流式响应时发射 MessageUpdate

**验收标准**:
- cargo check 通过
- 所有事件类型定义完整
- 事件发送时机正确
- UI 可订阅并响应事件

**提交信息**: `feat: add fine-grained event system`
