use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
pub enum ParsePointError {
    Format,
    X,
    Y,
    Z,
}

impl std::str::FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = s.split(',').collect();
        if splits.len() != 3 {
            return Err(ParsePointError::Format);
        }
        let x: i64 = splits[0].parse().map_err(|_| ParsePointError::X)?;
        let y: i64 = splits[1].parse().map_err(|_| ParsePointError::Y)?;
        let z: i64 = splits[2].parse().map_err(|_| ParsePointError::Z)?;

        Ok(Point { x, y, z })
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Point {
    fn abs(&self) -> i64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

#[derive(Debug)]
struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            size: (0..size).map(|_| 1).collect(),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        self.parent[i] = self.find(self.parent[i]);
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) {
        let mut parent_i = self.find(i);
        let mut parent_j = self.find(j);
        if parent_i != parent_j {
            if self.size[parent_i] < self.size[parent_j] {
                std::mem::swap(&mut parent_i, &mut parent_j);
            }
            self.parent[parent_j] = parent_i;
            self.size[parent_i] += self.size[parent_j];
        }
    }
}

fn heapfy(points: &[Point]) -> BinaryHeap<Reverse<(i64, usize, usize)>> {
    let mut heap = BinaryHeap::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; points.len()]; points.len()];

    for (i, a) in points.iter().enumerate() {
        for (j, b) in points.iter().enumerate() {
            if a == b || visited[i][j] || visited[j][i] {
                // println!("already calculated points[{:?}]={:?} points[{:?}]={:?}", i, points[i], j, points[j]);
                continue;
            }
            visited[i][j] = true;
            visited[j][i] = true;
            let distance = (*a - *b).abs();
            heap.push(Reverse((distance, i, j)));
            // println!("P[{:?}]={:?}-P[{:?}]={:?}={:?}", i, points[i], j, points[j], distance);
        }
    }

    heap
}

fn day08_part1(input: String, connections: usize) -> usize {
    let points: Vec<Point> = aoc::read_from_str(&input);
    let mut ordered_edges = heapfy(&points);

    let mut set: DisjointSet = DisjointSet::new(points.len());
    let mut count = 0;
    while let Some(edge) = ordered_edges.pop() {
        if count >= connections {
            break;
        }
        let Reverse((_, i, j)) = edge;
        set.union(i, j);
        count += 1;
    }
    let mut sizes = set.size;
    sizes.sort();
    let mut product = 1;
    for (i, size) in sizes.into_iter().rev().enumerate() {
        if i >= 3 {
            break;
        }
        product *= size;
    }

    product
}

fn day08_part2(input: String) -> i64 {
    let points: Vec<Point> = aoc::read_from_str(&input);
    let mut ordered_edges = heapfy(&points);

    let mut set: DisjointSet = DisjointSet::new(points.len());
    while let Some(edge) = ordered_edges.pop() {
        let Reverse((_, i, j)) = edge;
        set.union(i, j);
        let parent_i = set.find(i);
        let parent_j = set.find(j);
        if set.size[parent_i] == points.len() || set.size[parent_j] == points.len() {
            return points[i].x * points[j].x;
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689".to_string();
        assert_eq!(day08_part1(input, 10), 40);
    }

    #[test]
    fn puzzle_part1() {
        let input = std::fs::read_to_string("input/day_08.txt").expect("Failled to read file");
        assert_eq!(day08_part1(input, 1000), 57564);
    }

    #[test]
    fn example_part2() {
        let input = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689".to_string();
        assert_eq!(day08_part2(input), 25272);
    }

    #[test]
    fn puzzle_part2() {
        let input = std::fs::read_to_string("input/day_08.txt").expect("Failled to read file");
        assert_eq!(day08_part2(input), 133296744);
    }
}
