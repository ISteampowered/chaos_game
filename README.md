# chaos_game

simple implementations of [chaos_games](https://en.wikipedia.org/wiki/Chaos_game) and some others fractal ish things.

to compile and run an example open a terminal in the project folder and run `cargo build --example EXAMPLENAME` where EXAMPLENAME is the name of the file without the '.rs' extension. 

navigation:
* scroll: zoom in / zoom out.
* left click + drag: look around.
* right click + drag: translate the view point.
* enter: look at the origin (0.0, 0.0, 0.0).


the examples can be divided into two types: the ones that make use of chaos games and the ones that don't

chaos games: 
* [triangle.rs](examples/triangle.rs): generates a [Sierpinski triangle](https://en.wikipedia.org/wiki/Sierpinski_triangle)
* [triangle2.rs](examples/triangle2.rs): same as triangle but the colour of the points depend on its proximity to the vertices of the triangle
* [tetrahedron.rs](examples/tetrahedron.rs): generates a Sierpinski tetrahedron
* [pentagon.rs](examples/pentagon.rs): generates [this](https://en.wikipedia.org/wiki/Chaos_game#/media/File:Chaos_Game_pentagon-EH-2.png), or at least tries to 
* [pentagon_2.rs](examples/pentagon_2.rs): generates [this](https://en.wikipedia.org/wiki/Chaos_game#/media/File:Chaos_Game_pentagon-EH-1.png) example
* [cube.rs](examples/cube.rs) a small test to see what would happen if there where 8 points in a cube formation
other:
* [sponge.rs](examples/sponge.rs) an attempt to generate a [Menger sponge](https://en.wikipedia.org/wiki/Menger_sponge). Works well enough but starts laggin after too many generations, so do be carefull. Press 'S' to proceed to the next generation.
* [sponge_2.rs](examples/sponge_2.rs) a failed version of sponge.rs where I kept all the cubes that weren't supposed to be generated, still fun to look at. Press 'S' to proceed to the next generation.