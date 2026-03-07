# 阶段 2：规划与协调 (Planning & Coordination)

## 阶段目标

TODO Write，子 Agent，技能系统，任务系统（DAG + 任务状态管理）

## 主要内容

### 2.1 子 Agent 系统

**子 Agent 类型**：
1. **coder** - 编写代码
2. **reviewer** - 代码审查
3. **planner** - 任务规划
4. **explorer** - 探索/搜索
5. **joker** - 创意/娱乐

**子 Agent 通信**：
- 使用结构化消息格式（JSON）
- 主 Agent 生成结构化指令，传递给子 Agent
- 子 Agent 返回结构化结果

**子 Agent 架构**：
```
MainAgent
├── SubAgent[coder]
├── SubAgent[reviewer]
├── SubAgent[planner]
├── SubAgent[explorer]
└── SubAgent[joker]
```

### 2.2 TODO Write

**功能**：解析用户输入，自动生成 TODO 列表

**实现方式**：通过 LLM 智能解析

**TODO 格式**：
```rust
struct TodoItem {
    id: String,
    description: String,
    priority: Priority,
    depends_on: Vec<String>,  // 依赖的任务 ID
    assignee: Option<AgentType>,  // 分配的子 Agent
}
```

### 2.3 技能系统

**触发方式**：
- 斜杠命令（如 `/skill-name`）
- 启发式自动触发（根据上下文自动调用）

**技能示例**：
- `/search` - 网络搜索
- `/lsp` - LSP 操作
- `/git` - Git 操作
- `/debug` - 调试技能

### 2.4 任务系统（DAG）

**任务图结构**：
- 使用 DAG（有向无环图）管理任务依赖
- 支持拓扑排序
- 支持循环检测

**任务属性**：
```rust
struct Task {
    id: String,
    description: String,
    status: TaskStatus,  // Pending, InProgress, Completed, Failed
    priority: Priority,
    depends_on: Vec<String>,  // 依赖的任务 ID
    created_at: DateTime,
    deadline: Option<DateTime>,
    assignee: Option<AgentType>,
    result: Option<TaskResult>,
}

enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

enum Priority {
    Low,
    Medium,
    High,
    Critical,
}
```

**任务操作**：
- 创建任务
- 更新状态
- 添加依赖
- 拓扑排序
- 检测循环

## 实施步骤

### 2.1 子 Agent 系统实现

**步骤 2.1.1**: 定义子 Agent trait

```rust
trait SubAgent {
    fn agent_type(&self) -> AgentType;
    async fn execute(&self, context: &AgentContext) -> Result<AgentResponse>;
}
```

**步骤 2.1.2**: 实现各类型子 Agent

- CoderAgent
- ReviewerAgent
- PlannerAgent
- ExplorerAgent
- JokerAgent

**步骤 2.1.3**: 实现消息格式

```rust
struct AgentMessage {
    role: MessageRole,  // System, User, Assistant
    content: String,
    tool_calls: Vec<ToolCall>,
}

struct AgentResponse {
    content: String,
    tool_calls: Vec<ToolCall>,
    todo_items: Vec<TodoItem>,
}
```

### 2.2 TODO Write 实现

**步骤 2.2.1**: 设计 TODO 解析 prompt

**步骤 2.2.2**: 实现 TODO 生成逻辑

**步骤 2.2.3**: 集成到 Agent 循环

### 2.3 技能系统实现

**步骤 2.3.1**: 定义技能 trait

```rust
trait Skill {
    fn name(&self) -> &str;
    fn trigger(&self, input: &str) -> bool;
    async fn execute(&self, args: &str) -> Result<String>;
}
```

**步骤 2.3.2**: 实现内置技能

**步骤 2.3.3**: 实现触发检测（斜杠命令 + 启发式）

### 2.4 任务系统实现

**步骤 2.4.1**: 定义任务结构

**步骤 2.4.2**: 实现 DAG 管理

**步骤 2.4.3**: 实现任务调度

**步骤 2.4.4**: 实现依赖解析

## 验收标准

- [ ] 子 Agent 可以正确执行任务
- [ ] 主 Agent 可以委派任务给子 Agent
- [ ] TODO 可以自动生成
- [ ] 技能可以通过斜杠命令触发
- [ ] 启发式触发可以正常工作
- [ ] 任务 DAG 可以正确管理依赖
- [ ] 单元测试通过

## 技术细节

### 子 Agent 通信格式 (JSON)

```json
{
  "type": "sub_agent_request",
  "agent_type": "coder",
  "task": "Implement user authentication",
  "context": {
    "files": ["src/auth.rs"],
    "constraints": ["use bcrypt", "support JWT"]
  }
}
```

```json
{
  "type": "sub_agent_response",
  "status": "completed",
  "result": "...",
  "artifacts": ["src/auth.rs:new"]
}
```
