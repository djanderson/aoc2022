/// Day 15: Beacon Exclusion Zone
use std::collections::BTreeSet;
use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let telemetry: Vec<_> = input
        .lines()
        .map(|l| Telemetry::from_str(l).unwrap())
        .collect();

    println!("Part 1: {}", part1(&telemetry, 2_000_000));
    println!("Part 2: {}", part2(&telemetry));
}

// Algorithm:
//
// 1. Scan through input and determine (min, max) value of X
// 2. For y=2000000
// 3.   Find the ranges of X covered by the sensors
// 4.   Subtract the total covered range from (max_x - min_x)
//
// NOTE: The number of sensors is very small, so there is no benefit to merging the ranges
//       covered.
fn part1(telemetry: &[Telemetry], target_y: i32) -> i32 {
    // Find the range of x covered by each sensor at row `target_y`
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut ranges: Vec<RangeInclusive<i32>> = vec![];
    let mut target_line_beacons: BTreeSet<i32> = BTreeSet::new();

    for pkt in telemetry.iter() {
        let mdist = pkt.distance as i32;
        let ydist = pkt.sensor.y.abs_diff(target_y) as i32;
        let xdist = mdist - ydist;
        if xdist < 0 {
            continue; // this sensor covers none of the target row
        }
        let x_low = pkt.sensor.x - xdist;
        let x_high = pkt.sensor.x + xdist;
        min_x = min_x.min(x_low);
        max_x = max_x.max(x_high);
        ranges.push(RangeInclusive::new(x_low, x_high));

        // Beacons may not be unique, so use BST to ensure uniqueness
        if pkt.beacon.y == target_y {
            target_line_beacons.insert(pkt.beacon.x);
        }
    }

    // Count locations on the target line covered by sensors
    let mut count = 0;
    for i in min_x..=max_x {
        let covered = ranges.iter().any(|r| r.contains(&i));
        let beacon = target_line_beacons.contains(&i);
        if covered && !beacon {
            count += 1;
        }
    }

    count
}

// Algorithm:
//
// The search space 4000000 * 4000000 is far too large to brute force, but we can
// reduce the search space by realizing that if there is exactly one point in [0, 4000000]
// that is not covered by any sensor, than this point must necessarily be on the boundary
// of multiple sensors.
//
// 1. Create an iterator over the "boundary" of a sensor, i.e., all the points 1 step
//    farther than the distance to a sensor's closest beacon.
// 2. For each boundary point of each sensor, determine whether another sensor would have detected
//    a beacon there by calulating the distance between that point and each other sensor.
// 3. If any boundary point is not covered by any other sensor, it must be the distress beacon.
fn part2(telemetry: &[Telemetry]) -> isize {
    fn in_bounds(p: &Point) -> bool {
        p.x >= 0 && p.y >= 0 && p.x <= 4000000 && p.y <= 4000000
    }
    for (i, this) in telemetry.iter().enumerate() {
        let boundary = SensorBoundary::new(this.sensor.clone(), this.distance);
        for point in boundary {
            if !in_bounds(&point) {
                continue;
            }
            let mut covered = false;
            let other_sensors =
                telemetry
                    .iter()
                    .enumerate()
                    .filter_map(|(j, s)| if j != i { Some(s) } else { None });
            for other in other_sensors {
                if other.sensor.manhattan_distance(&point) <= other.distance {
                    covered = true;
                    break;
                }
            }
            if !covered {
                return point.x as isize * 4000000 + point.y as isize;
            }
        }
    }
    panic!("Not found");
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // https://en.wikipedia.org/wiki/Taxicab_geometry
    fn manhattan_distance(&self, other: &Point) -> u32 {
        let x_distance = self.x.abs_diff(other.x);
        let y_distance = self.y.abs_diff(other.y);
        x_distance + y_distance
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseTelemetryError;

impl FromStr for Point {
    type Err = ParseTelemetryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_s, y_s) = s
            .strip_prefix("x=")
            .and_then(|s| s.split_once(", y="))
            .ok_or(ParseTelemetryError)?;

        let x = x_s.parse::<i32>().map_err(|_| ParseTelemetryError)?;
        let y = y_s.parse::<i32>().map_err(|_| ParseTelemetryError)?;

        Ok(Point { x, y })
    }
}

#[derive(Debug)]
struct Telemetry {
    sensor: Point,
    beacon: Point,
    distance: u32,
}

impl FromStr for Telemetry {
    type Err = ParseTelemetryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((sensor_telem, beacon_telem)) = s.split_once(':') else {
            return Err(ParseTelemetryError);
        };
        let sensor_s = &sensor_telem[sensor_telem.find('x').ok_or(ParseTelemetryError)?..];
        let sensor = Point::from_str(sensor_s)?;
        let beacon_s = &beacon_telem[beacon_telem.find('x').ok_or(ParseTelemetryError)?..];
        let beacon = Point::from_str(beacon_s)?;
        let distance = sensor.manhattan_distance(&beacon);

        Ok(Telemetry {
            sensor,
            beacon,
            distance,
        })
    }
}

/// An iterator for points around (just outside) a sensor's boundary
#[derive(Debug)]
struct SensorBoundary {
    sensor: Point,
    distance: u32,
    current: Option<Point>,
    size: usize,
}

impl SensorBoundary {
    fn new(sensor: Point, distance: u32) -> Self {
        SensorBoundary {
            sensor,
            distance,
            current: None,
            // Consider distance to left and right (x2), above and below each (x2)
            // and directly left, top, bottom, and right of center.
            size: distance as usize * 4 + 4,
        }
    }
}

/// Iterate points around a sensor boundary.
///
/// Given a sensor (S) and beacon (B) in the following configuration (distance 1),
/// the perimeter (P) will move from left to right, heigh to low, as follows:
///
/// .....    .....    .....    ..P..    .....    .....    .....    .....
/// ..#..    .P#..    ..#..    ..#..    ..#..    ..#P.    ..#..    ..#..
/// P#SB. -> .#SB. -> .#SB. -> .#SB. -> .#SB. -> .#SB. -> .#SB. -> .#SBP
/// ..#..    ..#..    .P#..    ..#..    ..#..    ..#..    ..#P.    ..#..
/// .....    .....    .....    .....    ..P..    .....    .....    .....
///
impl Iterator for SensorBoundary {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let Point {
            x: sensor_x,
            y: sensor_y,
        } = self.sensor;
        if self.size == 0 {
            return None;
        }

        self.size -= 1;

        let distance = self.distance as i32;

        // First perimeter point
        let Some(Point { x: mut current_x, y: mut current_y }) = self.current else {
            let current = Point { x: sensor_x - distance - 1, y: sensor_y };
            self.current = Some(current.clone());
            return Some(current);
        };

        if current_y < sensor_y {
            current_y = sensor_y + (distance - sensor_x.abs_diff(current_x) as i32) + 1;
        } else {
            current_x += 1;
            current_y = sensor_y - (distance - sensor_x.abs_diff(current_x) as i32) - 1;
        }

        self.current = Some(Point {
            x: current_x,
            y: current_y,
        });
        self.current.clone()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

impl ExactSizeIterator for SensorBoundary {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boundary_values() {
        let s = "Sensor at x=2, y=2: closest beacon is at x=3, y=2";
        let t = Telemetry::from_str(s).unwrap();
        let b = SensorBoundary::new(t.sensor, t.distance);
        let actual: Vec<Point> = b.collect();
        let expected = vec![
            Point { x: 0, y: 2 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 3 },
            Point { x: 2, y: 0 },
            Point { x: 2, y: 4 },
            Point { x: 3, y: 1 },
            Point { x: 3, y: 3 },
            Point { x: 4, y: 2 },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn boundary_len() {
        let s = "Sensor at x=2, y=2: closest beacon is at x=3, y=2";
        let t = Telemetry::from_str(s).unwrap();
        let mut b = SensorBoundary::new(t.sensor, t.distance);
        assert_eq!(b.len(), 8);
        _ = b.next();
        assert_eq!(b.len(), 7);
        for _ in 0..6 {
            _ = b.next();
        }
        assert_eq!(b.len(), 1);
        assert!(b.next().is_some());
        assert!(b.next().is_none());
        assert_eq!(b.len(), 0);
    }
}
