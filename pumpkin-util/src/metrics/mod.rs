mod chart;

use os_info;
use scheduling::Scheduler;
use serde_json::{json, Map};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{ErrorKind, Write};
use std::sync::{Arc, Mutex};
use log::{logger, Log};
use reqwest::Error;
use crate::metrics::chart::{Chart, CustomChart};

pub struct Metrics {
    b_stats_version: u8,
    url: String,
    log_failed_request: bool,
    name: String, // The name of the server software
    uuid: String, // The uuid of the server
    //charts: [dyn CustomChart; 6] TODO only used for plugin
}

impl Metrics {
    async fn new(name: String, uuid: String, log_failed_request: bool) -> Self {
        Metrics {
            b_stats_version: 1,
            url: "https://bstats.org/submitData/server-implementation".to_string(),
            log_failed_request,
            name,
            uuid,
            //charts: vec![],
        }
    }

    //Starts the Scheduler which submits our data every 30 minutes.
    async fn start_submitting(self: Arc<Mutex<Self>>) {
        let initial_delay: u64 = 1000 * 60 * (3 + rand::random::<u64>() * 3);
        let second_delay: u64 = 1000 * 60 * (3 + rand::random::<u64>() * 30);

        Scheduler::delayed_once(std::time::Duration::from_millis(initial_delay), || {
            self.lock().unwrap().submit_data().await;
        }).start();

        let submit_task = Scheduler::delayed_recurring(
            std::time::Duration::from_millis(initial_delay + second_delay),
            std::time::Duration::from_millis(1000 * 60 * 30),
            || println!("1 SEC ELAPSED"),
        );
    }

    //Gets the plugin specific data.
    async fn plugin_data(&self) -> serde_json::Value {
        //TODO add custom charts for plugin
        json!({
            "customCharts": []
        })
    }

    //Gets the server specific data.
    async fn server_data(&self) -> Option<serde_json::Value>{
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
    async fn send_data(&self, data:  Option<serde_json::Value>) -> Result<(), Error>{
        let client = reqwest::Client::new();

        // Compress the data to save bandwidth
        let compressed_data = Self::compress(data.ok_or("Data can't be null for bstats").unwrap().to_string()).await;

        // Add headers and send data
        let request = client.post(self.url.as_str())
            .json(&json!({
            "Accept": "application/json",
            "Connection": "close",
            "Content-Encoding": "gzip",
            "Content-Length": compressed_data.len(),
            "Content-Type": "application/json",
            "User-Agent": "MC-Server/".to_owned() + self.b_stats_version.to_string().as_str()
        })).send().await?;

        Ok(())

    }

    //Gzips the given String.
    async fn compress(str: String) -> Vec<u64>{
        if str == "" {
            return vec![];
        }

        let output_stream:Vec<u64> = Vec::new();
        let mut gzip = GzEncoder::new(Vec::new(), Compression::default());
        gzip.write(str.as_bytes()).unwrap();
        gzip.try_finish().unwrap();
        output_stream
    }
}