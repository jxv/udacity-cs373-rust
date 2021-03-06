#![feature(tuple_indexing)]

#[deriving(Show,PartialEq,Clone)]
enum Space {
    O,
    X,
}

#[deriving(Show,PartialEq,Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

static GRID: [[Space, ..6], ..5] = [
    [O, X, O, O, O, O],
    [O, X, O, O, O, O],
    [O, X, O, O, O, O],
    [O, X, O, O, O, O],
    [O, O, O, O, X, O]];

#[allow(dead_code)]
static INIT: (uint, uint) = (0, 0);
static GOAL: (uint, uint) = (5 - 1, 6 - 1);

fn delta((y,x): (uint, uint), dir: Dir) -> Option<(uint, uint)> {
    let (y2, x2) = match dir {
        Up    => (y - 1, x),
        Down  => (y + 1, x),
        Left  => (y, x - 1),
        Right => (y, x + 1),
    };
    if y2 < GRID.len() && x2 < GRID[0].len() {
        Some((y2,x2))
    } else {
        None
    }
}

static DELTA: [Dir, ..4] = [Up, Down, Left, Right];

#[allow(dead_code)]
fn delta_name(dir: Dir) -> char {
    match dir {
        Up => '^',
        Down => 'v',
        Left => '<',
        Right => '>',
    }
}

static COST_STEP: uint = 1;

fn main() {
    for row in compute_value().iter() {
        println!("{}", row);
    }
}

fn compute_value() -> Vec<Vec<uint>> {
    let mut value = Vec::from_elem(GRID.len(), Vec::from_elem(GRID[0].len(), 99u));
    let mut open = vec![(0u, GOAL.0, GOAL.1)];
    *value.get_mut(GOAL.0).get_mut(GOAL.1) = 0u;

    loop {
        let (step, y, x) = match open.pop() {
            None => break,
            Some(next) => next,
        };
        for &d in DELTA.iter() {
            let (y2, x2) = match delta((y,x), d) {
                None => continue,
                Some((y2, x2)) => (y2, x2),
            };
            let step2 = COST_STEP + step;
            if value[y2][x2] > step2 && GRID[y2][x2] == O {
                *value.get_mut(y2).get_mut(x2) = step2;
                open.push((step2, y2, x2));
            }
        };
    }

    value
}
