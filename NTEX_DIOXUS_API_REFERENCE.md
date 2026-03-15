# ntex 3.x 和 Dioxus 0.7 API 参考指南

## ntex 3.x HTTP 服务器

### 基本服务器构建

```rust
use ntex::server::Server;
use ntex::web::{App, HttpResponse, HttpServer};

// 创建服务器
let server = Server::build()
    .bind("http", "127.0.0.1:8080", || {
        App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Hello") }))
    })?
    .shutdown_timeout(30)
    .run()
    .await?;
```

### HttpServer（推荐方式）

```rust
use ntex::web::{App, HttpResponse, HttpServer};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn handler() -> HttpResponse {
    HttpResponse::Ok().body("Hello World")
}
```

### 路由定义

```rust
// GET 请求
.route("/path", web::get().to(handler))

// POST 请求
.route("/path", web::post().to(handler))

// PUT 请求
.route("/path", web::put().to(handler))

// DELETE 请求
.route("/path", web::delete().to(handler))

// 路径参数
.route("/users/{id}", web::get().to(|path: web::types::Path<String>| async move {
    let id = path.into_inner();
    // ...
}))
```

### 请求体处理

```rust
// JSON 请求体
use ntex::web::{Json, Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

async fn login(req: Json<LoginRequest>) -> Result<Json<MyResponse>, Error> {
    let data = req.into_inner();
    // ...
    Ok(Json(response))
}

// 表单数据
use ntex::web::Form;

async fn submit(form: Form<MyForm>) -> String {
    let data = form.into_inner();
    // ...
}

// raw body
use ntex::web::web::Bytes;

async fn raw(body: Bytes) -> impl web::IntoResponse {
    // ...
}
```

### 响应类型

```rust
// HttpResponse
HttpResponse::Ok()
    .content_type("text/plain")
    .body("Hello")

// JSON 响应
use ntex::web::Json;
Json(serde_json::json!({"status": "ok"}))

// 重定向
HttpResponse::Found()
    .insert_header(("location", "/new-path"))
    .finish()

// 错误响应
web::Error::from_message(404, "Not found")
```

### 中间件和应用数据

```rust
use ntex::web::Data;

// 共享应用数据
let app_data = Data::new(MyState { value: 42 });

HttpServer::new(move || {
    App::new()
        .app_data(app_data.clone())
        .route("/path", web::get().to(handler))
})

// 在处理程序中使用
async fn handler(data: Data<MyState>) -> impl IntoResponse {
    let value = data.value;
    // ...
}
```

### 服务资源（Service）

```rust
// 使用 resource 和 service
web::resource("/api")
    .route(web::get().to(get_handler))
    .route(web::post().to(post_handler))

// 或使用 service（更灵活）
web::service(
    web::resource("/api")
        .route(web::get().to(get_handler))
)
```

---

## Dioxus 0.7 SSR

### 基本组件

```rust
use dioxus::prelude::*;

fn main() {
    dioxus_ssr::render_component_to_string(|cx| rsx! {
        div { "Hello World" }
    });
}

// 函数组件
fn MyComponent(cx: Scope) -> Element {
    rsx! {
        div { "Hello" }
    }
}

// 带参数的组件
fn Greeting(cx: Scope, name: String) -> Element {
    rsx! {
        h1 { "Hello, {name}!" }
    }
}
```

### rsx! 宏

```rust
// 基本元素
rsx! {
    div { "内容" }
    span { class: "highlight", "带样式的文本" }
}

// 属性
rsx! {
    div {
        class: "container",
        id: "main",
        style: "color: red;",
    }
}

// 子元素
rsx! {
    div {
        h1 { "标题" }
        p { "段落" }
    }
}

// 条件渲染
rsx! {
    if is_visible {
        div { "可见的内容" }
    }
}

// 列表渲染
rsx! {
    ul {
        for item in items {
            li { "{item}" }
        }
    }
}
```

### 状态管理（SSR 模式）

```rust
// 在 SSR 中，状态通常是只读的或通过 props 传递
fn Counter(cx: Scope) -> Element {
    let count = use_ref(cx, || 0);

    rsx! {
        div {
            p { "{count}" }
            button { onclick: move |_| count += 1, "增加" }
        }
    }
}
```

### 渲染到字符串

```rust
use dioxus_ssr::render_component_to_string;

// 简单渲染
let html = render_component_to_string(|cx| rsx! {
    div { "Hello" }
});

// 带参数的渲染
let html = render_component_to_string(move |cx| {
    rsx! {
        div { "{some_variable}" }
    }
});
```

### 文档头管理

```rust
rsx! {
    document::Document {
        head {
            meta { charset: "utf-8" };
            meta { name: "viewport", content: "width=device-width, initial-scale=1.0" };
            title { "页面标题" };
            link { rel: "stylesheet", href: "/styles.css" };
        }
        body {
            // ...
        }
    }
}
```

### 表单处理

```rust
fn LoginForm(cx: Scope) -> Element {
    let username = use_state(cx, String::new);
    let password = use_state(cx, String::new);

    let onsubmit = move |e: SubmitEvent| {
        e.prevent_default();
        println!("用户名: {}, 密码: {}", username, password);
    };

    rsx! {
        form { onsubmit,
            input {
                value: "{username}",
                oninput: move |e| username.set(e.value.clone()),
                placeholder: "用户名"
            }
            input {
                value: "{password}",
                oninput: move |e| password.set(e.value.clone()),
                placeholder: "密码",
                r#type: "password"
            }
            button { r#type: "submit", "登录" }
        }
    }
}
```

---

## 常用依赖

### Cargo.toml 配置

```toml
[dependencies]
# Web 框架
ntex = { version = "3", features = ["tls", "json", "websockets"] }
tokio = { version = "1", features = ["full"] }

# 序列化
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "1"

# 数据库
rusqlite = { version = "0.38", features = ["bundled"] }

# 命令行解析
clap = { version = "4", features = ["derive"] }

# Web 管理端
dioxus = { version = "0.7", features = ["ssr"] }
dioxus-ssr = "0.7"

# 错误处理
anyhow = "1"
thiserror = "2"

# 日志
log = "0.4"
env_logger = "0.11"

# 工具库
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
```

---

## 常见模式

### 1. 静态文件服务

```rust
use ntex::web::filesystem::Static;

HttpServer::new(|| {
    App::new()
        .service(Static::new("/", "./static"))
        .route("/api", web::get().to(api_handler))
})
```

### 2. 错误处理

```rust
use ntex::web::{Error, ErrorBadRequest};

async fn handler(json: Json<MyType>) -> Result<impl IntoResponse, Error> {
    if json.is_invalid() {
        return Err(ErrorBadRequest("Invalid data"));
    }
    Ok(Json(response))
}
```

### 3. 中间件

```rust
use ntex::web::dev::{Service, ServiceRequest, ServiceResponse};
use futures_util::future::Ready;

struct LoggingMiddleware;

impl Service for LoggingMiddleware {
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = ntex::dev::BoxError;
    type Future<'f> = Ready<Result<Self::Response, Self::Error>>;

    fn call(&self, req: ServiceRequest) -> Self::Future<'_> {
        println!("Request: {} {}", req.method(), req.path());
        std::future::ok(req.call_next().await?)
    }
}
```

---

## 版本差异说明

### ntex 2.x vs 3.x

- `Server::build().bind("name", addr, || {})` - 3.x 使用相同的 API
- 路由定义方式基本一致
- 3.x 有更严格的异步处理

### Dioxus 0.5 vs 0.7

- `dioxus_ssr` crate 需要单独添加
- 组件签名从 `fn(cx: Scope) -> Element` 保持一致
- 0.7 版本有更现代的 API 和更好的性能
