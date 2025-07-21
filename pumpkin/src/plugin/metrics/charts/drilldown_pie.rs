use crate::plugin::metrics::charts::{Chart, CustomChart};
use serde_json::{Map, Value, json};

pub struct DrilldownPie {
    chart: Chart,
    callable: fn() -> Map<String, Value>,
}

impl DrilldownPie {
    pub fn new(chart_id: &str, callable: fn() -> Map<String, Value>) -> Self {
        DrilldownPie {
            chart: Chart::new(chart_id).unwrap(),
            callable,
        }
    }
}

impl CustomChart for DrilldownPie {
    fn get_chart_data(&self) -> Option<Value> {
        //create a Map for all the value inside the chart
        let mut values: Map<String, Value> = Map::new();

        // If 0, return None because don't add value with 0 into the chart
        if Value::Object((self.callable)()).as_object().unwrap().len() == 0 {
            return None;
        }

        //for later checking
        let mut really_all_skipped = true;

        //check everything
        for (k1, v1) in Value::Object((self.callable)()).as_object().unwrap() {
            //create a map for the value insoed of v1
            let mut value = Map::new();
            //for later checking
            let mut all_skipped = true;

            //check what's inside v1 and insert everything in value
            for (k2, v2) in v1.as_object().unwrap() {
                value.insert(k2.clone(), v2.clone());
                all_skipped = false;
            }

            //if there were value in v1, insert it in values (with a 's')
            if !all_skipped {
                really_all_skipped = false;
                values.insert(k1.clone(), Value::Object(value));
            }
        }
        //if no data is added, return None because there are no data
        if really_all_skipped {
            return None;
        }

        //finally, return all the data
        Some(json!({
            "values":  values,
        }))
    }
}
