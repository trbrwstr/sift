use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Aggregator {
    pub total: usize,
    pub levels: HashMap<String, usize>,
    pub messages: HashMap<String, usize>,
}

impl Aggregator {
    pub fn process(&mut self, entry: &crate::types::LogEntry) {
        self.total += 1;

        if let Some(level) = &entry.level {
            if let Some(c) = self.levels.get_mut(level.as_str()) {
                *c += 1;
            } else {
                self.levels.insert(level.clone(), 1);
            }
        }

        if let Some(c) = self.messages.get_mut(entry.message.as_str()) {
            *c += 1;
        } else {
            self.messages.insert(entry.message.clone(), 1);
        }
    }

    pub fn merge(&mut self, other: Aggregator) {
        self.total += other.total;
        for (k, v) in other.levels {
            *self.levels.entry(k).or_insert(0) += v;
        }
        for (k, v) in other.messages {
            *self.messages.entry(k).or_insert(0) += v;
        }
    }

    pub fn top_messages(&self, n: usize) -> Vec<(&String, &usize)> {
        let mut v: Vec<_> = self.messages.iter().collect();
        let n = n.min(v.len());
        if n == 0 {
            return vec![];
        }
        // Partial sort: O(M) average to isolate top-n, then O(n log n) to sort them
        v.select_nth_unstable_by(n - 1, |(_, a), (_, b)| b.cmp(a));
        v[..n].sort_unstable_by(|(_, a), (_, b)| b.cmp(a));
        v.truncate(n);
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::LogEntry;

    fn entry(msg: &str, level: Option<&str>) -> LogEntry {
        LogEntry {
            timestamp: None,
            level: level.map(|s| s.to_string()),
            message: msg.to_string(),
            fields: vec![],
        }
    }

    #[test]
    fn process_increments_total() {
        let mut agg = Aggregator::default();
        agg.process(&entry("hello", None));
        assert_eq!(agg.total, 1);
    }

    #[test]
    fn process_counts_levels() {
        let mut agg = Aggregator::default();
        agg.process(&entry("a", Some("ERROR")));
        agg.process(&entry("b", Some("ERROR")));
        agg.process(&entry("c", Some("INFO")));
        assert_eq!(agg.levels["ERROR"], 2);
        assert_eq!(agg.levels["INFO"], 1);
    }

    #[test]
    fn merge_combines_totals() {
        let mut a = Aggregator::default();
        a.process(&entry("x", Some("ERROR")));

        let mut b = Aggregator::default();
        b.process(&entry("x", Some("INFO")));
        b.process(&entry("y", None));

        a.merge(b);
        assert_eq!(a.total, 3);
        assert_eq!(a.levels["ERROR"], 1);
        assert_eq!(a.levels["INFO"], 1);
    }

    #[test]
    fn top_messages_ordered_by_count() {
        let mut agg = Aggregator::default();
        for _ in 0..3 { agg.process(&entry("common", None)); }
        agg.process(&entry("rare", None));

        let top = agg.top_messages(2);
        assert_eq!(top[0].0, "common");
        assert_eq!(*top[0].1, 3);
        assert_eq!(top[1].0, "rare");
    }

    #[test]
    fn top_messages_n_larger_than_map() {
        let mut agg = Aggregator::default();
        agg.process(&entry("only", None));
        assert_eq!(agg.top_messages(10).len(), 1);
    }

    #[test]
    fn top_messages_empty() {
        let agg = Aggregator::default();
        assert!(agg.top_messages(5).is_empty());
    }
}
