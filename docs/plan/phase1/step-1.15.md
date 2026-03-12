# Step 1.15: Agent Loop 双循环实现

**目标**: 实现 pi-mono 的双循环 Agent Loop（核心难点）

**涉及文件**:
- `src/agent/loop.rs`

**具体改动**:
- 实现 agent_loop() - 带新 prompt 启动
- 实现 agent_loop_continue() - 从现有上下文继续
- 实现 run_loop() - 核心双循环逻辑：
  - Outer loop: 处理 follow-up 消息
  - Inner loop: 处理 tool calls 和 steering
  - 每个 tool 执行后检查 steering
  - 收到 steering 时跳过剩余 tools

- 关键逻辑：
  ```rust
  loop {  // outer: follow-up
      while has_tool_calls || has_pending {
          // inner: tool + steering
          process_pending_messages();
          assistant_msg = stream_response();
          
          for tool_call in assistant_msg.tool_calls() {
              result = execute_tool(tool_call);
              
              // 关键：每个 tool 后检查 steering
              if let Some(steering) = check_steering() {
                  skip_remaining_tools();
                  pending = steering;
                  break;
              }
          }
      }
      
      // 检查 follow-up
      if has_follow_up() {
          pending = dequeue_follow_up();
          continue; // 继续 outer loop
      }
      break;
  }
  ```

**参考实现** (pi-mono/packages/agent/src/agent-loop.ts):
- runLoop() 是核心双循环
- getSteeringMessages() 在每次 tool 后调用
- getFollowUpMessages() 在 agent 将停止时调用

**验收标准**:
- cargo check 通过
- 双循环逻辑正确
- 支持 steering 中断工具执行
- 支持 follow-up 连续对话

**提交信息**: `feat: implement agent loop with dual-loop architecture`
