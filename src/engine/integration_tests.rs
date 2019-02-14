#[cfg(test)]

    use super::board::*;
    use super::life;
    use super::life::tests;

    #[test]
    fn glider() {
        let glider1 = from_grid(r". . . . .
                                  . * . * .
                                  . . * * .
                                  . . * . .
                                  . . . . .");

        let glider2 = from_grid(r". . . . .
                                  . . . * .
                                  . * . * .
                                  . . * * .
                                  . . . . .");

        let glider3 = from_grid(r". . . . .
                                  . . * . .
                                  . . . * *
                                  . . * * .
                                  . . . . .");

        let glider4 = from_grid(r". . . . .
                                  . . . * .
                                  . . . . *
                                  . . * * *
                                  . . . . .");

        
        assert_eq!(life::evolve(&glider1), glider2);
        assert_eq!(life::evolve(&glider2), glider3);
        assert_eq!(life::evolve(&glider3), glider4);
    }

