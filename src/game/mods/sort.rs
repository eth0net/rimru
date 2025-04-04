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
            Order::Id => cmp_id,
            Order::Name => cmp_name,
            Order::Created => cmp_created,
            Order::Modified => cmp_modified,
            Order::Dependencies => cmp_dependencies,
            Order::Topological => cmp_topological,
        }
    }
}

fn cmp_id(a: &ModMetaData, b: &ModMetaData) -> Ordering {
    match a.id.cmp(&b.id) {
        Ordering::Equal => a.name.cmp(&b.name),
        other => other,
    }
}

fn cmp_name(a: &ModMetaData, b: &ModMetaData) -> Ordering {
    match a.name.cmp(&b.name) {
        Ordering::Equal => cmp_id(a, b),
        other => other,
    }
}

fn cmp_created(a: &ModMetaData, b: &ModMetaData) -> Ordering {
    match a.created.cmp(&b.created) {
        Ordering::Equal => cmp_name(a, b),
        other => other,
    }
}

fn cmp_modified(a: &ModMetaData, b: &ModMetaData) -> Ordering {
    match a.modified.cmp(&b.modified) {
        Ordering::Equal => cmp_name(a, b),
        other => other,
    }
}

fn cmp_dependencies(a: &ModMetaData, b: &ModMetaData) -> Ordering {
    let a_needs_b = a.depends_on(&b.id);
    let b_needs_a = b.depends_on(&a.id);

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
    use std::time::{Duration, SystemTime};

    use super::*;

    #[test]
    fn test_cmp_id() {
        let a1 = ModMetaData {
            id: "a".to_string(),
            name: "a1".to_string(),
            ..Default::default()
        };
        let a2 = ModMetaData {
            id: "a".to_string(),
            name: "a2".to_string(),
            ..Default::default()
        };
        let b = ModMetaData {
            id: "b".to_string(),
            name: "b".to_string(),
            ..Default::default()
        };

        assert_eq!(cmp_id(&a1, &a1), Ordering::Equal);
        assert_eq!(cmp_id(&a1, &a2), Ordering::Less);
        assert_eq!(cmp_id(&a1, &b), Ordering::Less);
        assert_eq!(cmp_id(&a2, &a1), Ordering::Greater);
        assert_eq!(cmp_id(&a2, &a2), Ordering::Equal);
        assert_eq!(cmp_id(&a2, &b), Ordering::Less);
        assert_eq!(cmp_id(&b, &a1), Ordering::Greater);
        assert_eq!(cmp_id(&b, &a2), Ordering::Greater);
        assert_eq!(cmp_id(&b, &b), Ordering::Equal);
    }

    #[test]
    fn test_cmp_name() {
        let a1 = ModMetaData {
            id: "a1".to_string(),
            name: "a".to_string(),
            ..Default::default()
        };
        let a2 = ModMetaData {
            id: "a2".to_string(),
            name: "a".to_string(),
            ..Default::default()
        };
        let b = ModMetaData {
            id: "b".to_string(),
            name: "b".to_string(),
            ..Default::default()
        };

        assert_eq!(cmp_name(&a1, &a1), Ordering::Equal);
        assert_eq!(cmp_name(&a1, &a2), Ordering::Less);
        assert_eq!(cmp_name(&a1, &b), Ordering::Less);
        assert_eq!(cmp_name(&a2, &a1), Ordering::Greater);
        assert_eq!(cmp_name(&a2, &a2), Ordering::Equal);
        assert_eq!(cmp_name(&a2, &b), Ordering::Less);
        assert_eq!(cmp_name(&b, &a1), Ordering::Greater);
        assert_eq!(cmp_name(&b, &a2), Ordering::Greater);
        assert_eq!(cmp_name(&b, &b), Ordering::Equal);
    }

    #[test]
    fn test_cmp_created() {
        let now = SystemTime::now();
        let future = now + Duration::from_secs(10);

        let a1 = ModMetaData {
            id: "a".to_string(),
            name: "a".to_string(),
            created: Some(now),
            ..Default::default()
        };
        let a2 = ModMetaData {
            id: "b".to_string(),
            name: "a".to_string(),
            created: Some(now),
            ..Default::default()
        };
        let b = ModMetaData {
            id: "b".to_string(),
            name: "b".to_string(),
            created: Some(future),
            ..Default::default()
        };

        assert_eq!(cmp_created(&a1, &a1), Ordering::Equal);
        assert_eq!(cmp_created(&a1, &a2), Ordering::Less);
        assert_eq!(cmp_created(&a1, &b), Ordering::Less);
        assert_eq!(cmp_created(&a2, &a1), Ordering::Greater);
        assert_eq!(cmp_created(&a2, &a2), Ordering::Equal);
        assert_eq!(cmp_created(&a2, &b), Ordering::Less);
        assert_eq!(cmp_created(&b, &a1), Ordering::Greater);
        assert_eq!(cmp_created(&b, &a2), Ordering::Greater);
        assert_eq!(cmp_created(&b, &b), Ordering::Equal);
    }

    #[test]
    fn test_cmp_modified() {
        let now = SystemTime::now();
        let future = now + Duration::from_secs(10);

        let a1 = ModMetaData {
            id: "a".to_string(),
            name: "a".to_string(),
            modified: Some(now),
            ..Default::default()
        };
        let a2 = ModMetaData {
            id: "b".to_string(),
            name: "a".to_string(),
            modified: Some(now),
            ..Default::default()
        };
        let b = ModMetaData {
            id: "b".to_string(),
            name: "b".to_string(),
            modified: Some(future),
            ..Default::default()
        };

        assert_eq!(cmp_modified(&a1, &a1), Ordering::Equal);
        assert_eq!(cmp_modified(&a1, &a2), Ordering::Less);
        assert_eq!(cmp_modified(&a1, &b), Ordering::Less);
        assert_eq!(cmp_modified(&a2, &a1), Ordering::Greater);
        assert_eq!(cmp_modified(&a2, &a2), Ordering::Equal);
        assert_eq!(cmp_modified(&a2, &b), Ordering::Less);
        assert_eq!(cmp_modified(&b, &a1), Ordering::Greater);
        assert_eq!(cmp_modified(&b, &a2), Ordering::Greater);
        assert_eq!(cmp_modified(&b, &b), Ordering::Equal);
    }
}
