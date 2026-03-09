# 阶段 5：协作 (Collaboration)

## 阶段目标

Agent 团队，团队协议，自主 Agent，worktree + 任务隔离

## 主要内容

### 5.1 Agent 团队

> **重要**：所有 Agent 类型统一定义在 `src/agents/types.rs`，详见主计划文档的「统一类型定义」章节。阶段5应复用阶段2实现的 Agent 代码。

**团队成员**（使用统一 AgentType）：
1. **Coder** - 编写代码
2. **Reviewer** - 代码审查
3. **Planner** - 任务规划
4. **Explorer** - 探索/搜索
5. **Joker** - 创意/娱乐

### 5.2 团队协议

**通信方式**：消息队列（每个 Agent 有独立队列）

**消息类型**：Request, Response, Broadcast, Event

### 5.3 自主 Agent

**自主程度**：半自主（需要团队协调）

### 5.4 Worktree 任务隔离

**设计**：任务管理目标，worktree 管理目录
- 每个任务对应一个 worktree
- Worktree 生命周期与任务生命周期绑定

---

## 子步骤规划（可独立提交）

### Step 5.1: 团队消息格式

**目标**: 定义团队通信的消息格式

**涉及文件**:
- `src/team/mod.rs` (新建)
- `src/team/message.rs` (新建)

**具体改动**:
- 定义 TeamMessage 结构体:
  - id, from, to, message_type, content, attachments, timestamp
- 定义 MessageType enum: Request, Response, Broadcast, Event
- 定义 Attachment 结构体

**验收标准**: cargo check 通过

---

### Step 5.2: Agent 邮箱

**目标**: 实现每个 Agent 的独立消息队列

**涉及文件**:
- `src/team/mailbox.rs` (新建)

**具体改动**:
- 定义 Mailbox<T> 结构体
- 实现 mpsc::Receiver 包装
- 实现非阻塞获取
- 实现广播订阅

**验收标准**: cargo check 通过 + 单元测试

---

### Step 5.3: TeamManager 定义

**目标**: 定义团队管理器接口

**涉及文件**:
- `src/team/manager.rs` (新建)

**具体改动**:
- 定义 TeamManager trait
- 定义 TeamEvent 结构体
- 定义 EventBus 结构体（用于广播）

**验收标准**: cargo check 通过

---

### Step 5.4: TeamManager 实现

**目标**: 实现团队管理器

**涉及文件**:
- `src/team/manager.rs` (修改)

**具体改动**:
- 实现 TeamManager 结构体
- 实现消息发送 send_message()
- 实现消息广播 broadcast()
- 实现 Agent 注册

**验收标准**: cargo check 通过

---

### Step 5.5: TeamAgent trait

**目标**: 定义团队 Agent 的通用接口

**涉及文件**:
- `src/team/agent.rs` (新建)

**具体改动**:
- 定义 TeamAgent trait:
  - agent_type()
  - run() - 主循环
  - handle_message()
- 定义 AgentState

**验收标准**: cargo check 通过

---

### Step 5.6: Agent 行为 - 消息处理

**目标**: 实现 Agent 消息处理逻辑

**涉及文件**:
- `src/team/behaviors/message.rs` (新建)

**具体改动**:
- 实现消息处理逻辑
- 实现请求解析
- 实现响应生成

**验收标准**: cargo check 通过

---

### Step 5.7: Agent 行为 - 事件监听

**目标**: 实现 Agent 事件监听

**涉及文件**:
- `src/team/behaviors/event.rs` (新建)

**具体改动**:
- 实现事件订阅
- 实现事件处理
- 实现事件过滤

**验收标准**: cargo check 通过

---

### Step 5.8: Agent 行为 - 自主决策

**目标**: 实现 Agent 自主决策

**涉及文件**:
- `src/team/behaviors/autonomy.rs` (新建)

**具体改动**:
- 实现 decide_next_action()
- 实现自主行动触发
- 实现决策频率控制

**验收标准**: cargo check 通过

---

### Step 5.9: TeamAgent 实现

**目标**: 实现可运行的团队 Agent

**涉及文件**:
- `src/team/team_agent.rs` (新建)

**具体改动**:
- 实现 TeamAgent 结构体
- 实现 run() 主循环
- 整合消息处理、事件监听、自主决策

**验收标准**: cargo check 通过

---

### Step 5.10: Agent 工厂

**目标**: 实现 Agent 创建工厂

**涉及文件**:
- `src/team/factory.rs` (新建)

**具体改动**:
- 实现 AgentFactory 结构体
- 实现 create_agent()
- 实现 agent 类型映射

**验收标准**: cargo check 通过

---

### Step 5.11: WorktreeManager 定义

**目标**: 定义 worktree 管理器接口

**涉及文件**:
- `src/worktree/mod.rs` (新建)
- `src/worktree/manager.rs` (新建)

**具体改动**:
- 定义 WorktreeManager trait
- 定义 WorktreeInfo 结构体

**验收标准**: cargo check 通过

---

### Step 5.12: WorktreeManager 实现

**目标**: 实现 worktree 管理器

**涉及文件**:
- `src/worktree/manager.rs` (修改)

**具体改动**:
- 实现 WorktreeManager 结构体
- 实现 create_for_task()
- 实现 get_for_task()
- 实现 remove_for_task()

**验收标准**: cargo check 通过

---

### Step 5.13: Git Worktree 操作

**目标**: 实现 Git worktree 命令封装

**涉及文件**:
- `src/worktree/git.rs` (新建)

**具体改动**:
- 实现 create_worktree() - git worktree add
- 实现 remove_worktree() - git worktree remove
- 实现 list_worktrees() - git worktree list
- 实现清理分支

**验收标准**: cargo check 通过 + 单元测试

---

### Step 5.14: Worktree 目录管理

**目标**: 实现 worktree 目录结构管理

**涉及文件**:
- `src/worktree/directory.rs` (新建)

**具体改动**:
- 实现目录创建
- 实现目录清理
- 实现锁机制（防止并发冲突）

**验收标准**: cargo check 通过

---

### Step 5.15: 任务与 Worktree 绑定

**目标**: 实现任务到 worktree 的映射

**涉及文件**:
- `src/worktree/binding.rs` (新建)

**具体改动**:
- 实现 TaskWorktreeBinding 结构体
- 实现 bind_task()
- 实现 get_workdir()
- 实现解绑

**验收标准**: cargo check 通过

---

### Step 5.16: 自动清理

**目标**: 实现 worktree 自动清理

**涉及文件**:
- `src/worktree/cleanup.rs` (新建)

**具体改动**:
- 实现任务完成时自动清理
- 实现超时清理
- 实现手动清理命令

**验收标准**: cargo check 通过

---

### Step 5.17: 配置项

**目标**: 添加协作相关配置

**涉及文件**:
- `src/config.rs` (修改)

**具体改动**:
- 添加 worktree_base_dir 配置
- 添加 auto_cleanup 配置

**验收标准**: cargo check 通过

---

### Step 5.18: 团队启动/停止

**目标**: 实现团队生命周期管理

**涉及文件**:
- `src/team/lifecycle.rs` (新建)

**具体改动**:
- 实现团队启动
- 实现优雅停止
- 实现健康检查

**验收标准**: cargo check 通过

---

### Step 5.19: 集成测试

**目标**: 端到端测试

**涉及文件**:
- `tests/team_test.rs` (新建)
- `tests/worktree_test.rs` (新建)

**具体改动**:
- 添加团队协作测试
- 添加 worktree 测试

**验收标准**: cargo test 通过

---

## 目录结构

```
src/
├── agent.rs              # 主 Agent（修改）
├── config.rs             # 配置（修改）
├── agents/               # 子 Agent 模块（复用阶段2代码）
│   ├── mod.rs
│   ├── types.rs          # Agent 类型定义
│   ├── trait.rs
│   ├── message.rs
│   ├── coder.rs
│   ├── reviewer.rs
│   ├── planner.rs
│   ├── explorer.rs
│   ├── joker.rs
│   └── registry.rs
├── team/                 # 团队模块（新建）
│   ├── mod.rs
│   ├── message.rs        # 消息格式
│   ├── mailbox.rs        # Agent 邮箱
│   ├── manager.rs        # 团队管理器
│   ├── agent.rs          # TeamAgent trait
│   ├── team_agent.rs     # TeamAgent 实现
│   ├── factory.rs        # Agent 工厂
│   ├── lifecycle.rs      # 生命周期
│   └── behaviors/
│       ├── mod.rs
│       ├── message.rs    # 消息处理
│       ├── event.rs      # 事件监听
│       └── autonomy.rs   # 自主决策
└── worktree/             # Worktree 模块（新建）
    ├── mod.rs
    ├── manager.rs        # 管理器
    ├── git.rs            # Git 操作
    ├── directory.rs       # 目录管理
    ├── binding.rs        # 任务绑定
    └── cleanup.rs        # 自动清理
```

---

## 提交记录模板

| Step | 提交信息 |
|------|----------|
| 5.1 | feat: add team message format |
| 5.2 | feat: implement agent mailbox |
| 5.3 | feat: define team manager interface |
| 5.4 | feat: implement team manager |
| 5.5 | feat: define team agent trait |
| 5.6 | feat: implement message handling behavior |
| 5.7 | feat: implement event listening behavior |
| 5.8 | feat: implement autonomy behavior |
| 5.9 | feat: implement team agent |
| 5.10 | feat: add agent factory |
| 5.11 | feat: define worktree manager interface |
| 5.12 | feat: implement worktree manager |
| 5.13 | feat: implement git worktree operations |
| 5.14 | feat: implement worktree directory management |
| 5.15 | feat: implement task-worktree binding |
| 5.16 | feat: implement auto cleanup |
| 5.17 | feat: add collaboration config options |
| 5.18 | feat: implement team lifecycle |
| 5.19 | test: add team and worktree tests |

---

## 验收标准

- [ ] 5 个 Agent 可以同时运行（复用阶段2实现）
- [ ] 消息队列正常工作
- [ ] 消息可以正确路由到目标 Agent
- [ ] Agent 可以进行团队协调
- [ ] 每个任务有对应的 worktree
- [ ] Worktree 正确隔离
- [ ] 任务完成时 worktree 正确清理
- [ ] 单元测试通过