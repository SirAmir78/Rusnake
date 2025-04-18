# Snake Game in Terminal (Rust)

This is a simple implementation of the classic Snake game, built using the Rust programming language and the `crossterm` library for terminal manipulation. The game runs in the terminal, using keyboard input to control the snake's movement and growing when it eats food.

## Features

- **Snake Movement**: The snake moves in four directions (Up, Down, Left, Right) using the arrow keys.
- **Food Generation**: Random food items appear on the screen, and the snake grows when it eats them.
- **Game Over**: The game ends when the snake collides with itself.
- **Terminal-based UI**: The game is rendered directly in the terminal using the `crossterm` crate.

## Prerequisites

Before running the game, you need to have the following installed:

- **Rust**: You can install Rust by following the instructions on the [official website](https://www.rust-lang.org/learn/get-started).
- **Cargo**: Cargo is the Rust package manager, which comes bundled with Rust.

## Installation

1. Clone the repository to your local machine:

   ```bash
   git clone https://github.com/SirAmir78/Rusnake.git
   cd snake-game-rust
