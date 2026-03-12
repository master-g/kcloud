# Step 1.8: read 工具实现

**目标**: 实现文件读取工具

**涉及文件**:
- `src/tools/read.rs`

**具体改动**:
- 实现 ReadTool 结构体
- 实现 Tool trait
- 功能：
  - 读取指定文件内容
  - 支持行号范围参数
  - 路径安全检查

**验收标准**: cargo check 通过 + 单元测试

**提交信息**: `feat: implement read tool`
