/*
Requires:
directories = "6.0.0"
sqlx = { version = "0.9.0-alpha.1", features = ["sqlite", "runtime-tokio"] }
tokio = { version = "1.50.0", features = ["rt-multi-thread", "macros"] }
clap = { version = "4.5.60", features = ["derive"] }
serde = { version = "1.0.228", features = ["derive"] }
toml = "1.0.6"
tracing = "0.1.44"
tracing-subscriber = "0.3.22"
tracing-appender = "0.2.4"
smart-default = "0.7.1"
chrono = { version = "0.4", features = ["serde"] }
thiserror = "2"
uuid = { version = "1", features = ["v4"] }
log = "0.4"
anyhow = "1"
ntex = "3"
serde_json = "1"
*/

use clap::Parser;
use directories::{BaseDirs, ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use smart_default::SmartDefault;
use std::{env, fs};
use std::str::FromStr;
use std::path::PathBuf;
use std::sync::OnceLock;
use tracing::Level;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Layer, Registry};
use tracing_subscriber::filter::LevelFilter;

pub const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

// ==================== DirectoryManager ====================
static PROJECT_PATHS: OnceLock<ProjectPath> = OnceLock::new();
pub type ProjectPath = ProjectPathV1;

#[derive(Debug)]
pub struct ProjectPathV1 {
    pub exe_dir: PathBuf,
    pub current_dir: PathBuf,
    pub proj_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub state_dir: Option<PathBuf>,
    pub preference_dir: PathBuf,
    pub home_dir: PathBuf,
    pub base_config_dir: PathBuf,
    pub base_data_dir: PathBuf,
    pub base_cache_dir: PathBuf,
    pub runtime_dir: Option<PathBuf>,
    pub desktop: Option<PathBuf>,
    pub document: Option<PathBuf>,
    pub download: Option<PathBuf>,
    pub audio: Option<PathBuf>,
    pub picture: Option<PathBuf>,
    pub video: Option<PathBuf>,
    pub public: Option<PathBuf>,
    pub font: Option<PathBuf>,
    pub template: Option<PathBuf>,
}

impl ProjectPathV1 {
    pub fn get() -> &'static ProjectPathV1 {
        PROJECT_PATHS.get_or_init(|| {
            let exe_path = env::current_exe().unwrap_or_default();
            let exe_dir = exe_path.parent().unwrap_or(&exe_path).to_path_buf();
            let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

            let proj = ProjectDirs::from("org", "oelabs", CARGO_PKG_NAME)
                .expect("Failed to get project directories");
            let base = BaseDirs::new().expect("Failed to get base directories");
            let user = UserDirs::new().expect("Failed to get user directories");

            let script_dir = base.home_dir().join("script");
            let _ = fs::create_dir_all(&script_dir);
            let _ = fs::create_dir_all(proj.config_dir());

            Self {
                exe_dir,
                current_dir,
                proj_dir: proj.data_dir().to_path_buf(),
                cache_dir: proj.cache_dir().to_path_buf(),
                state_dir: proj.state_dir().map(|p| p.to_path_buf()),
                preference_dir: proj.preference_dir().to_path_buf(),
                home_dir: base.home_dir().to_path_buf(),
                base_config_dir: base.config_dir().to_path_buf(),
                base_data_dir: base.data_dir().to_path_buf(),
                base_cache_dir: base.cache_dir().to_path_buf(),
                runtime_dir: base.runtime_dir().map(|p| p.to_path_buf()),
                desktop: user.desktop_dir().map(|p| p.to_path_buf()),
                document: user.document_dir().map(|p| p.to_path_buf()),
                download: user.download_dir().map(|p| p.to_path_buf()),
                audio: user.audio_dir().map(|p| p.to_path_buf()),
                picture: user.picture_dir().map(|p| p.to_path_buf()),
                video: user.video_dir().map(|p| p.to_path_buf()),
                public: user.public_dir().map(|p| p.to_path_buf()),
                font: user.font_dir().map(|p| p.to_path_buf()),
                template: user.template_dir().map(|p| p.to_path_buf()),
            }
        })
    }
}

// ==================== ConfigManager ====================
static CONFIG: OnceLock<Config> = OnceLock::new();
pub type Config = ConfigV1;

const CONFIG_FILENAME: &str = "config.toml";

fn get_config_path() -> PathBuf {
    let exe_dir = ProjectPath::get().exe_dir.clone();
    let exe_config = exe_dir.join(CONFIG_FILENAME);
    if exe_config.exists() {
        return exe_config;
    }

    let proj_dir = ProjectPath::get().proj_dir.clone();
    let proj_config = proj_dir.join(CONFIG_FILENAME);
    if proj_config.exists() {
        return proj_config;
    }

    proj_dir.join(CONFIG_FILENAME)
}

// ==================== ConfigV1 ====================

#[derive(Debug, Clone, Serialize, Deserialize, SmartDefault)]
pub struct ConfigV1 {
    #[default(CARGO_PKG_NAME.to_string())]
    pub app_name: String,
    #[default(CARGO_PKG_VERSION.to_string())]
    pub version: String,
    pub port_forwards: Vec<PortForward>,
    
    /// 本机主机名（仅 host 模式使用）
    /// 
    /// 在首次启动且配置文件不存在时，会通过键盘输入设置。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// 注册相关配置
    ///
    /// 为 Some 时自动注册到 VPS，为 None 时忽略
    pub registration: Option<std::net::IpAddr>,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortForward {
    // [Stable] 输入为被转发的ip地址和端口，输出为本机的端口（ip 为0.0.0.0）
    pub input: (std::net::IpAddr, u16),
    pub output: u16,
}

/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Ip {
    V4([u8; 4]),
    V6([u16; 8]),
}

impl Ip {
    pub fn from_str(ip: &str) -> Result<Self, String> {
        if ip.contains(':') {
            Self::parse_ipv6(ip)
        } else {
            Self::parse_ipv4(ip)
        }
    }

    fn parse_ipv4(ip: &str) -> Result<Self, String> {
        let parts: Vec<&str> = ip.split('.').collect();
        if parts.len() != 4 {
            return Err(format!("Invalid IPv4 address: {}", ip));
        }

        let bytes = [
            parts[0].parse::<u8>().map_err(|_| "Invalid IPv4 part")?,
            parts[1].parse::<u8>().map_err(|_| "Invalid IPv4 part")?,
            parts[2].parse::<u8>().map_err(|_| "Invalid IPv4 part")?,
            parts[3].parse::<u8>().map_err(|_| "Invalid IPv4 part")?,
        ];

        Ok(Ip::V4(bytes))
    }

    fn parse_ipv6(ip: &str) -> Result<Self, String> {
        // Handle IPv4-mapped IPv6 addresses like ::ffff:192.168.1.1
        if let Some(pos) = ip.rfind(':') {
            let possible_ipv4 = &ip[pos + 1..];
            if possible_ipv4.parse::<Ip>().is_ok() {
                // Convert IPv4 to last 2 segments of IPv6
                let ipv4 = possible_ipv4.parse::<Ip>().unwrap();
                if let Ip::V4(bytes) = ipv4 {
                    let mut segments = [0u16; 8];
                    segments[6] = ((bytes[0] as u16) << 8) | (bytes[1] as u16);
                    segments[7] = ((bytes[2] as u16) << 8) | (bytes[3] as u16);
                    
                    // prefix is before the last colon
                    let prefix = &ip[..pos];
                    if !prefix.is_empty() && prefix != "::" {
                        let prefix_parts: Vec<&str> = prefix.split(':').filter(|s| !s.is_empty()).collect();
                        if prefix_parts.len() > 6 {
                            return Err(format!("IPv6 with IPv4 suffix has too many segments: {}", ip));
                        }
                        for (i, part) in prefix_parts.iter().enumerate() {
                            segments[i] = u16::from_str_radix(part, 16)
                                .map_err(|_| format!("Invalid IPv6 segment: {}", part))?;
                        }
                    }
                    return Ok(Ip::V6(segments));
                }
            }
        }

        let mut segments = [0u16; 8];
        let mut seg_idx = 0;

        let mut parts: Vec<&str> = ip.split("::").collect();
        let (prefix, suffix) = if parts.len() == 2 {
            (parts[0], parts[1])
        } else if parts[0].is_empty() && parts[1].is_empty() {
            ("", "")
        } else if parts[0].is_empty() {
            ("", parts[1])
        } else if parts[1].is_empty() {
            (parts[0], "")
        } else {
            return Err(format!("Invalid IPv6 format: {}", ip));
        };

        if !prefix.is_empty() {
            let prefix_parts: Vec<&str> = prefix.split(':').filter(|s| !s.is_empty()).collect();
            for part in prefix_parts {
                if seg_idx >= 8 {
                    return Err(format!("IPv6 has too many segments: {}", ip));
                }
                segments[seg_idx] = u16::from_str_radix(part, 16)
                    .map_err(|_| format!("Invalid IPv6 segment: {}", part))?;
                seg_idx += 1;
            }
        }

        if !suffix.is_empty() {
            let suffix_parts: Vec<&str> = suffix.split(':').filter(|s| !s.is_empty()).collect();
            if seg_idx + suffix_parts.len() > 8 {
                return Err(format!("IPv6 has too many segments: {}", ip));
            }
            for part in suffix_parts {
                segments[seg_idx] = u16::from_str_radix(part, 16)
                    .map_err(|_| format!("Invalid IPv6 segment: {}", part))?;
                seg_idx += 1;
            }
        }

        // Fill remaining with zeros
        while seg_idx < 8 {
            seg_idx += 1;
        }

        Ok(Ip::V6(segments))
    }

    pub fn to_string(&self) -> String {
        match self {
            Ip::V4(bytes) => format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3]),
            Ip::V6(segments) => {
                let hex: Vec<String> = segments.iter().map(|s| format!("{:x}", s)).collect();
                hex.join(":")
            }
        }
    }
}

impl FromStr for Ip {
    type Err = String;

    fn from_str(ip: &str) -> Result<Self, Self::Err> {
        Ip::from_str(ip)
    }
}
*/

impl ConfigV1 {
    pub fn get() -> &'static Self {
        CONFIG.get_or_init(|| {
            let config_path = get_config_path();

            if config_path.exists() {
                if let Ok(config_str) = fs::read_to_string(&config_path) {
                    if let Ok(config) = toml::from_str::<Self>(&config_str) {
                        return config;
                    }
                }
            }

            let default_config = Self::default();
            let _ = fs::create_dir_all(ProjectPath::get().proj_dir.clone());
            let _ = fs::create_dir_all(ProjectPath::get().exe_dir.clone());

            let save_path = ProjectPath::get().proj_dir.join(CONFIG_FILENAME);
            if let Ok(toml_str) = toml::to_string_pretty(&default_config) {
                let _ = fs::write(&save_path, toml_str);
            }

            default_config
        })
    }

    /// 初始化配置（带主机名输入）
    /// 
    /// 与 get() 相同，但在配置文件不存在时会通过键盘输入主机名。
    /// 仅在 host 模式首次启动时调用。
    pub fn init_with_name_input() -> &'static Self {
        CONFIG.get_or_init(|| {
            let config_path = get_config_path();

            if config_path.exists() {
                if let Ok(config_str) = fs::read_to_string(&config_path) {
                    if let Ok(config) = toml::from_str::<Self>(&config_str) {
                        return config;
                    }
                }
            }

            let mut default_config = Self::default();
            
            // 首次启动，通过键盘输入主机名
            print!("请输入本机名称（host 模式）: ");
            std::io::Write::flush(&mut std::io::stdout()).ok();
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok();
            let name = input.trim().to_string();
            
            if !name.is_empty() {
                default_config.name = Some(name);
            }
            
            let _ = fs::create_dir_all(ProjectPath::get().proj_dir.clone());
            let _ = fs::create_dir_all(ProjectPath::get().exe_dir.clone());

            let save_path = ProjectPath::get().proj_dir.join(CONFIG_FILENAME);
            if let Ok(toml_str) = toml::to_string_pretty(&default_config) {
                let _ = fs::write(&save_path, toml_str);
            }

            default_config
        })
    }

    pub fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = ProjectPath::get().proj_dir.join(CONFIG_FILENAME);
        let toml_str = toml::to_string_pretty(self)?;
        fs::write(&config_path, toml_str)?;
        Ok(())
    }
}

// ==================== DatabaseManager ====================
static DATABASE: OnceLock<DatabaseManager> = OnceLock::new();

const DATABASE_FILE: &str = "database.db";

#[derive(Clone)]
pub struct DatabaseManager {
    pool: SqlitePool,
}

impl DatabaseManager {
    fn get_database_path() -> PathBuf {
        ProjectPath::get().proj_dir.join(DATABASE_FILE)
    }

    pub fn init() -> &'static Self {
        DATABASE.get_or_init(|| {
            let db_path = Self::get_database_path();

            let _ = fs::create_dir_all(ProjectPath::get().proj_dir.clone());

            let pool = tokio::runtime::Handle::current()
                .block_on(async {
                    SqlitePool::connect_with(
                        SqliteConnectOptions::new()
                            .filename(&db_path)
                            .create_if_missing(true),
                    )
                    .await
                    .expect("无法创建数据库连接池")
                });

            let manager = Self { pool };
            
            manager
        })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

// ==================== TracingManager ====================
static LOG: OnceLock<()> = OnceLock::new();
const LOG_DIR: &str = "logs";

/// 日志管理器
/// 
///负责初始化和配置 tracing 日志系统，功能包括：
/// 1. 创建日志目录 (`proj_dir/logs/`)
/// 2. 配置每日滚动的日志文件 (`app_YYYYMMDD.log`)
/// 3. 设置日志级别为 INFO
/// 4. 输出到 stdout (控制台) + 文件
/// 5. 保留最多 5 个历史日志文件
/// 
/// 使用示例：
/// ```
/// TracingManager::init();
/// tracing::info!("应用启动");
/// tracing::error!("发生错误");
/// ```
pub struct TracingManager;

impl TracingManager {
    /// 初始化日志系统
    /// 
    /// 此函数应在应用启动时调用一次，用于：
    /// - 创建日志目录 (proj_dir/logs)
    /// - 配置文件日志追加器（每日滚动）
    /// - 配置控制台输出（INFO 级别）
    /// - 配置文件日志（INFO 级别）
    /// 
    /// 日志文件格式: `app_YYYYYMMDD.log`
    pub fn init() {
        LOG.get_or_init(|| {
            let log_dir = ProjectPath::get().proj_dir.join(LOG_DIR);
            let _ = fs::create_dir_all(&log_dir);

            let file_appender = RollingFileAppender::builder()
                .rotation(Rotation::DAILY)
                .filename_prefix("app")
                .filename_suffix("log")
                .max_log_files(5)
                .build(log_dir)
                .expect("无法创建日志文件追加器");

            let subscriber = Registry::default()
                .with(
                    fmt::layer()
                        .with_writer(std::io::stdout)
                        .with_filter(LevelFilter::from_level(Level::INFO)),
                )
                .with(
                    fmt::layer()
                        .with_writer(file_appender)
                        .with_filter(LevelFilter::from_level(Level::INFO)),
                );

            subscriber.init();
        });
    }
}

// ==================== CLI ====================
static ARGS: OnceLock<CliArgs> = OnceLock::new();

/// 命令行参数
/// 
/// ## 字段说明
/// - `config`: 指定配置文件路径
/// - `verbose`: 启用详细输出
/// - `vps_port`: VPS 模式监听端口（当 Some 时为 VPS 模式，Host 模式为 None）
/// - `name`: 本机名称（host 模式首次启动时通过键盘输入，不通过 CLI）
/// 
/// ## 模式判断
/// - VPS 模式: `vps_port.is_some()`
/// - Host 模式: `vps_port.is_none()`
/// 
/// ## 示例
/// ```bash
/// # 运行 VPS 模式
/// atlas_proxy --vps-port 8080
/// 
/// # 运行 Host 模式
/// atlas_proxy
/// ```
#[derive(Debug, Parser, Default)]
#[command(name = CARGO_PKG_NAME)]
pub struct CliArgs {
    /// 配置文件路径
    #[arg(short, long)]
    pub config: Option<String>,

    /// 启用详细输出
    #[arg(short, long)]
    pub verbose: bool,

    /// VPS 模式监听端口
    /// 
    /// 当值为 Some 的时候为 VPS 模式，None 时为 Host 模式。
    /// 指定 VPS 服务器监听的端口号。
    #[arg(short, long)]
    pub vps_port: Option<u16>,
}

pub fn get_cli_args() -> &'static CliArgs {
    ARGS.get_or_init(CliArgs::parse)
}
