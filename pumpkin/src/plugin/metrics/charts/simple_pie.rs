use crate::plugin::metrics::charts::{Chart, CustomChart};
use serde_json::{Value, json};

pub struct SimplePie {
    chart: Chart,
    callable: String,
}

impl SimplePie {
    pub fn new(chart_id: &str, callable: String) -> Self {
        SimplePie {
            chart: Chart::new(chart_id).unwrap(),
            callable,
        }
    }
}

impl CustomChart for SimplePie {
    async fn get_chart_data(&self) -> Option<Value> {
        let value = &self.callable;
        //pass if null
        if !value.is_empty() {
            return None;
        }
        Some(json!({
            "value": value
        }))
    }
}
