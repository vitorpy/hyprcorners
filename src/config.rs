use std::time::Instant;
use std::collections::HashMap;

use hyprland::dispatch::{Dispatch, DispatchType};
use hyprland::Result;
use serde::{Deserialize, Serialize};
use tokio::{fs, io::AsyncReadExt, io::AsyncWriteExt};

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    #[serde(default)]
    pub monitors: HashMap<String, MonitorConfig>,
    #[serde(default)]
    pub global: Option<MonitorConfig>,
    #[serde(default = "sticky_timeout_default")]
    pub sticky_timeout: Option<u64>,
    #[serde(default = "timeout_default")]
    pub timeout: u64,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct MonitorConfig {
    pub top_right: Option<Corner>,
    pub top_left: Option<Corner>,
    pub bottom_right: Option<Corner>,
    pub bottom_left: Option<Corner>,
}

fn sticky_timeout_default() -> Option<u64> {
    None
}

fn timeout_default() -> u64 {
    50
}

fn radius_default() -> i64 {
    10
}

fn dispatcher_default() -> String {
    "workspace".to_string()
}

fn arg_default() -> String {
    "".to_string()
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Corner {
    #[serde(default = "radius_default")]
    pub radius: i64,
    #[serde(default = "dispatcher_default")]
    pub dispatcher: String,
    #[serde(default = "arg_default")]
    pub args: String,
}

impl std::default::Default for MonitorConfig {
    fn default() -> Self {
        Self {
            top_right: None,
            top_left: None,
            bottom_right: Some(Corner {
                radius: 10,
                dispatcher: "workspace".to_string(),
                args: "e+1".to_string(),
            }),
            bottom_left: Some(Corner {
                radius: 10,
                dispatcher: "workspace".to_string(),
                args: "e-1".to_string(),
            }),
        }
    }
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            monitors: HashMap::new(),
            global: Some(MonitorConfig::default()),
            sticky_timeout: None,
            timeout: 50,
        }
    }
}

impl Config {
    pub async fn get() -> Self {
        let config_path = expanduser::expanduser("~/.config/hypr")
            .expect("failed to find hyprland config directory");

        let config_path = config_path.join("hyprcorners.toml");

        let mut config = String::new();
        let mut fd = match fs::File::open(&config_path).await {
            Ok(f) => f,
            Err(_) => {
                let content = toml::to_string(&Config::default()).unwrap();

                let mut fd = fs::File::create(&config_path)
                    .await
                    .expect("failed to create config file");

                fd.write_all(content.as_bytes())
                    .await
                    .expect("failed to write to config file");

                return Config::default();
            }
        };
        fd.read_to_string(&mut config)
            .await
            .expect("failed to read config");

        let config: Config = toml::from_str(&config).expect("error parsing config");

        config
    }

    pub fn get_monitor_config(&self, monitor_name: &str) -> Option<&MonitorConfig> {
        self.monitors.get(monitor_name).or(self.global.as_ref())
    }
}

impl Corner {
    pub async fn dispatch(&self, sticky: &bool, last_switch: &mut Option<Instant>) -> Result<()> {
        Dispatch::call_async(DispatchType::Custom(&self.dispatcher, &self.args)).await?;

        if *sticky {
            *last_switch = Some(Instant::now());
        }

        Ok(())
    }
}

