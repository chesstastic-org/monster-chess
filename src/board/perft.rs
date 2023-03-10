use crate::Board;

pub type PerftBranch = ((String, String), PerftResults);

#[derive(Debug, Clone)]
pub struct PerftResults {
    pub nodes: u32,
    pub branches: Vec<PerftBranch>,
}

impl PerftResults {
    pub fn get_branch_results(&self, branch: (&str, &str)) -> PerftResults {
        let branch = (branch.0.to_string(), branch.1.to_string());
        self.branches
            .iter()
            .find(|el| el.0 == branch)
            .unwrap()
            .1
            .clone()
    }
}

impl Board {
    pub fn sub_perft(&mut self, depth: u32) -> u32 {
        if depth == 0 {
            return 1;
        }

        let mut nodes = 0;
        for node in self.generate_legal_moves() {
            self.make_move(&node);
            nodes += self.sub_perft(depth - 1);
            self.undo_move().unwrap();
        }

        nodes
    }

    pub fn assert_perfts<const T: usize>(&mut self, nodes: [ u32; T ]) {
        for (ind, true_nodes) in nodes.iter().enumerate() {
            let depth = (ind + 1) as u32;
            let nodes = self.sub_perft(depth);
            assert_eq!(&nodes, true_nodes, "Perft of {} for FEN {} is {} (not {}, the expected result)", depth, self.to_fen(), nodes, true_nodes);
        }
    }

    pub fn perft(&mut self, depth: u32) -> PerftResults {
        if depth == 0 {
            return PerftResults {
                nodes: 1,
                branches: vec![],
            };
        }

        let mut nodes = 0;
        let mut branches: Vec<PerftBranch> = vec![];
        for node in self.generate_legal_moves() {
            self.make_move(&node);
            let results = self.perft(depth - 1);
            nodes += results.nodes;
            branches.push((
                (
                    self.encode_position(node.from),
                    self.encode_position(node.to),
                ),
                results,
            ));
            self.undo_move().unwrap();
        }

        PerftResults { nodes, branches }
    }
}

#[cfg(test)]
mod tests {
    use crate::{BitSet, Chess, Board};

    #[test]
    fn startpos() {
        let mut board = Board::new(
            Chess::create(),
            2,
            (8, 8),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        );        

        board.assert_perfts([ 20, 400, 8902, 197281 ]);
        //board.assert_perfts([ 20, 400, 8902, 197281, 4865609 ]);
    }

    #[test]
    fn castling() {
        let mut board = Board::new(
            Chess::create(),
            2,
            (8, 8),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR w kqKQ - 0 1",
        );        

        board.assert_perfts([ 23, 460, 10664, 236936 ]);
    }

    #[test]
    fn white_promotion() {
        let mut board = Board::new(
            Chess::create(),
            2,
            (8, 8),
            "8/5P2/8/8/8/7K/8/n6k w - - 0 1",
        );        

        board.assert_perfts([ 7, 25, 299, 1931 ]);
    }
}
