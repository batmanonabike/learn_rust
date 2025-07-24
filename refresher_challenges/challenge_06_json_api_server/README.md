# Challenge 6: JSON API Server

**Difficulty:** 🔴 Advanced  
**Topics:** Web APIs, JSON, Database, Authentication, Testing, Error Handling

## The Challenge

Build a complete REST API server for a task management system with user authentication, data persistence, and comprehensive error handling. This challenge combines many concepts: HTTP servers, JSON serialization, database operations, and API design.

## Requirements

Create a REST API that supports:

1. **User management** (register, login, profile management)
2. **Task CRUD operations** (Create, Read, Update, Delete)
3. **Authentication** with JWT tokens
4. **Data persistence** with SQLite
5. **Comprehensive error handling** with proper HTTP status codes
6. **API documentation** with OpenAPI/Swagger
7. **Testing** with integration tests

### Dependencies

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
axum = "0.7"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
jsonwebtoken = "9.0"
bcrypt = "0.15"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }
tracing = "0.1"
tracing-subscriber = "0.3"
anyhow = "1.0"
thiserror = "1.0"

[dev-dependencies]
reqwest = { version = "0.11", features = ["json"] }
```

### API Endpoints

```
Authentication:
POST   /api/auth/register    - Register new user
POST   /api/auth/login       - Login user
GET    /api/auth/me          - Get current user profile

Tasks:
GET    /api/tasks            - List user's tasks (with pagination)
POST   /api/tasks            - Create new task
GET    /api/tasks/:id        - Get specific task
PUT    /api/tasks/:id        - Update task
DELETE /api/tasks/:id        - Delete task
PATCH  /api/tasks/:id/status - Update task status
```

### Data Models

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct User {
    id: Uuid,
    username: String,
    email: String,
    #[serde(skip_serializing)]
    password_hash: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct Task {
    id: Uuid,
    user_id: Uuid,
    title: String,
    description: Option<String>,
    status: TaskStatus,
    priority: TaskPriority,
    due_date: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "task_priority", rename_all = "lowercase")]
enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}
```

### Request/Response DTOs

```rust
#[derive(Debug, Deserialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct AuthResponse {
    token: String,
    user: UserResponse,
}

#[derive(Debug, Serialize)]
struct UserResponse {
    id: Uuid,
    username: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct CreateTaskRequest {
    title: String,
    description: Option<String>,
    priority: TaskPriority,
    due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
struct UpdateTaskRequest {
    title: Option<String>,
    description: Option<String>,
    priority: Option<TaskPriority>,
    due_date: Option<DateTime<Utc>>,
}
```

### Core Application Structure

```rust
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete, patch},
    Router,
};

#[derive(Clone)]
struct AppState {
    db: sqlx::SqlitePool,
    jwt_secret: String,
}

#[derive(Debug, thiserror::Error)]
enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Authentication failed")]
    Unauthorized,
    
    #[error("Not found")]
    NotFound,
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Internal server error")]
    Internal,
}
```

### Example Implementation

```rust
async fn create_task(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(request): Json<CreateTaskRequest>,
) -> Result<Json<Task>, ApiError> {
    let task_id = Uuid::new_v4();
    let now = Utc::now();
    
    let task = sqlx::query_as::<_, Task>(
        r#"
        INSERT INTO tasks (id, user_id, title, description, status, priority, due_date, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
    )
    .bind(task_id)
    .bind(user.id)
    .bind(&request.title)
    .bind(&request.description)
    .bind(TaskStatus::Todo)
    .bind(&request.priority)
    .bind(&request.due_date)
    .bind(now)
    .bind(now)
    .fetch_one(&state.db)
    .await?;
    
    Ok(Json(task))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::init();
    
    let database_url = "sqlite:tasks.db";
    let pool = sqlx::SqlitePool::connect(database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    let state = AppState {
        db: pool,
        jwt_secret: "your-secret-key".to_string(),
    };
    
    let app = Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/me", get(get_current_user))
        .route("/api/tasks", get(list_tasks).post(create_task))
        .route("/api/tasks/:id", get(get_task).put(update_task).delete(delete_task))
        .route("/api/tasks/:id/status", patch(update_task_status))
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::cors::CorsLayer::permissive());
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://localhost:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

## Learning Objectives

- Build complete REST APIs with Axum
- Implement JWT authentication and authorization
- Handle database operations with SQLx
- Create proper error handling and HTTP status codes
- Write comprehensive API tests
- Understand middleware and request/response flow

## Advanced Features

1. **Pagination and filtering:**
   ```rust
   #[derive(Debug, Deserialize)]
   struct TaskQuery {
       page: Option<u32>,
       limit: Option<u32>,
       status: Option<TaskStatus>,
       priority: Option<TaskPriority>,
   }
   ```

2. **API versioning:**
   ```rust
   let v1 = Router::new()
       .route("/tasks", get(v1::list_tasks))
       .route("/tasks/:id", get(v1::get_task));
       
   let v2 = Router::new()
       .route("/tasks", get(v2::list_tasks))
       .route("/tasks/:id", get(v2::get_task));
   
   Router::new()
       .nest("/api/v1", v1)
       .nest("/api/v2", v2)
   ```

3. **Rate limiting:**
   ```rust
   use tower::limit::RateLimitLayer;
   
   app.layer(RateLimitLayer::new(100, Duration::from_secs(60)))
   ```

## Database Schema

Create migrations in `migrations/` folder:

```sql
-- migrations/001_initial.sql
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id),
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL CHECK (status IN ('todo', 'in_progress', 'done')),
    priority TEXT NOT NULL CHECK (priority IN ('low', 'medium', 'high', 'urgent')),
    due_date TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_tasks_user_id ON tasks(user_id);
CREATE INDEX idx_tasks_status ON tasks(status);
CREATE INDEX idx_tasks_due_date ON tasks(due_date);
```

## Hints

<details>
<summary>Click to see hints</summary>

1. **JWT implementation:**
   ```rust
   #[derive(Debug, Serialize, Deserialize)]
   struct Claims {
       sub: String, // User ID
       exp: usize,  // Expiration time
   }
   ```

2. **Authentication middleware:**
   ```rust
   async fn auth_middleware(
       State(state): State<AppState>,
       request: Request,
       next: Next,
   ) -> Result<Response, ApiError> {
       // Extract and validate JWT token
   }
   ```

3. **Error handling:**
   ```rust
   impl IntoResponse for ApiError {
       fn into_response(self) -> Response {
           let (status, error_message) = match self {
               ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
               ApiError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
               // ... other cases
           };
           
           (status, Json(json!({"error": error_message}))).into_response()
       }
   }
   ```

</details>

## Testing

Create integration tests:

```rust
#[tokio::test]
async fn test_create_task() {
    let app = create_test_app().await;
    
    let user = create_test_user(&app).await;
    let token = login_user(&app, &user).await;
    
    let task_request = CreateTaskRequest {
        title: "Test Task".to_string(),
        description: Some("Test Description".to_string()),
        priority: TaskPriority::Medium,
        due_date: None,
    };
    
    let response = app
        .post("/api/tasks")
        .header("Authorization", format!("Bearer {}", token))
        .json(&task_request)
        .send()
        .await;
    
    assert_eq!(response.status(), 201);
    
    let task: Task = response.json().await;
    assert_eq!(task.title, "Test Task");
}
```

## Bonus Challenges

1. **WebSocket notifications** for real-time task updates
2. **File attachments** for tasks with file upload
3. **Team collaboration** with shared tasks
4. **API documentation** with Swagger/OpenAPI
5. **Docker deployment** with multi-stage builds
6. **Metrics and monitoring** with Prometheus

## Next Steps

Congratulations! You've completed the advanced challenges. These projects demonstrate mastery of Rust's key concepts. Consider exploring:

- Embedded Rust programming
- WebAssembly with Rust
- Game development with Bevy
- Blockchain development
- Contributing to open-source Rust projects
