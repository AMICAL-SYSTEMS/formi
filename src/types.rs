/// As defined in [`https://www.taygeta.com/forth/dpansa3.htm#A.3.1.3`]:
///
/// > A single-cell stack entry viewed without regard to typing is the
/// > fundamental data type of Forth. All other data types are actually
/// > represented by one or more single-cell stack entries.
pub type Cell = u64;

/// As defined in [`https://www.taygeta.com/forth/dpansa3.htm#A.3.1.3.1`]:
///
/// > A FALSE ([`FALSE_FLAG`]) flag is a single-cell datum with all bits unset,
/// > and a TRUE flag ([`TRUE_FLAG`]) is a single-cell datum with all bits set.
pub type Flag = Cell;

pub const TRUE_FLAG: Flag = u64::MAX;
pub const FALSE_FLAG: Flag = 0u64;

pub type Number = i64;
pub type UnsignedInteger = u64;
pub type SignedInteger = i64;

/// As defined in [`https://www.taygeta.com/forth/dpansa3.htm#A.3.1.3.3`]:
///
/// > An address is uniquely represented as a single cell unsigned number and
/// > can be treated as such when being moved to, from, or upon the stack.
/// > Conversely, each unsigned number represents a unique address (which
/// > is not necessarily an address of accessible memory). This one-to-one
/// > relationship between addresses and unsigned numbers forces an equivalence
/// > between address arithmetic and the corresponding operations on unsigned numbers.
pub type Address = UnsignedInteger;
