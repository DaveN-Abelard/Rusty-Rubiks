/*
    [4]
[3] [0] [2] [1]
    [5]
        | C4  C5 | 
        |   rd   |
        | C0  C1 | 

 C4  C0 | C0  C1 | C1  C5 | C5  C4
   bl   |   wh   |   gr   |   yl  
 C7  C2 | C2  C3 | C3  C6 | C6  C7

        | C2  C3 | 
        |   or   | 
        | C7  C6 | 

    wh: White = 0
    yl: Yellow = 1
    gr: Green = 2
    bl: Blue = 3
    rd: Red = 4
    or: Orange = 5
*/

use std::collections::HashMap;

fn main_OBSOLETE() {
    let cube: Vec<Vec<u8>> = vec![

        vec![0; 8], // Corner pieces [wrb, wrg, etc.]
        vec![1; 8], // Corner orientation []

        vec![2; 12], // Edge Pieces
        vec![3; 12] // Edge orientation
    ];
}

fn main() {
    use cubesim::prelude::{Cube, Face, Move, MoveVariant};
    use cubesim::FaceletCube;
    use cubesim::GeoCube;
 
    let cube = FaceletCube::new(3);
    let turned_cube = cube.apply_move(Move::U(MoveVariant::Double));
    println!("{:?}", turned_cube.state());
    
    let geo_cube: GeoCube = GeoCube::new(3);
    let turned_geo_cube: GeoCube = geo_cube.apply_move(Move::U(MoveVariant::Standard));
    println!("{:?}", turned_geo_cube.state())
}

fn printCube(cube: &Vec<Vec<u8>>) {

    let mut scores = HashMap::new();

    scores.insert(String::from("wh"), 0);
    scores.insert(String::from("yl"), 1);
    scores.insert(String::from("gr"), 2);
    scores.insert(String::from("bl"), 3);
    scores.insert(String::from("rd"), 4);
    scores.insert(String::from("or"), 5);

    for i in 0..6
    {
        printFace(cube, i)
    }
}

fn printFace(cube: &Vec<Vec<u8>>, face: u8) {
    /* 
    assumptions: cor. pieces have white as first 4, then yellow in second 4
    face is 0
    */

    

    
}