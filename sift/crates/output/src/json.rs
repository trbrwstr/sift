use logforge_core::aggregate::Aggregator;
use serde_json::json;

pub fn print_json(agg: &Aggregator) {
    let levels: serde_json::Value = agg
        .levels
        .iter()
        .map(|(k, v)| (k.clone(), *v as u64))
        .collect::<serde_json::Map<_, _>>()
        .into();

    let messages: serde_json::Value = agg
        .messages
        .iter()
        .take(20)
        .map(|(k, v)| (k.clone(), *v as u64))
        .collect::<serde_json::Map<_, _>>()
        .into();

    let output = json!({
        "total": agg.total,
        "levels": levels,
        "top_messages": messages
    });

    println!("{}", output);
}