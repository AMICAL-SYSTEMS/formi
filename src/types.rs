/// As defined in [`https://www.taygeta.com/forth/dpansa3.htm#A.3.1.3`]:
///
/// > A single-cell stack entry viewed without regard to typing is the
/// > fundamental data type of Forth. All other data types are actually
/// > represented by one or more single-cell stack entries.
pub type Cell = u64;
pub type CellPair = (Cell, Cell);

#[inline]
pub fn cell_pair_to_double_number(pair: CellPair) -> UnsignedDoubleCellInteger {
    // SAFETY: (u64, u64) (which is CellPair) and u128 (which is UnsignedDoubleCellInteger)
    // are homomorphic, and their values can be interchanged in memory, as they are
    // valid in all cases. Because we're transmuting values (not pointers), alignment
    // is not a concern.
    unsafe { std::mem::transmute::<CellPair, UnsignedDoubleCellInteger>(pair) }
}

#[inline]
pub fn double_number_to_cell_pair(ud: UnsignedDoubleCellInteger) -> CellPair {
    // SAFETY: (u64, u64) (which is CellPair) and u128 (which is UnsignedDoubleCellInteger)
    // are homomorphic, and their values can be interchanged in memory, as they are
    // valid in all cases. Because we're transmuting values (not pointers), alignment
    // is not a concern.
    unsafe { std::mem::transmute::<UnsignedDoubleCellInteger, CellPair>(ud) }
}

/// As defined in [`https://www.taygeta.com/forth/dpansa3.htm#A.3.1.3.1`]:
///
/// > A FALSE ([`FALSE_FLAG`]) flag is a single-cell datum with all bits unset,
/// > and a TRUE flag ([`TRUE_FLAG`]) is a single-cell datum with all bits set.
pub type Flag = Cell;

pub const TRUE_FLAG: Flag = u64::MAX;
pub const FALSE_FLAG: Flag = 0u64;

pub type Number = i64;
pub type UnsignedInteger = u64;
pub type UnsignedDoubleCellInteger = u128;
pub type SignedInteger = i64;
pub type SignedDoubleCellInteger = i128;

/// As defined in [`https://www.taygeta.com/forth/dpansa3.htm#A.3.1.3.3`]:
///
/// > An address is uniquely represented as a single cell unsigned number and
/// > can be treated as such when being moved to, from, or upon the stack.
/// > Conversely, each unsigned number represents a unique address (which
/// > is not necessarily an address of accessible memory). This one-to-one
/// > relationship between addresses and unsigned numbers forces an equivalence
/// > between address arithmetic and the corresponding operations on unsigned numbers.
pub type Address = UnsignedInteger;
