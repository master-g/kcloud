//! Agent state management for Kloud

use serde::{Deserialize, Serialize};

use crate::tools::{ToolCall, ToolResult};

/// Action representation
#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
	/// Description of the action to be taken, e.g. "Search for Rust async programming examples", "Execute code snippet to calculate Fibonacci numbers", etc.
	pub description: String,

	/// List of tool calls that need to be executed as part of this action, can be empty if no tools are needed
	pub tool_calls: Vec<ToolCall>,
}

/// LLM response representation, placeholder for now, will move to separate module later
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	/// Content of the response from the LLM, can include text, instructions, or other information for the agent to process
	pub content: String,

	/// Optional list of tool calls that the LLM wants to execute based on the response content, can be empty if no tools are needed
	pub tool_calls: Option<Vec<ToolCall>>,
}

/// Events that can trigger an agent loop iteration, such as new user input or tool results
#[derive(Debug)]
pub enum AgentEvent {
	/// New user input received, will trigger a new agent loop iteration
	UserInput(String),

	/// Response received from the LLM, will trigger processing of the response and potential tool calls
	LLMResponse(Response),

	/// Tool calls are ready to be executed, will trigger execution of the tools and processing of their results
	ToolCallsReady(Vec<ToolCall>),

	/// Tool execution completed, will trigger processing of the tool results and potential next steps
	ToolCompleted(ToolResult),

	/// Need confirmation from user before proceeding with an action, will trigger a confirmation prompt and wait for user response
	NeedConfirmation(Action),

	/// User approved the action, will proceed with executing the action and potential next steps
	UserApproved,

	/// User rejected the action, will abort the current plan and return to idle state
	UserRejected,

	/// User provided an alternative input or action, will trigger processing of the alternative and potential next steps
	UserAlternative(String),

	/// Agent has completed all tasks and is ready for new input, will return to idle state
	TaskCompleted,

	/// An error occurred during any step of the agent loop, will trigger error handling and return to idle state
	ErrorOccurred(String),
}

/// Represents the state of an agent loop
#[derive(Debug)]
pub enum AgentLoopState {
	/// The agent is idle, waiting for a new task or input
	Idle,
	/// The agent is waiting for a response from the LLM
	WaitingForLLM,
	/// The agent is executing one or more tools
	ExecutingTools {
		/// Remaining tool calls that are still being executed, if any
		remaining: Vec<ToolCall>,
	},
	/// The agent is processing the LLM response to determine next steps
	ProcessingSteering,
	/// The agent is processing the results of tool calls to determine next steps
	ProcessingFollowUp,
	/// The agent is waiting for user confirmation before proceeding with an action
	WaitingForConfirmation(Action),
	/// The agent has completed its tasks
	Completed,
	/// The agent has encountered an error
	Error(String),
}
