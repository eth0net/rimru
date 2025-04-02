use std::cmp::Ordering;

use super::ModMetaData;

/// Sort order for mods
pub enum Order {
    /// Sort alphabetically by name
    Name,
    /// Sort alphabetically by id
    Id,
    /// Sort by date created on disk
    Created,
    /// Sort by date modified on disk
    Modified,
    /// Sort alphabetically by dependency
    Dependencies,
    /// Sort topologically by dependency
    Topological,
    // todo: sort by steam details like subscribed, uploaded etc
}

impl Order {
    pub fn sort_fn(&self) -> impl Fn(&ModMetaData, &ModMetaData) -> Ordering {
        match self {
            Order::Name => cmp_name,
            Order::Id => cmp_id,
            Order::Created => cmp_created,
            Order::Modified => cmp_modified,
            Order::Dependencies => cmp_dependencies,
            Order::Topological => cmp_topological,
        }
    }
}

fn cmp_name(a: &ModMetaData, b: &ModMetaData) -> Ordering {
    match a.name.cmp(&b.name) {
        Ordering::Equal => a.id.cmp(&b.id),
        other => other,
    }
}

fn cmp_id(a: &ModMetaData, b: &ModMetaData) -> Ordering {
    match a.id.cmp(&b.id) {
        Ordering::Equal => a.name.cmp(&b.name),
        other => other,
    }
}

fn cmp_created(a: &ModMetaData, b: &ModMetaData) -> Ordering {
    a.created.cmp(&b.created)
}

fn cmp_modified(a: &ModMetaData, b: &ModMetaData) -> Ordering {
    a.modified.cmp(&b.modified)
}

fn cmp_dependencies(a: &ModMetaData, b: &ModMetaData) -> Ordering {
    let a_needs_b = a.dependencies.iter().any(|dep| dep.id == b.id);
    let b_needs_a = b.dependencies.iter().any(|dep| dep.id == a.id);

    if a_needs_b && b_needs_a {
        log::error!(
            "Circular dependency detected between {} and {}",
            a.name,
            b.name
        );
        cmp_name(a, b)
    } else if a_needs_b {
        Ordering::Greater
    } else if b_needs_a {
        Ordering::Less
    } else {
        cmp_name(a, b)
    }
}

fn cmp_topological(a: &ModMetaData, b: &ModMetaData) -> Ordering {
    match a.dependencies.len().cmp(&b.dependencies.len()) {
        Ordering::Equal => cmp_dependencies(a, b),
        order => order,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmp_name_eq() {
        let mut mods = vec![
            ModMetaData {
                id: "b".into(),
                name: "B".into(),
                ..Default::default()
            },
            ModMetaData {
                id: "a".into(),
                name: "A".into(),
                ..Default::default()
            },
            ModMetaData {
                id: "a".into(),
                name: "A".into(),
                ..Default::default()
            },
        ];
        mods.sort_unstable_by(cmp_name);
        assert_eq!(mods[0].id, "a".to_string());
        assert_eq!(mods[1].id, "a".to_string());
        assert_eq!(mods[2].id, "b".to_string());
    }

    #[test]
    fn test_cmp_name_ne() {
        let mut mods = vec![
            ModMetaData {
                id: "b".into(),
                name: "B".into(),
                ..Default::default()
            },
            ModMetaData {
                id: "a1".into(),
                name: "A".into(),
                ..Default::default()
            },
            ModMetaData {
                id: "a2".into(),
                name: "A".into(),
                ..Default::default()
            },
        ];
        mods.sort_unstable_by(cmp_name);
        assert_eq!(mods[0].id, "a1".to_string());
        assert_eq!(mods[1].id, "a2".to_string());
        assert_eq!(mods[2].id, "b".to_string());
    }

    #[test]
    fn test_cmp_id_eq() {
        let mut mods = vec![
            ModMetaData {
                id: "b".into(),
                name: "B".into(),
                ..Default::default()
            },
            ModMetaData {
                id: "a".into(),
                name: "A".into(),
                ..Default::default()
            },
            ModMetaData {
                id: "a".into(),
                name: "A".into(),
                ..Default::default()
            },
        ];
        mods.sort_unstable_by(cmp_id);
        assert_eq!(mods[0].name, "A".to_string());
        assert_eq!(mods[1].name, "A".to_string());
        assert_eq!(mods[2].name, "B".to_string());
    }

    #[test]
    fn test_cmp_id_ne() {
        let mut mods = vec![
            ModMetaData {
                id: "b".into(),
                name: "B".into(),
                ..Default::default()
            },
            ModMetaData {
                id: "a".into(),
                name: "A".into(),
                ..Default::default()
            },
            ModMetaData {
                id: "a".into(),
                name: "A".into(),
                ..Default::default()
            },
        ];
        mods.sort_unstable_by(cmp_id);
        assert_eq!(mods[0].name, "A".to_string());
        assert_eq!(mods[1].name, "A".to_string());
        assert_eq!(mods[2].name, "B".to_string());
    }
}
