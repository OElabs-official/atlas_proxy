# Atlas Proxy 架构设计文档

## 1. 概述

Atlas Proxy 是一个基于 Rust + ntex 的代理服务器应用，提供端口转发和 IP 地址管理功能。系统采用 VPS + 主机的分布式架构，支持多设备间的动态 IP 管理和端口映射。

### 1.1 核心特性

- **双工作模式**：VPS 模式（固定 IP）和主机模式（动态 IP），默认为主机模式
- **IP 地址管理**：主机向 VPS 注册、查询和注销 IP 地址，支持 IPv4 和 IPv6
- **端口转发**：基于 IP 地址的端口映射和转发
- **Web 管理端**：基于 Dioxus SSR 的管理界面
- **配置持久化**：SQLite 存储和 TOML 配置文件导出
- **端口递增**：默认端口 20000，启动失败自动递增 +1

### 1.2 技术栈

| 组件 | 技术 |
|------|------|
| 后端框架 | Rust + ntex |
| Web 管理端 | Dioxus SSR + Tailwind CSS |
| 命令行解析 | clap |
| 序列化 | serde + serde_json |
| 配置管理 | serde_toml |
| 数据库 | rusqlite (SQLite) |

## 2. 命令行接口

### 2.1 基本用法

```bash
# 主机模式（默认）
atlas_proxy

# VPS 模式
atlas_proxy --vps

# 指定配置文件
atlas_proxy --vps --config /path/to/config.toml

# 查看版本
atlas_proxy --version

# 查看帮助
atlas_proxy --help
```

### 2.2 命令行参数

| 参数 | 说明 | 默认值 |
|------|------|--------|
| `--vps` | 启用 VPS 模式 | false (默认主机模式) |
| `--config <FILE>` | 指定配置文件路径 | `config.toml` 或自动检测 |
| `--port <PORT>` | 服务端口 | 20000 |
| `--name <NAME>` | 本机名称（初次启动） | 交互式输入 |

### 2.3 端口递增机制

- 默认端口：20000
- 启动失败时自动递增 +1
- 最大重试次数：100 次
- 成功绑定后记录实际端口到 `projectdir/proxy.toml`

### 2.4 首次启动流程

1. 检测 `projectdir/proxy.toml` 是否存在
2. 若不存在，提示输入本机名称
3. 自动获取本机 IPv4 和 IPv6 地址
4. 创建默认配置文件
5. 进入正常启动流程

## 3. 系统架构

```
┌─────────────────────────────────────────────────────────────┐
│                      Internet                                │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                      VPS (固定 IP)                           │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  - IP 地址注册中心                                      │  │
│  │  - 主机注册/查询/注销 API                               │  │
│  │  - 转发表管理                                           │  │
│  │  - Web 管理端 (Dioxus SSR)                              │  │
│  └────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                            │
          ┌───────────────┬─┼───────────────┬─────────────────┐
          │               │ │               │                 │
          ▼               ▼ ▼               ▼                 ▼
┌────────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│   主机 A       │ │   主机 B     │ │   主机 C     │ │   主机 N     │
│  (动态 IP)     │ │  (动态 IP)   │ │  (动态 IP)   │ │  (动态 IP)   │
│  ┌──────────┐  │ │ ┌──────────┐ │ │ ┌──────────┐ │ │ ┌──────────┐ │
│  │ IP 注册  │  │ │ │ IP 注册  │ │ │ │ IP 注册  │ │ │ │ IP 注册  │ │
│  │ 转发表   │  │ │ │ 转发表   │ │ │ │ 转发表   │ │ │ │ 转发表   │ │
│  │ 服务端口 │  │ │ │ 服务端口 │ │ │ │ 服务端口 │ │ │ │ 服务端口 │ │
│  └──────────┘  │ │ │ 转发规则 │ │ │ │ 转发规则 │ │ │ │ 转发规则 │ │
│                │ │ │ 持久化   │ │ │ │ 持久化   │ │ │ │ 持久化   │ │
└────────────────┘ └──────────────┘ └──────────────┘ └──────────────┘
```

## 4. 模块设计

### 4.1 模块划分

```
src/
├── main.rs                 # 应用入口
├── config/                 # 配置管理
│   ├── mod.rs
│   └── settings.rs
├── models/                 # 数据模型
│   ├── mod.rs
│   ├── host.rs             # 主机模型
│   ├── ip_address.rs       # IP 地址模型
│   └── port_forward.rs     # 端口转发规则模型
├── store/                  # 数据存储
│   ├── mod.rs
│   ├── memory.rs           # 内存存储（开发）
│   └── persistence.rs      # 持久化存储
├── vps/                    # VPS 模式模块
│   ├── mod.rs
│   ├── server.rs           # VPS 服务器
│   └── handler.rs          # API 处理器
├── host/                   # 主机模式模块
│   ├── mod.rs
│   ├── client.rs           # 主机客户端
│   ├── registration.rs     # IP 注册管理
│   └── tunnel.rs           # 端口转发隧道
│   ├── port_mapping.rs       # 端口映射管理
│   └── config_export.rs      # 配置导出
└── web/                    # Web 管理端
    ├── mod.rs
    ├── routes.rs           # SSR 路由
    └── templates.rs        # 页面模板
```

### 4.2 核心数据模型

#### 主机模型 (Host)

```rust
pub struct Host {
    pub id: String,           // 主机唯一标识
    pub name: String,         // 主机名称
    pub ip: String,           // 当前 IP 地址
    pub last_seen: DateTime,  // 最后在线时间
    pub status: HostStatus,   // 在线/离线
    pub created_at: DateTime,
}
```

#### IP 地址模型 (IpAddress)

```rust
pub struct IpAddress {
    pub id: u64,
    pub host_id: String,
    pub ip: String,           // 分配的 IP 地址
    pub port: u16,            // 服务端口
    pub protocol: Protocol,   // TCP/UDP
    pub description: String,
    pub created_at: DateTime,
}
```

#### 端口转发规则 (PortForwardRule)

```rust
pub struct PortForwardRule {
    pub id: u64,
    pub name: String,
    pub listen_ip: String,
    pub listen_port: u16,
    pub forward_ip: String,   // 目标 IP
    pub forward_port: u16,
    pub protocol: Protocol,
    pub enabled: bool,
}
```

## 5. 工作模式

### 5.1 VPS 模式

**角色**：IP 地址注册中心和转发规则管理器

**主要功能**：
1. 接收主机的 IP 地址注册请求
2. 提供 IP 地址查询 API
3. 管理端口转发规则
4. 提供 Web 管理界面
5. 维护主机心跳检测

**运行方式**：
```bash
atlas_proxy --vps
```

**端口监听**：
- 默认端口：20000
- 启动失败自动递增 +1
- 支持 TLS/HTTPS

### 5.2 主机模式

**角色**：动态 IP 设备，向 VPS 注册并建立转发

**主要功能**：
1. **初次启动交互**：提示输入本机名称，保存到 `projectdir/proxy.toml`
2. **自动获取 IP**：同时支持 IPv4 和 IPv6
3. **向 VPS 注册**：IP 地址 + 端口 + 转发规则列表
4. **维护心跳连接**：定期向 VPS 发送心跳
5. **记录转发规则**：持久化到 SQLite，记录转发的 IP 和端口
6. **查询其他主机**：从 VPS 获取其他设备的 IP 信息

**运行方式**：
```bash
atlas_proxy
```

**配置文件路径**：
- 默认：`./proxy.toml` 或 `./config.toml`
- 项目目录：`projectdir/proxy.toml`

## 6. API 设计

### 6.1 VPS API

| 方法 | 路径 | 描述 |
|------|------|------|
| POST | /api/v1/hosts | 注册主机 |
| GET | /api/v1/hosts/{id} | 查询主机信息 |
| GET | /api/v1/hosts | 列出所有主机 |
| POST | /api/v1/hosts/{id}/ip | 注册 IP 地址 |
| DELETE | /api/v1/hosts/{id}/ip | 注销 IP 地址 |
| GET | /api/v1/ip_addresses | 查询所有 IP 地址 |
| GET | /api/v1/ip_addresses/{ip} | 查询指定 IP 信息 |
| POST | /api/v1/port_forwards | 创建转发规则 |
| GET | /api/v1/port_forwards | 查询转发规则 |
| DELETE | /api/v1/port_forwards/{id} | 删除转发规则 |
| GET | /api/v1/status | 系统状态 |

### 6.2 主机 API

| 方法 | 路径 | 描述 |
|------|------|------|
| GET | /api/v1/vps/status | 检查 VPS 连接 |
| POST | /api/v1/vps/register | 注册到 VPS |
| POST | /api/v1/vps/heartbeat | 发送心跳 |
| GET | /api/v1/vps/hosts | 查询主机列表 |
| GET | /api/v1/vps/ip/{host_id} | 查询指定主机 IP |

## 7. 端口转发流程

```
┌──────────────────────────────────────────────────────────────┐
│                     端口转发流程                              │
├──────────────────────────────────────────────────────────────┤
│ 1. 主机 A 启动服务，监听本地端口 (如 127.0.0.1:8080)         │
│ 2. 主机 A 向 VPS 注册 IP 和端口信息                          │
│ 3. VPS 保存转发规则：外部端口 → 主机 A 的 IP:port            │
│ 4. 主机 B 通过 VPS 查询主机 A 的 IP 地址                     │
│ 5. 主机 B 建立隧道连接：本地端口 → 主机 A 的 IP:port         │
│ 6. 数据流经隧道进行转发                                      │
└──────────────────────────────────────────────────────────────┘
```

## 8. 数据存储

### 8.1 SQLite 持久化存储

主机模式和 VPS 模式都使用 SQLite 进行数据持久化：

**主机模式表结构**：
```sql
-- 主机配置表
CREATE TABLE host_config (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,           -- 本机名称
    vps_url TEXT NOT NULL,        -- VPS 服务器地址
    port INTEGER NOT NULL,        -- 服务端口
    last_ip_v4 TEXT,              -- 最后记录的 IPv4
    last_ip_v6 TEXT,              -- 最后记录的 IPv6
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- IP 地址注册表
CREATE TABLE registered_ips (
    id INTEGER PRIMARY KEY,
    host_id INTEGER,
    ip_version TEXT NOT NULL,     -- 'ipv4' 或 'ipv6'
    ip_address TEXT NOT NULL,
    port INTEGER NOT NULL,
    protocol TEXT DEFAULT 'tcp',  -- 'tcp' 或 'udp'
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
    forward_ip TEXT NOT NULL,     -- 目标 IP
    forward_port INTEGER NOT NULL,
    protocol TEXT DEFAULT 'tcp',
    enabled INTEGER DEFAULT 1,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(host_id) REFERENCES host_config(id)
);
```

**VPS 模式表结构**：
```sql
-- 主机注册表
CREATE TABLE hosts (
    id INTEGER PRIMARY KEY,
    host_id TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    last_ip_v4 TEXT,
    last_ip_v6 TEXT,
    last_seen TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    status TEXT DEFAULT 'online',  -- 'online' 或 'offline'
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- IP 地址表
CREATE TABLE ip_addresses (
    id INTEGER PRIMARY KEY,
    host_id INTEGER,
    ip_version TEXT NOT NULL,
    ip_address TEXT NOT NULL,
    port INTEGER NOT NULL,
    protocol TEXT DEFAULT 'tcp',
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(host_id) REFERENCES hosts(id)
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

### 8.2 配置文件导出

主机模式支持将当前配置和转发规则导出为 TOML 格式：

```toml
# Atlas Proxy 主机配置
# 此文件由程序自动生成和管理

name = "my-host"              # 本机名称（初次启动时输入）
vps_url = "http://vps.example.com:20000"  # VPS 服务器地址

[listening]
port = 20000                  # 实际监听端口（自动递增确定）
bind_address = "0.0.0.0"

[registration]
ip_version = ["ipv4", "ipv6"] # 注册的 IP 版本
interval = 30                 # 心跳间隔（秒）

# 转发规则列表
[[port_forwards]]
name = "web-server"
listen_ip = "0.0.0.0"
listen_port = 80
forward_ip = "127.0.0.1"
forward_port = 8080
protocol = "tcp"

[[port_forwards]]
name = "ssh-access"
listen_ip = "0.0.0.0"
listen_port = 2222
forward_ip = "127.0.0.1"
forward_port = 22
protocol = "tcp"

[logging]
level = "info"
file = "./logs/proxy.log"
```

**导出命令**：
```bash
atlas_proxy --export-config > proxy.toml
```

## 9. 消息传递

### 9.1 主机到 VPS

- **注册消息**：`{"action": "register", "host_id": "...", "ip_v4": "...", "ip_v6": "...", "port": 8080}`
- **心跳消息**：`{"action": "heartbeat", "host_id": "...", "timestamp": 1234567890}`
- **注销消息**：`{"action": "unregister", "host_id": "..."}`
- **查询消息**：`{"action": "query", "target_host_id": "..."}`
- **响应消息**：`{"action": "response", "data": {...}}`

### 9.2 隧道连接

使用 WebSocket 或 TCP 长连接建立转发隧道：
```
主机 B ─── WebSocket/TCP ───> VPS ─── WebSocket/TCP ───> 主机 A
```

## 10. 配置文件

### 10.1 VPS 配置示例

```toml
# Atlas Proxy VPS 配置
mode = "vps"
server = "0.0.0.0:20000"
web_root = "./web/dist"

[storage]
type = "sqlite"
path = "./data/vps.db"

[logging]
level = "info"
file = "./logs/vps.log"
```

### 10.2 主机配置示例

```toml
# Atlas Proxy 主机配置
name = "my-host"
vps_url = "http://vps.example.com:20000"

[listening]
port = 20000
bind_address = "0.0.0.0"

[registration]
ip_version = ["ipv4", "ipv6"]
interval = 30

# 转发规则列表
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
``

## 11. IP 地址管理

### 11.1 IPv4 和 IPv6 支持

主机模式初次启动时会：
1. 自动检测本机所有网络接口
2. 同时获取 IPv4 和 IPv6 地址
3. 允许用户选择要注册的 IP 版本

**IP 地址检测命令**：
```bash
# IPv4
ip addr show | grep "inet " | awk '{print $2}'

# IPv6
ip addr show | grep "inet6 " | awk '{print $2}'
```

### 11.2 配置文件结构

```toml
# Atlas Proxy 主机配置
name = "my-host"
vps_url = "http://vps.example.com:20000"

[listening]
port = 20000
bind_address = "0.0.0.0"

[registration]
# 可选: ipv4, ipv6, both
ip_version = ["ipv4", "ipv6"]
interval = 30

# 转发规则列表
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

## 12. 安全考虑

1. **认证机制**：支持 API Token 或 JWT 认证
2. **HTTPS**：支持 TLS 加密通信
3. **IP 白名单**：可配置允许注册的 IP 范围
4. **速率限制**：防止 DDOS 攻击
5. **日志审计**：记录所有关键操作

## 13. 部署方案

### 13.1 VPS 部署

- 云服务器（阿里云、腾讯云等）
- 固定公网 IP
- Docker 容器化部署

### 13.2 主机部署

- 树莓派/NAS/本地服务器
- 动态公网 IP 或内网
- systemd 服务运行

## 14. 后续扩展

- [ ] 多级代理支持
- [ ] P2P 直连优化
- [ ] 访问控制列表 (ACL)
- [ ] 流量统计和监控
- [ ] 自动证书管理 (ACME)
