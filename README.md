![Schola Cover](schola-cover.png)

# Schola

Welcome to Schola, the place where we share solutions to the [Tonk Curriculum][tonk-curriculum].

## Setup

Requirements:

1. Rust ([installation instructions](https://www.rust-lang.org/tools/install)).
1. Access to the [Tonk Curriculum][tonk-curriculum].

## Directory Structure

For now, solutions are organized by [week][tonk-curriculum] and username, but we can change this
structure in the future if needed:

```
week_{{number}}/{{username}}/src/main.rs
```

Each solution is an independent Rust project, complete with its own `Cargo.toml` file that manages
dependencies and configurations.

```
solutions/
└── week_1/
    ├── goblinoats/
    │   └── src/
    │       └── main.rs
    ├── jackddouglas/
    │   └── ...
    └── arthurgousset/
        └── ...
```

## Usage

To add your solution for a new week:

1. Initialize a project for the specific week and your username:

   ```sh
   $ cargo new --bin solutions/week_{{number}}/{{username}}
   ```

   For example:

   ```sh
   $ cargo new --bin solutions/week_1/arthurgousset
   ```

1. Write your solution code inside the generated `src/main.rs` file
   ```sh
   $ cd solutions/week_{{number}}/{{username}}
   ```
1. Build the project:
   ```sh
   $ cargo build
   ```
1. Run the project:
   ```sh
   $ cargo run
   ```

> [!NOTE]  
> For context: The `--bin` flag specifies that the new project is a binary crate (i.e., it contains
> an executable). Without this flag, Cargo would create a library crate by default.

<!-- References -->

[tonk-curriculum]:
  https://www.notion.so/tonk/Foundation-for-Applied-Cryptography-0a33951054b84cd68c3e030bed945003
