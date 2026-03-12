# Step 1.6: 工具 trait 定义

**目标**: 定义工具的基础 trait

**涉及文件**:
- `src/tools/mod.rs`

**具体改动**:
- 创建 tools 模块目录
- 定义 Tool trait:
  - name() - 工具名称
  - description() - 工具描述
  - execute() - 执行工具
- 定义 ToolResult 结构体
- 定义 ToolCall 结构体
- 实现路径安全检查函数

**验收标准**: cargo check 通过

**提交信息**: `feat: add tool trait definition`
