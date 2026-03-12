# Step 1.14: Agent 结构体与消息队列

**目标**: 实现 Agent 结构体、消息队列和状态管理（参考 pi-mono）

**涉及文件**:
- `src/agent/agent.rs`

**具体改动**:
- 定义 Agent 结构体，包含：
  - state: AgentState (运行时状态)
  - steering_queue: Vec<AgentMessage> (中断队列)
  - follow_up_queue: Vec<AgentMessage> (后续队列)
  - steering_mode: QueueMode (OneAtATime/All)
  - follow_up_mode: QueueMode
  - listeners: Vec<...> (事件监听器)
  - abort_controller: Option<CancellationToken>

- 实现核心方法：
  - new() - 创建 Agent
  - prompt() - 启动对话
  - steer() - 注入中断消息
  - follow_up() - 注入后续消息
  - dequeue_steering() - 出队 steering (支持模式)
  - dequeue_follow_up() - 出队 follow-up
  - abort() - 中止执行
  - reset() - 重置状态

**参考实现** (pi-mono/packages/agent/src/agent.ts):
- steering() 可以中断正在执行的 agent
- followUp() 在 agent 完成后继续
- QueueMode 控制消息批量或单条处理

**验收标准**:
- cargo check 通过
- Agent 结构体完整
- 队列操作 (steer/follow_up/dequeue) 工作正常
- 支持订阅事件监听

**提交信息**: `feat: add agent struct with message queues`
