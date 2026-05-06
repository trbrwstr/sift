use crate::types::LogEntry;

pub fn matches(entry: &LogEntry, query: &Option<String>) -> bool {
    let Some(q) = query else { return true };

    // supports:
    // status=500
    // status>400 AND message=error

    q.split(" AND ").all(|cond| evaluate(entry, cond))
}

fn evaluate(entry: &LogEntry, cond: &str) -> bool {
    if let Some((k, v)) = cond.split_once('=') {
        return match k {
            "message" => entry.message.contains(v),
            _ => entry.fields.iter().any(|(fk, fv)| fk == k && fv == v),
        };
    }

    if let Some((k, v)) = cond.split_once('>') {
        let Ok(threshold) = v.parse::<i64>() else { return false };
        return entry.fields.iter().any(|(fk, fv)| {
            fk == k && fv.parse::<i64>().ok().map_or(false, |n| n > threshold)
        });
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::LogEntry;

    fn entry(message: &str, fields: Vec<(&str, &str)>) -> LogEntry {
        LogEntry {
            timestamp: None,
            level: None,
            message: message.to_string(),
            fields: fields.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        }
    }

    #[test]
    fn no_query_matches_all() {
        assert!(matches(&entry("hello", vec![]), &None));
    }

    #[test]
    fn message_equality() {
        let e = entry("request failed", vec![]);
        assert!(matches(&e, &Some("message=failed".into())));
        assert!(!matches(&e, &Some("message=success".into())));
    }

    #[test]
    fn field_equality() {
        let e = entry("req", vec![("status", "500")]);
        assert!(matches(&e, &Some("status=500".into())));
        assert!(!matches(&e, &Some("status=200".into())));
    }

    #[test]
    fn field_greater_than() {
        let e = entry("req", vec![("status", "500")]);
        assert!(matches(&e, &Some("status>400".into())));
        assert!(!matches(&e, &Some("status>500".into())));
    }

    #[test]
    fn non_numeric_threshold_returns_false() {
        let e = entry("req", vec![("status", "200")]);
        assert!(!matches(&e, &Some("status>abc".into())));
    }

    #[test]
    fn and_condition() {
        let e = entry("error occurred", vec![("status", "500")]);
        assert!(matches(&e, &Some("status=500 AND message=error".into())));
        assert!(!matches(&e, &Some("status=500 AND message=success".into())));
    }

    #[test]
    fn unknown_operator_returns_false() {
        let e = entry("req", vec![]);
        assert!(!matches(&e, &Some("status<200".into())));
    }
}