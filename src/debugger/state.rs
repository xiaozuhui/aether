// src/debugger/state.rs
//! Debugger state management

use crate::debugger::breakpoint::{Breakpoint, BreakpointType};
use std::collections::HashMap;

/// Execution control mode
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionMode {
    /// Normal execution (check breakpoints only)
    Normal,
    /// Step into function calls
    StepInto,
    /// Step over function calls
    StepOver,
    /// Step out of current function
    StepOut,
    /// Continue until next breakpoint
    Continue,
}

/// Core debugger state
pub struct DebuggerState {
    /// All breakpoints indexed by ID
    breakpoints: HashMap<usize, Breakpoint>,
    /// Next breakpoint ID to assign
    next_breakpoint_id: usize,
    /// Current execution mode
    execution_mode: ExecutionMode,
    /// Current source file location (file, line)
    current_location: Option<(String, usize)>,
    /// Stack depth for step over/step out operations
    step_over_depth: Option<usize>,
    /// Whether debugger is active
    is_active: bool,
}

impl DebuggerState {
    /// Create a new debugger state
    pub fn new() -> Self {
        DebuggerState {
            breakpoints: HashMap::new(),
            next_breakpoint_id: 1,
            execution_mode: ExecutionMode::Normal,
            current_location: None,
            step_over_depth: None,
            is_active: false,
        }
    }

    /// Activate the debugger
    pub fn activate(&mut self) {
        self.is_active = true;
    }

    /// Deactivate the debugger
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Check if debugger is active
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Set a breakpoint and return its ID
    pub fn set_breakpoint(&mut self, bp_type: BreakpointType) -> usize {
        let id = self.next_breakpoint_id;
        self.next_breakpoint_id += 1;
        self.breakpoints.insert(id, Breakpoint::new(id, bp_type));
        id
    }

    /// Remove a breakpoint by ID
    pub fn remove_breakpoint(&mut self, id: usize) -> bool {
        self.breakpoints.remove(&id).is_some()
    }

    /// Remove all breakpoints
    pub fn remove_all_breakpoints(&mut self) {
        self.breakpoints.clear();
    }

    /// Enable or disable a breakpoint
    pub fn toggle_breakpoint(&mut self, id: usize, enabled: bool) -> bool {
        if let Some(bp) = self.breakpoints.get_mut(&id) {
            bp.enabled = enabled;
            true
        } else {
            false
        }
    }

    /// Get a breakpoint by ID
    pub fn get_breakpoint(&self, id: usize) -> Option<&Breakpoint> {
        self.breakpoints.get(&id)
    }

    /// List all breakpoints
    pub fn list_breakpoints(&self) -> Vec<&Breakpoint> {
        let mut bps: Vec<_> = self.breakpoints.values().collect();
        bps.sort_by_key(|bp| bp.id);
        bps
    }

    /// Set the execution mode
    pub fn set_execution_mode(&mut self, mode: ExecutionMode) {
        self.execution_mode = mode;
    }

    /// Get the current execution mode
    pub fn execution_mode(&self) -> &ExecutionMode {
        &self.execution_mode
    }

    /// Set the step over depth
    pub fn set_step_over_depth(&mut self, depth: usize) {
        self.step_over_depth = Some(depth);
    }

    /// Clear the step over depth
    pub fn clear_step_over_depth(&mut self) {
        self.step_over_depth = None;
    }

    /// Update current location
    pub fn update_location(&mut self, file: String, line: usize) {
        self.current_location = Some((file, line));
    }

    /// Get current location
    pub fn current_location(&self) -> Option<&(String, usize)> {
        self.current_location.as_ref()
    }

    /// Check if a function breakpoint should trigger
    pub fn should_pause_at_function(&mut self, func_name: &str) -> bool {
        if !self.is_active {
            return false;
        }

        for bp in self.breakpoints.values_mut() {
            if bp.is_function_breakpoint(func_name) && bp.enabled {
                bp.hit_count += 1;
                if bp.hit_count > bp.ignore_count {
                    return true;
                }
            }
        }
        false
    }

    /// Check if execution should pause at the given location
    pub fn should_pause(&mut self, file: &str, line: usize, call_stack_depth: usize) -> bool {
        if !self.is_active {
            return false;
        }

        // Update current location
        self.current_location = Some((file.to_string(), line));

        match self.execution_mode {
            ExecutionMode::Normal => {
                // Check breakpoints only
                for bp in self.breakpoints.values_mut() {
                    if bp.should_trigger(file, line) {
                        return true;
                    }
                }
                false
            }
            ExecutionMode::StepInto => {
                // Pause at next statement
                self.execution_mode = ExecutionMode::Normal;
                true
            }
            ExecutionMode::StepOver => {
                // Pause when stack depth returns to the same level
                if let Some(target_depth) = self.step_over_depth {
                    if call_stack_depth <= target_depth {
                        self.step_over_depth = None;
                        self.execution_mode = ExecutionMode::Normal;
                        return true;
                    }
                }
                false
            }
            ExecutionMode::StepOut => {
                // Pause when stack depth becomes less than current
                if let Some(target_depth) = self.step_over_depth {
                    if call_stack_depth < target_depth {
                        self.step_over_depth = None;
                        self.execution_mode = ExecutionMode::Normal;
                        return true;
                    }
                }
                false
            }
            ExecutionMode::Continue => {
                // Pause only at breakpoints
                for bp in self.breakpoints.values_mut() {
                    if bp.should_trigger(file, line) {
                        self.execution_mode = ExecutionMode::Normal;
                        return true;
                    }
                }
                false
            }
        }
    }
}

impl Default for DebuggerState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_breakpoint() {
        let mut state = DebuggerState::new();
        let id = state.set_breakpoint(BreakpointType::Line {
            file: "test.aether".to_string(),
            line: 10,
        });

        assert_eq!(id, 1);
        assert!(state.get_breakpoint(id).is_some());
    }

    #[test]
    fn test_remove_breakpoint() {
        let mut state = DebuggerState::new();
        let id = state.set_breakpoint(BreakpointType::Line {
            file: "test.aether".to_string(),
            line: 10,
        });

        assert!(state.remove_breakpoint(id));
        assert!(!state.remove_breakpoint(id));
        assert!(state.get_breakpoint(id).is_none());
    }

    #[test]
    fn test_toggle_breakpoint() {
        let mut state = DebuggerState::new();
        let id = state.set_breakpoint(BreakpointType::Line {
            file: "test.aether".to_string(),
            line: 10,
        });

        assert!(state.toggle_breakpoint(id, false));
        assert!(!state.get_breakpoint(id).unwrap().enabled);
        assert!(state.toggle_breakpoint(id, true));
        assert!(state.get_breakpoint(id).unwrap().enabled);
    }

    #[test]
    fn test_should_pause_normal_mode() {
        let mut state = DebuggerState::new();
        state.activate();

        let _: usize = state.set_breakpoint(BreakpointType::Line {
            file: "test.aether".to_string(),
            line: 10,
        });

        assert!(state.should_pause("test.aether", 10, 0));
        assert!(!state.should_pause("test.aether", 11, 0));
    }

    #[test]
    fn test_step_into_mode() {
        let mut state = DebuggerState::new();
        state.activate();
        state.set_execution_mode(ExecutionMode::StepInto);

        // Should pause immediately
        assert!(state.should_pause("test.aether", 10, 0));
        // After stepping, mode returns to normal
        assert_eq!(state.execution_mode(), &ExecutionMode::Normal);
    }

    #[test]
    fn test_step_over_mode() {
        let mut state = DebuggerState::new();
        state.activate();
        state.set_execution_mode(ExecutionMode::StepOver);
        state.set_step_over_depth(2);

        // Should not pause while depth > 2
        assert!(!state.should_pause("test.aether", 10, 3));
        // Should pause when depth returns to 2
        assert!(state.should_pause("test.aether", 11, 2));
    }

    #[test]
    fn test_step_out_mode() {
        let mut state = DebuggerState::new();
        state.activate();
        state.set_execution_mode(ExecutionMode::StepOut);
        state.set_step_over_depth(2);

        // Should not pause while depth >= 2
        assert!(!state.should_pause("test.aether", 10, 2));
        // Should pause when depth < 2
        assert!(state.should_pause("test.aether", 11, 1));
    }

    #[test]
    fn test_function_breakpoint() {
        let mut state = DebuggerState::new();
        state.activate();
        state.set_breakpoint(BreakpointType::Function {
            name: "myFunc".to_string(),
        });

        assert!(state.should_pause_at_function("myFunc"));
        assert!(!state.should_pause_at_function("otherFunc"));
    }
}
