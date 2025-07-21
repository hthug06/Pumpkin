mod charts;

use crate::plugin::metrics::charts::CustomChart;
use crate::plugin::metrics::charts::simple_pie::SimplePie;
use crate::server::{CURRENT_MC_VERSION, Server};
use crate::SHOULD_STOP;
use flate2::Compression;
use flate2::write::GzEncoder;
use os_info;
use reqwest::Error;
use serde_json::{Map, json, Value, Number};
use std::io::Write;
use std::process::Command;
use std::sync::atomic::Ordering;
use std::time::Duration;
use rand::Rng;
use tokio::time::interval;
use uuid::Uuid;
use pumpkin_config::BASIC_CONFIG;
use crate::plugin::metrics::charts::single_line_chart::SingleLineChart;

//Trying to replic Metrics.java from Paper
pub struct Metrics<'a> {
    server: &'a Server,
    b_stats_version: u8,
    url: String,
    log_failed_request: bool,
    name: String,                      // The name of the server software
    uuid: String,                      // The uuid of the server
    charts: Vec<Box<dyn CustomChart>>, //All the created charts
}

impl<'a> Metrics<'a>{
    async fn new(name: String, uuid: String, log_failed_request: bool, server: &'a Server) -> Self {
        let m = Metrics {
            server,
            b_stats_version: 1,
            url: "https://bstats.org/submitData/server-implementation".to_string(),
            log_failed_request,
            name,
            uuid,
            charts: Vec::new(),
        };

        m.start_submitting().await;

        m
    }

    async fn add_custom_chart(&mut self, chart: Box<dyn CustomChart>) {
        self.charts.push(chart);
    }

    //Starts the Scheduler which submits our data every 30 minutes.
    async fn start_submitting(&self) {

        log::info!("metrics : start submitting");

        let initial_delay = 1000.0 * 60.0 * (3.0 + rand::rng().random_range(0.0..1.0) * 3.0);
        log::info!("metrics : initial delay: {}", initial_delay);
        let second_delay = 1000.0 * 60.0 * (3.0 + rand::rng().random_range(0.0..1.0) * 30.0);
        log::info!("metrics : second_delay: {}", second_delay);

        // Wait for a short duration
        tokio::time::sleep(Duration::from_millis(initial_delay as u64)).await;
        self.submit_data().await;
        tokio::time::sleep(Duration::from_millis(second_delay as u64)).await;

        let mut interval = interval(Duration::from_millis(1000 * 60 * 30));

        loop {
            interval.tick().await;
            if !SHOULD_STOP.load(Ordering::Relaxed) {
                self.submit_data().await;
            }
        }
    }

    //Gets the plugin specific data.
    async fn plugin_data(&self) -> serde_json::Value {
        //TODO add custom charts for plugin
        json!({
            "customCharts": []
        })
    }

    //Gets the server specific data.
    async fn server_data(&self) -> Option<serde_json::Value> {
        let info = os_info::get();

        let os_name = info.os_type();
        let os_arch = info.architecture().unwrap();
        let os_version = info.version();
        let core_count = num_cpus::get();

        Some(json!({
            "serverUUID": self.uuid,
            "osName": os_name,
            "osArch": os_arch,
            "osVersion": os_version,
            "coreCount": core_count,
        }))
    }

    // Collects the data and sends it afterwards.
    async fn submit_data(&self) {
        log::info!("metrics : submit_data");
        let server_data = self.server_data().await;
        let plugin_data = self.plugin_data().await;

        //create a map with everything inside
        let mut data = Map::new();
        for (k, v) in server_data.unwrap().as_object().unwrap() {
            data.insert(k.clone(), v.clone());
        }
        for (k, v) in plugin_data.as_object().unwrap() {
            data.insert(k.clone(), v.clone());
        }

        //recreate a json with the map
        let json_data = json!(data);
        match self.send_data(Some(json_data)).await {
            Err(e) => eprintln!("{}", e),
            _ => (),
        }
    }

    // Send the data to bStats server
    async fn send_data(&self, data: Option<serde_json::Value>) -> Result<(), Error> {
        log::info!("metrics : send_data");
        let client = reqwest::Client::new();

        // Compress the data to save bandwidth
        let compressed_data = Self::compress(
            data.ok_or("Data can't be null for bstats")
                .unwrap()
                .to_string(),
        )
        .await;

        // Add headers and send data
        let _request = client
            .post(self.url.as_str())
            .json(&json!({
                "Accept": "application/json",
                "Connection": "close",
                "Content-Encoding": "gzip",
                "Content-Length": compressed_data.len(),
                "Content-Type": "application/json",
                "User-Agent": "MC-Server/".to_owned() + self.b_stats_version.to_string().as_str()
            }))
            .send()
            .await?;

        log::info!("metrics : data_send??");

        Ok(())
    }

    //Gzips the given String.
    async fn compress(str: String) -> Vec<u64> {
        log::info!("metrics : start compressing");
        if str == "" {
            return vec![];
        }

        let output_stream: Vec<u64> = Vec::new();
        let mut gzip = GzEncoder::new(Vec::new(), Compression::default());
        gzip.write(str.as_bytes()).unwrap();
        gzip.try_finish().unwrap();
        log::info!("metrics : compressing done");
        output_stream
    }
}

pub struct PumpkinMetrics;

impl PumpkinMetrics {
    pub async fn start_metrics(server: &Server) {
        log::info!("Starting metrics server");
        //TODO Create the config file
        let uuid = Uuid::new_v4();

        let mut metrics =
            Metrics::new("Pumpkin".to_string(), uuid.to_string(), false, server).await;

        log::info!("metrics::new passed");
        metrics
            .add_custom_chart(Box::new(SimplePie::new("minecraft_version", || {
                CURRENT_MC_VERSION.to_string()
            }))).await;

        log::info!("metrics : minecraft_version passed");

        metrics
            .add_custom_chart(Box::new(SingleLineChart::new("players",  || {
                //Value::Number(Number::from(metrics.server.get_player_count())) Need Future it's to hard for me aaaaaaa :(
                Value::Number(Number::from(1))
            })))
            .await;

        log::info!("metrics : players passed");

        metrics
            .add_custom_chart(Box::new(SimplePie::new("online_mode", || {
                BASIC_CONFIG.online_mode.to_string()
            })))
            .await;

        log::info!("metrics : online_mode passed");

        metrics
            .add_custom_chart(Box::new(SimplePie::new("pumpkin_version", || {
                env!("CARGO_PKG_VERSION").to_string()
            })))
            .await;

        log::info!("metrics : pumpkin_version passed");

        metrics
            .add_custom_chart(Box::new(SimplePie::new("rust_version", || {
                let output = Command::new("cargo")
                    .arg("--version")
                    .output()
                    .expect("bstast: Failed to execute command to get cargo version");

                return if output.status.success() {
                    str::from_utf8(&output.stdout)
                        .expect(
                            "bstats: the output of the command to get cargo version id not UTF-8",
                        )
                        .trim()
                        .to_string()
                } else {
                    String::from("Unknow")
                }
            })))
            .await;

        log::info!("metrics : rust_version passed");

    }
}
