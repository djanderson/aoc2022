/// Day 12: Hill Climbing Algorithm
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fs;
use std::rc::Rc;

type Coordinates = (usize, usize);
type GridSquare = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    coordinates: Coordinates,
    height: u8,
    distance: usize,
}

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut heightmap: Vec<Vec<GridSquare>> = vec![];
    let mut heap: BinaryHeap<GridSquare> = BinaryHeap::new();
    let mut visited: HashSet<Coordinates> = HashSet::new();
    let mut start: Coordinates = Default::default();
    let mut end: Coordinates = Default::default();

    // Initialize "heightmap"
    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (j, height) in line.bytes().enumerate() {
            let coordinates = (i, j);
            let mut node = Node::new(coordinates, height, usize::MAX);
            match height {
                b'S' => {
                    start = coordinates;
                    node.height = b'a';
                    node.distance = 0;
                }
                b'E' => {
                    end = coordinates;
                    node.height = b'z';
                }
                _ => {}
            }
            row.push(Rc::new(RefCell::new(node)));
        }
        heightmap.push(row);
    }

    // I use Dijkstra's Algorithm with an edge cost of 1 for this problem:
    //
    // 1. initialize start node to distance 0, and all other nodes to infinity (usize::MAX)
    // 2. add our start node to a min-heap
    // 3. pop the next position off the heap in order of smallest distance
    //    (if it's the end, we're done)
    // 4. search its neighbors that:
    //   - have a "height" no more than one above our current position, and
    //   - that the algorithm has not already "visited"
    // 5. if the current position's distance + 1 (the one step that it would take to move to
    //    the neighbor) is less than the neighbors existing distance, update it and add it
    //    into the heap.
    // 6. Add the current position to the "visited" set.
    // 7. goto 3.

    heap.push(heightmap[start.0][start.1].clone());

    while let Some(position) = heap.pop() {
        let coordinates = position.borrow().coordinates;
        if coordinates == end {
            // Found optimal path to end position, print distance in number of steps
            println!("Part 1: {}", position.borrow().distance);
            break;
        }
        for neighbor in neighbors(&position, &heightmap, &visited) {
            let new_distance = position.borrow().distance + 1;
            let existing_distance = neighbor.borrow().distance;
            if new_distance < existing_distance {
                // We found a better route to the neighbor, add it to frontier
                neighbor.borrow_mut().distance = new_distance;
                heap.push(neighbor.clone());
            }
        }
        visited.insert(coordinates);
    }
}

/// Determine a vector of neighbors that have yet to be visited that we can move to.
fn neighbors(
    position: &GridSquare,
    heightmap: &Vec<Vec<GridSquare>>,
    visited: &HashSet<Coordinates>,
) -> Vec<GridSquare> {
    let mut result = vec![];
    let (row, col) = position.borrow().coordinates;
    let height = position.borrow().height;

    let mut check_neighbor = |r: usize, c: usize| {
        let node = &heightmap[r][c];
        if !visited.contains(&(r, c)) && node.borrow().height <= height + 1 {
            result.push(Rc::clone(node));
        }
    };

    // Look up.
    if row > 0 {
        check_neighbor(row - 1, col);
    }
    // Look down.
    if row < heightmap.len() - 1 {
        check_neighbor(row + 1, col);
    }
    // Look left.
    if col > 0 {
        check_neighbor(row, col - 1);
    }
    // Look right.
    if col < heightmap[row].len() - 1 {
        check_neighbor(row, col + 1);
    }

    result
}

impl Node {
    fn new(coordinates: Coordinates, height: u8, distance: usize) -> Self {
        Node {
            coordinates,
            height,
            distance,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coordinates == other.coordinates
    }
}

impl Eq for Node {}

// Reversed ordering so that we have a min-heap
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
