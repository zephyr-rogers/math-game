# Math Quiz Game

A simple command-line math quiz game written in Rust. The game generates random math questions and checks the user's answers, keeping track of the score.

## Features

- Randomly generated math questions with increasing difficulty based on the score.
- Support for addition, subtraction, multiplication, and division operations.
- Cross-platform screen clearing to enhance user experience.
- Simple error handling for non-numeric inputs.
- Game ends when the user provides an incorrect answer.

## Requirements

- Rust (install from [rust-lang.org](https://www.rust-lang.org/))

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/zephyr-rogers/math-game.git
    ```

2. Navigate to the project directory:

    ```bash
    cd math-game
    ```

3. Build the project:

    ```bash
    cargo build --release
    ```

## Usage

1. Run the game:

    ```bash
    cargo run
    ```

2. Answer the math questions as they appear. The game will clear the screen after each question for a clean interface.

3. The game continues until you answer a question incorrectly. Your final score will be displayed.

4. Press Enter to exit the game.

## How It Works

- The game generates math questions based on the current score, with the difficulty increasing as the score increases.
- It supports four basic math operations: addition (`+`), subtraction (`-`), multiplication (`*`), and division (`/`).
- The game ensures that division questions always have integer results by generating numbers accordingly.
- The terminal screen is cleared at the start of each loop iteration to keep the interface clean.

## Clearing Screen Implementation

- **Windows**: Uses the `cls` command.
- **Unix-based Systems**: Uses the `clear` command.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

If you'd like to contribute to this project, please open an issue or submit a pull request.

## Contact

For any questions or feedback, please reach out to [Zephyr Rogers](mailto:zcrogers2004@gmail.com).
