//! Types and functions for working with permissions.
use core::cmp::Ordering;

/// Compares two permission `matches` and calculates which one takes priority over other.
pub fn compare_permission_depth(perm: &str, other_perm: &str) -> Ordering {
    let get_depth = |matches: &str| matches.chars().filter(|c| '.'.eq(c)).count();
    let ord = get_depth(perm).cmp(&get_depth(other_perm));

    if let Ordering::Equal = ord {
        let p_split = perm.split('.');
        let op_split = other_perm.split('.');
        match (p_split.last(), op_split.last()) {
            (Some(p_last), Some(op_last)) => match (p_last, op_last) {
                ("*", _) => Ordering::Less,
                (_, "*") => Ordering::Greater,
                _ => Ordering::Equal,
            },
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    } else {
        ord
    }
}

/// Checks if a permission is allowed in some permission collection.
///
/// Returns `None` if no permissions were matched.
pub fn has_permission<Matches, I>(perms: I, query: &str) -> Option<bool>
where
    Matches: AsRef<str>,
    I: Iterator<Item = (Matches, bool)>,
{
    let mut matching_perms = perms.filter(|(matches, _)| {
        matches
            .as_ref()
            .split('.')
            .zip(query.split('.'))
            .all(|(m, c)| m == "*" || c == m)
    });

    let mut matched = matching_perms.next();

    for perm in matching_perms {
        let ordering =
            compare_permission_depth(perm.0.as_ref(), matched.as_ref().unwrap().0.as_ref());
        if let Ordering::Greater = ordering {
            matched = Some(perm);
        }
    }

    matched.map(|(_, ok)| ok)
}

#[cfg(test)]
mod tests {
    use super::has_permission;

    use core::array::IntoIter;

    #[test]
    fn test_perm_compare_equal_allow() {
        let ok = has_permission(IntoIter::new([("messages.send", true)]), "messages.send");
        assert_eq!(ok, Some(true));
    }

    #[test]
    fn test_perm_compare_equal_deny() {
        let ok = has_permission(IntoIter::new([("messages.send", false)]), "messages.send");
        assert_eq!(ok, Some(false));
    }

    #[test]
    fn test_perm_compare_nonequal_allow() {
        let ok = has_permission(IntoIter::new([("messages.sendd", true)]), "messages.send");
        assert_eq!(ok, None);
    }

    #[test]
    fn test_perm_compare_nonequal_deny() {
        let ok = has_permission(IntoIter::new([("messages.sendd", false)]), "messages.send");
        assert_eq!(ok, None);
    }

    #[test]
    fn test_perm_compare_glob_allow() {
        let perms = IntoIter::new([("messages.*", true)]);
        let ok = has_permission(perms.clone(), "messages.send");
        assert_eq!(ok, Some(true));
        let ok = has_permission(perms.clone(), "messages.view");
        assert_eq!(ok, Some(true));
    }

    #[test]
    fn test_perm_compare_glob_deny() {
        let perms = IntoIter::new([("messages.*", false)]);
        let ok = has_permission(perms.clone(), "messages.send");
        assert_eq!(ok, Some(false));
        let ok = has_permission(perms.clone(), "messages.view");
        assert_eq!(ok, Some(false));
    }

    #[test]
    fn test_perm_compare_specific_deny() {
        let perms = IntoIter::new([("messages.*", true), ("messages.send", false)]);
        let ok = has_permission(perms, "messages.send");
        assert_eq!(ok, Some(false));
    }

    #[test]
    fn test_perm_compare_specific_allow() {
        let perms = IntoIter::new([("messages.*", false), ("messages.send", true)]);
        let ok = has_permission(perms, "messages.send");
        assert_eq!(ok, Some(true));
    }

    #[test]
    fn test_perm_compare_depth_allow() {
        let perms = IntoIter::new([
            ("messages.*", false),
            ("messages.send", false),
            ("messages.send.send", true),
        ]);
        let ok = has_permission(perms, "messages.send.send");
        assert_eq!(ok, Some(true));
    }

    #[test]
    fn test_perm_compare_depth_deny() {
        let perms = IntoIter::new([
            ("messages.*", true),
            ("messages.send.send", false),
            ("messages.send", true),
        ]);
        let ok = has_permission(perms, "messages.send.send");
        assert_eq!(ok, Some(false));
    }
}
