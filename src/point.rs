#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn move_to(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}

pub struct S {
    a: String,
}

impl S {
    fn get_non_empty_a(&self) -> Option<&str> {
        if self.a.is_empty() {
            None
        } else {
            Some(&self.a)
        }
    }

    pub fn get_a_or_inset_new(&mut self, input_s: &str) -> Option<&str> {
        // ↓这里借用了&self，&mut self生命周期结束
        let may_empty_a = self.get_non_empty_a();

        if None == may_empty_a {
            // 这里借用了&mut self
            self.a = input_s.to_string(); 
        } 

        self.get_non_empty_a()

        // if let Some(a) = may_empty_a {
        //     Some(a)
        // } else {
        //     // 但这里又要借用&mut self
        //     self.a = input_s.to_string(); // ! 报错：cannot assign to `self.a` because it is borrowed
        //     Some(&self.a)
        // }
    }
}