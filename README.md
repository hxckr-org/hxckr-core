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

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/extheoisah/hxckr-core.git
   cd hxckr-core
   ```

2. **Run the Application**:

   ```bash
   cargo run
   ```

## Project Structure

```plaintext
hxckr-core/
│
├── src/
│   ├── app/            # Application Layer
│   ├── domain/         # Domain Layer
│   ├── service/        # Infrastructure Layer
│   ├── main.rs         # Entry point
│
├── tests/              # Integration tests
├── Cargo.toml          # Cargo configuration
└── README.md           # Project documentation
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
