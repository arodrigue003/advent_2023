use crate::common::{test_part_one_common, test_part_two_common};
use crate::dayxx::DayXX;

static INPUT_EXAMPLE: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

#[test]
fn test_part_one() {
    test_part_one_common(DayXX::default(), INPUT_EXAMPLE, 54);
}

#[test]
fn test_part_two() {
    test_part_two_common(DayXX::default(), INPUT_EXAMPLE, 0);
}
