use crate::plugin::metrics::charts::{Chart, CustomChart};
use serde_json::{Map, Value, json};

pub struct AdvancedPie {
    chart: Chart,
    callable: Map<String, Value>,
}

impl AdvancedPie {
    pub fn new(chart_id: &str, callable: Map<String, Value>) -> Self {
        AdvancedPie {
            chart: Chart::new(chart_id).unwrap(),
            callable,
        }
    }
}

impl CustomChart for AdvancedPie {
    async fn get_chart_data(&self) -> Option<Value> {
        //create a Map for all the value inside the chart
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

        //add all the value inside the map
        let mut all_skipped = true;
        for (k, v) in Value::Object(self.callable.clone()).as_object().unwrap() {
            if v == 0 {
                continue;
            }

            all_skipped = false;
            values.insert(k.clone(), v.clone());
        }

        //if no data is added, return None because there are no data
        if all_skipped {
            return None;
        }

        //finally, return all the data
        Some(json!({
            "values":  values,
        }))
    }
}
