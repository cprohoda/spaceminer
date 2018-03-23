use rand::{Rng, thread_rng};

struct Stage {
    stage: StageData,
    rotation: f32,
    angle: f32,
}

impl Stage {
    pub fn new(generate_stage: &Fn() -> StageData, starting_rotation: f32, starting_angle: f32) -> Stage {
        Stage {
            stage: generate_stage(),
            rotation: starting_rotation,
            angle: starting_angle,
        }
    }

    fn rotate(&mut self) {
        // TODO: add stage rotation
    }

    pub fn update(&mut self, mined: Vec<usize>) {
        // TODO: add stage rotation
    }
}

pub fn generate_stage() -> [[u8; 10]; 10] {
    const dimension: usize = 10usize;
    let mut stage = [[127u8; dimension]; dimension];
    let mut rng = thread_rng();
    let border_value = 0;
    let mut center = &mut 127u8;
    let mut north = 127u8; 
    let mut northeast = 127u8;
    let mut east = 127u8;
    let mut southeast = 127u8;
    let mut south = 127u8;
    let mut southwest = 127u8;
    let mut west = 127u8;
    let mut northwest = 127u8;

    for x in 0..100 {
        for (x, row) in stage.iter().enumerate() {
            for (y, column) in row.iter().enumerate() {
                center = &mut stage[x][y];
                if x > 0 && x < dimension - 1 && y > 0 && y < dimension - 1 {
                    north = stage[x-1][y];
                    northeast = stage[x-1][y+1];
                    east = stage[x][y+1];
                    southeast = stage[x+1][y+1];
                    south = stage[x+1][y];
                    southwest = stage[x+1][y-1];
                    west = stage[x][y-1];
                    northwest = stage[x-1][y-1];
                } else if x > 0 && x < dimension - 1 && y == 0 {
                    north = stage[x-1][y];
                    northeast = stage[x-1][y+1];
                    east = stage[x][y+1];
                    southeast = stage[x+1][y+1];
                    south = stage[x+1][y];
                    southwest = border_value;
                    west = border_value;
                    northwest = border_value;
                } else if x > 0 && x < dimension - 1 && y == dimension - 1 {
                    north = stage[x-1][y];
                    northeast = border_value;
                    east = border_value;
                    southeast = border_value;
                    south = stage[x+1][y];
                    southwest = stage[x+1][y-1];
                    west = stage[x][y-1];
                    northwest = stage[x-1][y-1];
                } else if x == 0 && y > 0 && y < dimension - 1 {
                    north = border_value;
                    northeast = border_value;
                    east = stage[x][y+1];
                    southeast = stage[x+1][y+1];
                    south = stage[x+1][y];
                    southwest = stage[x+1][y-1];
                    west = stage[x][y-1];
                    northwest = border_value;
                } else if x == 0 && y == 0 {
                    north = border_value;
                    northeast = border_value;
                    east = stage[x][y+1];
                    southeast = stage[x+1][y+1];
                    south = stage[x+1][y];
                    southwest = border_value;
                    west = border_value;
                    northwest = border_value;
                } else if x == 0 && y == dimension -1 {
                    north = border_value;
                    northeast = border_value;
                    east = border_value;
                    southeast = border_value;
                    south = stage[x+1][y];
                    southwest = stage[x+1][y-1];
                    west = stage[x][y-1];
                    northwest = border_value;
                } else if x == dimension - 1 && y > 0 && y < dimension - 1 {
                    north = stage[x-1][y];
                    northeast = stage[x-1][y+1];
                    east = stage[x][y+1];
                    southeast = border_value;
                    south = border_value;
                    southwest = border_value;
                    west = stage[x][y-1];
                    northwest = stage[x-1][y-1];
                } else if x == dimension - 1 && y == 0 {
                    north = stage[x-1][y];
                    northeast = stage[x-1][y+1];
                    east = stage[x][y+1];
                    southeast = border_value;
                    south = border_value;
                    southwest = border_value;
                    west = border_value;
                    northwest = border_value;
                } else if x == dimension - 1 && y == dimension - 1 {
                    north = stage[x-1][y];
                    northeast = border_value;
                    east = border_value;
                    southeast = border_value;
                    south = border_value;
                    southwest = border_value;
                    west = stage[x][y-1];
                    northwest = stage[x-1][y-1];
                }

                if ( *center/2 + rng.gen::<u8>()/2) <= ( *center/4 + north/8 + south/8 + east/8 + west/8 + northwest/16 + northeast/16 + southwest/16 + southeast/16 ) { // TODO: find a better averaging algorithm re integer division
                    if center < &mut 255 {
                        *center = *center + 1;
                    }
                } else {
                    if center > &mut 0 {
                        *center = *center - 1;
                    }
                }
            }
        }
        println!("Stage generator iteration");
        for x in stage.iter() {
            println!("{:?}", x);
        }
    }

    stage
}

enum StageData {
    Standard([[u8; 100]; 100]),
    Large([[u8; 500]; 500]),
}
