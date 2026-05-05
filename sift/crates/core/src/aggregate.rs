use crate::types::LogEntry;
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
            *self.levels.entry(level.clone()).or_insert(0) += 1;
        }

        *self.messages.entry(entry.message.clone()).or_insert(0) += 1;
    }

    pub fn top_messages(&self, n: usize) -> Vec<(&String, &usize)> {
        let mut v: Vec<_> = self.messages.iter().collect();
        v.sort_by_key(|(_, count)| *count);
        v.into_iter().rev().take(n).collect()
    }
}