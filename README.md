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
The UI should be fairly self-explanatory. The game is played by two players, each taking turns removing a number of sticks from any pile. The player who removes the last stick loses.

## Roadmap
- [ ] Basic game logic
- [ ] UI
  - [ ] Game options
  - [ ] Game state (choice of piles and number of sticks)
  - [ ] Player turn
  - [ ] Game over
- [ ] Local network multiplayer
  - [ ] Communication protocol
  - [ ] Server
  - [ ] Client
- [ ] AI opponent