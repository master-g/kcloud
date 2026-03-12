# Step 1.11: bash 工具实现

**目标**: 实现命令执行工具

**涉及文件**:
- `src/tools/bash.rs`

**具体改动**:
- 实现 BashTool 结构体
- 实现 Tool trait
- 功能：
  - 执行 shell 命令
  - 命令白名单（可选）
  - 超时处理
  - 输出捕获（stdout/stderr）

**验收标准**: cargo check 通过 + 单元测试

**提交信息**: `feat: implement bash tool`
