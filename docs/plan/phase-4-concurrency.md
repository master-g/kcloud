# 阶段 4：并发 (Concurrency)

## 阶段目标

后台任务，任务队列和并发执行，用户可配置并发数量

## 主要内容

### 4.1 核心并发模型

**设计理念**：
- 主循环保持单线程（顺序执行）
- 子进程 I/O 并行化
- 守护线程后台监控

### 4.2 线程安全任务队列

- 使用 mpsc 通道
- Arc + Mutex 包装

### 4.3 守护线程

- 监控所有子进程
- 子进程完成后写入通知队列
- 处理超时

### 4.4 通知队列

- 线程安全的结果队列
- TaskResult 包含 stdout, stderr, exit_code, duration

### 4.5 主循环集成

- 每次 LLM 调用前排空通知队列
- 任务分发到守护线程

---

## 子步骤规划（可独立提交）

### Step 4.1: 任务数据结构

**目标**: 定义任务数据结构

**涉及文件**:
- `src/concurrency/mod.rs` (新建)
- `src/concurrency/task.rs` (新建)

**具体改动**:
- 定义 Task 结构体:
  - id, command, args, workdir
- 定义 TaskStatus enum: Pending, Running, Completed, Failed, Timeout

**验收标准**: cargo check 通过

---

### Step 4.2: 任务队列实现

**目标**: 实现线程安全任务队列

**涉及文件**:
- `src/concurrency/queue.rs` (新建)

**具体改动**:
- 定义 TaskQueue 结构体
- 使用 mpsc::channel
- 使用 Arc<Mutex<Receiver>> 共享
- 实现 submit() 和 recv()

**验收标准**: cargo check 通过 + 单元测试

---

### Step 4.3: 任务结果结构

**目标**: 定义任务结果数据结构

**涉及文件**:
- `src/concurrency/result.rs` (新建)

**具体改动**:
- 定义 TaskResult 结构体:
  - task_id, stdout, stderr, exit_code, duration
- 实现序列化（用于日志）

**验收标准**: cargo check 通过

---

### Step 4.4: 通知队列实现

**目标**: 实现线程安全通知队列

**涉及文件**:
- `src/concurrency/notification.rs` (新建)

**具体改动**:
- 定义 NotificationQueue 结构体
- 使用 Arc<Mutex<Vec<TaskResult>>>
- 实现 push() 和 try_pop()
- 实现 drain() 批量获取

**验收标准**: cargo check 通过 + 单元测试

---

### Step 4.5: 子进程执行器

**目标**: 实现子进程执行逻辑

**涉及文件**:
- `src/concurrency/executor.rs` (新建)

**具体改动**:
- 定义 Executor 结构体
- 实现 spawn_child() - 启动子进程
- 实现 wait_child() - 等待子进程完成
- 实现 kill_child() - 杀死子进程

**验收标准**: cargo check 通过 + 单元测试

---

### Step 4.6: 进程管理器

**目标**: 管理多个子进程

**涉及文件**:
- `src/concurrency/process.rs` (新建)

**具体改动**:
- 定义 ProcessManager 结构体
- 维护活跃进程映射
- 实现 add() - 添加新进程
- 实现 remove() - 移除完成进程

**验收标准**: cargo check 通过

---

### Step 4.7: 守护线程 - 主循环

**目标**: 实现守护线程主循环

**涉及文件**:
- `src/concurrency/daemon.rs` (新建)

**具体改动**:
- 定义 Daemon 结构体
- 实现 run() - 主循环
- 从任务队列获取任务
- 启动子进程
- 等待完成
- 写入通知队列

**验收标准**: cargo check 通过

---

### Step 4.8: 守护线程 - 超时处理

**目标**: 实现超时检测和处理

**涉及文件**:
- `src/concurrency/daemon.rs` (修改)

**具体改动**:
- 实现超时检测逻辑
- 实现超时处理（杀死进程）
- 超时结果写入通知队列

**验收标准**: cargo check 通过

---

### Step 4.9: 守护线程 - 并发限制

**目标**: 实现并发数量限制

**涉及文件**:
- `src/concurrency/daemon.rs` (修改)

**具体改动**:
- 实现 max_concurrent 配置
- 实现等待队列
- 控制同时运行的进程数

**验收标准**: cargo check 通过

---

### Step 4.10: 守护线程启动/停止

**目标**: 实现守护线程生命周期管理

**涉及文件**:
- `src/concurrency/daemon.rs` (修改)

**具体改动**:
- 实现 spawn() - 启动守护线程
- 实现 shutdown() - 优雅停止
- 实现 submit() - 提交任务

**验收标准**: cargo check 通过

---

### Step 4.11: 配置项

**目标**: 添加并发相关配置

**涉及文件**:
- `src/config.rs` (修改)

**具体改动**:
- 添加 max_concurrent 配置
- 添加 task_timeout 配置

**验收标准**: cargo check 通过

---

### Step 4.12: Agent 集成 - 任务提交

**目标**: 将后台任务集成到 Agent

**涉及文件**:
- `src/agent.rs` (修改)

**具体改动**:
- 修改 BashTool 使用守护线程
- 实现异步任务提交

**验收标准**: cargo check 通过

---

### Step 4.13: Agent 集成 - 结果处理

**目标**: 处理任务完成结果

**涉及文件**:
- `src/agent.rs` (修改)

**具体改动**:
- 添加 NotificationQueue 引用
- 实现 poll_results() - 轮询结果
- 在 LLM 调用前排空队列

**验收标准**: cargo check 通过

---

### Step 4.14: 错误处理

**目标**: 完善错误处理

**涉及文件**:
- `src/concurrency/error.rs` (新建)
- `src/error.rs` (修改)

**具体改动**:
- 定义 ConcurrencyError
- 处理超时错误
- 处理进程错误

**验收标准**: cargo check 通过

---

### Step 4.15: 测试

**目标**: 添加并发相关测试

**涉及文件**:
- `tests/concurrency_test.rs` (新建)

**具体改动**:
- 添加任务队列测试
- 添加守护线程测试
- 添加超时测试

**验收标准**: cargo test 通过

---

## 目录结构

```
src/
├── agent.rs              # 主 Agent（修改）
├── config.rs             # 配置（修改）
├── error.rs              # 错误类型（修改）
└── concurrency/          # 并发模块（新建）
    ├── mod.rs
    ├── task.rs           # 任务结构
    ├── queue.rs          # 任务队列
    ├── result.rs         # 任务结果
    ├── notification.rs   # 通知队列
    ├── executor.rs       # 子进程执行器
    ├── process.rs       # 进程管理
    ├── daemon.rs        # 守护线程
    └── error.rs         # 并发错误
```

---

## 提交记录模板

| Step | 提交信息 |
|------|----------|
| 4.1 | feat: add task data structures |
| 4.2 | feat: implement thread-safe task queue |
| 4.3 | feat: add task result structure |
| 4.4 | feat: implement notification queue |
| 4.5 | feat: implement process executor |
| 4.6 | feat: implement process manager |
| 4.7 | feat: implement daemon main loop |
| 4.8 | feat: add timeout handling |
| 4.9 | feat: add concurrency limit |
| 4.10 | feat: add daemon lifecycle management |
| 4.11 | feat: add concurrency config options |
| 4.12 | feat: integrate task submission to agent |
| 4.13 | feat: integrate result handling to agent |
| 4.14 | feat: add concurrency error handling |
| 4.15 | test: add concurrency tests |

---

## 验收标准

- [ ] 任务队列线程安全
- [ ] 守护线程正确监控子进程
- [ ] 子进程完成后结果进入通知队列
- [ ] 主循环每次调用 LLM 前排空通知队列
- [ ] 并发数量可配置
- [ ] 超时处理正常工作
- [ ] 单元测试通过
