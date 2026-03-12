# Step 1.10: edit 工具实现

**目标**: 实现文件编辑工具

**涉及文件**:
- `src/tools/edit.rs`

**具体改动**:
- 实现 EditTool 结构体
- 实现 Tool trait
- 功能：
  - 按行号替换
  - 按内容替换
  - 路径安全检查

**验收标准**: cargo check 通过 + 单元测试

**提交信息**: `feat: implement edit tool`
