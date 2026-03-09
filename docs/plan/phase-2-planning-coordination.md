# 阶段 2：规划与协调 (Planning & Coordination)

## 阶段目标

TODO Write，子 Agent，技能系统，任务系统（DAG + 任务状态管理）

## 主要内容

### 2.1 子 Agent 系统

> **重要**：所有 Agent 类型统一定义在 `src/agents/types.rs`，详见主计划文档的「统一类型定义」章节。

**子 Agent 类型**（使用统一 AgentType）：
1. **Coder** - 编写代码
2. **Reviewer** - 代码审查
3. **Planner** - 任务规划
4. **Explorer** - 探索/搜索
5. **Joker** - 创意/娱乐

**子 Agent 通信**：
- 使用结构化消息格式（JSON）
- 主 Agent 生成结构化指令，传递给子 Agent
- 子 Agent 返回结构化结果

### 2.2 TODO Write

**功能**：解析用户输入，自动生成 TODO 列表

**实现方式**：通过 LLM 智能解析

### 2.3 技能系统

**触发方式**：
- 斜杠命令（如 `/skill-name`）
- 启发式自动触发（根据上下文自动调用）

### 2.4 任务系统（DAG）

**职责划分**：
- **TaskScheduler**（本阶段）：负责任务的创建、分解、依赖管理
- **Daemon**（阶段4）：负责任务的实际执行、并发控制

**数据流**：TaskScheduler 生成任务 → 提交给 Daemon → Daemon 执行并返回结果

**任务图结构**：
- 使用 DAG（有向无环图）管理任务依赖
- 支持拓扑排序
- 支持循环检测

---

## 子步骤规划（可独立提交）

### Step 2.1: 子 Agent 类型定义

**目标**: 定义所有子 Agent 的类型

**涉及文件**:
- `src/agents/types.rs` (新建)

**具体改动**:
- 定义 AgentType enum:
  - Main
  - Coder
  - Reviewer
  - Planner
  - Explorer
  - Joker

**验收标准**: cargo check 通过

---

### Step 2.2: 子 Agent trait 定义

**目标**: 定义子 Agent 的通用接口

**涉及文件**:
- `src/agents/mod.rs` (新建)
- `src/agents/trait.rs` (新建)

**具体改动**:
- 定义 SubAgent trait:
  ```rust
  trait SubAgent {
      fn agent_type(&self) -> AgentType;
      fn name(&self) -> &str;
      fn description(&self) -> &str;
      async fn execute(&self, context: &AgentContext) -> Result<AgentResponse>;
  }
  ```
- 定义 AgentContext 结构体
- 定义 AgentResponse 结构体

**验收标准**: cargo check 通过

---

### Step 2.3: 子 Agent 消息格式

**目标**: 定义 Agent 间通信的消息格式

**涉及文件**:
- `src/agents/message.rs` (新建)

**具体改动**:
- 定义 AgentMessage 结构体:
  - id, from, to, message_type, content, attachments, timestamp
- 定义 MessageType enum: Request, Response, Broadcast, Event
- 实现序列化/反序列化

**验收标准**: cargo check 通过

---

### Step 2.4: CoderAgent 实现

**目标**: 实现代码编写子 Agent

**涉及文件**:
- `src/agents/coder.rs` (新建)

**具体改动**:
- 实现 CoderAgent 结构体
- 实现 SubAgent trait
- 功能：
  - 接收代码编写任务
  - 使用工具编写代码
  - 返回结果

**验收标准**: cargo check 通过 + 单元测试

---

### Step 2.5: ReviewerAgent 实现

**目标**: 实现代码审查子 Agent

**涉及文件**:
- `src/agents/reviewer.rs` (新建)

**具体改动**:
- 实现 ReviewerAgent 结构体
- 实现 SubAgent trait
- 功能：
  - 接收代码审查任务
  - 分析代码质量
  - 返回审查意见

**验收标准**: cargo check 通过 + 单元测试

---

### Step 2.6: PlannerAgent 实现

**目标**: 实现任务规划子 Agent

**涉及文件**:
- `src/agents/planner.rs` (新建)

**具体改动**:
- 实现 PlannerAgent 结构体
- 实现 SubAgent trait
- 功能：
  - 接收任务分解请求
  - 规划任务步骤
  - 返回任务列表

**验收标准**: cargo check 通过 + 单元测试

---

### Step 2.7: ExplorerAgent 实现

**目标**: 实现探索/搜索子 Agent

**涉及文件**:
- `src/agents/explorer.rs` (新建)

**具体改动**:
- 实现 ExplorerAgent 结构体
- 实现 SubAgent trait
- 功能：
  - 搜索代码库
  - 查找文件/符号
  - 返回搜索结果

**验收标准**: cargo check 通过 + 单元测试

---

### Step 2.8: JokerAgent 实现

**目标**: 实现创意/娱乐子 Agent

**涉及文件**:
- `src/agents/joker.rs` (新建)

**具体改动**:
- 实现 JokerAgent 结构体
- 实现 SubAgent trait
- 功能：
  - 提供创意建议
  - 生成代码示例
  - 娱乐互动

**验收标准**: cargo check 通过 + 单元测试

---

### Step 2.9: Agent 注册表

**目标**: 注册所有子 Agent

**涉及文件**:
- `src/agents/registry.rs` (新建)

**具体改动**:
- 实现 AgentRegistry 结构体
- 实现 agent 查找和创建
- 实现 agent 列表

**验收标准**: cargo check 通过

---

### Step 2.10: 主 Agent 委派逻辑

**目标**: 实现主 Agent 向子 Agent 委派任务

**涉及文件**:
- `src/agent.rs` (修改)

**具体改动**:
- 添加子 Agent 委派功能
- 实现消息发送
- 实现结果收集

**验收标准**: cargo check 通过

---

### Step 2.11: TODO 数据结构

**目标**: 定义 TODO 数据结构

**涉及文件**:
- `src/todo/mod.rs` (新建)
- `src/todo/item.rs` (新建)

**具体改动**:
- 定义 TodoItem 结构体:
  - id, description, priority, depends_on, assignee, status, created_at
- 定义 Priority enum: Low, Medium, High, Critical
- 定义 TodoStatus enum: Pending, InProgress, Completed, Cancelled

**验收标准**: cargo check 通过

---

### Step 2.12: TODO 生成逻辑

**目标**: 实现 LLM 解析生成 TODO

**涉及文件**:
- `src/todo/generator.rs` (新建)

**具体改动**:
- 设计 TODO 解析 prompt
- 实现 LLM 调用
- 实现 TODO 解析和创建

**验收标准**: cargo check 通过

---

### Step 2.13: TODO 管理器

**目标**: 实现 TODO 列表管理

**涉及文件**:
- `src/todo/manager.rs` (新建)

**具体改动**:
- 实现 TodoManager 结构体
- 实现 CRUD 操作
- 实现 TODO 展示

**验收标准**: cargo check 通过

---

### Step 2.14: 技能 trait 定义

**目标**: 定义技能的通用接口

**涉及文件**:
- `src/skills/mod.rs` (新建)
- `src/skills/trait.rs` (新建)

**具体改动**:
- 定义 Skill trait:
  ```rust
  trait Skill {
      fn name(&self) -> &str;
      fn aliases(&self) -> Vec<&str>;
      fn description(&self) -> &str;
      fn trigger(&self, input: &str) -> bool;
      async fn execute(&self, args: &str) -> Result<String>;
  }
  ```

**验收标准**: cargo check 通过

---

### Step 2.15: 技能注册

**目标**: 注册所有技能

**涉及文件**:
- `src/skills/registry.rs` (新建)

**具体改动**:
- 实现 SkillRegistry 结构体
- 实现技能查找（按名称或别名）
- 实现技能列表

**验收标准**: cargo check 通过

---

### Step 2.16: 启发式触发实现

**目标**: 实现技能的自动触发

**涉及文件**:
- `src/skills/trigger.rs` (新建)

**具体改动**:
- 实现启发式匹配逻辑
- 实现自动触发检测

**验收标准**: cargo check 通过

---

### Step 2.17: 任务数据结构

**目标**: 定义任务数据结构

**涉及文件**:
- `src/tasks/mod.rs` (新建)
- `src/tasks/models.rs` (新建)

**具体改动**:
- 定义 Task 结构体:
  - id, description, status, priority, depends_on, created_at, deadline, assignee, result
- 定义 TaskStatus enum: Pending, InProgress, Completed, Failed
- 定义 TaskResult 结构体

**验收标准**: cargo check 通过

---

### Step 2.18: DAG 实现

**目标**: 实现任务依赖图

**涉及文件**:
- `src/tasks/graph.rs` (新建)

**具体改动**:
- 实现 TaskGraph 结构体
- 实现添加边/节点
- 实现拓扑排序
- 实现循环检测

**验收标准**: cargo check 通过 + 单元测试

---

### Step 2.19: 任务调度器

**目标**: 实现任务调度逻辑

**涉及文件**:
- `src/tasks/scheduler.rs` (新建)

**具体改动**:
- 实现 TaskScheduler 结构体
- 实现任务状态管理
- 实现依赖解析

**验收标准**: cargo check 通过

---

### Step 2.20: 集成测试

**目标**: 端到端测试

**涉及文件**:
- `tests/agent_delegation_test.rs` (新建)
- `tests/todo_test.rs` (新建)
- `tests/skills_test.rs` (新建)

**具体改动**:
- 添加子 Agent 委派测试
- 添加 TODO 生成测试
- 添加技能触发测试

**验收标准**: cargo test 通过

---

## 目录结构

```
src/
├── agent.rs              # 主 Agent（修改）
├── agents/               # 子 Agent 模块（新建）
│   ├── mod.rs
│   ├── types.rs          # Agent 类型定义
│   ├── trait.rs         # SubAgent trait
│   ├── message.rs       # 消息格式
│   ├── coder.rs         # CoderAgent
│   ├── reviewer.rs      # ReviewerAgent
│   ├── planner.rs       # PlannerAgent
│   ├── explorer.rs      # ExplorerAgent
│   ├── joker.rs         # JokerAgent
│   └── registry.rs      # Agent 注册表
├── todo/                 # TODO 模块（新建）
│   ├── mod.rs
│   ├── item.rs          # TODO 数据结构
│   ├── generator.rs     # TODO 生成逻辑
│   └── manager.rs       # TODO 管理器
├── skills/              # 技能模块（新建）
│   ├── mod.rs
│   ├── trait.rs         # Skill trait
│   ├── registry.rs     # 技能注册
│   └── trigger.rs       # 启发式触发
└── tasks/               # 任务模块（新建）
    ├── mod.rs
    ├── models.rs        # 任务数据结构
    ├── graph.rs         # DAG 实现
    └── scheduler.rs     # 任务调度器
```

---

## 提交记录模板

| Step | 提交信息 |
|------|----------|
| 2.1 | feat: define sub-agent types |
| 2.2 | feat: add SubAgent trait definition |
| 2.3 | feat: add agent message format |
| 2.4 | feat: implement CoderAgent |
| 2.5 | feat: implement ReviewerAgent |
| 2.6 | feat: implement PlannerAgent |
| 2.7 | feat: implement ExplorerAgent |
| 2.8 | feat: implement JokerAgent |
| 2.9 | feat: add agent registry |
| 2.10 | feat: add agent delegation logic |
| 2.11 | feat: add TODO data structures |
| 2.12 | feat: implement TODO generation |
| 2.13 | feat: implement TODO manager |
| 2.14 | feat: add Skill trait definition |
| 2.15 | feat: add skill registry |
| 2.16 | feat: implement heuristic triggering |
| 2.17 | feat: add task data structures |
| 2.18 | feat: implement task DAG |
| 2.19 | feat: implement task scheduler |
| 2.20 | test: add integration tests |

---

## 验收标准

- [ ] 5 个子 Agent 可以正确执行任务
- [ ] 主 Agent 可以委派任务给子 Agent
- [ ] TODO 可以通过 LLM 自动生成
- [ ] 技能可以通过斜杠命令触发
- [ ] 启发式触发可以正常工作
- [ ] 任务 DAG 可以正确管理依赖
- [ ] 单元测试通过