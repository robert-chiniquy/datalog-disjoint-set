use ascent::ascent;
use ascent::Dual;

use common::Collection;
use common::PartitionVec;

pub fn run(max: usize) -> Result<PartitionVec<u32>, Box<dyn std::error::Error>> {
    let mut prog = AscentProgram::default();
    prog.input = (0..max).map(|i| (i,)).collect();
    prog.run();

    let mut c = Collection::new();
    for (p1, p2) in prog.parent.into_iter().map(|(o, d)| (o, d.0)) {
        c.union(p1 as u32, p2 as u32);
    }
    Ok(c.items())
}

type O = usize;
type P = usize;

ascent! {
    relation input(O);

    input(o)  <-- for o in (1..1000);

    lattice parent(O, Dual<P>);

    parent(O1, Dual(*O2)), parent(O2, Dual(*O1)) <-- input(O1), input(O2), if equality(*O1,*O2);


}

pub fn equality(a: O, b: O) -> bool {
    (a % 2 == 0 && b % 2 == 0)
        || (a % 3 == 0 && b % 3 == 0)
        || (a % 2 != 0 && b % 2 != 0 && a % 3 != 0 && b % 3 != 0)
}
