use common::{Collection, PartitionVec};

pub fn run(max: usize) -> PartitionVec<u32> {
    let mut c = Collection::new();
    for p1 in 1..max {
        for p2 in 1..max {
            if equality(p1, p2) {
                c.union(p1 as u32, p2 as u32);
            }
        }
    }
    c.items()
}

type O = usize;
type P = usize;

pub fn equality(a: O, b: O) -> bool {
    (a % 2 == 0 && b % 2 == 0)
        || (a % 3 == 0 && b % 3 == 0)
        || (a % 2 != 0 && b % 2 != 0 && a % 3 != 0 && b % 3 != 0)
}
