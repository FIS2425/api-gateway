# HyperGate 🚀🚪 - A Rust API Gateway for Microservices

## ⚙️ Project Setup

### 1. Clone the Repository ⬇️

To begin, clone the repository using SSH, then install all necessary dependencies by running:

```bash
cargo update
```

### 2. Environment Variables 🗝️

Create a `config.yaml` file by duplicating the `config.yaml.example` file provided in the repository. Add the services and parameters required for your application to run.

### 3. Development 🛠️

For development, run the following command to start the application:

```bash
cargo run --serve --conf <CONFIG_PATH> --specs <SPEC_PATH> --html <HTML_PATH>
```

This command will run the API Gateway in development mode. Ensure you specify the appropriate paths for your configuration file, OpenAPI spec, and HTML output.

### 4. Production 🚀

For production builds, first build the release version of the application and then run it with the following commands:

```bash
cargo build --release
./target/release/hypergate --serve --conf <CONFIG_PATH> --specs <SPEC_PATH> --html <HTML_PATH>
```

This starts the API Gateway in production mode. Be sure to set up the correct paths for the configuration file, OpenAPI spec, and SwaggerUI HTML.

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

### 7. Logging 📝

To add logs to your application, use the `logger` object provided in the `src/config/logger.rs` file. The logger is configured to write logs to the console and a file in the `logs` directory. There are three log levels available:
- `info`
- `warn`
- `err`

#### Example:
You can log messages by adding the following code to your methods:

```javascript
logger.info("This is an info message", &[("key1", "val1"), ("key2", "val2")]);
logger.warn("This is an info message", &[("key1", "val1"), ("key2", "val2")]);
logger.err("This is an info message", &[("key1", "val1"), ("key2", "val2")]);
```

## Docker Setup 🐳

To run the application in a Docker container:

1. Set the `config.yaml` to point to the deployed microservices.

Once these changes are made, ensure Docker is installed and running on your system, then build and start the container with:

```bash
docker compose up --build -d
```

This command launches your deployed Docker image in detached mode.

---

## 🚀 Commands

The `Api-Gateway` includes two main commands: **merge** and **serve**.

### 1. `merge` — Merge OpenAPI Specs

This subcommand merges OpenAPI specifications into a single HTML output.

#### Usage:

```bash
cargo run -- merge --specs <SPEC_DIR> --output <OUTPUT_PATH>
```

- `--url`: The URL of the API Gateway.
- `--specs`: Directory of OpenAPI specs to merge.
- `--output`: Output path of the merged HTML OpenAPI spec.

#### Example:

```bash
cargo run -- merge --specs "./docs/" --output "./static/openapi.yaml"
```

In this example, the command merges the OpenAPI specs located in the `docs/` directory and generates a merged HTML spec at `static/openapi.html`. The `--url` argument specifies the API Gateway URL.

### 2. `serve` — Serve the API Gateway

This subcommand serves the API Gateway, accepting the configuration file, OpenAPI spec, and SwaggerUI HTML path.

#### Usage:

```bash
cargo run -- serve --conf <CONFIG_PATH> --specs <SPEC_PATH> --html <HTML_PATH>
```

- `--conf`: Path to the configuration file.
- `--specs`: Path to the OpenAPI spec.
- `--html`: Path to the SwaggerUI HTML.

#### Example:

```bash
cargo run -- serve --conf "config.yaml" --specs "./static/openapi.yaml" --html "./static/openapi.html"
```
