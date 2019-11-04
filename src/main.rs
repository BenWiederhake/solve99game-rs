#[macro_use]
extern crate static_assert_macro;
extern crate openssl;

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::iter::FusedIterator;

// FIXME: Use 'try_into' instead of casts.
// Or maybe even https://docs.rs/index_vec/0.1.0/index_vec/

const BASE : usize = 10;
// Theoretically, 8 is pretty safe.
// https://en.wikipedia.org/wiki/Birthday_problem#Probability_table
const HASH_BYTES : usize = 16;

// FIXME: Make these into actual wrapper types, so that no type confusion can occur.
type BoardCell = u8;
type BoardHash = [u8; HASH_BYTES];
type BoardIndex = u32;  // Overflow implies memory usage of >219 GiB.
type MoveIndex = u8;
type CellIndex = MoveIndex;
type Cost = u8;

#[derive(Clone, Copy)]
enum BoardMove {
    Duplicate,
    CrossDownFrom(CellIndex, CellIndex),
    CrossRightFrom(CellIndex, CellIndex),
}

impl BoardMove {
    fn fmt_cross(&self, f: &mut fmt::Formatter<'_>, pos: BoardCell, dir: char) -> fmt::Result {
        let row = pos / (BASE - 1) as BoardCell;
        let col = pos % (BASE - 1) as BoardCell;
        write!(f, "{},{},{}", col, row, dir)
    }
}

impl fmt::Debug for BoardMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoardMove::Duplicate =>
                write!(f, "'expand'"),
            BoardMove::CrossDownFrom(start, _) =>
                self.fmt_cross(f, *start, 'v'),
            BoardMove::CrossRightFrom(start, _) =>
                self.fmt_cross(f, *start, '>'),
        }
    }
}

#[derive(Clone, Debug, Hash)]
struct FullBoard(Vec<BoardCell>);

impl FullBoard {
    pub fn new() -> FullBoard {
        static_assert!(BASE <= BoardCell::max_value() as usize);
        let mut cells : Vec<BoardCell> = Vec::with_capacity((BASE - 1) * 3);
        for i in 1..BASE {
            cells.push(i as BoardCell);
        }
        for i in 1..BASE {
            cells.push(1 as BoardCell);
            cells.push(i as BoardCell);
        }
        FullBoard(cells)
    }

    fn new_duplicated(parent: &FullBoard) -> FullBoard {
        let mut cells = parent.0.clone();
        cells.reserve(cells.len());
        for &c in parent.0.iter() {
            if c != 0 {
                cells.push(c);
            }
        }
        FullBoard(cells)
    }

    fn new_without(parent: &FullBoard, skip_a: CellIndex, skip_b: CellIndex) -> FullBoard {
        /* TODO: This and `clean_dead_around` has probably the most
         * potential for optimization. */
        assert!((skip_a as usize) < parent.0.len() && (skip_b as usize) < parent.0.len(),
            "skip_a={}, skip_b={}, len={}, board={:?}",
            skip_a, skip_b, parent.0.len(), parent);
        let mut intermediate = parent.clone();
        assert!(is_match(intermediate.0[skip_a as usize], intermediate.0[skip_b as usize]),
            "tried to eliminate pair {}@{} {}@{}",
            intermediate.0[skip_a as usize], skip_a,
            intermediate.0[skip_b as usize], skip_b);
        intermediate.0[skip_a as usize] = 0;
        intermediate.0[skip_b as usize] = 0;
        intermediate.clean_dead_around(skip_b);
        intermediate.clean_dead_around(skip_a);
        intermediate
    }

    fn clean_dead_around(&mut self, around: CellIndex) {
        let around = around as usize;
        if around >= self.0.len() || 0 != self.0[around] {
            return
        }

        let first_dead = match self.find_prev(around) {
            None => 0,
            Some((prev_nondead, _)) => prev_nondead + 1,
        };
        assert!(first_dead <= around);
        let last_dead = match self.find_first(around) {
            None => self.0.len() - 1,
            Some((next_nondead, _)) => next_nondead - 1,
        };
        assert!(last_dead >= around);

        let elim_rows = (last_dead - first_dead + 1) / (BASE - 1);
        if 0 == elim_rows {
            // Nothing to eliminate.
            return;
        }

        let mut new_cells = Vec::with_capacity(self.0.len());
        new_cells.extend_from_slice(&self.0[..first_dead]);
        new_cells.extend_from_slice(&self.0[(first_dead + elim_rows * (BASE - 1))..]);

        std::mem::swap(&mut self.0, &mut new_cells);
    }

    fn find_first(&self, start_index: usize) -> Option<(usize, BoardCell)> {
        for (i, &element) in self.0[start_index..].iter().enumerate() {
            if 0 != element {
                return Some((start_index + i, element));
            }
        }
        None
    }

    fn find_prev(&self, start_index: usize) -> Option<(usize, BoardCell)> {
        for (i, &element) in self.0[..start_index].iter().enumerate().rev() {
            if 0 != element {
                return Some((i, element));
            }
        }
        None
    }

    fn find_down(&self, start_index: usize) -> Option<(usize, BoardCell)> {
        for (i, &element) in self.0[start_index..].iter().step_by(BASE - 1).enumerate().skip(1) {
            if 0 != element {
                return Some((start_index + i * (BASE - 1), element));
            }
        }
        None
    }

    pub fn get_hash(&self) -> BoardHash {
        // TODO: Could be made more efficient, but whatever.
        let all_hash_bytes = openssl::sha::sha256(format!("{:?}", self).as_ref());
        let mut hash_bytes = [0u8; HASH_BYTES];
        for (i, &b) in all_hash_bytes[..HASH_BYTES].iter().enumerate() {
            hash_bytes[i] = b;
        }
        hash_bytes
    }

    pub fn moves(&self) -> MovesIter {
        MovesIter::new(&self)
    }

    pub fn apply_move(&self, the_move: BoardMove) -> FullBoard {
        match the_move {
            BoardMove::Duplicate =>
                FullBoard::new_duplicated(self),
            BoardMove::CrossDownFrom(a, b) =>
                FullBoard::new_without(self, a, b),
            BoardMove::CrossRightFrom(a, b) =>
                FullBoard::new_without(self, a, b),
        }
    }

    pub fn is_goal(&self) -> bool {
        fn is_zero(c: &BoardCell) -> bool { 0 == *c };
        self.0.iter().all(is_zero)
    }

    pub fn compute_h_score(&self) -> Cost {
        fn is_nonzero(c: &&BoardCell) -> bool { 0 != **c };
        let num_cells = self.0.iter().filter(is_nonzero).count();
        if num_cells % 2 == 0 {
            /* "Cross" only eliminates two cells each move. */
            (num_cells / 2) as Cost
        } else {
            /* Odd number of cells.  Need also "duplicate" and "cross". */
            (num_cells / 2 + 2) as Cost
        }
    }
}

#[derive(Debug)]
struct MovesIter<'a> {
    board: &'a FullBoard,
    next_cell: CellIndex,
    reported_first: bool,
}

impl MovesIter<'_> {
    pub fn new<'a>(board: &'a FullBoard) -> MovesIter<'a> {
        let first_nonzero_index = board.find_first(0).expect("Moves for empty Board?!").0;
        MovesIter{
            board,
            next_cell: first_nonzero_index as CellIndex,
            reported_first: false,
        }
    }
}

impl FusedIterator for MovesIter<'_> {}
impl Iterator for MovesIter<'_> {
    type Item = BoardMove;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.next_cell as usize >= self.board.0.len() {
                if self.reported_first {
                    return None;
                } else {
                    self.reported_first = true;
                    return Some(BoardMove::Duplicate);
                }
            }

            // Pointing at a non-empty cell.
            let this_val = self.board.0[self.next_cell as usize];
            assert!(0 != this_val);
            // Try looking "down".
            if !self.reported_first {
                self.reported_first = true;
                if let Some((match_index, match_val)) =
                        self.board.find_down(self.next_cell as usize) {
                    if is_match(this_val, match_val) {
                        return Some(BoardMove::CrossDownFrom(
                            self.next_cell, match_index as CellIndex));
                    }
                }
            }

            // For either reason, no need to look "down".
            assert!(self.reported_first);
            // Therefore, try looking "right".
            match self.board.find_first((self.next_cell + 1) as usize) {
                None => {
                    self.reported_first = false;
                    self.next_cell = self.board.0.len() as CellIndex;
                    continue;
                },
                Some((match_index, match_val)) => {
                    // Set up for next iteration
                    self.reported_first = false;
                    let start_cell = self.next_cell;
                    self.next_cell = match_index as CellIndex;
                    if is_match(this_val, match_val) {
                        return Some(BoardMove::CrossRightFrom(
                            start_cell, match_index as CellIndex));
                    }
                }
            }
        }
    }
}

fn is_match(lhs: BoardCell, rhs: BoardCell) -> bool {
    (lhs == rhs) || (lhs + rhs == BASE as BoardCell)
}

#[derive(Clone, Copy, Debug)]
struct IncrementalNode {
    pub prev_index: BoardIndex,
    pub move_index: MoveIndex,
}

fn reconstruct_rev_moves(node_index: BoardIndex, all_set: &Vec<IncrementalNode>)
        -> Vec<MoveIndex> {
    let mut rev_moves = Vec::with_capacity(20);
    {
        let mut current_index = node_index;
        while current_index != 0 {
            let current_incr: IncrementalNode = all_set[current_index as usize];
            rev_moves.push(current_incr.move_index);
            current_index = current_incr.prev_index;
        }
    }
    rev_moves
}

fn reconstruct_path(node_index: BoardIndex, all_set: &Vec<IncrementalNode>, start: &FullBoard)
        -> (Vec<BoardMove>, FullBoard) {
    let rev_moves = reconstruct_rev_moves(node_index, all_set);

    let mut moves = Vec::with_capacity(rev_moves.len());
    let mut board = start.clone();
    for &move_index in rev_moves.iter().rev() {
        let the_move = board.moves().nth(move_index.into()).unwrap();
        moves.push(the_move);
        board = board.apply_move(the_move);
    }

    (moves, board)
}

fn reconstruct_state(node_index: BoardIndex, all_set: &Vec<IncrementalNode>, start: &FullBoard)
        -> (Cost, FullBoard) {
    let (moves, board) = reconstruct_path(node_index, all_set, start);
    (moves.len() as Cost, board)
}

fn run(start: &FullBoard) -> Vec<BoardMove> {
    let mut open_set = BTreeMap::<Cost, Vec<BoardIndex>>::new();
    let mut all_set = Vec::<IncrementalNode>::new();
    // Using a HashMap instead of a BTreeMap should be more memory-efficient,
    // even though HashMap has a load_factor of 2.
    let mut seen = HashMap::<BoardHash, (Cost, BoardIndex)>::new();
    let mut steps = 0;

    {
        use std::mem;
        let bytes_incr = (mem::size_of::<BoardIndex>() + mem::size_of::<IncrementalNode>()) as f32;
        let bytes_seen = mem::size_of::<(BoardHash, (Cost, BoardIndex))>() as f32 * 1.5;
        println!("Each node needs a total of {:.2} bytes ({} incremental, {} seen).",
            bytes_incr + bytes_seen, bytes_incr, bytes_seen);
        println!("The 'seen' HashMap is responsible for {:.2} bytes of that, instead of {}+eps.",
            mem::size_of::<(BoardHash, (Cost, BoardIndex))>() as f32 * 1.5,
            mem::size_of::<BoardHash>() + mem::size_of::<Cost>() + mem::size_of::<BoardIndex>());
        println!("IncrementalNode is {} instead of {} + {} = {}",
            mem::size_of::<IncrementalNode>(),
            mem::size_of::<BoardIndex>(),
            mem::size_of::<MoveIndex>(),
            mem::size_of::<BoardIndex>() + mem::size_of::<MoveIndex>());
        println!("Theoretical minimum is {} + {} + {} + {} + {} = {}",
            mem::size_of::<BoardIndex>(),
            mem::size_of::<BoardIndex>() + mem::size_of::<MoveIndex>(),
            mem::size_of::<BoardHash>(),
            mem::size_of::<Cost>(),
            mem::size_of::<BoardIndex>(),
            mem::size_of::<BoardIndex>() +
            mem::size_of::<BoardIndex>() + mem::size_of::<MoveIndex>() +
            mem::size_of::<BoardHash>() +
            mem::size_of::<Cost>() +
            mem::size_of::<BoardIndex>());
        println!("---------------------------------------------");
    }

    // Seed
    all_set.push(IncrementalNode{prev_index: 0, move_index: 0});
    open_set.insert(0, vec![0]);
    // Run A*
    loop {
        /* Pop a single element from the "open" set.
         * Lots of special cases because of reasons. */
        let (remove_gh, current_gh_score, current_incremental_index) =
            match open_set.iter_mut().next() {
                None => return Vec::with_capacity(0),
                Some((&current_gh, current_stack)) => {
                    assert!(!current_stack.is_empty());
                    if current_stack.len() == 1 {
                        (Some(current_gh), current_gh, current_stack.pop().unwrap())
                    } else {
                        (None, current_gh, current_stack.pop().unwrap())
                    }
                }
            };
        if let Some(gh_val) = remove_gh {
            open_set.remove(&gh_val);
        }
        if current_incremental_index == BoardIndex::max_value() {
            /* It was just a placeholder. */
            continue;
        }

        let (current_g_score, current_fullstate) = reconstruct_state(
            current_incremental_index, &all_set, start);
        let reporting = (steps < 50) || (steps % 10000 == 0);
        if reporting {
            println!("Looking at {:?}@all, g+h={}, {:?}",
                current_incremental_index, current_gh_score, current_fullstate);
            let open_size: usize = open_set.iter().map(|e| e.1.len()).sum();
            println!("\tTurn {:3}: {:7} steps, {:7} open, {:7} closed",
                current_g_score, steps, open_size, all_set.len());
        }
        steps += 1;
        /* Usually we would need to check whether we have reached the goal here.
         * However, since the moves in 99game have uniform cost, we can push
         * this test one iteration up into the expansion phase. */

        for (move_index, the_move) in current_fullstate.moves().enumerate() {
            let neighbor_fullstate = current_fullstate.apply_move(the_move);
            if reporting && false {
                println!("\t{:?} to {:?}", the_move, neighbor_fullstate);
            }
            let neighbor_incremental = IncrementalNode{
                prev_index: current_incremental_index,
                move_index: move_index as u8,
            };
            /* Checking for the solution here is ONLY valid because moves in
             * 99game has uniform cost. */
            if neighbor_fullstate.is_goal() {
                let neighbor_all_index = all_set.len() as BoardIndex;
                all_set.push(neighbor_incremental);
                return reconstruct_path(neighbor_all_index, &all_set, start).0;
            }
            let neighbor_g_score = current_g_score + 1;
            let neighbor_gh_score = neighbor_g_score + neighbor_fullstate.compute_h_score();
            let neighbor_hash = neighbor_fullstate.get_hash();
            /* TODO: Use Entry API to avoid duplicate lookup */
            if let Some(&(oldneighbor_gh_score, oldneighbor_open_index)) = seen.get(&neighbor_hash) {
                assert!(oldneighbor_open_index != BoardIndex::max_value());
                if oldneighbor_gh_score <= neighbor_gh_score {
                    /* Don't consider "neighbor" any further. as it will be handled in time by "oldneighbor". */
                    continue;
                }

                /* We need to replace "oldneighbor".
                 * Set the old entry to a placeholder in order to "replace" it. */
                let oldneighbor_all_index: &mut BoardIndex =
                    //&mut open_set[&oldneighbor_gh_score][oldneighbor_open_index as usize];
                    // But nooo, IndexMut doesn't exist anymore:
                    // https://github.com/rust-lang/rust/pull/23559
                    &mut open_set.get_mut(&oldneighbor_gh_score).unwrap().get_mut(oldneighbor_open_index as usize).unwrap();
                let neighbor_all_index: BoardIndex = *oldneighbor_all_index;
                *oldneighbor_all_index = BoardIndex::max_value();

                /* Create new, "earlier" entry: */
                all_set[neighbor_all_index as usize] = neighbor_incremental;
                let neighbor_stack: &mut Vec<_> = open_set.entry(neighbor_gh_score).or_default();
                let neighbor_open_index = neighbor_stack.len() as BoardIndex;
                neighbor_stack.push(neighbor_all_index);
                *seen.get_mut(&neighbor_hash).unwrap() = (neighbor_gh_score as Cost, neighbor_open_index);
            } else {
                /* `None`.  FIXME: Write distinction nicer.
                 * We have discovered an entirely new state. */
                let neighbor_all_index = all_set.len() as BoardIndex;
                all_set.push(neighbor_incremental);
                let neighbor_stack: &mut Vec<_> = open_set.entry(neighbor_gh_score).or_default();
                let neighbor_open_index = neighbor_stack.len() as BoardIndex;
                neighbor_stack.push(neighbor_all_index);
                let old_entry = seen.insert(neighbor_hash, (neighbor_gh_score as Cost, neighbor_open_index));
                assert!(old_entry == None);
            }
        }
    }
}

fn main() {
    println!("Running with BASE = {}", BASE);
    let start = FullBoard::new();
    println!("Initial board = {:?}", start);
    println!("Hash = {:?}", start.get_hash());
    let moves = run(&start);
    println!("=== Can win after {} turns! ===\n{:?}", moves.len(), moves);
}
