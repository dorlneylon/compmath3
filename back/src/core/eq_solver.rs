use std::cell::Cell;

pub enum Method {
    LeftRect,
    RightRect,
    MidRect,
    Trap,
    Simpson,
}

pub struct Solver {
    pub task: fn(f32) -> f32,
    pub method: Method,
    pub n: Cell<usize>,
    pub eps: f32,
    pub lb: f32,
    pub rb: f32,
    pub acc: Cell<f32>,
    pub errors: Cell<String>,
    pub dots: Vec<f32>,
}

impl Solver {
    pub fn New(
        task: fn(f32) -> f32,
        method: Method,
        n: usize,
        eps: f32,
        lb: f32,
        rb: f32,
        dots: Vec<f32>,
    ) -> Solver {
        Solver {
            task,
            method,
            n: Cell::new(n),
            eps,
            lb,
            rb,
            acc: Cell::new(0.0),
            errors: Cell::new(String::new()),
            dots,
        }
    }

    pub fn solve(&self) -> f32 {
        let mut sum = 0.0;
        self.check_convergence();
        for i in 0..self.dots.len() - 1 {
            println!("{} {} {}", self.dots[i], self.dots[i + 1], sum);
            sum += match self.method {
                Method::LeftRect => self.rect(0.0, self.dots[i], self.dots[i + 1] - 1e-8),
                Method::RightRect => self.rect(1.0, self.dots[i], self.dots[i + 1] - 1e-8),
                Method::MidRect => self.rect(0.5, self.dots[i], self.dots[i + 1] - 1e-8),
                Method::Trap => self.trap(self.dots[i], self.dots[i + 1] - 1e-8),
                Method::Simpson => self.simpson(self.dots[i], self.dots[i + 1] - 1e-8),
            };
        }
        sum
    }

    fn rect(&self, c: f32, lb: f32, rb: f32) -> f32 {
        let h = ((rb - lb) * (self.dots.len() as f32)) / (self.n.get() as f32);
        let mut sum = 0.0;

        for i in 0..(self.n.get() as i32) {
            sum += self.f(lb + (i as f32 + c) * h) * h;
        }

        sum
    }

    fn trap(&self, lb: f32, rb: f32) -> f32 {
        let h = ((rb - lb) * (self.dots.len() as f32)) / (self.n.get() as f32);
        let mut sum = (self.f(lb) + self.f(rb)) / 2.0;

        for i in 1..((self.n.get() as i32) / (self.dots.len() as i32)) {
            sum += self.f(lb + i as f32 * h);
        }

        sum * h
    }

    fn simpson(&self, lb: f32, rb: f32) -> f32 {
        let h = ((rb - lb) * (self.dots.len() as f32)) / (self.n.get() as f32);
        let mut sum = 0.0;
        let mut x = lb;

        for i in 0..((self.n.get() as i32) / (self.dots.len() as i32)) - 1 {
            let k1 = self.f(x);
            let k2 = self.f(x + h);
            let simpson = (k1 + 4.0 * k2 + self.f(x + h / 2.0)) * h / 6.0;
            sum += simpson;
            x += h;
        }

        sum
    }

    fn f(&self, x: f32) -> f32 {
        ((self.task)(x + 1e-2) - (self.task)(x)) / 1e-2
    }

    fn check_convergence(&self) -> bool {
        let mut x = self.lb;

        while x < self.rb {
            println!("{}", ((self.task)(x) - (self.task)(x + 1e-8)).abs());
            if ((self.task)(x) - (self.task)(x + 1e-8)).is_nan()
                || ((self.task)(x) - (self.task)(x + 1e-8)).abs() > 1e-2
            {
                self.errors.set("Does not converge".to_string());
                return false;
            }

            x += 1e-2;
        }

        true
    }
}
