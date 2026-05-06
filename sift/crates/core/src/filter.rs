use crate::types::LogEntry;

pub fn matches(entry: &LogEntry, query: &Option<String>) -> bool {
    let Some(q) = query else { return true };

    // supports: field=value, field>N, field>=N, field<N, field<=N, AND
    // special keys: message (substring match), level (exact match)
    q.split(" AND ").all(|cond| evaluate(entry, cond.trim()))
}

fn evaluate(entry: &LogEntry, cond: &str) -> bool {
    // Check two-char operators before single-char to avoid splitting ">=500" on ">"
    if let Some((k, v)) = cond.split_once(">=") {
        return numeric_cmp(entry, k, v, |n, t| n >= t);
    }
    if let Some((k, v)) = cond.split_once("<=") {
        return numeric_cmp(entry, k, v, |n, t| n <= t);
    }
    if let Some((k, v)) = cond.split_once('>') {
        return numeric_cmp(entry, k, v, |n, t| n > t);
    }
    if let Some((k, v)) = cond.split_once('<') {
        return numeric_cmp(entry, k, v, |n, t| n < t);
    }
    if let Some((k, v)) = cond.split_once('=') {
        return string_eq(entry, k, v);
    }

    false
}

fn numeric_cmp(entry: &LogEntry, key: &str, raw: &str, op: impl Fn(i64, i64) -> bool) -> bool {
    let Ok(threshold) = raw.parse::<i64>() else { return false };
    entry.fields.iter().any(|(fk, fv)| {
        fk == key && fv.parse::<i64>().ok().map_or(false, |n| op(n, threshold))
    })
}

fn string_eq(entry: &LogEntry, key: &str, value: &str) -> bool {
    match key {
        "message" => entry.message.contains(value),
        "level"   => entry.level.as_deref().map_or(false, |l| l == value),
        _         => entry.fields.iter().any(|(fk, fv)| fk == key && fv == value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::LogEntry;

    fn entry(message: &str, level: Option<&str>, fields: Vec<(&str, &str)>) -> LogEntry {
        LogEntry {
            timestamp: None,
            level: level.map(str::to_string),
            message: message.to_string(),
            fields: fields.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        }
    }

    #[test]
    fn no_query_matches_all() {
        assert!(matches(&entry("hello", None, vec![]), &None));
    }

    #[test]
    fn message_substring() {
        let e = entry("request failed", None, vec![]);
        assert!(matches(&e, &Some("message=failed".into())));
        assert!(!matches(&e, &Some("message=success".into())));
    }

    #[test]
    fn level_exact_match() {
        let e = entry("msg", Some("ERROR"), vec![]);
        assert!(matches(&e, &Some("level=ERROR".into())));
        assert!(!matches(&e, &Some("level=INFO".into())));
    }

    #[test]
    fn field_equality() {
        let e = entry("req", None, vec![("status", "500")]);
        assert!(matches(&e, &Some("status=500".into())));
        assert!(!matches(&e, &Some("status=200".into())));
    }

    #[test]
    fn field_greater_than() {
        let e = entry("req", None, vec![("status", "500")]);
        assert!(matches(&e, &Some("status>400".into())));
        assert!(!matches(&e, &Some("status>500".into())));
    }

    #[test]
    fn field_greater_than_or_equal() {
        let e = entry("req", None, vec![("status", "500")]);
        assert!(matches(&e, &Some("status>=500".into())));
        assert!(!matches(&e, &Some("status>=501".into())));
    }

    #[test]
    fn field_less_than() {
        let e = entry("req", None, vec![("status", "200")]);
        assert!(matches(&e, &Some("status<300".into())));
        assert!(!matches(&e, &Some("status<200".into())));
    }

    #[test]
    fn field_less_than_or_equal() {
        let e = entry("req", None, vec![("status", "200")]);
        assert!(matches(&e, &Some("status<=200".into())));
        assert!(!matches(&e, &Some("status<=199".into())));
    }

    #[test]
    fn non_numeric_threshold_returns_false() {
        let e = entry("req", None, vec![("status", "200")]);
        assert!(!matches(&e, &Some("status>abc".into())));
    }

    #[test]
    fn and_condition() {
        let e = entry("error occurred", None, vec![("status", "500")]);
        assert!(matches(&e, &Some("status=500 AND message=error".into())));
        assert!(!matches(&e, &Some("status=500 AND message=success".into())));
    }

    #[test]
    fn unknown_operator_returns_false() {
        let e = entry("req", None, vec![]);
        assert!(!matches(&e, &Some("status~200".into())));
    }
}
