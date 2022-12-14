use std::collections::HashSet;

use regex::Regex;

use super::Point;

pub fn tuning_freq(input: &str) -> isize {
    let sensors = sensors_and_beacons(input);
    const MAX: usize = 20;

    for y in 0..MAX {
        let beaconless = beaconless(&sensors, y as isize)
            .into_iter()
            .flat_map(|range| (range.0..range.1).collect::<HashSet<_>>())
            .collect::<HashSet<_>>();
        for x in 0..MAX {
            if !beaconless.contains(&(x as isize)) {
                println!("{x}, {y}");
            }
        }
    }

    0
}

pub fn n_beaconless(input: &str) -> isize {
    let sensors = sensors_and_beacons(input);

    beaconless(&sensors, 10)
        .into_iter()
        .flat_map(|range| (range.0..range.1).collect::<HashSet<_>>())
        .collect::<HashSet<_>>()
        .len() as isize
}

fn beaconless(sensors: &[(Point, Point, usize)], row: isize) -> Vec<(isize, isize)> {
    sensors.iter().fold(Vec::new(), |mut acc, &(s, b, d)| {
        let delta = isize::abs_diff(s.y, row);
        if d < delta {
            acc
        } else {
            let wing_span = d as isize - delta as isize;
            if b.y == row {
                let left = ((s.x as isize - wing_span), b.x);
                let right = (b.x, (s.x as isize + wing_span));
                acc.push(left);
                acc.push(right);
            } else {
                acc.push(((s.x as isize - wing_span), (s.x as isize + wing_span)));
            }
            acc
        }
    })
}

fn sensors_and_beacons(input: &str) -> Vec<(Point, Point, usize)> {
    let points = Regex::new(&format!(
        "{0}{2}, {1}{2}:.*{0}{2}, {1}{2}",
        "x=", "y=", r"(-?\d+)"
    ))
    .unwrap();

    input
        .lines()
        .map(|line| {
            let numbers = points.captures(line).unwrap();
            // first capture group is entire match
            let (s, b) = (
                Point {
                    x: numbers[1].parse::<isize>().unwrap(),
                    y: numbers[2].parse::<isize>().unwrap(),
                },
                Point {
                    x: numbers[3].parse::<isize>().unwrap(),
                    y: numbers[4].parse::<isize>().unwrap(),
                },
            );
            (s, b, s.manhattan_distance(&b))
        })
        .collect::<Vec<_>>()
}
