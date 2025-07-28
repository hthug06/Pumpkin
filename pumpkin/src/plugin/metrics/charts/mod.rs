pub mod advanced_bar_chart;
pub mod advanced_pie;
pub mod drilldown_pie;
pub mod multi_line_chart;
pub mod simple_bar_chart;
pub mod simple_pie;
pub mod single_line_chart;

use serde_json::{Value, json};
// Trait because chart_data
pub trait CustomChart: Send + Sync {
    fn id(&self) -> &str {
        ""
    }
    fn get_requested_json(&self) -> Option<Value> {
        let data = self.get_chart_data();
        if data == None {
            return None;
        }
        Some(json!({
            "chartId": self.id().to_string(),
            "data": data
        }))
    }
    fn get_chart_data(&self) -> Option<Value>;
}

// need this because they do a class in a class
pub struct Chart {
    id: String,
}

impl Chart {
    pub fn new(chart_id: &str) -> Result<Self, String> {
        if chart_id.is_empty() {
            return Err("Chart id is required".to_string());
        }
        Ok(Chart {
            id: chart_id.to_string(),
        })
    }
}

//use this so the chart can get the id
impl CustomChart for Chart {
    fn get_chart_data(&self) -> Option<Value> {
        Some(json!("")) // Not used so we don't care
    }
}
