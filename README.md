# Habits

This project is a web application built with Rust using the Actix framework. It leverages various libraries for web development, authentication, and database interaction.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Running the Application](#running-the-application)
- [Development Mode](#development-mode)
- [Contributing](#contributing)
- [License](#license)

## Prerequisites

Ensure you have the following installed on your system:

- Rust (stable version)
- Cargo (Rust's package manager)
- Git (for cloning the repository)

## Installation

1. **Navigate to the Project Directory**

   Change your current directory to the project directory:

   ```cd server```

2. **Install Dependencies**

   Run the following command to install the project dependencies:

   ```cargo build```

   This command will download and compile all the dependencies listed in `Cargo.toml`.

    ## Running the Application

    To run the application, use the following command:

    ```cargo run```

    This command will compile and run your application. By default, the application will start on `http://localhost:8080`.

## Development Mode

To run the application in development mode, use the following command:

```cargo watch -x run```