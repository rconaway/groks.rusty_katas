#[cfg(test)]

    use super::board::Board;
    use super::life::Life;

    #[ignore]
    #[test]
    fn glider() {
        let glider1 = Board::new(r". . . . .
                                  . * . * .
                                  . . * * .
                                  . . * . .
                                  . . . . .");

        let glider2 = Board::new(r". . . . .
                                  . . . * .
                                  . * . * .
                                  . . * * .
                                  . . . . .");

        let glider3 = Board::new(r". . . . .
                                  . . * . .
                                  . . . * *
                                  . . * * .
                                  . . . . .");

        let glider4 = Board::new(r". . . . .
                                  . . . * .
                                  . . . . *
                                  . . * * *
                                  . . . . .");

        
        assert_eq!(Life::evolve(glider1), glider2);
        assert_eq!(Life::evolve(glider2), glider3);
        assert_eq!(Life::evolve(glider3), glider4);
    }

