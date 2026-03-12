# Step 1.7: LLM 客户端

**目标**: 实现 LLM API 调用封装

**涉及文件**:
- `src/llm/mod.rs`
- `src/llm/client.rs`
- `src/llm/models.rs`

**具体改动**:
- 定义 LLM 客户端 trait
- 实现 OpenAI 兼容 API 客户端
- 定义请求/响应结构体
- 实现流式响应支持（如需要）
- 实现重试和错误处理

**验收标准**: cargo check 通过

**提交信息**: `feat: add LLM client implementation`
