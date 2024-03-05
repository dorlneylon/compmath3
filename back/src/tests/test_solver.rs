mod tests {
    use crate::core::eq_solver::*;

    #[test]
    fn test_chords() {
        let eps = 0.01;
        let solver = Solver::new(
            Task::Eq(|x| x * x * x - x + 4.0),
            Method::Chords,
            100,
            eps,
            -2.0,
            1.5,
        );

        println!("{:?}", solver.solve());
    }

    #[test]
    fn test_secants() {
        let eps = 0.01;
        let solver = Solver::new(
            Task::Eq(|x| x * x * x - x + 4.0),
            Method::Secants,
            100,
            eps,
            -2.0,
            1.5,
        );

        println!("{:?}", solver.solve());
    }

    #[test]
    fn test_simple_it() {
        let eps = 0.01;
        let solver = Solver::new(
            Task::Eq(|x| 12.0 / 11.0 * x - 1.0 / 11.0 * x * x * x - 4.0 / 11.0),
            Method::SimpleIt,
            100,
            eps,
            -5.0,
            -2.0,
        );

        println!("{:?}", solver.solve());
    }

    #[test]
    fn test_simple_it_sys() {
        let eps = 0.01;
        let solver = Solver::new(
            Task::Sys((|x, y| (y + 2.0).sin() - 1.5, |x, y| 0.5 - (x - 2.0).cos())),
            // Task::Sys((
            //     |x, y| 0.3 - 0.1 * x * x - 0.2 * y * y,
            //     |x, y| 0.7 - 0.2 * x * x - 0.1 * x * y,
            // )),
            Method::SimpleIt,
            100,
            eps,
            // -1.7,
            // 1.3,
            0.0,
            1.0,
        );
        println!("{:?}", solver.solve());
        println!("{:?}", solver.errors.take());
    }
}
