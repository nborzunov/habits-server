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

## Sutup Database

1. Login to supabase
```npx supabase login```

2. Link the project
```npx supabase link --project-ref $PROJECT_ID```

3. Run docker desktop app

4. Start the database
```npx supabase start```

## Push changes to database

1. Create new migration
```npx supabase migration new $MIGRATION_NAME```

3. Put your changes into created migration file in supabase/migrations folder

4. Apply the new migration to your local database
```npx supabase db up```
Or reset db and apply new migration
```npx supabase db reset```

5. Push changes to the remote database
```npx supabase db push```

6. Update diesel schemas
```diesel print-schema > src/schema.rs```

7. Update autogenerated models
```diesel_ext --model > src/models.rs -s src/schema.rs```

- Note: If you have an issue with utf-8 characters, you can try this command in powershell:
```$PSDefaultParameterValues['Out-File:Encoding'] = 'utf8'```

## Pull changes from database
```npx supabase db pull```

