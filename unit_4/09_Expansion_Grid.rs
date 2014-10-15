#![feature(tuple_indexing)]

#[deriving(Show,PartialEq,Clone)]
enum Space {
    O,
    X,
}

static GRID: [[Space, ..6], ..5] = [
    [O, O, X, O, O, O],
    [O, O, X, O, O, O],
    [O, O, O, O, X, O],
    [O, O, X, X, X, O],
    [O, O, O, O, X, O]];

static INIT: (int, int) = (0, 0);
static GOAL: (int, int) = (5-1, 6-1);

static DELTA: [(int, int), ..4] = [
    (-1, 0),
    (0, -1),
    (1, 0),
    (0, 1)];

#[allow(dead_code)]
static DELTA_NAME: [char, ..4] = ['^', '<','v','>'];

static COST: int = 1;


fn main() {
    // expected: (4, 5, 11)
    for row in search().iter() {
        println!("{}", row);
    }
}


fn search() -> Vec<Vec<int>> {
    let mut closed = Vec::from_elem(GRID.len(), Vec::from_elem(GRID[0].len(), false));
    *closed.get_mut(INIT.0 as uint).get_mut(INIT.1 as uint) = true;

    let mut expand = Vec::from_elem(GRID.len(), Vec::from_elem(GRID[0].len(), -1));
    *expand.get_mut(INIT.0 as uint).get_mut(INIT.1 as uint) = 0i;
    let mut exp_id = 1i;

    let mut open = vec![(0i, INIT.0, INIT.1)];
    let mut found = false;
    let mut resign = false;

    while !found && !resign {
        if open.len() == 0 {
            resign = true;
        } else {
            open.sort();
            let (g, y, x) = open.pop().unwrap();
            if y == GOAL.0 && x == GOAL.1 {
                found = true;
            } else {
                for d in DELTA.iter() {
                    let y2 = y + d.0;
                    let x2 = x + d.1;

                    let yu = y2 as uint;
                    let xu = x2 as uint;
                    if y2 >= 0 && yu < GRID.len() && x2 >= 0 && xu < GRID[0].len() {
                        if !closed[yu][xu] && GRID[yu][xu] == O {
                            let g2 = g + COST;
                            open.push((g2, y2, x2));
                            *closed.get_mut(yu).get_mut(xu) = true;
                            *expand.get_mut(yu).get_mut(xu) = exp_id;
                            exp_id += 1;
                        }
                    }
                }
            }
        }
    }
    expand
}

/*
fn search() -> Option<(int, int, int)> {
    let mut open = Vec::new();
    let mut close = Vec::new();

    open.push((INIT.0, INIT.1, 0i));

    loop {
        let cell = match open.pop() {
            None => break,
            Some(cell) => cell,
        };
        close.push(cell);
        
        // Test for reached goal?
        if cell.0 == GOAL.0 && cell.1 == GOAL.1 {
            return Some(cell);
        }

        // Scan for neighbors.
        let mut neighbors = Vec::new();
        for d in DELTA.iter() {
            let n = (cell.0 + d.0, cell.1 + d.1, cell.2 + COST);
            // Within grid?
            if n.0 >= 0 && n.0 < GRID.len() as int && n.1 >= 0 && n.1 < GRID[0].len() as int {
                neighbors.push(n);
            }
        }

        // Is the neighboring space an open space?
        for &n in neighbors.iter() {
            if GRID[n.0 as uint][n.1 as uint] == X {
                // No! Add it to the close vec to ignore it!
                close.push(n);
            }
        }

        // Add all non-scanned neighbors to the open vec.
        // Ignore previously scanned neighbors.
        for &n in neighbors.iter() {
            if     !open.iter().any(|node|  { node.0 == n.0 && node.1 == n.1 })
                && !close.iter().any(|node| { node.0 == n.0 && node.1 == n.1 }) {
                open.push(n);
            }
        }
    }

    None
}
*/
