# Prison Dilemna Simulator

Prison Dilemna Simulator is a game program in Rust, using Fyrox, that simulates the concept of the Prisoner's Dilemna. In this game, you get to simulate a bunch of little creatures called bugsters, who wander around the arena and collide with each other. Depending on the bugster's personality type, they gain and lose resources when they do so.

|                         | **Greedy (Self)** | **Cooperative (Self)** |
| ----------------------- | ----------------- | ---------------------- |
| **Greedy (Other)**      | -1                | -2                     |
| **Cooperative (Other)** | +3                | +2                     |

## Installation

```bash
cargo build
cargo run --bin executor
```

## How to play

When you start the game you are brought into the start menu. Here you can select the number of bugsters of each personality you wish to spawn. Each bugster starts with 10 health which changes as seen above. When a bugster drops to 0 HP it dies.
![alt text](https://github.com/Naton-Cai/Prisoner-Dilemma-Simulator/blob/master/Assests/Screenshots/screenshot1.png "Screenshot of Start Menu")
![alt text](https://github.com/Naton-Cai/Prisoner-Dilemma-Simulator/blob/master/Assests/Screenshots/screenshot2.png "Screenshot of Gameplay")

## Things that didn't work

One of the big things not implemented was proper health displays for each bugster entity. Fyrox 1.0.0-rc.1 change how rendering UI elements onto textures worked and the documentation has not been updated to match. Further experimentation will have to be done to properly implemnt this feature.

## Things learned

This is the first big project I have developed in Rust, with the addition of developing using a whole new game engine with Fyrox, I found the process a bit difficult. Not suprisingly, since this is the first rust program where I had to manage it, I had inital problems with the borrow checker, it still takea bit to understand but I am sure I will eventually get full acclimated to it. I had problems managing the fyrox crates. Coming from Python, I assumed all methods for a node in Fyrox were automatically built into the module for that specific node. This is not the case as I learned, many base methods are implented into a base scene node module. This result in a lot of the tutorials in the Fyrox documentation not working which proved initally frustrating.
