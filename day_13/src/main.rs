use core::cmp::Ordering;
use std::env;
use std::fs;
use std::fmt;

#[derive(Eq, Debug)]
enum ElfPacketData {
    ElfNumeric(u32),
    ElfList(Vec<ElfPacketData>)
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
#[derive(Debug)]
struct ElfPacket {
    data: Vec<ElfPacketData>
}

impl fmt::Display for ElfPacketData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ElfPacketData::ElfNumeric(val) => {
                write!(f, "{},", val)
            },
            ElfPacketData::ElfList(list) => {
                write!(f, " [ ")?;
                for c in list {
                    write!(f, "{}", c)?;
                }
                write!(f, " ] ")
            },
        }
    }
}

impl fmt::Display for ElfPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Packet: ((")?;
        for d in &self.data {
            write!(f, "{}", d)?;
        }
        write!(f, "))")
    }
}

impl Ord for ElfPacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ElfPacketData::ElfNumeric(val), ElfPacketData::ElfNumeric(other_val)) => {
                return val.cmp(other_val);
            },
            (ElfPacketData::ElfNumeric(val), ElfPacketData::ElfList(_other_list)) => {
                return ElfPacketData::ElfList(vec![ElfPacketData::ElfNumeric(val.clone())]).cmp(other);
            },
            (ElfPacketData::ElfList(_list), ElfPacketData::ElfNumeric(other_val)) => {
                return self.cmp(&ElfPacketData::ElfList(vec![ElfPacketData::ElfNumeric(other_val.clone())]));
            },
            (ElfPacketData::ElfList(list), ElfPacketData::ElfList(other_list)) => {
                for i in 0..std::cmp::max(list.len(), other_list.len()) {
                    match (list.get(i), other_list.get(i)) {
                        (Some(a), Some(b)) => {
                            match a.cmp(b) {
                                Ordering::Greater => { return Ordering::Greater; }
                                Ordering::Less => return Ordering::Less,
                                Ordering::Equal => (),
                            }
                        },
                        (Some(_a), None) => return Ordering::Greater,
                        (None, Some(_b)) => return Ordering::Less,
                        (None, None) => return Ordering::Equal,
                    }
                }

                return Ordering::Equal;
            }
        }
    }
}

impl PartialOrd for ElfPacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ElfPacketData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ElfPacketData::ElfNumeric(val), ElfPacketData::ElfNumeric(other_val)) => {
                return val == other_val;
            },
            (ElfPacketData::ElfNumeric(val), ElfPacketData::ElfList(_other_list)) => {
                return &ElfPacketData::ElfList(vec![ElfPacketData::ElfNumeric(val.clone())]) == other;
            },
            (ElfPacketData::ElfList(_list), ElfPacketData::ElfNumeric(other_val)) => {
                return self == (&ElfPacketData::ElfList(vec![ElfPacketData::ElfNumeric(other_val.clone())]));
            },
            (ElfPacketData::ElfList(list), ElfPacketData::ElfList(other_list)) => {
                for i in 0..std::cmp::max(list.len(), other_list.len()) {
                    match (list.get(i), other_list.get(i)) {
                        (Some(a), Some(b)) => {
                            if a != b {
                                return false;
                            }
                        },
                        (Some(_a), None) => return false,
                        (None, Some(_b)) => return false,
                        (None, None) => return true,
                    }
                }

                return true;
            }
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Problem 1: {}", problem_1(&contents));
    println!("Problem 2: {}", problem_2(&contents));
}

fn problem_1(input: &str) -> u32 {
    let pairs = input.split("\n\n")
        .map(|packets| packets.split('\n').map(|packet| parse_packet(packet)).collect::<Vec<ElfPacket>>())
        .collect::<Vec<Vec<ElfPacket>>>();

    let mut correct_order_count = 0;
    for (i, pair) in pairs.iter().enumerate() {
        if pair.len() != 2 {
            panic!("Invalid parsing");
        }

        if pair[0].data <= pair[1].data {
            // correct order
            correct_order_count += i + 1;
        }
    }

    correct_order_count.try_into().unwrap()
}

fn problem_2(input: &str) -> u32 {
    let mut packets = input.split("\n").filter(|l| *l != "")
        .map(|packet| parse_packet(packet)).collect::<Vec<ElfPacket>>();

    packets.push(ElfPacket { data: vec![ElfPacketData::ElfNumeric(2)]});
    packets.push(ElfPacket { data: vec![ElfPacketData::ElfNumeric(6)]});

    packets.sort_unstable();

    let identifier_pairs = packets.iter().enumerate()
        .filter(|(_i, p)| p.data.len() == 1 && (p.data.first().unwrap() == &ElfPacketData::ElfNumeric(2) || p.data.first().unwrap() == &ElfPacketData::ElfNumeric(6)))
        .collect::<Vec<(usize, &ElfPacket)>>();

    return identifier_pairs.iter().map(|(i, _p)| (*i + 1) as u32).product();
}

fn parse_token(current: &mut Vec<ElfPacketData>, token_iter: &mut dyn Iterator<Item = char>) {
    let mut current_parse_value: Option<u32> = None;
    while let Some(c) = token_iter.next() {
        match c {
            '[' => {
                let mut child = vec![];
                parse_token(&mut child, token_iter);
                current.push(ElfPacketData::ElfList(child));
            },
            ']' => {
                // if we were parsing a value, log it into the current array
                if let Some(current_val) = current_parse_value {
                    current.push(ElfPacketData::ElfNumeric(current_val));
                }
                return;
            },
            ',' => {
                // all previous characters are an uint, and we know we have at least one more uint
                if let Some(current_val) = current_parse_value {
                    current.push(ElfPacketData::ElfNumeric(current_val));
                }
                current_parse_value = None;
            },
            number => {
                // we are parsing an uint
                // 4 -> 4
                // 4 -> (40) + 4
                // 4 -> (440) = 4
                let cur_val = current_parse_value.unwrap_or(0) * 10 + number.to_digit(10).unwrap();
                current_parse_value = Some(cur_val);
            },
        };
    }
}

fn parse_packet(input: &str) -> ElfPacket {
    let mut root_array = vec![];
    let mut input_iterator = input.chars().into_iter();
    parse_token(&mut root_array, &mut input_iterator);

    ElfPacket {
        data: root_array
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_compares() {
        assert!(parse_packet("[1]") > parse_packet("[]"));
        assert!(parse_packet("[10]") > parse_packet("[1]"));
        assert!(parse_packet("[1]") == parse_packet("[1]"));
        assert!(parse_packet("[]") == parse_packet("[]"));
        assert!(parse_packet("[]") < parse_packet("[1]"));
        assert!(parse_packet("[1]") < parse_packet("[2]"));
        assert!(parse_packet("[2]") > parse_packet("[1]"));
        assert!(parse_packet("[2]") == parse_packet("[[2]]"));
        assert!(parse_packet("[[2]]") == parse_packet("[2]"));
        assert!(parse_packet("[[2]]") > parse_packet("[]"));
        assert!(parse_packet("[]") < parse_packet("[[]]"));
        assert!(parse_packet("[[]]") == parse_packet("[[]]"));
        assert!(parse_packet("[[2]]") == parse_packet("[[2]]"));
        assert!(parse_packet("[[1],[2]]") == parse_packet("[[1],[2]]"));
        assert!(parse_packet("[1,1,3,1,1]") < parse_packet("[1,1,5,1,1]"));
        assert!(parse_packet("[1,1,10,1,1]") == parse_packet("[1,1,10,1,1]"));
        assert!(parse_packet("[11,111,1111,11111,111111]") == parse_packet("[11,111,1111,11111,111111]"));
        assert!(parse_packet("[1,1,5,1,1]") > parse_packet("[1,1,3,1,1]"));
        assert!(parse_packet("[[1],[2,3,4]]") < parse_packet("[[1],4]"));
        assert!(parse_packet("[9]") > parse_packet("[[8,7,6]]"));
        assert!(parse_packet("[[4,4],4,4]") < parse_packet("[[4,4],4,4,4]"));
        assert!(parse_packet("[7,7,7,7]") > parse_packet("[7,7,7]"));
        assert!(parse_packet("[]") < parse_packet("[3]"));
        assert!(parse_packet("[[[]]]") > parse_packet("[[]]"));
        assert!(parse_packet("[1,[2,[3,[4,[5,6,7]]]],8,9]") > parse_packet("[1,[2,[3,[4,[5,6,0]]]],8,9]"));
    }

    #[test]
    fn first() {
        let input = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert_eq!(13, problem_1(&input));
    }

    #[test]
    fn second() {
        let input = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert_eq!(0, problem_2(&input));
    }
}