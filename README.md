# Term Nim

## Description
A terminal application for playing the game of Nim on a local network.

Made for fun.

## Running
To run the application, run the following command in the root directory of the project:
```
cargo run [--release]
```

## Playing
The UI should be fairly self-explanatory. The game is played by two players, each taking turns removing a number of sticks from any pile. The player who cannot make a move loses.

## Roadmap
- [x] Basic game logic
- [x] UI
  - [x] Main Menu
  - [x] Game options
  - [x] Game state (choice of piles and number of sticks)
  - [x] Player turn
  - [x] Game over
- [X] Local network multiplayer
  - [X] Communication protocol
  - [X] Server
  - [X] Client
- [ ] AI opponent
- [ ] Error handling