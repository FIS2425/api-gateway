# Template: Node.js Microservice

## ⚙️ Project Setup

### 1. Clone the Repository ⬇️

To begin, clone the repository using SSH, then install all necessary dependencies by running:

```bash
cargo update
```

### 2. Environment Variables 🗝️

Create a `config.yaml` file by duplicating the `config.yaml.example` file provided in the repository. Add the services and parameters required for your application to run.

### 3. Development 🛠️

For development, use the following command:

```bash
cargo run
```

### 4. Production 🚀

For production builds, start the application with:

```bash
cargo build --release && ./target/release/api-gateway
```

### 5. Linting 🧹

For automatic linting fixes, use:

```bash
rustfmt <FileName>
```

### 6. Testing 🧪

To run tests, use the following command:

```bash
cargo test
```

### 8. Logging 📝

To add logs to your application, use the `logger` object provided in the `src/config/logger.rs` file. The logger is configured to write logs to the console and a file in the `logs` directory. There are three log levels available:
- `info`
- `warn`
- `err`

#### Example:
You can log messages adding the following code to your methods:

```javascript
logger.info("This is an info message", &[("key1", "val1"), ("key2", "val2")]);
logger.warn("This is an info message", &[("key1", "val1"), ("key2", "val2")]);
logger.err("This is an info message", &[("key1", "val1"), ("key2", "val2")]);
```

## Docker Setup 🐳

To run the application in a Docker container:

1. Set the config.yaml to point to the deployed microservices.

Once these changes are made, ensure Docker is installed and running on your system, then build and start the container with:

```bash
docker compose up --build -d
```

This command launches your deployed Docker image in detached mode.
