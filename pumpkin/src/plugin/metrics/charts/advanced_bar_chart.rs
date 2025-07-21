use crate::plugin::metrics::charts::{Chart, CustomChart};
use serde_json::Value::Array;
use serde_json::{Map, Value, json};

pub struct AdvancedBarChart {
    chart: Chart,
    callable: fn() -> Map<String, Value>,
}

impl AdvancedBarChart {
    pub fn new(chart_id: &str, callable: fn() -> Map<String, Value>) -> Self {
        AdvancedBarChart {
            chart: Chart::new(chart_id).unwrap(),
            callable,
        }
    }
}

impl CustomChart for AdvancedBarChart {
    fn get_chart_data(&self) -> Option<Value> {
        let mut values = Map::new();

        // If 0, return None because don't add value with 0 into the chart
        if Value::Object((self.callable)())
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
        for (k, v) in Value::Object((self.callable)()).as_object().unwrap() {
            //don't add value that are equal to 0
            if v.as_i64().iter().len() == 0 {
                continue;
            }
            all_skipped = false;

            //add all the categories to 1 key
            let mut category_values = Vec::new();
            for category_value in v.clone().as_array().unwrap() {
                category_values.push(category_value.clone())
            }
            values.insert(k.clone(), Array(category_values));
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
