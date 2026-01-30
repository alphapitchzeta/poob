use crate::game::Game;
use crate::movegen::*;
use crate::moves::*;
use std::time::Instant;

pub fn perft(depth: usize, game: Game) -> usize {
    if depth == 0 {
        return 1;
    }

    let moves = game.enumerate_moves();
    let moves_iter = MoveListIterator::new(&moves);
    let mut nodes = 0;

    for mv in moves_iter {
        let mut next_game = game.clone();

        next_game.unchecked_make_move(mv.mv);

        nodes += perft(depth - 1, next_game);
    }

    nodes
}

pub struct PerftCase<'a> {
    game: Game<'a>,
    case: Vec<DepthNode>,
}

impl<'a> PerftCase<'a> {
    pub fn from_str(s: &str, move_gen: &'a MoveGenerator) -> Option<Self> {
        let mut chunks = s.split(';');
        let fen = chunks.next()?;
        let mut depth_nodes = Vec::new();

        for chunk in chunks {
            depth_nodes.push(DepthNode::from_str(chunk)?);
        }

        let game = Game::from_fen(fen, move_gen).ok()?;

        Some(Self {
            game,
            case: depth_nodes,
        })
    }
}

impl PerftCase<'_> {
    pub fn test(&self) -> (String, bool) {
        let mut result_string = format!("Testing position {}\n\n", self.game.to_fen());
        let mut success = true;

        for depth_node in self.case.iter() {
            result_string.push_str(&format!("Depth {}\n", depth_node.depth));

            let test_game = self.game.clone();

            let timer = Instant::now();
            let searched_nodes = perft(depth_node.depth, test_game);

            let elapsed = timer.elapsed();

            result_string.push_str(&format!(
                "Searched nodes: {}\nActual nodes: {}\n",
                searched_nodes, depth_node.nodes
            ));

            if searched_nodes != depth_node.nodes {
                result_string.push_str("FAILURE\n");
                success = false;
            } else {
                result_string.push_str("SUCCESS\n");
            }

            result_string.push_str(&format!("{:?}\n\n", elapsed));
        }

        (result_string, success)
    }
}

struct DepthNode {
    depth: usize,
    nodes: usize,
}

impl DepthNode {
    fn from_str(s: &str) -> Option<Self> {
        let mut chunks = s.trim().split_ascii_whitespace();
        let mut depth_chunk = chunks.next()?.chars();
        depth_chunk.next();

        let depth_str = depth_chunk.collect::<String>();
        let depth = depth_str.parse::<usize>().ok()?;

        let nodes_str = chunks.next()?;
        let nodes = nodes_str.parse::<usize>().ok()?;

        Some(Self { depth, nodes })
    }
}
