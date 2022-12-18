use regex::Regex;
use std::env;
use std::fs;

struct SensorRange {
    start_inclusive: i32,
    end_inclusive: i32
}

impl SensorRange {
    fn get_absolute_range(&self) -> i32 {
        (self.start_inclusive - self.end_inclusive).abs() + 1
    }
}


#[derive(Debug)]
struct Sensor {
    position_x: i32,
    position_y: i32,

    closest_beacon_x: i32,
    closest_beacon_y: i32
}

impl Sensor {
    fn get_sensor_radius(&self) -> i32 {
        return (self.position_x - self.closest_beacon_x).abs() + (self.position_y - self.closest_beacon_y).abs();
    }

    fn get_row_coverage_range(&self, row: i32) -> Option<SensorRange> {
        let radius = self.get_sensor_radius(); // DOES NOT INCLUDE CENTER POSITION

        let lowest_coverage_position = self.position_y - radius;
        let highest_coverage_position = self.position_y + radius;

        if row < lowest_coverage_position || row > highest_coverage_position {
            return None;
        }

        // compute the length of the distance between where the radius matches the x-left most portion
        let hypotenus = radius;
        let y_dis = (self.position_y - row).abs();
        let x_dis = hypotenus - y_dis; // center position doesn't count


        let left_most_x = self.position_x - x_dis;
        let right_most_x_inclusive = self.position_x + x_dis;

        Some(SensorRange {
            start_inclusive: left_most_x,
            end_inclusive: right_most_x_inclusive
        })
    }
}

fn condense_sensor_ranges(sensor_ranges: &mut Vec<SensorRange>) -> () {
    for x in 0..sensor_ranges.len() - 1 {
        for forward in x+1..sensor_ranges.len() {
            if sensor_ranges[x].end_inclusive < sensor_ranges[forward].start_inclusive {
                // easy case, ignore, done looking forward
            } else {
                // move the start index forward to the end index
                sensor_ranges[forward].start_inclusive = sensor_ranges[x].end_inclusive + 1;
                // we'll prune defunct ranges at the end

            }
        }
    }

    sensor_ranges.retain(|s| s.start_inclusive <= s.end_inclusive);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Problem 1: {}", problem_1(&contents, 2000000));
    println!("Problem 2: {}", problem_2(&contents, 4000000));
}

fn problem_1(input: &str, row_to_search: i32) -> usize {
    let sensors = parse_input(input);

    let mut beacons_in_row = sensors.iter()
        .filter(|s| s.closest_beacon_y == row_to_search)
        .map(|s| (s.closest_beacon_x, s.closest_beacon_y))
        .collect::<Vec<(i32, i32)>>();
    beacons_in_row.dedup();
    let beacons_in_row = beacons_in_row.len();

    let mut sensor_ranges = sensors.iter().map(|s| s.get_row_coverage_range(row_to_search)).filter(|s| s.is_some()).map(|s| s.unwrap()).collect::<Vec<SensorRange>>();
    sensor_ranges.sort_by(|a, b| a.start_inclusive.cmp(&b.start_inclusive));
    condense_sensor_ranges(&mut sensor_ranges);

    let mut sensor_length:usize = 0;
    for sensor in sensor_ranges {
        sensor_length += sensor.get_absolute_range() as usize;
    }

    sensor_length - beacons_in_row
}

fn problem_2(input: &str, acceptance_range: usize) -> usize {
    let sensors = parse_input(input);

    for y in 0..acceptance_range {
        let mut sensor_ranges = sensors.iter().map(|s| s.get_row_coverage_range(y.try_into().unwrap())).filter(|s| s.is_some()).map(|s| s.unwrap()).collect::<Vec<SensorRange>>();
        sensor_ranges.sort_by(|a, b| a.start_inclusive.cmp(&b.start_inclusive));
        condense_sensor_ranges(&mut sensor_ranges);

        if sensor_ranges[0].start_inclusive > 0 {
            // its the first element that has a gap!
            return y;
        }

        for x in 0..sensor_ranges.len() - 1 {
            // check if there is a gap in this row inbetween the sensors
            if sensor_ranges[x].end_inclusive + 1 < sensor_ranges[x+1].start_inclusive {
                // we have found the gap!
                return y + ((sensor_ranges[x].end_inclusive as usize + 1) * 4000000);
            }
        }
    }

    unreachable!();
}

fn parse_input(input: &str) -> Vec<Sensor> {
    let re = Regex::new(r"(-?\d+)").unwrap();

    let stripped_values = re.captures_iter(input).map(|cap| cap[1].parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut sensors = vec![];
    for x in (0..stripped_values.len()).step_by(4) {
        sensors.push(Sensor {
            position_x: stripped_values[x],
            position_y: stripped_values[x + 1],
            closest_beacon_x: stripped_values[x + 2],
            closest_beacon_y: stripped_values[x + 3],
        });

    }

    sensors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn condense_sensor_ranges_single_nothing() {
        let mut sensors = vec![
            SensorRange {
                start_inclusive: 0,
                end_inclusive: 5
            }
        ];

        condense_sensor_ranges(&mut sensors);
        assert_eq!(1, sensors.len());
        assert_eq!(0, sensors[0].start_inclusive);
        assert_eq!(5, sensors[0].end_inclusive);
        assert_eq!(6, sensors[0].get_absolute_range());
    }

    #[test]
    fn condense_sensor_ranges_mutli_nothing() {
        let mut sensors = vec![
            SensorRange {
                start_inclusive: -5,
                end_inclusive: 5
            },
            SensorRange {
                start_inclusive: -4,
                end_inclusive: 5
            },
            SensorRange {
                start_inclusive: -3,
                end_inclusive: 5
            },
            SensorRange {
                start_inclusive: -2,
                end_inclusive: 5
            },
            SensorRange {
                start_inclusive: -1,
                end_inclusive: 5
            },
        ];

        condense_sensor_ranges(&mut sensors);
        assert_eq!(1, sensors.len());
        assert_eq!(-5, sensors[0].start_inclusive);
        assert_eq!(5, sensors[0].end_inclusive);
        assert_eq!(11, sensors[0].get_absolute_range());
    }

    #[test]
    fn condense_sensor_ranges_mutli_2_nothing() {
        let mut sensors = vec![
            SensorRange {
                start_inclusive: -5,
                end_inclusive: 5
            },
            SensorRange {
                start_inclusive: 6,
                end_inclusive: 10
            },
            SensorRange {
                start_inclusive: 11,
                end_inclusive: 16
            },
            SensorRange {
                start_inclusive: 17,
                end_inclusive: 22
            },
            SensorRange {
                start_inclusive: 23,
                end_inclusive: 27
            },
        ];

        condense_sensor_ranges(&mut sensors);
        assert_eq!(5, sensors.len());
        assert_eq!(-5, sensors[0].start_inclusive);
        assert_eq!(5, sensors[0].end_inclusive);
        assert_eq!(11, sensors[0].get_absolute_range());
    }

    #[test]
    fn ranges_correct() {
        let test_sensor = Sensor {
            position_x: 10,
            position_y: 10,
            closest_beacon_x: 10,
            closest_beacon_y: 20,
        };
        assert_eq!(10, test_sensor.get_sensor_radius());
        assert!(test_sensor.get_row_coverage_range(21).is_none());
        assert!(test_sensor.get_row_coverage_range(-1).is_none());
        assert_eq!(21, test_sensor.get_row_coverage_range(10).unwrap().get_absolute_range());
        assert_eq!(0, test_sensor.get_row_coverage_range(10).unwrap().start_inclusive);
        assert_eq!(20, test_sensor.get_row_coverage_range(10).unwrap().end_inclusive);

        assert_eq!(1, test_sensor.get_row_coverage_range(0).unwrap().get_absolute_range());
        assert_eq!(10, test_sensor.get_row_coverage_range(0).unwrap().start_inclusive);
        assert_eq!(10, test_sensor.get_row_coverage_range(0).unwrap().end_inclusive);

        assert_eq!(1, test_sensor.get_row_coverage_range(20).unwrap().get_absolute_range());
        assert_eq!(10, test_sensor.get_row_coverage_range(20).unwrap().start_inclusive);
        assert_eq!(10, test_sensor.get_row_coverage_range(20).unwrap().end_inclusive);

        assert_eq!(3, test_sensor.get_row_coverage_range(19).unwrap().get_absolute_range());
        assert_eq!(9, test_sensor.get_row_coverage_range(19).unwrap().start_inclusive);
        assert_eq!(11, test_sensor.get_row_coverage_range(19).unwrap().end_inclusive);
    }

    #[test]
    fn parses() {
        let input = "\
Sensor at x=-1111, y=-2222: closest beacon is at x=-3333, y=-4444";
        let results = parse_input(&input);
        assert_eq!(1, results.len());
        assert_eq!(-1111, results[0].position_x);
        assert_eq!(-2222, results[0].position_y);
        assert_eq!(-3333, results[0].closest_beacon_x);
        assert_eq!(-4444, results[0].closest_beacon_y);
        assert_eq!(4444, results[0].get_sensor_radius());
    }

    #[test]
    fn test() {
        let input = "Sensor at x=8, y=7: closest beacon is at x=2, y=10";
        let sensors = parse_input(&input);
        assert_eq!(9, sensors[0].get_sensor_radius());
        assert_eq!(1, sensors[0].get_row_coverage_range(-2).unwrap().get_absolute_range());
        assert_eq!(8, sensors[0].get_row_coverage_range(-2).unwrap().start_inclusive);
        assert_eq!(8, sensors[0].get_row_coverage_range(-2).unwrap().end_inclusive);
        assert_eq!(1, sensors[0].get_row_coverage_range(16).unwrap().get_absolute_range());
        assert_eq!(8, sensors[0].get_row_coverage_range(16).unwrap().start_inclusive);
        assert_eq!(8, sensors[0].get_row_coverage_range(16).unwrap().end_inclusive);
        assert_eq!(19, sensors[0].get_row_coverage_range(7).unwrap().get_absolute_range());
        assert_eq!(-1, sensors[0].get_row_coverage_range(7).unwrap().start_inclusive);
        assert_eq!(17, sensors[0].get_row_coverage_range(7).unwrap().end_inclusive);
    }

    #[test]
    fn first() {
        let input = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        assert_eq!(26, problem_1(&input, 10));
    }

    #[test]
    fn second() {
        let input = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        assert_eq!(56000011, problem_2(&input, 20));
    }
}

