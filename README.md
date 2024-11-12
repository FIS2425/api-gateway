# Template: Node.js Microservice

## ⚙️ Project Setup

### 1. Clone the Repository ⬇️

To begin, clone the repository using SSH, then install all necessary dependencies by running:

```bash
npm install
```

### 2. Environment Variables 🗝️

Create a `.env` file by duplicating the `.env.example` file provided in the repository. Add your MongoDB credentials and update the `URL` as needed.

### 3. Development 🛠️

For development, use the following command:

```bash
npm run dev
```

This command runs the project with `node --watch`, allowing hot-reloading during development, similar to `nodemon`.

### 4. Production 🚀

For production builds, start the application with:

```bash
npm start
```

### 5. Linting 🧹

To check for code quality using linters, run:

```bash
npm run lint
```

For automatic linting fixes, use:

```bash
npm run lint-fix
```

### 6. Testing 🧪

To run tests, use the following command:

```bash
npm test
```

### 7. Populating the Database 📚

To populate the database with sample data, run:

```bash
npm run populate <FileName>
```

### 8. Logging 📝

To add logs to your application, use the `logger` object provided in the `config/logger.js` file. The logger is configured to write logs to the console and a file in the `logs` directory. There are five log levels available:
- `error`
- `warn`
- `info`
- `http`
- `verbose`
- `debug`

#### Example:
You can log messages adding the following code to your methods:

```javascript
logger.error('This is an error message');
logger.http('Incoming HTTP request', {
  method: 'GET',
  url: '/api/appointments',
  userUid: 'a71b0cbd-7edd-4ae1-919b-403a33fba2eb',
  params: { date: '2024-11-04', status: 'confirmed' }
});
```

## Docker Setup 🐳

To run the application in a Docker container:

1. Modify the `/workdir` setting in the `Dockerfile` to match the corresponding microservice directory.
2. Update the `deploy-docker` workflow to deploy the correct microservice.

Once these changes are made, ensure Docker is installed and running on your system, then build and start the container with:

```bash
docker compose up -d
```

This command launches your deployed Docker image in detached mode.
