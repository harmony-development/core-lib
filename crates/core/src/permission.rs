//! Types and functions for working with permissions.
use std::cmp::Ordering;

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
pub fn has_permission<'a, Perm, I>(perms: I, query: &str) -> Option<bool>
where
    Perm: std::borrow::Borrow<(&'a str, bool)>,
    I: Iterator<Item = Perm>,
{
    let mut matching_perms = perms
        .filter(|p| {
            let (matches, _) = p.borrow();
            matches
                .split('.')
                .zip(query.split('.'))
                .all(|(m, c)| m == "*" || c == m)
        })
        .collect::<Vec<_>>();

    matching_perms.sort_unstable_by(|p, op| {
        let (m, _) = p.borrow();
        let (om, _) = op.borrow();
        compare_permission_depth(m, om)
    });

    matching_perms.pop().map(|p| p.borrow().1)
}

#[cfg(test)]
mod tests {
    use super::has_permission;

    #[test]
    fn test_perm_compare_equal_allow() {
        let ok = has_permission([("messages.send", true)].iter(), "messages.send");
        assert_eq!(ok, Some(true));
    }

    #[test]
    fn test_perm_compare_equal_deny() {
        let ok = has_permission(
            std::array::IntoIter::new([("messages.send", false)]),
            "messages.send",
        );
        assert_eq!(ok, Some(false));
    }

    #[test]
    fn test_perm_compare_nonequal_allow() {
        let ok = has_permission([("messages.sendd", true)].iter(), "messages.send");
        assert_eq!(ok, None);
    }

    #[test]
    fn test_perm_compare_nonequal_deny() {
        let ok = has_permission([("messages.sendd", false)].iter(), "messages.send");
        assert_eq!(ok, None);
    }

    #[test]
    fn test_perm_compare_glob_allow() {
        let perms = [("messages.*", true)];
        let ok = has_permission(perms.iter(), "messages.send");
        assert_eq!(ok, Some(true));
        let ok = has_permission(perms.iter(), "messages.view");
        assert_eq!(ok, Some(true));
    }

    #[test]
    fn test_perm_compare_glob_deny() {
        let perms = [("messages.*", false)];
        let ok = has_permission(perms.iter(), "messages.send");
        assert_eq!(ok, Some(false));
        let ok = has_permission(perms.iter(), "messages.view");
        assert_eq!(ok, Some(false));
    }

    #[test]
    fn test_perm_compare_specific_deny() {
        let perms = [("messages.*", true), ("messages.send", false)];
        let ok = has_permission(perms.iter(), "messages.send");
        assert_eq!(ok, Some(false));
    }

    #[test]
    fn test_perm_compare_specific_allow() {
        let perms = [("messages.*", false), ("messages.send", true)];
        let ok = has_permission(perms.iter(), "messages.send");
        assert_eq!(ok, Some(true));
    }

    #[test]
    fn test_perm_compare_depth_allow() {
        let perms = [
            ("messages.*", false),
            ("messages.send", false),
            ("messages.send.send", true),
        ];
        let ok = has_permission(perms.iter(), "messages.send.send");
        assert_eq!(ok, Some(true));
    }

    #[test]
    fn test_perm_compare_depth_deny() {
        let perms = [
            ("messages.*", true),
            ("messages.send", true),
            ("messages.send.send", false),
        ];
        let ok = has_permission(perms.iter(), "messages.send.send");
        assert_eq!(ok, Some(false));
    }
}
