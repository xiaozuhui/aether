// src/debugger/breakpoint.rs
//! Breakpoint management for the Aether debugger

/// Represents the type of a breakpoint
#[derive(Debug, Clone, PartialEq)]
pub enum BreakpointType {
    /// Line breakpoint at a specific file and line
    Line { file: String, line: usize },
    /// Function entry breakpoint
    Function { name: String },
    /// Conditional breakpoint (evaluates condition before pausing)
    Conditional {
        file: String,
        line: usize,
        condition: String,
    },
}

/// Represents a breakpoint
#[derive(Debug, Clone, PartialEq)]
pub struct Breakpoint {
    pub id: usize,
    pub bp_type: BreakpointType,
    pub enabled: bool,
    pub hit_count: usize,
    pub ignore_count: usize, // Skip first N hits
}

impl Breakpoint {
    /// Create a new breakpoint
    pub fn new(id: usize, bp_type: BreakpointType) -> Self {
        Breakpoint {
            id,
            bp_type,
            enabled: true,
            hit_count: 0,
            ignore_count: 0,
        }
    }

    /// Check if this breakpoint should trigger at the given location
    pub fn should_trigger(&mut self, file: &str, line: usize) -> bool {
        if !self.enabled {
            return false;
        }

        let matches = match &self.bp_type {
            BreakpointType::Line { file: f, line: l } => f == file && *l == line,
            BreakpointType::Conditional {
                file: f, line: l, ..
            } => f == file && *l == line,
            BreakpointType::Function { .. } => false, // Function breakpoints are checked separately
        };

        if matches && self.hit_count >= self.ignore_count {
            self.hit_count += 1;
            true
        } else if matches {
            self.hit_count += 1;
            false
        } else {
            false
        }
    }

    /// Check if this is a function breakpoint for the given function name
    pub fn is_function_breakpoint(&self, func_name: &str) -> bool {
        match &self.bp_type {
            BreakpointType::Function { name } => name == func_name,
            _ => false,
        }
    }

    /// Get the condition expression if this is a conditional breakpoint
    pub fn get_condition(&self) -> Option<&str> {
        match &self.bp_type {
            BreakpointType::Conditional { condition, .. } => Some(condition.as_str()),
            _ => None,
        }
    }

    /// Get a string representation of the breakpoint location
    pub fn location_string(&self) -> String {
        match &self.bp_type {
            BreakpointType::Line { file, line } => {
                format!("{}:{}", file, line)
            }
            BreakpointType::Function { name } => {
                format!("function '{}'", name)
            }
            BreakpointType::Conditional {
                file,
                line,
                condition,
            } => {
                format!("{}:{} if {}", file, line, condition)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_creation() {
        let bp = Breakpoint::new(
            1,
            BreakpointType::Line {
                file: "test.aether".to_string(),
                line: 10,
            },
        );

        assert_eq!(bp.id, 1);
        assert!(bp.enabled);
        assert_eq!(bp.hit_count, 0);
    }

    #[test]
    fn test_breakpoint_trigger() {
        let mut bp = Breakpoint::new(
            1,
            BreakpointType::Line {
                file: "test.aether".to_string(),
                line: 10,
            },
        );

        assert!(bp.should_trigger("test.aether", 10));
        assert!(!bp.should_trigger("test.aether", 11));
        assert!(!bp.should_trigger("other.aether", 10));
    }

    #[test]
    fn test_disabled_breakpoint() {
        let mut bp = Breakpoint::new(
            1,
            BreakpointType::Line {
                file: "test.aether".to_string(),
                line: 10,
            },
        );
        bp.enabled = false;

        assert!(!bp.should_trigger("test.aether", 10));
    }

    #[test]
    fn test_ignore_count() {
        let mut bp = Breakpoint::new(
            1,
            BreakpointType::Line {
                file: "test.aether".to_string(),
                line: 10,
            },
        );
        bp.ignore_count = 2;

        assert!(!bp.should_trigger("test.aether", 10)); // hit 1, ignored
        assert!(!bp.should_trigger("test.aether", 10)); // hit 2, ignored
        assert!(bp.should_trigger("test.aether", 10)); // hit 3, triggers
    }

    #[test]
    fn test_function_breakpoint() {
        let bp = Breakpoint::new(
            1,
            BreakpointType::Function {
                name: "myFunc".to_string(),
            },
        );

        assert!(bp.is_function_breakpoint("myFunc"));
        assert!(!bp.is_function_breakpoint("otherFunc"));
    }
}
