# 阶段 5：协作 (Collaboration)

## 阶段目标

Agent 团队，团队协议，自主 Agent，worktree + 任务隔离

## 主要内容

### 5.1 Agent 团队

**团队成员**（5 个 Agent）：
1. **coder** - 编写代码
2. **reviewer** - 代码审查
3. **planner** - 任务规划
4. **explorer** - 探索/搜索
5. **joker** - 创意/娱乐

**团队架构**：
```
TeamManager
├── CoderAgent (队列: coder_inbox)
├── ReviewerAgent (队列: reviewer_inbox)
├── PlannerAgent (队列: planner_inbox)
├── ExplorerAgent (队列: explorer_inbox)
└── JokerAgent (队列: joker_inbox)
```

### 5.2 团队协议

**通信方式**：消息队列（每个 Agent 有独立队列）

**消息格式**：
```rust
struct TeamMessage {
    id: String,
    from: AgentType,
    to: AgentType,
    message_type: MessageType,
    content: String,
    attachments: Vec<Attachment>,
    timestamp: DateTime,
}

enum MessageType {
    Request,    // 请求
    Response,   // 响应
    Broadcast,  // 广播
    Event,      // 事件
}
```

**通信流程**：
```
Agent A → 发送消息 → TeamManager → 路由到 Agent B 的队列 → Agent B 处理
```

### 5.3 自主 Agent

**自主程度**：半自主（需要团队协调）

**协调机制**：
- Agent 可以主动向其他 Agent 发送消息
- Agent 可以广播消息给团队
- Agent 可以监听团队事件

**Agent 行为**：
```rust
impl AgentBehavior for CoderAgent {
    async fn run(&self) {
        loop {
            // 1. 检查自己的消息队列
            while let Some(msg) = self.inbox.try_recv() {
                self.handle_message(msg).await;
            }
            
            // 2. 检查团队事件
            if let Some(event) = self.team_events.try_recv() {
                self.handle_event(event).await;
            }
            
            // 3. 自主行动（如果有）
            if let Some(action) = self.decide_next_action().await {
                self.execute(action).await;
            }
            
            sleep(Duration::from_millis(100)).await;
        }
    }
}
```

### 5.4 Worktree 任务隔离

**设计**：任务管理目标，worktree 管理目录

**对应关系**：
- 每个任务 (Task) 对应一个 worktree
- Worktree 生命周期与任务生命周期绑定
- 任务创建时创建 worktree，任务完成时删除 worktree

**目录结构**：
```
.worktrees/
├── task_001/
│   └── (git worktree)
├── task_002/
│   └── (git worktree)
└── ...
```

**任务 → Worktree 映射**：
```rust
struct WorktreeManager {
    base_dir: PathBuf,
    task_worktrees: HashMap<String, PathBuf>,
}

impl WorktreeManager {
    fn create_for_task(&self, task_id: &str) -> Result<PathBuf>;
    fn get_for_task(&self, task_id: &str) -> Option<PathBuf>;
    fn remove_for_task(&self, task_id: &str) -> Result<()>;
}
```

**Git Worktree 操作**：
```rust
fn create_worktree(branch: &str, path: &Path) -> Result<()>;
fn remove_worktree(path: &Path) -> Result<()>;
fn list_worktrees() -> Result<Vec<WorktreeInfo>>;
```

## 实施步骤

### 5.1 Agent 团队实现

**步骤 5.1.1**: 定义 Agent trait

**步骤 5.1.2**: 实现 TeamManager

**步骤 5.1.3**: 实现各类型 Agent

### 5.2 团队协议实现

**步骤 5.2.1**: 定义消息格式

**步骤 5.2.2**: 实现消息队列

**步骤 5.2.3**: 实现消息路由

### 5.3 自主 Agent 实现

**步骤 5.3.1**: 实现 Agent 行为 trait

**步骤 5.3.2**: 实现消息处理

**步骤 5.3.3**: 实现事件监听

**步骤 5.3.4**: 实现自主决策

### 5.4 Worktree 隔离实现

**步骤 5.4.1**: 设计 worktree 目录结构

**步骤 5.4.2**: 实现 WorktreeManager

**步骤 5.4.3**: 实现 Git worktree 操作

**步骤 5.4.4**: 实现任务与 worktree 绑定

## 验收标准

- [ ] 5 个 Agent 可以同时运行
- [ ] 消息队列正常工作
- [ ] 消息可以正确路由到目标 Agent
- [ ] Agent 可以进行团队协调
- [ ] 每个任务有对应的 worktree
- [ ] Worktree 正确隔离
- [ ] 任务完成时 worktree 正确清理
- [ ] 单元测试通过

## 技术细节

### 消息示例

```json
{
  "id": "msg_001",
  "from": "planner",
  "to": "coder",
  "type": "request",
  "content": "实现用户登录功能",
  "attachments": [
    {
      "type": "spec",
      "path": "/specs/auth.md"
    }
  ],
  "timestamp": "2024-01-01T10:00:00Z"
}
```

### Worktree 配置

```toml
[collaboration]
worktree_base_dir = ".worktrees"
auto_cleanup = true  # 任务完成后自动清理 worktree
```
