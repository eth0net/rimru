use std::fmt::Display;

#[derive(Clone, Debug, Default)]
pub struct ModIssues {
    pub mod_id: String,
    pub missing_dependencies: Vec<String>,
    pub load_order_violations: Vec<String>, // mods violating load_after/before rules
}

impl ModIssues {
    pub fn new(mod_id: String) -> Self {
        Self {
            mod_id,
            missing_dependencies: Vec::new(),
            load_order_violations: Vec::new(),
        }
    }

    pub fn add_missing_dependency(&mut self, dependency: String) {
        self.missing_dependencies.push(dependency);
    }

    pub fn add_load_order_violation(&mut self, violation: String) {
        self.load_order_violations.push(violation);
    }

    pub fn has_issues(&self) -> bool {
        self.has_warnings() || self.has_errors()
    }

    pub fn has_warnings(&self) -> bool {
        !self.load_order_violations.is_empty()
    }

    pub fn has_errors(&self) -> bool {
        !self.missing_dependencies.is_empty()
    }
}

impl Display for ModIssues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sections = Vec::new();

        if !self.missing_dependencies.is_empty() {
            sections.push(format!(
                "Missing dependencies:\n- {}",
                self.missing_dependencies.join("\n- ")
            ));
        }

        if !self.load_order_violations.is_empty() {
            sections.push(format!(
                "Load order violations:\n- {}",
                self.load_order_violations.join("\n- ")
            ));
        }

        write!(f, "{}", sections.join("\n\n"))
    }
}
