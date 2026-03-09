# 阶段 3：内存管理 (Memory Management)

## 阶段目标

三层上下文压缩

## 主要内容

### 3.1 第一层压缩：工具结果压缩

**触发时机**：每次调用 LLM 之前

**实现方式**：
- 将旧的 tool result 替换为占位符
- 保留工具调用的历史记录
- 压缩实际结果内容

### 3.2 第二层压缩：阈值触发压缩

**触发时机**：达到上下文限制时

**实现方式**：
1. 动态计算阈值（当前模型最大上下文的一定比例，如 80%）
2. 超过阈值时：
   - 保存完整对话到磁盘
   - 让 LLM 生成摘要
   - 用摘要替换完整对话历史

### 3.3 第三层压缩：用户主动压缩

**触发时机**：用户使用 `/compact` 命令

**实现方式**：
- 与第二层相同的方式
- 由用户主动触发
- 压缩程度最高

### 3.4 混合存储管理

**策略**：
- 活跃上下文保存在内存中（快速访问）
- 历史上下文缓存到磁盘（节省内存）
- 支持按需加载历史上下文

---

## 子步骤规划（可独立提交）

### Step 3.1: Token 计数器

**目标**: 实现 token 计数功能

**涉及文件**:
- `src/memory/tokenizer.rs` (新建)

**具体改动**:
- 实现 TokenCounter 结构体
- **优先级1**：实现简单 token 计数（按字符/词，1 token ≈ 4 字符）
- **优先级2**：集成 `tiktoken-rs` crate 实现准确计数
- 配置项：max_tokens, threshold_ratio

> **注意**：准确 token 计数对上下文压缩至关重要，建议实现 tiktoken 集成。

**验收标准**: cargo check 通过 + 单元测试

---

### Step 3.2: 消息类型定义

**目标**: 定义对话消息类型

**涉及文件**:
- `src/memory/message.rs` (新建)

**具体改动**:
- 定义 Message enum:
  - User(String)
  - Assistant(String, Vec<ToolCall>)
  - ToolResult { call_id, result }
  - Compressed { summary, original_path }
- 实现序列化/反序列化

**验收标准**: cargo check 通过

---

### Step 3.3: 占位符格式设计

**目标**: 设计工具结果占位符格式

**涉及文件**:
- `src/memory/placeholder.rs` (新建)

**具体改动**:
- 设计占位符格式:
  ```
  <placeholder: tool_name args, size_info>
  ```
- 实现占位符生成函数
- 实现占位符解析函数

**验收标准**: cargo check 通过

---

### Step 3.4: 第一层压缩 - 工具结果替换

**目标**: 实现工具结果替换为占位符

**涉及文件**:
- `src/memory/compression/layer1.rs` (新建)

**具体改动**:
- 实现 compress_tool_results() 函数
- 保留工具调用历史
- 替换实际结果为占位符

**验收标准**: cargo check 通过 + 单元测试

---

### Step 3.5: 对话历史管理器

**目标**: 实现对话历史管理

**涉及文件**:
- `src/memory/history.rs` (新建)

**具体改动**:
- 实现 ConversationHistory 结构体
- 实现添加消息
- 实现获取历史
- 实现 token 计数

**验收标准**: cargo check 通过

---

### Step 3.6: 第二层压缩 - 对话保存

**目标**: 实现对话保存到磁盘

**涉及文件**:
- `src/memory/storage.rs` (新建)

**具体改动**:
- 实现 ConversationStorage 结构体
- 实现保存对话到 .conversation/ 目录
- 实现对话文件格式（JSON）
- 实现对话 ID 生成

**验收标准**: cargo check 通过

---

### Step 3.7: 第二层压缩 - 摘要生成

**目标**: 实现 LLM 摘要生成

**涉及文件**:
- `src/memory/summarizer.rs` (新建)

**具体改动**:
- 设计摘要 prompt
- 实现 LLM 调用
- 实现摘要解析

**验收标准**: cargo check 通过

---

### Step 3.8: 第二层压缩 - 阈值检测

**目标**: 实现上下文阈值检测

**涉及文件**:
- `src/memory/threshold.rs` (新建)

**具体改动**:
- 实现 threshold_check() 函数
- 动态计算阈值（max_tokens * threshold_ratio）
- 实现触发检测

**验收标准**: cargo check 通过 + 单元测试

---

### Step 3.9: 第二层压缩 - 历史替换

**目标**: 实现对话历史替换

**涉及文件**:
- `src/memory/compression/layer2.rs` (新建)

**具体改动**:
- 实现 compress_conversation() 函数
- 保存完整对话到磁盘
- 生成摘要
- 用摘要替换历史

**验收标准**: cargo check 通过

---

### Step 3.10: 第三层压缩 - /compact 命令

**目标**: 实现用户主动压缩命令

**涉及文件**:
- `src/memory/commands.rs` (新建)
- `src/cli.rs` (修改)

**具体改动**:
- 添加 /compact 命令
- 复用第二层压缩逻辑
- 最高程度压缩

**验收标准**: cargo check 通过

---

### Step 3.11: 上下文管理器

**目标**: 实现统一的上下文管理器

**涉及文件**:
- `src/memory/context.rs` (新建)
- `src/memory/mod.rs` (新建)

**具体改动**:
- 实现 ContextManager 结构体
- 整合所有压缩层
- 实现主压缩入口

**验收标准**: cargo check 通过

---

### Step 3.12: 按需加载

**目标**: 实现历史上下文按需加载

**涉及文件**:
- `src/memory/loader.rs` (新建)

**具体改动**:
- 实现 load_history() 函数
- 实现历史搜索
- 实现部分加载

**验收标准**: cargo check 通过

---

### Step 3.13: 集成到 Agent

**目标**: 将内存管理集成到主 Agent

**涉及文件**:
- `src/agent.rs` (修改)

**具体改动**:
- 添加 ContextManager
- 在 LLM 调用前触发压缩
- 处理 /compact 命令

**验收标准**: cargo check 通过

---

### Step 3.14: 配置项

**目标**: 添加内存管理相关配置

**涉及文件**:
- `src/config.rs` (修改)

**具体改动**:
- 添加 max_tokens 配置
- 添加 threshold_ratio 配置
- 添加 history_dir 配置

**验收标准**: cargo check 通过

---

### Step 3.15: 测试

**目标**: 添加内存管理测试

**涉及文件**:
- `tests/memory_test.rs` (新建)

**具体改动**:
- 添加 token 计数测试
- 添加压缩测试
- 添加存储测试

**验收标准**: cargo test 通过

---

## 目录结构

```
src/
├── agent.rs              # 主 Agent（修改）
├── config.rs             # 配置（修改）
└── memory/               # 内存管理模块（新建）
    ├── mod.rs
    ├── message.rs        # 消息类型
    ├── history.rs        # 对话历史
    ├── tokenizer.rs      # Token 计数
    ├── placeholder.rs    # 占位符格式
    ├── storage.rs        # 磁盘存储
    ├── summarizer.rs     # 摘要生成
    ├── threshold.rs      # 阈值检测
    ├── loader.rs         # 按需加载
    ├── commands.rs       # /compact 命令
    ├── context.rs        # 上下文管理器
    └── compression/
        ├── mod.rs
        ├── layer1.rs     # 第一层压缩
        └── layer2.rs     # 第二层压缩
```

---

## 提交记录模板

| Step | 提交信息 |
|------|----------|
| 3.1 | feat: add token counter |
| 3.2 | feat: define message types |
| 3.3 | feat: design placeholder format |
| 3.4 | feat: implement layer 1 compression |
| 3.5 | feat: implement conversation history |
| 3.6 | feat: implement conversation storage |
| 3.7 | feat: implement summarizer |
| 3.8 | feat: implement threshold detection |
| 3.9 | feat: implement layer 2 compression |
| 3.10 | feat: add /compact command |
| 3.11 | feat: implement context manager |
| 3.12 | feat: implement on-demand loading |
| 3.13 | feat: integrate memory management to agent |
| 3.14 | feat: add memory config options |
| 3.15 | test: add memory tests |

---

## 验收标准

- [ ] 第一层压缩正常工作（工具结果被正确替换为占位符）
- [ ] 第二层压缩正常工作（阈值触发时自动压缩）
- [ ] 第三层压缩正常工作（/compact 命令可以触发压缩）
- [ ] 对话历史正确保存到磁盘
- [ ] 摘要质量合理
- [ ] 混合存储正常工作
- [ ] 单元测试通过