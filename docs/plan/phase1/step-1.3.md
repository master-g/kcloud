# Step 1.3: 错误类型定义

**目标**: 定义项目使用的错误类型

**涉及文件**:
- `src/error.rs`

**具体改动**:
- 创建 error 模块
- 定义 Error enum，使用 thiserror
- 实现 std::error::Error
- 添加常见错误类型：
  - ConfigError - 配置错误
  - ToolError - 工具执行错误
  - AgentError - Agent 错误
  - LLMError - LLM API 错误

**验收标准**: cargo check 通过

**提交信息**: `feat: add error types with thiserror`
