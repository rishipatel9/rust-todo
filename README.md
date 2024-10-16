# Todo CLI

A simple command-line interface (CLI) application written in Rust to manage your to-do tasks. This application allows you to add, list, and mark tasks as completed, with the data stored in a JSON file.

## Features

- Add new tasks to your to-do list.
- List all tasks, showing their status (completed or not).
- Mark tasks as completed.

## Getting Started

### Prerequisites

To run this project, you'll need to have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)

### Installation

1. Clone the repository:
   ```bash
   https://github.com/rishipatel9/rust-todo.git
   cd todo-cli
    ```


2. Build the Project

   ```bash
   cargo build
   ```

3. Run the project:

   ```bash
    cargo run --[Command]
   ```

## Usage

Adding a task

1. Add a task:

   ```bash
    cargo run -- add "Learn rust"
   ```

Listing todos

2. List todos:

   ```bash
    cargo run -- list
   ```

Marking todos

3. Marking todo as completed:

    ```bash
    cargo run -- completed 1
    ```

4. Example

    ```bash
    cargo run -- add "Learn go"

    cargo run -- list

    cargo run -- completed 1
    ```
5. Example of todo.json

    ```bash
    [
        {
            "id": 1,
            "title": "Finish Rust project",
            "completed": false
        },
        {
            "id": 2,
            "title": "Write README",
            "completed": true
        }
    ]
    ```