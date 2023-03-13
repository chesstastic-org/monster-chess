use super::{
    actions::{Action, HistoryMove, UndoMoveError},
    edges::Edges,
    game::Game,
    pieces::Piece,
    Board,
};

pub type PerftBranch = (String, PerftResults);

#[derive(Debug, Clone)]
pub struct PerftResults {
    pub nodes: u64,
    pub branches: Vec<PerftBranch>,
}

impl PerftResults {
    pub fn get_branch_results(&self, branch: &str) -> PerftResults {
        self.branches
            .iter()
            .find(|el| el.0 == branch)
            .unwrap()
            .1
            .clone()
    }
}

impl<'a> Board<'a> {
    pub fn sub_perft(&mut self, depth: u32, legality: bool) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut nodes = 0;
        let moves = if legality {
            self.generate_legal_moves(0)
        } else {
            self.generate_moves(0)
        };
        for node in moves {
            self.make_move(&node);
            nodes += self.sub_perft(depth - 1, legality);
            self.undo_move();
        }

        nodes
    }

    pub fn assert_perft(&mut self, depth: u32, true_nodes: u64) {
        let nodes = self.sub_perft(depth, true);
        assert_eq!(
            nodes,
            true_nodes,
            "Perft of {} for FEN {} is {} (not {}, the expected result)",
            depth,
            self.to_fen(),
            nodes,
            true_nodes
        );
    }

    pub fn assert_perfts<const T: usize>(&mut self, nodes: [u64; T]) {
        for (ind, true_nodes) in nodes.iter().enumerate() {
            let depth = (ind + 1) as u32;
            self.assert_perft(depth, *true_nodes);
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
        for node in self.generate_legal_moves(0) {
            self.make_move(&node);
            let results = self.perft(depth - 1);
            nodes += results.nodes;
            branches.push((self.encode_action(&node), results));
            self.undo_move();
        }

        PerftResults { nodes, branches }
    }
}
