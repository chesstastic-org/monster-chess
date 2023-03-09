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
            self.make_move(node);
            nodes += self.sub_perft(depth - 1);
            self.undo_move().unwrap();
        }

        nodes
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
            self.make_move(node);
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

/*
u64 Perft(int depth)
{
  MOVE move_list[256];
  int n_moves, i;
  u64 nodes = 0;

  if (depth == 0)
    return 1ULL;

  n_moves = GenerateLegalMoves(move_list);
  for (i = 0; i < n_moves; i++) {
    MakeMove(move_list[i]);
    nodes += Perft(depth - 1);
    UndoMove(move_list[i]);
  }
  return nodes;
}
*/