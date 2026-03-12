# Step 1.4: 配置管理

**目标**: 实现配置加载逻辑

**涉及文件**:
- `src/config.rs`

**具体改动**:
- 定义 Config 结构体
- 实现配置加载（从 ~/.config/kloud/config.toml）
- 支持环境变量覆盖
- 配置项：
  - LlmConfig (api_key, api_base_url, model)
  - AgentConfig (workdir, max_concurrent)
  - ToolsConfig (allowed_commands)

**验收标准**: cargo check 通过

**提交信息**: `feat: add configuration management`
