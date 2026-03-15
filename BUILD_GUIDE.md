# Atlas Proxy 构建指南

## 项目结构

```
atlas_proxy/
├── Cargo.toml                    # 项目依赖配置
├── ARCHITECTURE.md              # 架构设计文档
├── NTEX_DIOXUS_API_REFERENCE.md # ntex 3.x 和 Dioxus 0.7 API 参考
├── src/
│   ├── main.rs                  # 应用入口
│   ├── config/                  # 配置管理
│   │   ├── mod.rs
│   │   └── settings.rs
│   ├── models/                  # 数据模型
│   │   ├── mod.rs
│   │   ├── host.rs
│   │   ├── ip_address.rs
│   │   └── port_forward.rs
│   ├── store/                   # 数据存储
│   │   ├── mod.rs
│   │   └── persistence.rs
│   ├── vps/                     # VPS 模式模块
│   │   ├── mod.rs
│   │   ├── server.rs
│   │   └── handler.rs
│   ├── host/                    # 主机模式模块
│   │   ├── mod.rs
│   │   ├── client.rs
│   │   ├── registration.rs
│   │   ├── tunnel.rs
│   │   ├── port_mapping.rs
│   │   └── config_export.rs
│   └── web/                     # Web 管理端
│       ├── mod.rs
│       ├── routes.rs
│       └── templates.rs
└── target/                      # 编译输出（自动生成）
```

## 依赖版本

| 依赖 | 版本 | 说明 |
|------|------|------|
| ntex | 3.x | Web 框架 |
| tokio | 1.x | 异步运行时 |
| serde | 1.x | 序列化 |
| serde_json | 1.x | JSON 序列化 |
| toml | 1.x | TOML 配置 |
| rusqlite | 0.38.x | SQLite 数据库 |
| clap | 4.x | 命令行解析 |
| dioxus | 0.7.x | Web UI |
| dioxus-ssr | 0.7.x | SSR 渲染 |
| anyhow | 1.x | 错误处理 |
| thiserror | 2.x | 错误定义 |
| log | 0.4.x | 日志 |
| env_logger | 0.11.x | 日志初始化 |
| uuid | 1.x | UUID 生成 |
| chrono | 0.4.x | 时间处理 |
| ipnetwork | 0.21.x | IP 网络处理 |
| config | 0.15.x | 配置管理 |
| directories | 6.x | 目录处理 |

## 构建命令

### 开发构建

```bash
cargo build
```

### 优化构建

```bash
cargo build --release
```

### 运行

```bash
# 主机模式（默认）
cargo run

# VPS 模式
cargo run -- --vps

# 指定端口
cargo run -- --port 8080

# 指定名称
cargo run -- --name my-host
```

### 文档生成

```bash
# 生成 Rust 文档
cargo doc --open --no-deps

# 生成特定 crate 的文档
cargo doc --document-private-items --open --no-deps --package ntex
```

### 测试

```bash
cargo test
```

## 配置文件

### config.toml（主机模式）

```toml
name = "my-host"
vps_url = "http://vps.example.com:20000"

[listening]
port = 20000
bind_address = "0.0.0.0"

[registration]
ip_version = ["ipv4", "ipv6"]
interval = 30

[[port_forwards]]
name = "web-server"
listen_ip = "0.0.0.0"
listen_port = 80
forward_ip = "127.0.0.1"
forward_port = 8080
protocol = "tcp"

[logging]
level = "info"
file = "./logs/proxy.log"
```

## 数据库

### 主机模式数据库 (host.db)

```sql
-- 主机配置表
CREATE TABLE host_config (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    vps_url TEXT NOT NULL,
    port INTEGER NOT NULL,
    last_ip_v4 TEXT,
    last_ip_v6 TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 注册的 IP 地址表
CREATE TABLE registered_ips (
    id INTEGER PRIMARY KEY,
    host_id INTEGER,
    ip_version TEXT NOT NULL,
    ip_address TEXT NOT NULL,
    port INTEGER NOT NULL,
    protocol TEXT DEFAULT 'tcp',
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(host_id) REFERENCES host_config(id)
);

-- 转发规则表
CREATE TABLE port_forwards (
    id INTEGER PRIMARY KEY,
    host_id INTEGER,
    name TEXT NOT NULL,
    listen_ip TEXT NOT NULL,
    listen_port INTEGER NOT NULL,
    forward_ip TEXT NOT NULL,
    forward_port INTEGER NOT NULL,
    protocol TEXT DEFAULT 'tcp',
    enabled INTEGER DEFAULT 1,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(host_id) REFERENCES host_config(id)
);
```

### VPS 模式数据库 (vps.db)

```sql
-- 主机注册表
CREATE TABLE hosts (
    id INTEGER PRIMARY KEY,
    host_id TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    last_ip_v4 TEXT,
    last_ip_v6 TEXT,
    last_seen TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    status TEXT DEFAULT 'online',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- IP 地址表
CREATE TABLE ip_addresses (
    id INTEGER PRIMARY KEY,
    host_id TEXT NOT NULL,
    ip_version TEXT NOT NULL,
    ip_address TEXT NOT NULL,
    port INTEGER NOT NULL,
    protocol TEXT DEFAULT 'tcp',
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(host_id) REFERENCES hosts(host_id)
);

-- 转发规则表
CREATE TABLE port_forward_rules (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    listen_ip TEXT NOT NULL,
    listen_port INTEGER NOT NULL,
    forward_ip TEXT NOT NULL,
    forward_port INTEGER NOT NULL,
    protocol TEXT DEFAULT 'tcp',
    enabled INTEGER DEFAULT 1,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## API 端点

### VPS API

| 方法 | 路径 | 描述 |
|------|------|------|
| GET | /api/v1/status | 系统状态 |
| POST | /api/v1/hosts | 注册主机 |
| GET | /api/v1/hosts | 列出所有主机 |
| GET | /api/v1/hosts/{id} | 查询主机信息 |
| POST | /api/v1/hosts/{id}/ip | 注册 IP 地址 |
| DELETE | /api/v1/hosts/{id}/ip | 注销 IP 地址 |
| GET | /api/v1/ip_addresses | 查询所有 IP 地址 |
| GET | /api/v1/ip_addresses/{ip} | 查询指定 IP 信息 |
| POST | /api/v1/port_forwards | 创建转发规则 |
| GET | /api/v1/port_forwards | 查询转发规则 |
| DELETE | /api/v1/port_forwards/{id} | 删除转发规则 |

### Web 管理界面

- 访问 `http://localhost:20000` 查看管理界面

## 注意事项

1. **端口递增**：默认端口 20000，启动失败时自动递增 +1
2. **IP 地址检测**：主机模式初次启动时自动检测 IPv4 和 IPv6
3. **配置优先级**：命令行参数 > 环境变量 > 配置文件 > 默认值
4. **数据库位置**：默认在项目目录下创建 `host.db` 或 `vps.db`
5. **日志级别**：通过 `RUST_LOG` 环境变量控制，如 `RUST_LOG=info`

## 故障排查

### 编译错误

```bash
# 清理项目
cargo clean

# 重新构建
cargo build
```

### 数据库问题

```bash
# 删除数据库（谨慎操作）
rm host.db
rm vps.db
```

### 日志查看

```bash
# 查看详细日志
RUST_LOG=debug cargo run
```
