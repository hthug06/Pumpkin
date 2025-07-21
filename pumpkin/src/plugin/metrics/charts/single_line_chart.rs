use crate::plugin::metrics::charts::{Chart, CustomChart};
use serde_json::{Value, json};

pub struct SingleLineChart {
    chart: Chart,
    callable: fn() -> Value,
}

impl SingleLineChart {
    pub fn new(chart_id: &str, callable: fn() -> Value) -> Self {
        SingleLineChart {
            chart: Chart::new(chart_id).unwrap(),
            callable,
        }
    }
}

impl CustomChart for SingleLineChart {
    fn get_chart_data(&self) -> Option<Value> {
        if (self.callable)() == 0 {
            return None;
        }
        //finally, return all the data
        Some(json!({
            "value": (self.callable)(),
        }))
    }
}
