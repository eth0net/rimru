use std::{cmp::Ordering, fmt::Display};

use crate::game::mods::ModMetaData;

/// Sort order for mods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Order::Id => write!(f, "ID"),
            Order::Name => write!(f, "Name"),
            Order::Created => write!(f, "Date Created"),
            Order::Modified => write!(f, "Date Modified"),
            Order::Dependencies => write!(f, "Dependencies"),
            Order::Topological => write!(f, "Topological"),
        }
    }
}

pub(crate) fn cmp_id(a: &ModMetaData, b: &ModMetaData) -> Ordering {
    match a.id.cmp(&b.id) {
        Ordering::Equal => {
            log::warn!(
                "Two mods have identical id '{}', but different names or paths: '{}' ({:?}) vs '{}' ({:?})",
                a.id,
                a.name,
                a.path,
                b.name,
                b.path
            );
            a.name.cmp(&b.name)
        }
        other => other,
    }
}

pub(crate) fn cmp_name(a: &ModMetaData, b: &ModMetaData) -> Ordering {
    match a.name.cmp(&b.name) {
        Ordering::Equal => cmp_id(a, b),
        other => other,
    }
}

pub(crate) fn cmp_created(a: &ModMetaData, b: &ModMetaData) -> Ordering {
    match a.created.cmp(&b.created) {
        Ordering::Equal => cmp_id(a, b),
        other => other,
    }
}

pub(crate) fn cmp_modified(a: &ModMetaData, b: &ModMetaData) -> Ordering {
    match a.modified.cmp(&b.modified) {
        Ordering::Equal => cmp_id(a, b),
        other => other,
    }
}

pub(crate) fn cmp_dependencies(a: &ModMetaData, b: &ModMetaData) -> Ordering {
    // Force load-after conflict
    let force_after_a = a.force_load_after(&b.id);
    let force_after_b = b.force_load_after(&a.id);
    if force_after_a && force_after_b {
        log::error!(
            "Force load-after conflict between {} and {}",
            a.name,
            b.name
        );
        return cmp_id(a, b);
    }

    // Force load-before conflict
    let force_before_a = a.force_load_before(&b.id);
    let force_before_b = b.force_load_before(&a.id);
    if force_before_a && force_before_b {
        log::error!(
            "Force load-before conflict between {} and {}",
            a.name,
            b.name
        );
        return cmp_id(a, b);
    }

    if force_after_a || force_before_b {
        return Ordering::Greater;
    }
    if force_after_b || force_before_a {
        return Ordering::Less;
    }

    // Regular load-after conflict
    let after_a = a.load_after(&b.id);
    let after_b = b.load_after(&a.id);
    if after_a && after_b {
        log::error!("Load-after conflict between {} and {}", a.name, b.name);
        return cmp_id(a, b);
    }

    // Regular load-before conflict
    let before_a = a.load_before(&b.id);
    let before_b = b.load_before(&a.id);
    if before_a && before_b {
        log::error!("Load-before conflict between {} and {}", a.name, b.name);
        return cmp_id(a, b);
    }

    if after_a || before_b {
        return Ordering::Greater;
    }
    if after_b || before_a {
        return Ordering::Less;
    }

    // Dependency relations
    let a_needs_b = a.depends_on(&b.id);
    let b_needs_a = b.depends_on(&a.id);

    if a_needs_b && b_needs_a {
        log::error!(
            "Circular dependency detected between {} and {}",
            a.name,
            b.name
        );
        cmp_id(a, b)
    } else if a_needs_b {
        Ordering::Greater
    } else if b_needs_a {
        Ordering::Less
    } else {
        cmp_id(a, b)
    }
}

pub(crate) fn cmp_topological(a: &ModMetaData, b: &ModMetaData) -> Ordering {
    match a.dependencies.len().cmp(&b.dependencies.len()) {
        Ordering::Equal => cmp_dependencies(a, b),
        order => order,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::{
        collections::BTreeMap,
        time::{Duration, SystemTime},
    };

    use crate::game::mods::ModDependency;

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

    fn make_mod(id: &str) -> ModMetaData {
        ModMetaData {
            id: id.into(),
            ..Default::default()
        }
    }

    #[test]
    fn test_dependency_ordering() {
        let c = make_mod("c");
        let b = ModMetaData {
            id: "b".into(),
            dependencies: BTreeMap::from([("c".into(), ModDependency::from(&c))]),
            ..Default::default()
        };
        let a = ModMetaData {
            id: "a".into(),
            dependencies: BTreeMap::from([("b".into(), ModDependency::from(&b))]),
            ..Default::default()
        };

        assert_eq!(
            cmp_dependencies(&a, &b),
            Ordering::Greater,
            "a > b: a depends on b"
        );
        assert_eq!(
            cmp_dependencies(&b, &c),
            Ordering::Greater,
            "b > c: b depends on c"
        );
        assert_eq!(
            cmp_dependencies(&a, &c),
            Ordering::Less,
            "a < c: a does not depend on c, but b does"
        );
        assert_eq!(
            cmp_dependencies(&b, &a),
            Ordering::Less,
            "b < a: b does not depend on a, but a depends on b"
        );
        assert_eq!(
            cmp_dependencies(&c, &a),
            Ordering::Greater,
            "c > a: neither depends on the other, fallback to name"
        );
        assert_eq!(
            cmp_dependencies(&c, &b),
            Ordering::Less,
            "c < b: c does not depend on b, but b depends on c"
        );
    }

    #[test]
    fn test_load_after_and_before() {
        let mut x = make_mod("x");
        let mut y = make_mod("y");
        x.load_after.insert("y".into());
        assert_eq!(
            cmp_dependencies(&x, &y),
            Ordering::Greater,
            "x > y: x.load_after(y)"
        );

        y.load_before.insert("x".into());
        assert_eq!(
            cmp_dependencies(&x, &y),
            Ordering::Greater,
            "x > y: y.load_before(x)"
        );

        let mut y2 = make_mod("y2");
        let x2 = make_mod("x2");
        y2.load_after.insert("x2".into());
        assert_eq!(
            cmp_dependencies(&x2, &y2),
            Ordering::Less,
            "x2 < y2: y2.load_after(x2)"
        );
    }

    #[test]
    fn test_force_load_after_and_before() {
        let mut m = make_mod("m");
        let mut n = make_mod("n");
        m.force_load_after.insert("n".into());
        assert_eq!(
            cmp_dependencies(&m, &n),
            Ordering::Greater,
            "m > n: m.force_load_after(n)"
        );

        n.force_load_before.insert("m".into());
        assert_eq!(
            cmp_dependencies(&m, &n),
            Ordering::Greater,
            "m > n: n.force_load_before(m)"
        );

        let mut n2 = make_mod("n2");
        let m2 = make_mod("m2");
        n2.force_load_after.insert("m2".into());
        assert_eq!(
            cmp_dependencies(&m2, &n2),
            Ordering::Less,
            "m2 < n2: n2.force_load_after(m2)"
        );
    }

    #[test]
    fn test_conflicting_load_relations() {
        // Conflict: both load_after each other
        let mut a = make_mod("a");
        let mut b = make_mod("b");
        a.load_after.insert("b".into());
        b.load_after.insert("a".into());
        assert_eq!(
            cmp_dependencies(&a, &b),
            cmp_name(&a, &b),
            "conflict: both a.load_after(b) and b.load_after(a), fallback to name"
        );

        // Conflict: both load_before each other
        let mut c = make_mod("c");
        let mut d = make_mod("d");
        c.load_before.insert("d".into());
        d.load_before.insert("c".into());
        assert_eq!(
            cmp_dependencies(&c, &d),
            cmp_name(&c, &d),
            "conflict: both c.load_before(d) and d.load_before(c), fallback to name"
        );

        // Conflict: load_after and load_before both ways
        let mut e = make_mod("e");
        let mut f = make_mod("f");
        e.load_after.insert("f".into());
        f.load_before.insert("e".into());
        f.load_after.insert("e".into());
        e.load_before.insert("f".into());
        assert_eq!(
            cmp_dependencies(&e, &f),
            cmp_name(&e, &f),
            "conflict: both e.load_after(f)/f.load_before(e) and f.load_after(e)/e.load_before(f), fallback to name"
        );

        // Conflict: both force_load_after each other
        let mut p = make_mod("p");
        let mut q = make_mod("q");
        p.force_load_after.insert("q".into());
        q.force_load_after.insert("p".into());
        assert_eq!(
            cmp_dependencies(&p, &q),
            cmp_name(&p, &q),
            "conflict: both p.force_load_after(q) and q.force_load_after(p), fallback to name"
        );

        // Conflict: both force_load_before each other
        let mut r = make_mod("r");
        let mut s = make_mod("s");
        r.force_load_before.insert("s".into());
        s.force_load_before.insert("r".into());
        assert_eq!(
            cmp_dependencies(&r, &s),
            cmp_name(&r, &s),
            "conflict: both r.force_load_before(s) and s.force_load_before(r), fallback to name"
        );

        // Conflict: force_load_after and force_load_before both ways
        let mut t = make_mod("t");
        let mut u = make_mod("u");
        t.force_load_after.insert("u".into());
        u.force_load_before.insert("t".into());
        u.force_load_after.insert("t".into());
        t.force_load_before.insert("u".into());
        assert_eq!(
            cmp_dependencies(&t, &u),
            cmp_name(&t, &u),
            "conflict: both t.force_load_after(u)/u.force_load_before(t) and u.force_load_after(t)/t.force_load_before(u), fallback to name"
        );
    }

    #[test]
    fn test_cmp_topological() {
        let d = ModMetaData {
            id: "d".to_string(),
            ..Default::default()
        };
        let c = ModMetaData {
            id: "c".to_string(),
            dependencies: BTreeMap::from([("d".into(), ModDependency::from(&d))]),
            ..Default::default()
        };
        let b = ModMetaData {
            id: "b".to_string(),
            dependencies: BTreeMap::from([
                ("c".into(), ModDependency::from(&c)),
                ("d".into(), ModDependency::from(&d)),
            ]),
            ..Default::default()
        };
        let a = ModMetaData {
            id: "a".into(),
            dependencies: BTreeMap::from([("b".into(), ModDependency::from(&b))]),
            ..Default::default()
        };

        assert_eq!(cmp_topological(&a, &a), Ordering::Equal, "a == a");
        assert_eq!(cmp_topological(&a, &b), Ordering::Less, "a < b");
        assert_eq!(cmp_topological(&a, &c), Ordering::Less, "a < c");
        assert_eq!(cmp_topological(&a, &d), Ordering::Greater, "a > d");
        assert_eq!(cmp_topological(&b, &a), Ordering::Greater, "b > a");
        assert_eq!(cmp_topological(&b, &b), Ordering::Equal, "b == b");
        assert_eq!(cmp_topological(&b, &c), Ordering::Greater, "b > c");
        assert_eq!(cmp_topological(&b, &d), Ordering::Greater, "b > d");
        assert_eq!(cmp_topological(&c, &a), Ordering::Greater, "c > a");
        assert_eq!(cmp_topological(&c, &b), Ordering::Less, "c < b");
        assert_eq!(cmp_topological(&c, &c), Ordering::Equal, "c == c");
        assert_eq!(cmp_topological(&c, &d), Ordering::Greater, "c > d");
        assert_eq!(cmp_topological(&d, &a), Ordering::Less, "d < a");
        assert_eq!(cmp_topological(&d, &b), Ordering::Less, "d < b");
        assert_eq!(cmp_topological(&d, &c), Ordering::Less, "d < c");
        assert_eq!(cmp_topological(&d, &d), Ordering::Equal, "d == d");
    }
}
