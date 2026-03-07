# 阶段 4：并发 (Concurrency)

## 阶段目标

后台任务，任务队列和并发执行，用户可配置并发数量

## 主要内容

### 4.1 核心并发模型

**设计理念**：
- 主循环保持单线程（顺序执行）
- 子进程 I/O 并行化
- 守护线程后台监控

**架构**：
```
┌─────────────────────────────────────┐
│           主循环 (单线程)            │
│  ┌─────────────────────────────┐   │
│  │ 1. 处理用户输入              │   │
│  │ 2. 调用 LLM                  │   │
│  │ 3. 排空通知队列              │   │
│  │ 4. 分发新任务                │   │
│  └─────────────────────────────┘   │
└─────────────────────────────────────┘
         ↑              ↓
    通知队列      任务队列
         ↑              ↓
┌─────────────────────────────────────┐
│        守护线程 (单线程)             │
│  - 监控所有子进程                   │
│  - 子进程完成后写入通知队列          │
└─────────────────────────────────────┘
         ↑
┌─────────────────────────────────────┐
│        子进程池 (多线程)             │
│  - 最大 N 个并发                     │
│  - I/O 操作并行化                    │
└─────────────────────────────────────┘
```

### 4.2 线程安全任务队列

**数据结构**：
```rust
use std::sync::mpsc;
use std::sync::Arc;

struct TaskQueue {
    sender: mpsc::Sender<Task>,
    receiver: Arc<Mutex<mpsc::Receiver<Task>>>,
}

struct Task {
    id: String,
    command: String,
    args: Vec<String>,
    workdir: PathBuf,
}
```

### 4.3 守护线程

**职责**：
- 监控所有子进程
- 子进程完成后写入通知队列
- 处理超时

**实现**：
```rust
struct DaemonThread {
    task_queue: TaskQueue,
    notification_queue: NotificationQueue,
    max_concurrent: usize,
}

impl DaemonThread {
    fn spawn() -> JoinHandle<()> {
        // 启动守护线程
    }
    
    fn submit_task(&self, task: Task) {
        // 提交新任务
    }
}
```

### 4.4 通知队列

**数据结构**：
```rust
struct NotificationQueue {
    results: Arc<Mutex<Vec<TaskResult>>>,
}

struct TaskResult {
    task_id: String,
    stdout: String,
    stderr: String,
    exit_code: i32,
    duration: Duration,
}
```

### 4.5 主循环集成

**流程**：
```
loop {
    // 1. 排空通知队列
    while let Ok(result) = notification_queue.try_recv() {
        handle_result(result);
    }
    
    // 2. 处理用户输入
    let input = read_input();
    
    // 3. 调用 LLM
    let response = llm.invoke(&input).await;
    
    // 4. 分发新任务
    for tool_call in response.tool_calls {
        if let Tool::Bash(command) = tool_call {
            daemon.submit_task(command);
        }
    }
}
```

## 实施步骤

### 4.1 任务队列实现

**步骤 4.1.1**: 定义任务结构

**步骤 4.1.2**: 实现线程安全的任务队列

**步骤 4.1.3**: 实现任务提交和获取

### 4.2 守护线程实现

**步骤 4.2.1**: 设计守护线程架构

**步骤 4.2.2**: 实现子进程管理

**步骤 4.2.3**: 实现超时处理

**步骤 4.2.4**: 实现通知写入

### 4.3 通知队列实现

**步骤 4.3.1**: 定义通知结构

**步骤 4.3.2**: 实现线程安全的通知队列

### 4.4 主循环集成

**步骤 4.4.1**: 修改主循环逻辑

**步骤 4.4.2**: 实现通知队列排空

**步骤 4.4.3**: 实现任务分发

### 4.5 配置支持

**步骤 4.5.1**: 添加并发数量配置项

**步骤 4.5.2**: 实现配置读取

## 验收标准

- [ ] 任务队列线程安全
- [ ] 守护线程正确监控子进程
- [ ] 子进程完成后结果进入通知队列
- [ ] 主循环每次调用 LLM 前排空通知队列
- [ ] 并发数量可配置
- [ ] 超时处理正常工作
- [ ] 单元测试通过

## 技术细节

### 配置项

```toml
[agent]
max_concurrent = 5  # 最大并发子进程数
task_timeout = 300  # 任务超时时间（秒）
```

### 错误处理

- 子进程超时 → 杀死进程，返回超时错误
- 子进程崩溃 → 返回错误信息
- 队列满 → 等待或返回队列满错误
