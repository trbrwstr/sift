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
        return entry.fields.iter().any(|(fk, fv)| {
            fk == k && fv.parse::<i64>().ok().map_or(false, |n| n > v.parse().unwrap_or(0))
        });
    }

    false
}