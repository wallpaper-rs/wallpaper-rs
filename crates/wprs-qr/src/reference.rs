use crate::Code;
use crate::tests::decode;

fn rows(text: &str) -> Vec<String> {
    let code = Code::new(text).unwrap();
    (0..code.size())
        .map(|y| (0..code.size()).map(|x| if code.dark(x, y) { '#' } else { '.' }).collect())
        .collect()
}

const GOLDEN: [&str; 21] = [
    "#######...##..#######",
    "#.....#.#...#.#.....#",
    "#.###.#...###.#.###.#",
    "#.###.#...##..#.###.#",
    "#.###.#..#..#.#.###.#",
    "#.....#.#...#.#.....#",
    "#######.#.#.#.#######",
    "........####.........",
    "....####......##...#.",
    ".###...##...#..#.#.##",
    "..#..##..#.##.#####..",
    "#..###.#.##..#..#.#..",
    ".#.#..##.##.#.....#.#",
    "........#..##.....#.#",
    "#######.##..#.#.....#",
    "#.....#.#..#..##.#...",
    "#.###.#.##.....######",
    "#.###.#...###..#.#.##",
    "#.###.#..##....####..",
    "#.....#..#.##...#.#.#",
    "#######..##.###...###",
];

#[test]
fn reference_matrix_of_a_small_input() {
    assert_eq!(rows("A"), GOLDEN);
    assert_eq!(decode(&Code::new("A").unwrap()), "A");
}

#[test]
fn fixed_patterns_of_every_size() {
    for text in ["A", &"b".repeat(60), &"c".repeat(300)] {
        let code = Code::new(text).unwrap();
        let n = code.size();
        assert_eq!((n - 17) % 4, 0);
        for (cx, cy) in [(0, 0), (n - 7, 0), (0, n - 7)] {
            for y in 0..7 {
                for x in 0..7 {
                    let ring = x == 0 || y == 0 || x == 6 || y == 6;
                    let core = (2..=4).contains(&x) && (2..=4).contains(&y);
                    assert_eq!(code.dark(cx + x, cy + y), ring || core);
                }
            }
        }
        for i in 8..n - 8 {
            assert_eq!(code.dark(i, 6), i % 2 == 0);
            assert_eq!(code.dark(6, i), i % 2 == 0);
        }
        assert!(code.dark(8, n - 8));
    }
}
