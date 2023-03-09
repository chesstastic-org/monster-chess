use crate::Board;

impl Board {
    pub fn perft(&mut self, depth: u32) -> u32 {
        if depth == 0 { return 1; }

        let mut nodes = 0;
        for node in self.generate_moves() {
            self.make_move(node);
            nodes += self.perft(depth - 1);
            self.undo_move().unwrap();
        }

        nodes
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