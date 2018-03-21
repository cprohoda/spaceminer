use rand::Rng;

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

    pub fn update(&mut self, mined: []) {
        // TODO: add stage rotation
        self.stage[]
    }
}

pub fn generate_stage() -> StageData {
    let dimension = 500;
    let mut stage: StageData::Large = [[127; dimension]; dimension];
    let mut rng = rand::thread_rng();
    let border_value = 0;

    for x in 0..100 {
        for (x, row) in stage.iter().enumerate() {
            for (y, column) in row.iter().enumerate() {
                let center = &mut stage[x][y];
                if x > 0 && x < dimension - 1 && y > 0 && y < dimension - 1 {
                    let north = stage[x-1][y];
                    let notheast = stage[x-1][y+1];
                    let east = stage[x][y+1];
                    let southeast = stage[x+1][y+1];
                    let south = stage[x+1][y];
                    let southwest = stage[x+1][y-1];
                    let west = stage[x][y-1];
                    let northwest = stage[x-1][y-1];
                } else if x > 0 && x < dimension - 1 && y == 0 {
                    let north = stage[x-1][y];
                    let notheast = stage[x-1][y+1];
                    let east = stage[x][y+1];
                    let southeast = stage[x+1][y+1];
                    let south = stage[x+1][y];
                    let southwest = border_value;
                    let west = border_value;
                    let northwest = border_value;
                } else if x > 0 && x < dimension - 1 && y == dimension - 1 {
                    let north = stage[x-1][y];
                    let notheast = border_value;
                    let east = border_value;
                    let southeast = border_value;
                    let south = stage[x+1][y];
                    let southwest = stage[x+1][y-1];
                    let west = stage[x][y-1];
                    let northwest = stage[x-1][y-1];
                } else if x == 0 && y > 0 && y < dimension - 1 {
                    let north = border_value;
                    let notheast = border_value;
                    let east = stage[x][y+1];
                    let southeast = stage[x+1][y+1];
                    let south = stage[x+1][y];
                    let southwest = stage[x+1][y-1];
                    let west = stage[x][y-1];
                    let northwest = border_value;
                } else if x == 0 && y == 0 {
                    let north = border_value;
                    let notheast = border_value;
                    let east = stage[x][y+1];
                    let southeast = stage[x+1][y+1];
                    let south = stage[x+1][y];
                    let southwest = border_value;
                    let west = border_value;
                    let northwest = border_value;
                } else if x == 0 && y == dimension -1 {
                    let north = border_value;
                    let notheast = border_value;
                    let east = border_value;
                    let southeast = border_value;
                    let south = stage[x+1][y];
                    let southwest = stage[x+1][y-1];
                    let west = stage[x][y-1];
                    let northwest = stage[x-1][y-1];
                } else if x == dimension - 1 && y > 0 && y < dimension - 1 {
                    let north = stage[x-1][y];
                    let notheast = stage[x-1][y+1];
                    let east = stage[x][y+1];
                    let southeast = border_value;
                    let south = border_value;
                    let southwest = border_value;
                    let west = stage[x][y-1];
                    let northwest = stage[x-1][y-1];
                } else if x == dimension - 1 && y == 0 {
                    let north = stage[x-1][y];
                    let notheast = stage[x-1][y+1];
                    let east = stage[x][y+1];
                    let southeast = border_value;
                    let south = border_value;
                    let southwest = border_value;
                    let west = border_value;
                    let northwest = border_value;
                } else if x == dimension - 1 && y == dimension - 1 {
                    let north = stage[x-1][y];
                    let notheast = border_value;
                    let east = border_value;
                    let southeast = border_value;
                    let south = border_value;
                    let southwest = border_value;
                    let west = stage[x][y-1];
                    let northwest = stage[x-1][y-1];
                }

                if ( center/2 + rng.gen::<u8>()/2) <= ( center/4 + north/8 + south/8 + east/8 + west/8 + northwest/16 + northeast/16 + southwest/16 + southeast/16 ) { // TODO: find a better averaging algorithm re integer division
                    center += 1;
                } else {
                    center -= 1;
                }
            }
        }
    }

    stage
}

enum StageData {
    Standard([[u8; 100]; 100]),
    Large([[u8; 500]; 500]),
}
