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
   $ cargo new --bin solutions/week_{{number}}/{{username}} --vcs none
   ```

   For example:

   ```sh
   $ cargo new --bin solutions/week_1/arthurgousset --vcs none
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
> For context:
>
> 1. the `--bin` flag specifies that the new project is a binary crate (i.e., it contains an
>    executable). Without this flag, Cargo would create a library crate by default.
> 2. the `--vcs none` flag specifies that the new project should not be initialized with version
>    control. We're managing version control at the repository level, so we don't need it at the
>    project level.

## Version Control

To keep the repository clean and organized, I propose the following guidelines:

1. Create a branch for each new week: `{{username}}/week_{{number}}/` (e.g. `arthurgousset/week_1`).
   ```sh
   $ git checkout -b {{username}}/week_{{number}}
   ```
2. Commit your changes to the branch.
3. Open a pull request to merge your branch into `main` when you're ready to submit your solution.
   Merge the PR using
   "[squash and merge](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/incorporating-changes-from-a-pull-request/about-pull-request-merges#squash-and-merge-your-commits)".
   That'll keep the commit history on `main` nice and tidy.

This ensures that each solution is isolated and can be reviewed independently.

<!-- References -->

[tonk-curriculum]:
  https://www.notion.so/tonk/Foundation-for-Applied-Cryptography-0a33951054b84cd68c3e030bed945003
