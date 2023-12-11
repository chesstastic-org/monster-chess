use super::{
    game::{MoveLegalResponse, NORMAL_MODE},
    Board, actions::HistoryMove,
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
            .expect("Should have found a perft branch.")
            .1
            .clone()
    }
}

impl<'a, const T: usize> Board<'a, T> {
    pub fn perft(&mut self, depth: u32, legality: bool) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut nodes = 0;

        let psuedolegal = self.game.controller.use_pseudolegal();
        let psuedolegal_check = legality && psuedolegal;

        let moves = if psuedolegal {
            self.generate_moves(NORMAL_MODE)
        } else {
            self.generate_legal_moves(NORMAL_MODE)
        };

        if depth == 1 && !psuedolegal_check {
            return moves.len() as u64;   
        }
        
        for node in moves {
            let mut undo: Option<HistoryMove<T>> = None;

            if psuedolegal_check {
                let MoveLegalResponse { is_legal, made_move } = self.game.controller.is_legal(self, &node, false);

                if !is_legal {
                    if let Some(made_move) = made_move {
                        self.undo_move(made_move);
                    }
                    continue;
                }  

                if let Some(made_move) = made_move {
                    undo = made_move;
                }
            }

            if undo.is_none() {
                undo = self.make_move(&node);
            }

            nodes += self.perft(depth - 1, legality);
            self.undo_move(undo);
        }

        nodes
    }

    pub fn assert_perft(&mut self, depth: u32, true_nodes: u64) {
        let nodes = self.perft(depth, true);
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

    pub fn assert_perfts<const S: usize>(&mut self, nodes: [u64; S]) {
        for (ind, true_nodes) in nodes.iter().enumerate() {
            let depth = (ind + 1) as u32;
            self.assert_perft(depth, *true_nodes);
        }
    }

    pub fn branch_perft(&mut self, depth: u32) -> PerftResults {
        if depth == 0 {
            return PerftResults {
                nodes: 1,
                branches: vec![],
            };
        }

        let mut nodes = 0;
        let mut branches: Vec<PerftBranch> = vec![];
        for node in self.generate_legal_moves(NORMAL_MODE) {
            let undo = self.make_move(&node);
            let results = self.branch_perft(depth - 1);
            nodes += results.nodes;
            branches.push((self.encode_action(&node), results));
            self.undo_move(undo);
        }

        PerftResults { nodes, branches }
    }
}
