#[macro_use]
extern crate static_assert_macro;
extern crate openssl;

use std::collections;

const BASE : usize = 4;
// Theoretically, 8 is pretty safe.
// https://en.wikipedia.org/wiki/Birthday_problem#Probability_table
const HASH_BYTES : usize = 16;

type BoardCell = u8;
type BoardHash = [u8; HASH_BYTES];
type PrevIndex = u32;  // Overflow implies memory usage of >219 GiB.
type MoveIndex = u8;
type Cost = u8;

#[derive(Debug, Hash)]
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

    fn find_next(&self, start_index: usize) -> Option<(usize, BoardCell)> {
        for (i, &element) in self.0[(start_index + 1)..].iter().enumerate() {
            if 0 != element {
                return Some((i, element));
            }
        }
        None
    }

    fn find_prev(&self, start_index: usize) -> Option<(usize, BoardCell)> {
        for (i, &element) in self.0[..start_index].iter().rev().enumerate() {
            if 0 != element {
                return Some((i, element));
            }
        }
        None
    }

    fn find_down(&self, start_index: usize) -> Option<(usize, BoardCell)> {
        for (i, &element) in self.0[(start_index + 1)..].iter().step_by(BASE - 1).enumerate() {
            if 0 != element {
                return Some((i, element));
            }
        }
        None
    }

    fn get_hash(&self) -> BoardHash {
        // TODO: Could be made more efficient, but whatever.
        let all_hash_bytes = openssl::sha::sha256(format!("{:?}", self).as_ref());
        let mut hash_bytes = [0u8; HASH_BYTES];
        for (i, &b) in all_hash_bytes[..HASH_BYTES].iter().enumerate() {
            hash_bytes[i] = b;
        }
        hash_bytes
    }
}

struct IncrementalNode {
    pub prev_index : PrevIndex,
    pub movenr_ish : MoveIndex,
}

impl IncrementalNode {
}

fn run(start: &FullBoard) -> Vec<FullBoard> {
    let mut open_set = collections::BTreeMap::<Cost, Vec<PrevIndex>>::new();
    let mut all_set = Vec::<IncrementalNode>::new();
    // Using a HashMap instead of a BTreeMap should be more memory-efficient,
    // even though HashMap has a load_factor of 2.
    let mut seen = collections::HashMap::<BoardHash, PrevIndex>::new();

    {
        use std::mem;
        let bytes_incr = (mem::size_of::<PrevIndex>() + mem::size_of::<IncrementalNode>()) as f32;
        let bytes_seen = mem::size_of::<(BoardHash, PrevIndex)>() as f32 * 1.5;
        println!("Each node needs a total of {:.2} bytes ({} incremental, {} seen).",
            bytes_incr + bytes_seen, bytes_incr, bytes_seen);
        println!("The 'seen' HashMap is responsible for {:.2} bytes of that, instead of {}+eps.",
            mem::size_of::<(BoardHash, PrevIndex)>() as f32 * 1.5,
            mem::size_of::<BoardHash>() + mem::size_of::<PrevIndex>());
        println!("IncrementalNode is {} instead of {} + {} = {}",
            mem::size_of::<IncrementalNode>(),
            mem::size_of::<PrevIndex>(),
            mem::size_of::<MoveIndex>(),
            mem::size_of::<PrevIndex>() + mem::size_of::<MoveIndex>());
        println!("Theoretical minimum is {} + {} + {} + {} = {}",
            mem::size_of::<PrevIndex>(),
            mem::size_of::<PrevIndex>() + mem::size_of::<MoveIndex>(),
            mem::size_of::<BoardHash>(),
            mem::size_of::<PrevIndex>(),
            mem::size_of::<PrevIndex>() +
            mem::size_of::<PrevIndex>() + mem::size_of::<MoveIndex>() +
            mem::size_of::<BoardHash>() +
            mem::size_of::<PrevIndex>());
        println!("---------------------------------------------");
    }

    unreachable!();
}

fn main() {
    println!("Running with BASE = {}", BASE);
    let start = FullBoard::new();
    println!("Initial board = {:?}", start);
    println!("Hash = {:?}", start.get_hash());
    run(&start);
}
