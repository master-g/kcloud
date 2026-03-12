# Step 1.12: git 工具实现

**目标**: 实现 Git 操作工具

**涉及文件**:
- `src/tools/git.rs`

**具体改动**:
- 实现 GitTool 结构体
- 实现 Tool trait
- 功能：
  - git status - 查看状态
  - git add / git commit - 提交更改
  - git branch / git checkout - 分支操作
  - git diff - 查看差异
  - 封装常用 Git 操作（作为 bash 的特化版本）

**验收标准**: cargo check 通过 + 单元测试

**提交信息**: `feat: implement git tool`
