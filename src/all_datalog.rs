use ascent::ascent;
use ascent::Dual;

pub fn run(max: usize) -> Vec<(usize, usize)> {
    let mut prog = AscentProgram::default();
    prog.input = (0..max).map(|i| (i,)).collect();
    prog.run();
    prog.parent.into_iter().map(|(o, d)| (o, d.0)).collect()
}

type O = usize;
type P = usize;

ascent! {
    relation input(O);

    lattice parent(O, Dual<P>);

    parent(O1, Dual(*O2)), parent(O2, Dual(*O1)) <-- input(O1), input(O2), if equality(*O1,*O2);

    // This rule reduces the forest to stars
    parent(O1, O3), parent(O2.0, Dual(*O1)) <-- parent(O1, O2), parent(O2.0, O3), if *O1 != O2.0, if O2 != O3, if *O1 != O3.0;

}

pub fn equality(a: O, b: O) -> bool {
    (a % 2 == 0 && b % 2 == 0)
        || (a % 3 == 0 && b % 3 == 0)
        || (a % 2 != 0 && b % 2 != 0 && a % 3 != 0 && b % 3 != 0)
}
