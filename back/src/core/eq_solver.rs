use std::cell::Cell;

pub enum Task {
    Eq(fn(f32) -> f32),
    Sys((fn(f32, f32) -> f32, fn(f32, f32) -> f32)),
}

pub enum Method {
    Chords,
    Secants,
    SimpleIt,
}

pub struct Solver {
    pub task: Task,
    pub method: Method,
    pub n: usize,
    pub eps: f32,
    pub lb: f32,
    pub rb: f32,
    pub acc: Cell<Vec<f32>>,
    pub iters: Cell<usize>,
    pub errors: Cell<String>,
}

impl Solver {
    pub fn new(task: Task, method: Method, n: usize, eps: f32, lb: f32, rb: f32) -> Solver {
        Solver {
            task,
            method,
            n,
            eps,
            lb,
            rb,
            acc: Cell::new(vec![]),
            iters: Cell::new(0),
            errors: Cell::new(String::new()),
        }
    }

    pub fn solve(&self) -> (f32, f32) {
        match &self.task {
            Task::Eq(f) => self.solve_eq(f),
            Task::Sys((f, g)) => self.simple_it_sys(f, g),
        }
    }

    fn solve_eq(&self, f: &fn(f32) -> f32) -> (f32, f32) {
        match &self.method {
            Method::Chords => self.chords(f),
            Method::Secants => self.secants(f),
            Method::SimpleIt => self.simple_it(f),
        }
    }

    fn secants(&self, f: &fn(f32) -> f32) -> (f32, f32) {
        let mut xn = self.rb;
        let mut xl = self.lb;

        for i in 0..self.n {
            let xm = xn - ((xn - xl) * f(xn)) / (f(xn) - f(xl));
            if (xm - xn).abs() <= self.eps {
                self.acc.set(vec![xm - xn]);
                self.iters.set(i + 1);
                return (xm, f(xm));
            }
            xl = xn;
            xn = xm;
        }

        self.acc.set(vec![xn - xl]);
        self.iters.set(self.n);
        (xn, f(xn))
    }

    fn chords(&self, f: &fn(f32) -> f32) -> (f32, f32) {
        let mut xn = self.rb;
        let mut xl = self.lb;

        for i in 0..self.n {
            let xm = xn - ((self.lb - xn) * f(xn)) / (f(self.lb) - f(xn));
            if (xm - xn).abs() <= self.eps {
                self.acc.set(vec![xm - xn]);
                self.iters.set(i + 1);
                return (xm, f(xm));
            }
            xl = xn;
            xn = xm;
        }

        self.acc.set(vec![xn - xl]);
        self.iters.set(self.n);
        (xn, f(xn))
    }

    fn simple_it(&self, f: &fn(f32) -> f32) -> (f32, f32) {
        self.simple_it_conv();
        let mut xn = self.rb;
        let mut xl = self.lb;

        for i in 0..self.n {
            xn = f(xn);
            if (xn - xl).abs() <= self.eps {
                self.acc.set(vec![xn - xl]);
                self.iters.set(i + 1);
                return (xn, f(xn));
            }
            xl = xn;
        }

        self.acc.set(vec![xn - xl]);
        self.iters.set(self.n);
        (xn, f(xn))
    }

    fn simple_it_sys(&self, f: &fn(f32, f32) -> f32, g: &fn(f32, f32) -> f32) -> (f32, f32) {
        self.simple_it_conv();

        let mut xn = self.lb;
        let mut xl = self.lb;
        let mut yn = self.rb;
        let mut yl = self.rb;

        for i in 0..self.n {
            xn = f(xn, yn);
            yn = g(xn, yn);
            if (xn - xl).abs() <= self.eps && (yn - yl).abs() <= self.eps {
                self.acc.set(vec![xn - xl, yn - yl]);
                self.iters.set(i + 1);
                return (xn, yn);
            }
            xl = xn;
            yl = yn;
        }

        self.acc.set(vec![xn - xl, yn - yl]);
        self.iters.set(self.n);
        (xn, yn)
    }

    fn simple_it_conv(&self) {
        let mut q = 0.0;
        match &self.task {
            Task::Eq(f) => {
                let mut x = self.lb;
                let df = |x| (f(x + self.eps) - f(x)) / self.eps;

                while x < self.rb {
                    if (df(x) - q) > self.eps {
                        q = df(x);
                    }
                    x += self.eps;
                }
            }
            Task::Sys((f, g)) => {
                let mut x = self.lb - self.eps;
                let df = |x| (f(x + self.eps, self.rb) - f(x, self.rb)) / self.eps;
                let dg = |x| (g(x + self.eps, self.rb) - g(x, self.rb)) / self.eps;

                while x < self.lb + self.eps {
                    if (df(x) - q) > self.eps {
                        q = df(x);
                    }
                    if (dg(x) - q) > self.eps {
                        q = dg(x);
                    }
                    x += self.eps;
                }

                let mut y = self.rb - self.eps;
                let df = |y| (f(self.lb, y + self.eps) - f(self.lb, y)) / self.eps;
                let dg = |y| (g(self.lb, y + self.eps) - g(self.lb, y)) / self.eps;

                while y < self.rb + self.eps {
                    if (df(y) - q) > self.eps {
                        q = df(y);
                    }
                    if (dg(y) - q) > self.eps {
                        q = dg(y);
                    }
                    y += self.eps;
                }
            }
        };

        if q > 1.0 {
            self.errors
                .set("The sufficient condition of convolution is not satisfied".to_string());
        }
    }
}
