use crate::plugin::metrics::charts::{Chart, CustomChart};
use serde_json::{Map, Value, json};

pub struct SimpleBarChart {
    chart: Chart,
    callable: Map<String, Value>,
}

impl SimpleBarChart {
    pub fn new(chart_id: &str, callable: Map<String, Value>) -> Self {
        SimpleBarChart {
            chart: Chart::new(chart_id).unwrap(),
            callable,
        }
    }
}

impl CustomChart for SimpleBarChart {
    fn get_chart_data(&self) -> Option<Value> {
        let mut values = Map::new();

        // If 0, return None because don't add value with 0 into the chart
        if Value::Object(self.callable.clone())
            .as_object()
            .unwrap()
            .len()
            == 0
        {
            return None;
        }

        //add all to values
        for (k, v) in Value::Object(self.callable.clone()).as_object().unwrap() {
            values.insert(k.clone(), Value::Array(vec![v.clone()]));
        }

        //finally, return all the data
        Some(json!({
            "values": values,
        }))
    }
}
