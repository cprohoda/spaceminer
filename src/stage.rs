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
    let mut stage: StageData::Large = [[127; 500]; 500];
    let mut rng = rand::thread_rng();

    for x in 1..100 {
        for (x, row) in stage.iter().enumerate() {
            for (y, column) in row.iter().enumerate {

                }
            }
        }
    }
}



enum StageData {
    Standard([[u8; 100]; 100]),
    Large([[u8; 500]; 500]),
}
