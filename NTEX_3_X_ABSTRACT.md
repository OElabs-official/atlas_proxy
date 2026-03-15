# ntex 3.x 抽象文档

## 核心概念

### 1. 服务器构建

```rust
// ntex 3.x 使用新的服务器构建方式
web::server(async || {
    App::new()
        .middleware(middleware::Logger::default())
        .service((
            web::resource("/path").route(web::get().to(handler)),
            web::resource("/json").route(web::post().to(json_handler)),
        ))
})
.bind("127.0.0.1:8080")?
.run()
.await
```

### 2. 路由定义

```rust
// 使用 web::resource 和 web::route
web::resource("/users")
    .route(web::get().to(list_users))
    .route(web::post().to(create_user))

// 带路径参数
web::resource("/users/{id}")
    .route(web::get().to(get_user))
    .route(web::delete().to(delete_user))
```

### 3. Handler 函数

```rust
// 简单响应
async fn index() -> &'static str {
    "Hello world!"
}

// 返回 HttpResponse
async fn handler() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

// 返回 Result
async fn handler() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hello"))
}
```

### 4. 提取器 (Extractors)

```rust
// JSON 提取器
async fn handler(item: web::types::Json<MyObj>) -> HttpResponse {
    HttpResponse::Ok().json(&item.0)
}

// 路径参数
async fn handler(path: web::types::Path<(String,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Hello {}", path.0))
}

// Query 参数
async fn handler(query: web::types::Query<Option<String>>) -> HttpResponse {
    // ...
}

// Form 表单
async fn handler(form: web::types::Form<MyParams>) -> HttpResponse {
    // ...
}

// 状态管理
async fn handler(state: web::types::State<MyState>) -> HttpResponse {
    // ...
}
```

### 5. 状态管理

```rust
// 使用 .state() 方法设置全局状态
web::server(async || {
    App::new()
        .state(MyState::new())
        .service(handler)
})

// 在 handler 中使用 State 提取器
async fn handler(state: web::types::State<MyState>) -> HttpResponse {
    // 使用 state
}
```

### 6. 中间件

```rust
use ntex::web::middleware;

web::server(async || {
    App::new()
        .middleware(middleware::Logger::default())
        .middleware(middleware::Compress::default())
})
```

### 7. 静态文件服务

```rust
use ntex_files as fs;

web::server(async || {
    App::new()
        .service(fs::Files::new("/static", "static").show_files_listing())
})
```

### 8. 错误处理

```rust
use ntex::web::{Error, WebResponseError};

impl WebResponseError for MyError {
    fn error_response(&self, _: &HttpRequest) -> HttpResponse {
        match self {
            MyError::NotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
```

### 9. JSON 配置

```rust
// 设置 JSON 提取器限制
web::server(async || {
    App::new()
        .state(web::types::JsonConfig::default().limit(4096))
})
```

## 与 ntex 2.x 的主要差异

| 2.x 版本 | 3.x 版本 | 说明 |
|--|--|--|
| `Server::build().bind("name", addr, || {})` | `web::server(async || { App::new() })` | 新的服务器构建方式 |
| `app_data::Data<T>` | `State<T>` | 状态管理更简单 |
| `web::Json<T>` | `web::types::Json<T>` | 提取器在 types 模块中 |
| `web::Error` | `Error` | 错误类型直接从 ntex::web 导入 |
| `HttpResponse::json()` | `HttpResponse::Ok().json()` | JSON 响应需要先构建 HttpResponse |

## 推荐的项目结构

```
src/
├── main.rs              # 服务器启动入口
├── handlers/            # 处理程序
│   ├── mod.rs
│   ├── users.rs
│   ├── products.rs
├── models.rs            # 数据模型
├── state.rs             # 共享状态
└── middleware.rs        # 自定义中间件
```

## 示例：完整的 REST API

```rust
use ntex::web::{self, middleware, App, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

// 获取所有用户
async fn get_users() -> HttpResponse {
    HttpResponse::Ok().json(vec![])
}

// 创建用户
async fn create_user(
    user: web::types::Json<User>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(&user.0))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    web::server(async || {
        App::new()
            .middleware(middleware::Logger::default())
            .service((
                web::resource("/users").route(web::get().to(get_users)),
                web::resource("/users").route(web::post().to(create_user)),
            ))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```
