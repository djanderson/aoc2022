/// Day 15: Beacon Exclusion Zone
use std::collections::BTreeSet;
use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;

// Algorithm:
//
// 1. Scan through input and determine (min, max) value of X
// 2. For y=2000000
// 3.   Find the ranges of X covered by the sensors
// 4.   Subtract the total covered range from (max_x - min_x)
//
// NOTE: The number of sensors is very small, so there is no benefit to merging the ranges
//       covered.
pub fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let telemetry: Vec<_> = input
        .lines()
        .map(|l| Telemetry::from_str(l).unwrap())
        .collect();

    let target_y: i32 = 2000000;

    // Find the range of x covered by each sensor at row `target_y`
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut ranges: Vec<RangeInclusive<i32>> = vec![];
    let mut target_line_beacons: BTreeSet<i32> = BTreeSet::new();

    for pkt in telemetry.iter() {
        let mdist = pkt.manhattan_distance() as i32;
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

    println!("Part 1: {count}");
}

#[derive(Debug, PartialEq)]
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
}

impl Telemetry {
    fn manhattan_distance(&self) -> u32 {
        self.sensor.manhattan_distance(&self.beacon)
    }
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

        Ok(Telemetry { sensor, beacon })
    }
}
