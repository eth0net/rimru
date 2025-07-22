#[derive(Debug, Clone)]
pub struct ModIssue {
    pub mod_id: String,
    pub missing_dependencies: Vec<String>,
    pub dependencies_after: Vec<String>, // dependencies that load after this mod (optional warning)
    pub load_order_violations: Vec<String>, // mods violating load_after/before rules
}
