use crate::plugin::metrics::charts::{Chart, CustomChart};
use serde_json::{Number, Value, json};

pub struct SingleLineChart {
    chart: Chart,
    callable: Value,
}

impl SingleLineChart {
    pub fn new(chart_id: &str, callable: Value) -> Self {
        SingleLineChart {
            chart: Chart::new(chart_id).unwrap(),
            callable,
        }
    }
}

impl CustomChart for SingleLineChart {
    async fn get_chart_data(&self) -> Option<Value> {
        if self.callable == Value::Number(Number::from(0)) {
            return None;
        }
        //finally, return all the data
        Some(json!({
            "value": self.callable.to_string(),
        }))
    }
}
