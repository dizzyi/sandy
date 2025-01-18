# Sandy
Sandy is an extensible, minimal, 3D visualization engine written in rust on bevy, 
that allow spatial and kinematic visualization.

## Tour Guide
Follow the [tour guide](./manual/tour.md)
to understand basic of Sandy.

## Features
### 3D visualization
Sandy allow easy 3d visualization, with simple objects.

#### After Image
Sandy have buildin after image features,
which allow the engine to visualize the kinematic of objects.

### Extensible 
Sandy allows user configuring environment and controlling 3D objects,
by expose api, allow user to use the lua programming language
to interact with the engine.

#### Hot Reload
Sandy will watch the project file directory recursively.

### Plotter
Sandy have an internal plotter windows,
that allow user to plot value against times.

## Installation
### Releases
Dowload latest releases for your os in [release page](https://github.com/dizzyi/sandy/releases)

### Compile from sources

#### Install rust lang
Go to the [rust install page](https://www.rust-lang.org/tools/install)
and install the `rustup`.

#### Environment Setup
Sandy use the [bevy game engine](https://bevyengine.org/), 
which require extra setup.

[Follow the setup guide](https://bevyengine.org/learn/quick-start/getting-started/setup/)

#### Git Clone Repo
    git clone git@github.com:dizzyi/sandy.git

#### Run Project
    cargo run -p sandy
this gon take a while, go watch a movie or sum


## TODO
- [ ] Gizmo render api
- [ ] Symbolic Engine









