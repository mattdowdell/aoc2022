//! ...

use std::cmp::{max, min};
use std::collections::HashSet;
use std::error::Error;
use std::io::{BufRead, Lines};
use std::str::FromStr;

use itertools::Itertools;

pub struct Map {
    sensors: Vec<Sensor>,
}

impl Map {
    ///
    pub fn from<T>(lines: Lines<T>) -> Result<Self, Box<dyn Error>>
    where
        T: BufRead,
    {
        let mut sensors = Vec::new();

        for line in lines {
            let line = line?;

            if line.is_empty() {
                continue;
            }

            let (sensor_str, beacon_str) = line.split_once(':').ok_or("failed to split line")?;

            let mut sensor = Sensor::from_str(sensor_str)?;
            let beacon = Beacon::from_str(beacon_str)?;

            sensor.closest = beacon.location;
            sensors.push(sensor);
        }

        Ok(Self { sensors })
    }

    ///
    pub fn covers(&self, target_row: i32) -> i32 {
        let ranges = self.ranges(target_row);
        // println!("{:?}", ranges);

        ranges.iter().fold(0, |acc, r| acc + r.len()) - self.items_on(target_row)
    }

    ///
    pub fn find_hole(&self, target_row: i32) -> Option<i128> {
        let ranges = self.ranges(target_row);
        // println!("i={}, {:?}", target_row, ranges);

        if ranges.len() == 1 {
            None
        } else {
            Some(ranges[0].end as i128 + 1)
        }
    }

    ///
    pub fn ranges(&self, target_row: i32) -> Vec<Range> {
        let mut ranges = Vec::new();

        for sensor in self.sensors.iter() {
            if let Some(range) = sensor.range(target_row) {
                ranges.push(range);
            }
        }

        ranges.sort_by_key(|x| x.start);

        ranges
            .into_iter()
            .coalesce(|a, b| {
                if a.touches(&b) {
                    Ok(a.merge(&b))
                } else {
                    Err((a, b))
                }
            })
            .collect()
    }

    ///
    pub fn items_on(&self, target_row: i32) -> i32 {
        // TODO: handle beacons being outside the range?
        let mut total = 0;
        let mut beacons = HashSet::new();

        for sensor in self.sensors.iter() {
            if sensor.location.y == target_row {
                // println!("sensor {:?}", sensor.location);
                total += 1;
            }

            beacons.insert(sensor.closest);
        }

        for beacon in beacons.iter() {
            if beacon.y == target_row {
                // println!("beacon {:?}", beacon);
                total += 1;
            }
        }

        total
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Item {
    Sensor,
    Beacon,
    Covered,
}

impl From<Item> for char {
    fn from(item: Item) -> Self {
        match item {
            Item::Sensor => 'S',
            Item::Beacon => 'B',
            Item::Covered => '#',
        }
    }
}

pub struct Sensor {
    pub location: Coordinate,
    pub closest: Coordinate,
}

impl Sensor {
    ///
    pub fn range(&self, target_row: i32) -> Option<Range> {
        let distance = self.location.manhattan_distance(self.closest);
        let offset = (self.location.y - target_row).abs();

        if distance >= offset {
            let range = Range::from_mid_point(self.location.x, distance - offset);
            // println!(
            //     "sensor in range {:?} ({}) {:?}",
            //     self.location, distance, range
            // );
            Some(range)
        } else {
            None
        }
    }
}

impl FromStr for Sensor {
    type Err = Box<dyn Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value
            .strip_prefix("Sensor at x=")
            .ok_or("missing expected sensor prefix")?;
        let (x, value) = value.split_once(',').ok_or("missing expected delimiter")?;
        let y = value.strip_prefix(" y=").ok_or("missing expected prefix")?;

        let location = Coordinate::new(x.parse()?, y.parse()?);

        Ok(Self {
            location,
            closest: Coordinate::default(),
        })
    }
}

pub struct Beacon {
    pub location: Coordinate,
}

impl FromStr for Beacon {
    type Err = Box<dyn Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value
            .strip_prefix(" closest beacon is at x=")
            .ok_or("missing beacon expected prefix")?;
        let (x, value) = value.split_once(',').ok_or("missing expected delimiter")?;
        let y = value.strip_prefix(" y=").ok_or("missing expected prefix")?;

        let location = Coordinate::new(x.parse()?, y.parse()?);

        Ok(Self { location })
    }
}

///
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    ///
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    ///
    pub fn manhattan_distance(&self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub distance: i32,
    pub location: Coordinate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Range {
    pub start: i32,
    pub end: i32,
}

impl Range {
    ///
    pub fn from_mid_point(mid_point: i32, width: i32) -> Self {
        Self {
            start: mid_point - width,
            end: mid_point + width,
        }
    }

    ///
    pub fn touches(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }

    ///
    pub fn merge(&self, other: &Self) -> Self {
        if !self.touches(other) {
            todo!();
        }

        Self {
            start: min(self.start, other.start),
            end: max(self.end, other.end),
        }
    }

    ///
    pub fn len(&self) -> i32 {
        self.end - self.start + 1
    }
}
