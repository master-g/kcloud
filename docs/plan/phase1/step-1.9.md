# Step 1.9: write 工具实现

**目标**: 实现文件写入工具

**涉及文件**:
- `src/tools/write.rs`

**具体改动**:
- 实现 WriteTool 结构体
- 实现 Tool trait
- 功能：
  - 创建新文件
  - 覆盖现有文件
  - 路径安全检查

**验收标准**: cargo check 通过 + 单元测试

**提交信息**: `feat: implement write tool`
