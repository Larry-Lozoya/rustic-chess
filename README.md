# Rustic Chess

Rustic Chess is an multiplayer chess game built in Rust, leveraging the Bevy game engine for a visually engaging experience. The game aims to provide a robust, responsive, and secure platform for players to enjoy classic chess with friends.

## Features
- **Pass And Play:** Play locally with a friend by sharing a single device.
- **Modern Interface:** Built with Bevy for smooth, high-performance graphics.
- **Intuitive Gameplay:** Features standard chess rules and timers, possibly a leaderboard.
- **Online Multiplayer:** TBD.

## Installation

### Prerequisites
1. Install **Rust**:
   Follow the official guide to install Rust and set up Visual Studio Code:  
   [Rust in Visual Studio Code](https://code.visualstudio.com/docs/languages/rust)

2. Verify Rust installation:  
   ```bash
   rustc --version

### Steps to Run
1. Clone the repository:  
   ```bash
   git clone https://github.com/<your-username>/Rustic-Chess.git
   cd Rustic-Chess

2. Build the project:  
   ```bash
   cargo build
   
3. Run the game:  
   ```bash
   cargo run

## Running the Game
**Once the game is built and running, follow these instructions to start playing:**

#### Local Play (Pass and Play):
- After starting the game with `cargo run`, you'll see the chessboard displayed.
- Player 1 (White) makes the first move by clicking/tapping on a piece and selecting a valid destination square.
- Pass the device to Player 2 (Black) for their turn and the chessboard will flip.
- Continue alternating turns until the game ends (checkmate, stalemate, draw, or flagging).

### Game Controls:
- **Select a Piece:** Click or tap on a piece to select it. Valid moves will be highlighted(TBD).
- **Move a Piece:** Click or tap on a highlighted square to move the selected piece.
- **Undo Move (Optional):** TBD.

## TechStack
- **Programing Language:** Rust
- **Game Engine:** Bevy
- **Serialization/Deserialization:** Serde
- **Dependency Management:** Cargo

## Author Information
- **Larry Lozoya Alvarado**  
  [LinkedIn](https://www.linkedin.com/in/larrylozoyaalvarado/) | [Email](mailto:larrylozoya54@gmail.com)

- **Shane Hargrave**  
  [LinkedIn](https://www.linkedin.com/in/shane-hargrave-311a82286/) | [Email](mailto:shane.hargrave@western.edu)
