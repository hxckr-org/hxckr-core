# HXCKR

HXCKR is a modular Learning Management System (LMS) designed to facilitate bitcoin technical education through structured challenges and exercises. The motivation behind this project is to provide a scalable and flexible platform for learning technical concepts through hands-on experience.

## Project Structure

The project is organized into three main layers:

- **Domain Layer**: Core business logic, including entities, value objects, and domain services. This layer is independent of external frameworks and services.
- **Application Layer**: Orchestrates use cases by coordinating between the domain layer and external services. It defines the flow of data but does not contain business logic.
- **Infrastructure Layer**: Implements integrations with external systems, such as databases, APIs, and web frameworks. This layer contains the adapters that interact with the outside world.

## Features

- **Challenges and Exercises**: Users can engage in structured challenges that consist of multiple exercises or tasks, each designed to teach specific technical concepts.
- **Submissions and Validation**: The system supports user submissions, which are validated against predefined test cases using server-side test runners.
- **Progress Tracking**: Users' progress is tracked across challenges, with achievements and leaderboard rankings displayed for motivation.
- **Extensible Design**: The system is designed to be modular and easily extensible, with clear separation of concerns.
- **Self-Deployable**: The goal is to ensure that the infra can be replicated and used by bitcoin education platforms while managing the instance themselves.

## Getting Started

### Prerequisites

- Rust and Cargo are required to build and run the application (this is provided via Nix). You can also install them globally using [rustup](https://rustup.rs/).
- PostgreSQL is used as the database for this project (this is provided via docker compose using Nix flakes).
- Nix flake is used to manage the dependencies in the development environment. You can install it following the instructions [here](https://nixos.org/download.html).
- Docker is used to manage the database. You can install it following the instructions [here](https://docs.docker.com/get-docker/).
- Diesel CLI is used to manage the database schema (this is provided via Nix flakes).

1 **Clone the Repository**:

   ```bash
   git clone https://github.com/extheoisah/hxckr-core.git
   cd hxckr-core
   ```

2.**Setup the Development Environment**:
> [!IMPORTANT]
> Before running the following command, make sure you have nix-package manager installed on your system. If not, you can install it following the instructions provided [here](https://nixos.org/download.html).
> You should configure your nix to use flake by setting your ~/.config/nix/nix.conf file to use flake as follows: `experimental-features = nix-command flakes`

then run the following command to install the dependencies and setup the development environment in the project directory:

   ```bash
    nix develop
   ```

Alternatively, you can run the following command to install the dependencies and setup the development environment:

   ```bash
   nix develop --experimental-features 'nix-command flakes'
   ```

3.**Run the Application**:
If you are running the application for the first time, please follow the instructions [here](CONTRIBUTING.md) to setup the database. Then run the following command to start the application:

   ```bash
   cargo run
   ```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request. For more details, please refer to the [CONTRIBUTING.md](CONTRIBUTING.md) file.

## Docker

### Building the Docker Image

To build the Docker image locally:

1. Ensure you have Docker installed on your machine.
2. Navigate to the project root directory in your terminal.
3. Build the Docker image:
   ```
   docker build -t hxckr-core:local .
   ```

### Running the Docker Container

To run the Docker container:

```
docker run -p
```

## Publishing Docker Images

This project uses GitHub Actions to automatically publish Docker images to DockerHub. If you fork this repository, you'll need to set up the publishing process:

1. Create a DockerHub account if you don't have one.
2. Create a new repository on DockerHub for your images.
3. In your GitHub repository settings, add the following secrets:
   - `DOCKERHUB_USERNAME`: Your DockerHub username
   - `DOCKERHUB_TOKEN`: A DockerHub access token (create this in your DockerHub account settings)
4. Update the GitHub Actions workflow file (`.github/workflows/docker-publish.yml`) with your DockerHub repository name.

The workflow will build and push a new Docker image on each push to the main branch and when a new release is created.
