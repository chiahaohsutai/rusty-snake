# Snake :video_game: :snake:

Implementation of the game Snake built using `Rust`'s `macroquad` crate.

<img width="350" alt="Screenshot 2024-09-21 at 6 37 28 PM" src="https://github.com/user-attachments/assets/2433b10a-d892-432b-a7ca-4fb01e4452ef">

## Installation

You need `Rust` intalled in your system with the `Cargo` package manager.
* Clone the repository: `git clone <repository-url>`.
* Change directory to the project root: `cd rusty-snake`.
* In your terminal under the project root run: `cargo run`.
* Enjoy!

## Rules and Features

Movement:
* The snake moves continuously in one direction, and the player can change the direction using arrow keys (up, down, left, right) or WASD.
* The snake moves one step at a time in the direction specified by the player.
* The snake cannot move backwards directly (e.g., if it's moving right, it can't suddenly move left).

Eating Food:
* Food randomly appears on the game screen represented as a golden dot.
* When the snake moves over the food, it "eats" it, causing the snake to grow longer.
* The more food the snake eats, the longer it becomes.
* After consuming food, new food appears at a different location on the screen.

Avoiding Collisions:
* The game ends if the snake runs into its own b (this version has no walls).
* If you reach the edge of the screen the snake reappears on the opposite edge.

Speed Increase:
* Each time you eact food the game speed increases (but its capped at a maximum speed).

## Controls

* `Q` for quitting the game.
* `Esc` for quitting the GUI (closing the application).
* `WASD` or arrow keys for moving the snake.
* `Enter` to start the game.
