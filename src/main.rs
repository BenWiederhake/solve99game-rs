#[macro_use]
extern crate static_assert_macro;
extern crate openssl;

use std::iter::FusedIterator;
use std::collections;

// FIXME: Use 'try_into' instead of casts.

const BASE : usize = 4;
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

#[derive(Clone, Copy, Debug)]
enum BoardMove {
    Duplicate,
    CrossDownFrom(CellIndex, CellIndex),
    CrossRightFrom(CellIndex, CellIndex),
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
        for (i, &element) in self.0[(start_index + 1)..].iter().step_by(BASE - 1).enumerate() {
            if 0 != element {
                return Some(((start_index + 1) + i * BASE - 1, element));
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
        unimplemented!();
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
                        self.board.find_down((self.next_cell + 1) as usize) {
                    if is_match(this_val, match_val) {
                        return Some(BoardMove::CrossDownFrom(self.next_cell, match_index as CellIndex));
                    }
                }
            }

            // For either reason, no need to look "down".
            assert!(self.reported_first);
            // Therefore, try looking "right".
            unimplemented!();
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

fn reconstruct_state(node_index: BoardIndex, all_set: &Vec<IncrementalNode>, start: &FullBoard)
        -> (Cost, FullBoard) {
    let mut rev_moves = Vec::with_capacity(20);
    {
        let mut current_index = node_index;
        while current_index != 0 {
            let current_incr: IncrementalNode = all_set[current_index as usize];
            rev_moves.push(current_incr.move_index);
            current_index = current_incr.prev_index;
        }
    }
    let rev_moves = rev_moves;  // drop "mut"

    // Determine beforehand so we can consume `rev_moves`:
    let num_moves = rev_moves.len();
    let mut board = start.clone();
    for &move_index in rev_moves.iter().rev() {
        let the_move = board.moves().nth(move_index.into()).unwrap();
        board = board.apply_move(the_move);
    }

    (num_moves as Cost, board)
}

fn run(start: &FullBoard) -> Vec<FullBoard> {
    let mut open_set = collections::BTreeMap::<Cost, Vec<BoardIndex>>::new();
    let mut all_set = Vec::<IncrementalNode>::new();
    // Using a HashMap instead of a BTreeMap should be more memory-efficient,
    // even though HashMap has a load_factor of 2.
    let mut seen = collections::HashMap::<BoardHash, BoardIndex>::new();

    {
        use std::mem;
        let bytes_incr = (mem::size_of::<BoardIndex>() + mem::size_of::<IncrementalNode>()) as f32;
        let bytes_seen = mem::size_of::<(BoardHash, BoardIndex)>() as f32 * 1.5;
        println!("Each node needs a total of {:.2} bytes ({} incremental, {} seen).",
            bytes_incr + bytes_seen, bytes_incr, bytes_seen);
        println!("The 'seen' HashMap is responsible for {:.2} bytes of that, instead of {}+eps.",
            mem::size_of::<(BoardHash, BoardIndex)>() as f32 * 1.5,
            mem::size_of::<BoardHash>() + mem::size_of::<BoardIndex>());
        println!("IncrementalNode is {} instead of {} + {} = {}",
            mem::size_of::<IncrementalNode>(),
            mem::size_of::<BoardIndex>(),
            mem::size_of::<MoveIndex>(),
            mem::size_of::<BoardIndex>() + mem::size_of::<MoveIndex>());
        println!("Theoretical minimum is {} + {} + {} + {} = {}",
            mem::size_of::<BoardIndex>(),
            mem::size_of::<BoardIndex>() + mem::size_of::<MoveIndex>(),
            mem::size_of::<BoardHash>(),
            mem::size_of::<BoardIndex>(),
            mem::size_of::<BoardIndex>() +
            mem::size_of::<BoardIndex>() + mem::size_of::<MoveIndex>() +
            mem::size_of::<BoardHash>() +
            mem::size_of::<BoardIndex>());
        println!("---------------------------------------------");
    }

    // Seed
    all_set.push(IncrementalNode{prev_index: 0, move_index: 0});
    open_set.insert(0, vec![0]);
    // Run
    loop {
        let (remove_gh, current_incremental_index) = match open_set.iter_mut().next() {
            None => break,
            Some((&current_gh, current_stack)) => {
                assert!(!current_stack.is_empty());
                if current_stack.len() == 1 {
                    (Some(current_gh), current_stack.pop().unwrap())
                } else {
                    (None, current_stack.pop().unwrap())
                }
            }
        };
        if let Some(gh_val) = remove_gh {
            open_set.remove(&gh_val);
        }
        println!("Looking at {:?}", current_incremental_index);
        let (current_g_score, current_fullstate) = reconstruct_state(
            current_incremental_index, &all_set, start);

        unimplemented!();
    }

    unimplemented!();
}

fn main() {
    println!("Running with BASE = {}", BASE);
    let start = FullBoard::new();
    println!("Initial board = {:?}", start);
    println!("Hash = {:?}", start.get_hash());
    run(&start);
}
