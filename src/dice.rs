use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dice {
    pub face: i16,
    pub count: i16,
    pub bp: i8,
    pub kq: i8,
    pub result: Option<i32>,
}

impl Dice {
    pub fn roll(&mut self) {
        let mut result = 0i32;
        let mut rng = rand::thread_rng();
        for _ in 0..self.count {
            result += rng.gen_range(1..self.face) as i32;
        }
        self.result = Some(result);
    }

    pub fn roll_and_get(&mut self) -> i32 {
        self.roll();
        self.result.unwrap()
    }

    pub fn expr(&self) -> String {
        format!("{}D{}", self.count, self.face)
        //ToDo: bp, kq
    }
}
