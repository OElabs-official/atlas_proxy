# Atlas Proxy 开发状态报告

## 项目信息

**项目名称**: Atlas Proxy
**项目类型**: 分布式端口转发代理
**开发语言**: Rust (后端) + React (前端)
**最后更新**: 2026-03-11 (添加开发任务清单)

---

## 1. 项目概述

### 1.1 系统架构

```
┌─────────────────────────────────────────────────────────────────────┐
│                        Internet                                     │
└─────────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      VPS (固定 IP)                                  │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │  - IP 地址注册中心                                            │  │
│  │  - 主机注册/查询/注销 API                                     │  │
│  │  - 转发表管理                                                 │  │
│  │  - Web 管理端 (React + Vite)                                  │  │
│  └───────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
                                   │
                     ┌─────────────┼─────────────┐
                     │             │             │
                     ▼             ▼             ▼
            ┌─────────────┐ ┌─────────┐ ┌─────────────┐
            │   主机 A    │ │ 主机 B  │ │   主机 C    │
            │ (动态 IP)   │ │(动态 IP)│ │ (动态 IP)   │
            └─────────────┘ └─────────┘ └─────────────┘
```

### 1.2 技术栈

| 组件 | 技术 | 版本 |
|------|------|------|
| 后端框架 | Rust + ntex | 3.x |
| Web 框架 | React | 18.3.x |
| 构建工具 | Vite | 5.4.x |
| 样式框架 | Tailwind CSS | 3.4.x |
| 数据库 | SQLite + sqlx | 0.8.x |
| 序列化 | serde + serde_json | 1.x |
| 配置管理 | toml | 1.x |
| 命令行解析 | clap | 4.x |
| 错误处理 | anyhow | 1.x |
| 异步运行时 | tokio | 1.x |

---

## 2. 项目结构

### 2.1 后端结构

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
│   └── persistence.rs      # 持久化存储 (SQLite)
├── vps/                    # VPS 模式模块
│   ├── mod.rs
│   ├── handler.rs          # API 处理器
├── host/                   # 主机模式模块
│   ├── mod.rs
│   ├── client.rs           # 主机客户端
│   ├── registration.rs     # IP 注册管理
│   ├── tunnel.rs           # 隧道管理
│   ├── port_mapping.rs     # 端口映射管理
│   └── config_export.rs    # 配置导出
├── prelude.rs              # 基础设施模块 (目录管理、数据库、日志)
└── web/                    # Web 管理端 (已废弃 - 使用 React 前端)
```

### 2.2 前端结构

```
frontend/
├── src/
│   ├── main.jsx            # React 入口
│   ├── App.jsx             # 主应用组件
│   └── index.css           # 全局样式
├── dist/                   # 构建输出
├── index.html              # HTML 模板
├── vite.config.js          # Vite 配置
├── tailwind.config.js      # Tailwind 配置
├── tsconfig.json           # TypeScript 配置
└── package.json            # 依赖管理
```

---

## 3. 功能实现状态

### 3.1 后端功能

| 功能模块 | 状态 | 说明 |
|----------|------|------|
| **配置管理** | ✅ 完成 | 支持 TOML 配置文件和命令行参数 |
| **数据库持久化** | ✅ 完成 | SQLite 数据库，支持 VPS 和 Host 两种模式 |
| **VPS 模式** | ✅ 完成 | HTTP 服务器和 API 端点已实现 |
| **Host 模式** | ✅ 完成 | Web 管理界面，支持命令行注册 |
| **IP 地址管理** | ✅ 完成 | 过滤回环地址，只发送 routable IP |
| **端口转发** | ⚠️ 基础完成 | 数据模型完成，转发引擎待实现 |
| **Web API** | ✅ 完成 | GET/POST 端点已实现 |
| **心跳机制** | ⚠️ 基础完成 | 代码存在，需通过 Web 界面触发 |
| **配置导出** | ⚠️ 基础完成 | 代码存在，未集成到主流程 |
| **基础设施模块** | ✅ 完成 | prelude.rs 提供目录管理、数据库、日志功能 |

### 3.2 前端功能

| 功能模块 | 状态 | 说明 |
|----------|------|------|
| **项目初始化** | ✅ 完成 | React + Vite + Tailwind 搭建完成 |
| **状态显示** | ✅ 完成 | 显示系统状态和模式信息 |
| **主机列表** | ✅ 完成 | 显示注册的主机列表 |
| **快速操作** | ⚠️ 占位 | 按钮已创建，功能待实现 |
| **端口转发表** | ⚠️ 占位 | 表格已创建，数据来源待完善 |
| **响应式设计** | ✅ 完成 | 支持移动端和桌面端 |

---

## 4. API 实现状态

### 4.1 VPS API

| 方法 | 路径 | 状态 | 说明 |
|------|------|------|------|
| GET | /api/v1/status | ✅ 完成 | 返回系统状态 |
| GET | /api/v1/hosts | ⚠️ 基础完成 | 返回主机列表 (JSON) |
| POST | /api/v1/hosts | ⚠️ 占位 | 主机注册 (待实现) |
| GET | /api/v1/ip_addresses | ⚠️ 基础完成 | 返回 IP 地址列表 |
| GET | /api/v1/port_forwards | ⚠️ 基础完成 | 返回转发规则列表 |

### 4.2 Host API

| 方法 | 路径 | 状态 | 说明 |
|------|------|------|------|
| GET | /api/v1/vps/status | ⚠️ 基础完成 | 检查 VPS 连接（命令行注册时使用） |
| POST | /api/v1/vps/register | ✅ 完成 | 注册到 VPS（--register 命令行参数） |
| POST | /api/v1/vps/heartbeat | ⚠️ 基础完成 | 发送心跳（可由 Web 界面触发） |

---

## 5. 数据库设计

### 5.1 VPS 模式表结构

| 表名 | 字段 | 状态 |
|------|------|------|
| hosts | id, host_id, name, last_ip_v4, last_ip_v6, last_seen, status, created_at | ✅ 完成 |
| ip_addresses | id, host_id, ip_version, ip_address, port, protocol, description, created_at | ✅ 完成 |
| port_forward_rules | id, name, listen_ip, listen_port, forward_ip, forward_port, protocol, enabled, created_at | ✅ 完成 |

### 5.2 Host 模式表结构

| 表名 | 字段 | 状态 |
|------|------|------|
| host_config | id, name, vps_url, port, last_ip_v4, last_ip_v6, created_at | ✅ 完成 |
| registered_ips | id, host_id, ip_version, ip_address, port, protocol, description, created_at | ✅ 完成 |
| port_forwards | id, host_id, name, listen_ip, listen_port, forward_ip, forward_port, protocol, enabled, created_at | ✅ 完成 |

---

## 6. 编译状态

### 6.1 后端编译

```bash
cargo build 2>&1 | tail -5
# 输出: Finished `dev` profile [unoptimized + debuginfo]
```

**状态**: ✅ 编译成功
**警告数**: 22 (均为未使用代码警告)

### 6.2 前端编译

```bash
cd frontend && npm run build 2>&1
# 输出: ✓ built in 5.45s
```

**状态**: ✅ 构建成功
**输出目录**: `frontend/dist/`

---

## 7. 开发任务清单 (TODO List)

### 7.1 任务列表

| ID | 任务 | 模块 | 优先级 | 状态 |
|----|------|------|--------|------|
| #1 | 实现端口转发规则管理 | vps | 高 | ✅ 完成 |
| #2 | 性能优化和代码重构 | refactor | 低 | pending |
| #3 | 添加单元测试 | testing | 低 | pending |
| #4 | 添加日志审计功能 | vps | 低 | pending |
| #5 | 实现 IP 地址注册和注销功能 | vps | 高 | ✅ 完成 |
| #6 | 完成主机注册功能 (POST /api/v1/hosts) | vps | 高 | ✅ 完成 |
| #7 | 添加用户认证机制 | vps | 中 | pending |
| #8 | 完善 Web 管理界面 | frontend | 中 | pending |
| #9 | 实现配置导出功能 | host | 中 | pending |
| #10 | 实现隧道连接功能 | host | 高 | pending |

### 7.2 详细任务描述

#### 高优先级 (P0) - 已完成 ✅

**#6 完成主机注册功能 (POST /api/v1/hosts)**
- ✅ 解析注册请求体
- ✅ 验证主机信息（名称、IP 地址等）
- ✅ 保存主机到数据库
- ✅ 返回注册结果（成功/失败消息）

**#5 实现 IP 地址注册和注销功能**
- ✅ POST /api/v1/ip_addresses - 注册 IP 地址
- ✅ DELETE /api/v1/ip_addresses/{ip} - 注销 IP 地址
- ✅ GET /api/v1/ip_addresses - 查询 IP 地址列表
- ✅ GET /api/v1/ip_addresses/{ip} - 查询指定 IP

**#1 实现端口转发规则管理**
- ✅ POST /api/v1/port_forwards - 创建规则
- ✅ DELETE /api/v1/port_forwards/{id} - 删除规则
- ✅ GET /api/v1/port_forwards - 查询转发规则列表

#### 高优先级 (P0) - 进行中

**#10 实现隧道连接功能**
- ⏳ 创建 WebSocket/TCP 隧道连接
- ⏳ 维护隧道状态
- ⏳ 转发数据流
- ⏳ 隧道断开处理

#### 中优先级 (P1) - 重要功能

**#8 完善 Web 管理界面**
- 主机管理页面（添加/删除/编辑）
- IP 地址管理页面
- 端口转发规则管理页面
- 实时状态更新

**#7 添加用户认证机制**
- Token 认证或 JWT 认证
- 认证中间件
- 保护敏感 API 端点

**#9 实现配置导出功能**
- 集成到主流程
- 支持导出到文件
- 配置备份功能

#### 低优先级 (P2) - 可选功能

**#4 添加日志审计功能**
- 记录所有关键操作
- 主机注册/注销日志
- IP 地址变更日志
- 端口转发规则变更日志
- API 访问日志

**#3 添加单元测试**
- 数据库操作测试
- API 端点测试
- 配置解析测试
- 前端组件测试

**#2 性能优化和代码重构**
- 减少未使用代码警告
- 优化数据库查询
- 添加连接池管理
- 代码重构和文档注释

---

### 7.3 开发建议

1. **优先顺序**: 建议按照 ID 顺序开发（#6 → #5 → #1 → #10）
2. **测试驱动**: 每个功能开发完成后及时添加测试
3. **文档同步**: 新增 API 需同步更新 API 文档
4. **代码审查**: 重要功能完成前进行代码审查

---

## 8. 使用说明

### 8.1 启动 VPS 模式

```bash
# 编译后端
cargo build --release

# 启动 VPS 模式
cargo run --release -- --vps

# 或指定端口
cargo run --release -- --vps --port 20000
```

### 8.2 启动 Host 模式

```bash
# 启动主机模式 (默认)
cargo run --release

# 首次启动会提示输入本机名称
```

### 8.3 启动前端开发服务器

```bash
cd frontend
npm run dev
```

### 8.4 构建前端生产版本

```bash
cd frontend
npm run build
```

---

## 9. 配置文件

### 9.1 默认配置路径

- 项目根目录: `./config.toml`
- 主机配置: `./proxy.toml` (首次启动自动生成)

### 9.2 配置示例

```toml
# VPS 模式配置
mode = "vps"
listening = { bind_address = "0.0.0.0", port = 20000 }

# Host 模式配置
name = "my-host"
vps_url = "http://vps.example.com:20000"
listening = { bind_address = "0.0.0.0", port = 20000 }
```

---

## 10. 已知问题

1. **ntex 3.x API 变化**: 项目已适配 ntex 3.x 的重大 API 变化
2. **rusqlite 0.38**: 已修复 `.optional()` 方法移除的问题
3. **Error 类型**: 已统一使用 anyhow::Error
4. **未使用代码**: 大量占位代码产生警告，不影响功能

---

## 11. 开发规范

### 11.1 依赖版本管理

- **禁止降级**: 任何时候不得降级依赖版本
- **使用最新版**: 始终使用 crates.io 上的最新稳定版本
- **兼容性**: 遇到 breaking change 时参考官方文档迁移

### 11.2 代码规范

- Rust 代码遵循 rustfmt 格式
- 前端代码遵循 eslint 规范
- 使用 anyhow 进行错误处理
- 使用 tokio 进行异步编程

---

## 12. 相关文档

- [ARCHITECTURE.md](./ARCHITECTURE.md) - 完整架构设计文档
- [Cargo.toml](./Cargo.toml) - 后端依赖配置
- [frontend/package.json](./frontend/package.json) - 前端依赖配置

---

## 13. 最近更新 (2026-03-11)

### 修复的编译问题

1. **main.rs**:
   - 将 `use anyhow::Error` 改为 `use anyhow::Result`
   - 更新所有函数返回类型为 `Result<...>` (使用 anyhow::Error)

2. **store/persistence.rs**:
   - 将 `use std::error::Error` 改为 `use anyhow::Error`
   - 更新所有方法返回类型为 `Result<..., Error>`

3. **host/*.rs**:
   - 将 `use std::error::Error` 改为 `use anyhow::{Result, anyhow};`
   - 将所有字符串错误转换 `.into()` 改为 `anyhow!("message")`

4. **清理未使用导入**:
   - config/mod.rs
   - host/mod.rs
   - models/mod.rs
   - host/config_export.rs

### 编译结果

- **后端**: ✅ 编译成功 (29 个警告 - 未使用代码)
- **前端**: ✅ 构建成功 (vite build 5.45s)

### 最近更新 (2026-03-12) - IP 地址过滤功能增强

#### 新增功能
- ✅ `is_routable_ip()` - 静态方法，检查 IP 是否为有效的公网/局域网地址
- ✅ `get_routable_ips()` - 获取所有 routable IP 地址列表

#### 过滤规则
**IPv4 地址过滤**：
- 回环地址 (127.0.0.0/8)
- 链路本地地址 (169.254.0.0/16)
- 未指定地址 (0.0.0.0)
- 广播地址 (255.255.255.255)
- 多播地址 (224.0.0.0/4)
- 文档地址 (192.0.0.0/24, 198.51.100.0/24, 203.0.113.0/24)

**IPv6 地址过滤**：
- 回环地址 (::1)
- 未指定地址 (::)
- 链路本地地址 (fe80::/10)
- 组播地址 (ff00::/8)
- 文档地址 (2001:db8::/32)

#### 影响范围
- `get_or_set_name()` - 首次启动时只保存 routable IP
- `register_to_vps()` - 注册到 VPS 时只发送 routable IP
- `send_heartbeat()` - 心跳更新时只发送 routable IP

---

### 最近更新 (2026-03-11) - 高优先级功能实现完成

#### 已完成的高优先级功能

1. **主机注册功能 (POST /api/v1/hosts)**:
   - ✅ 验证主机名称唯一性
   - ✅ 保存主机信息到数据库
   - ✅ 返回成功/失败响应

2. **IP 地址管理**:
   - ✅ GET /api/v1/ip_addresses - 查询所有 IP 地址
   - ✅ POST /api/v1/ip_addresses - IP 注册端点
   - ✅ DELETE /api/v1/ip_addresses/{ip} - IP 注销端点
   - ✅ 新增 `vps_delete_ip_address()` 方法
   - ✅ 新增 `vps_update_host_seen()` 方法

3. **端口转发规则管理**:
   - ✅ GET /api/v1/port_forwards - 查询所有转发规则
   - ✅ POST /api/v1/port_forwards - 创建规则端点
   - ✅ DELETE /api/v1/port_forwards/{id} - 删除规则端点
   - ✅ 新增 `vps_insert_port_forward()` 方法（返回规则 ID）

4. **数据库增强**:
   - ✅ 新增 `vps_get_host_by_name()` 方法（根据名称查找主机）
   - ✅ 新增 `vps_delete_ip_address()` 方法
   - ✅ 新增 `vps_update_host_seen()` 方法

#### 代码结构优化
- ✅ 创建 `src/vps/handler.rs` - 统一处理 VPS API 逻辑
- ✅ 更新 `src/vps/mod.rs` - 导出 handler 模块
- ✅ 清理未使用的导入
- ✅ 适配 ntex 3.x API

---

## 14. 开发任务清单 (2026-03-11)

### 14.1 任务概览

已创建 10 个开发任务，使用 TaskCreate 工具进行跟踪管理：

| ID | 任务 | 模块 | 优先级 |
|----|------|------|--------|
| #1 | 实现端口转发规则管理 | vps | 高 |
| #2 | 性能优化和代码重构 | refactor | 低 |
| #3 | 添加单元测试 | testing | 低 |
| #4 | 添加日志审计功能 | vps | 低 |
| #5 | 实现 IP 地址注册和注销功能 | vps | 高 |
| #6 | 完成主机注册功能 (POST /api/v1/hosts) | vps | 高 |
| #7 | 添加用户认证机制 | vps | 中 |
| #8 | 完善 Web 管理界面 | frontend | 中 |
| #9 | 实现配置导出功能 | host | 中 |
| #10 | 实现隧道连接功能 | host | 高 |

### 14.2 任务管理命令

```bash
# 查看所有任务
/tasks

# 获取任务详情
/task-get <task_id>

# 更新任务状态
/task-update <task_id> --status in_progress
/task-update <task_id> --status completed
```

### 14.3 建议开发顺序

1. **第一阶段** (高优先级) - 已完成 ✅:
   - ✅ #6 完成主机注册功能
   - ✅ #5 实现 IP 地址注册和注销功能
   - ✅ #1 实现端口转发规则管理

2. **第二阶段** (高优先级):
   - ⏳ #10 实现隧道连接功能

3. **第三阶段** (中优先级):
   - ⏳ #8 完善 Web 管理界面
   - ⏳ #7 添加用户认证机制
   - ⏳ #9 实现配置导出功能

4. **第四阶段** (低优先级):
   - ⏳ #4 添加日志审计功能
   - ⏳ #3 添加单元测试
   - ⏳ #2 性能优化和代码重构

---

### 最近更新 (2026-03-14)

#### prelude.rs 重构完成

**问题**:
- prelude.rs 中的 `ConfigV1` 类型与项目实际使用的 `config::Config` 冲突
- 缺失 tracing subscriber 相关导入导致编译错误

**解决**:
- ✅ 移除 prelude.rs 中的 `ConfigV1` 类型和相关代码
- ✅ 添加 tracing subscriber 所需的 trait 导入 (`Layer`, `SubscriberExt`, `SubscriberInitExt`)
- ✅ 修复 ProjectPathV1 的可见性警告（改为 pub struct）
- ✅ 清理未使用导入（serde、smart_default、HashMap、LogLevelFilter）
- ✅ main.rs 改为显式导入 `config::Config` 而非通过 prelude

**当前状态**:
- ✅ prelude.rs 编译成功，无错误
- ⚠️ prelude.rs 功能暂未被其他模块使用（基础设施模块待集成）

---

**项目维护者**: Atlas Proxy Team




project_dir 需要修改，使用xdg
