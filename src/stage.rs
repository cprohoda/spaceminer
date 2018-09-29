use rand::{Rng, thread_rng};

const X_DIMENSION: usize = 10;
const Y_DIMENSION: usize = 10;
const BORDER_VALUE: u8 = 0;

struct Stage {
    stage: [[u8; X_DIMENSION]; Y_DIMENSION],
    rotation: f32,
    angle: f32,
}

impl Stage {
    pub fn new(generate_stage: &Fn() -> [[u8; X_DIMENSION]; Y_DIMENSION], starting_rotation: f32, starting_angle: f32) -> Stage {
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

#[derive(PartialEq)]
enum LeftRight {
    Left,
    Center,
    Right,
}

#[derive(PartialEq)]
enum DownUp {
    Down,
    Center,
    Up,
}

fn check_stage_border(stage: [[u8; X_DIMENSION]; Y_DIMENSION], x: usize, y: usize, lr: LeftRight, du: DownUp) -> u8 {
    if (lr == LeftRight::Left && x == 0) ||
       (lr == LeftRight::Right && x == X_DIMENSION - 1) ||
       (du == DownUp::Down && y == 0) ||
       (du == DownUp::Up && y == Y_DIMENSION-1) {
        BORDER_VALUE
    } else {
        let x_modified = match lr {
            LeftRight::Left => x-1,
            LeftRight::Center => x,
            LeftRight::Right => x+1,
        };
        let y_modified = match du {
            DownUp::Down => y-1,
            DownUp::Center => y,
            DownUp::Up => y+1,
        };
        stage[x_modified][y_modified]
    }
}

pub fn default_generate_stage() -> [[u8; X_DIMENSION]; Y_DIMENSION] {
    let mut stage = [[127u8; X_DIMENSION]; Y_DIMENSION];
    let mut rng = thread_rng();

    for i in 0..100 {
        for x in 0..X_DIMENSION-1 {
            for y in 0..Y_DIMENSION-1 {
                let center_up = check_stage_border(stage, x, y, LeftRight::Center, DownUp::Up);
                let center_down = check_stage_border(stage, x, y, LeftRight::Center, DownUp::Down);
                let right_center = check_stage_border(stage, x, y, LeftRight::Right, DownUp::Center);
                let right_up = check_stage_border(stage, x, y, LeftRight::Right, DownUp::Up);
                let right_down = check_stage_border(stage, x, y, LeftRight::Right, DownUp::Down);
                let left_center = check_stage_border(stage, x, y, LeftRight::Left, DownUp::Center);
                let left_up = check_stage_border(stage, x, y, LeftRight::Left, DownUp::Up);
                let left_down = check_stage_border(stage, x, y, LeftRight::Left, DownUp::Down);
                let weighted_average = stage[x][y]/4 +
                                       center_up/8 + center_down/8 + right_center/8 + left_center/8 +
                                       right_up/16 + right_down/16 + left_up/16 + left_down/16; // TODO: find a better averaging algorithm re integer division or switch to float

                if stage[x][y]/2 + rng.gen::<u8>()/2 <= weighted_average {
                    if stage[x][y] < 255 {
                        stage[x][y] = stage[x][y] + 1;
                    }
                } else {
                    if stage[x][y] > 0 {
                        stage[x][y] = stage[x][y] - 1;
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
