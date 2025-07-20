use crate::plugin::metrics::charts::{Chart, CustomChart};
use serde_json::{Map, Value, json};

pub struct MultiLineChart {
    chart: Chart,
    callable: Map<String, Value>,
}

impl MultiLineChart {
    pub fn new(chart_id: &str, callable: Map<String, Value>) -> Self {
        MultiLineChart {
            chart: Chart::new(chart_id).unwrap(),
            callable,
        }
    }
}

impl CustomChart for MultiLineChart {
    async fn get_chart_data(&self) -> Option<Value> {
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

        //for future test
        let mut all_skipped = true;

        //add all to values
        for (k, v) in Value::Object(self.callable.clone()).as_object().unwrap() {
            if v == 0 {
                continue;
            }
            all_skipped = false;
            values.insert(k.clone(), v.clone());
        }

        //if nothing, return None
        if all_skipped {
            return None;
        }

        //finally, return all the data
        Some(json!({
            "values": values,
        }))
    }
}
