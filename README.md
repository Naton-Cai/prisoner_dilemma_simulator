# Prison Dilemma Simulator

Prison Dilemma Simulator is a game program in Rust, using Fyrox, that simulates the concept of the Prisoner's Dilemma. In this game, you get to simulate a bunch of little creatures called bugsters, who wander around the arena and collide with each other. Depending on the bugster's personality type, they gain and lose resources when they do so.

|                         | **Greedy (Self)** | **Cooperative (Self)** |
| ----------------------- | ----------------- | ---------------------- |
| **Greedy (Other)**      | -1                | -2                     |
| **Cooperative (Other)** | +3                | +2                     |

## Installation

```bash
cargo build
cargo run --bin executor
```

## Notes for Linux

The alsa-sys crate dependency requires the ALSA development libraries and so may not build correctly on Linux, If you also encounter this problem, install the libraries using the commands below.

### Debian/Ubuntu

```bash
sudo apt-get update
sudo apt-get install libasound2-dev pkg-config build-essential
```

### Fedora

```bash
sudo dnf install alsa-lib-devel
```

### Arch Linux

```bash
sudo pacman -S alsa-lib
```

## How to play

When you start the game, you are brought into the start menu. Here you can select the number of bugsters of each personality you wish to spawn. Each bugster starts with 10 health, which changes as seen above. A bugster's size changes depending on the HP. When a bugster drops to 0 HP, it dies.
![alt text](https://github.com/Naton-Cai/Prisoner-Dilemma-Simulator/blob/master/Assests/Screenshots/screenshot1.png "Screenshot of Start Menu")
![alt text](https://github.com/Naton-Cai/Prisoner-Dilemma-Simulator/blob/master/Assests/Screenshots/screenshot2.png "Screenshot of Gameplay")

## Things that didn't work

One of the big things not implemented was proper health displays for each bugster entity. Fyrox 1.0.0-rc.1 changed how rendering UI elements onto textures worke,d and the documentation has not been updated to match. Further experimentation will have to be done to properly implemnt this feature. Additionally, managing different viewport sizes is not that well-documented and has proven to be difficult for the game.

## Things learned

This is the first big project I have developed in Rust, with the addition of developing using a whole new game engine with Fyrox, I found the process a bit difficult. Not surprisingly, since this is the first Rust program where I had to manage the borrow checker, I had initial problems with it. It still takes a bit to understand, but I am sure I will eventually get fully acclimated to it. I had problems managing the fyrox crates. Coming from Python, I assumed all methods for a node in Fyrox were automatically built into the module for that specific node. This is not the case as I learned, many base methods are implented into a base scene node module. This result in a lot of the tutorials in the Fyrox documentation not working, which proved initially frustrating.
