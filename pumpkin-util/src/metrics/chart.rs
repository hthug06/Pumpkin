
// Trait because chart_data
pub trait CustomChart: Sized{
    async fn id(&self) -> &str;
    async fn chart(&self){}
}

// need this because they do a class in a class
pub struct Chart {
    id: String,
}

impl Chart {
    pub fn new(chart_id: &str) -> Self {
        if chart_id.is_empty() {
            panic!("ChartId cannot be null or empty!");
        }
        Chart {
            id: chart_id.to_string(),
        }
    }
}

//use this so the chart can get the id
impl CustomChart for Chart {
    async fn id(&self) -> &str {
        &self.id
    }
}

pub struct BarChart {
    chart: Chart,
}

impl BarChart {
    pub fn new(chart_id: &str) -> Self {
        BarChart {
            chart: Chart::new(chart_id),
        }
    }
}

impl CustomChart for BarChart {
    async fn id(&self) -> &str {self.chart.id.as_str()}
    
}

