use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal {
        version: usize,
        val: usize,
    },
    Operator {
        version: usize,
        type_id: usize,
        sub_packets: Vec<Packet>,
    },
}

impl Packet {
    fn decode(bits: Vec<Bit>) -> Vec<Self> {
        let mut bits = bits;
        let mut packets = Vec::new();

        while bits.len() > 7 {
            let (packet, length) = Packet::decode_one(&bits);

            packets.push(packet);

            bits = bits[length..].to_vec();
        }

        packets
    }

    fn decode_one(bits: &Vec<Bit>) -> (Self, usize) {
        let fold_fn = |acc, b: &u8| acc << 1 | (*b as usize);
        let version = bits[0..3].iter().fold(0, fold_fn);
        let type_id = bits[3..6].iter().fold(0, fold_fn);
        match type_id {
            4 => {
                let mut val = 0;
                let mut length = 6;

                loop {
                    let left = length;
                    let right = left + 5;
                    let sub = bits[left + 1..right].iter().fold(0, fold_fn);

                    val = val << 4 | sub;
                    length += 5;

                    if bits[left] == 0 {
                        break;
                    }
                }

                (Self::Literal { version, val }, length)
            }
            _ => {
                let start = 6;

                let length_type_id = bits[start];

                let (sub_packets, length) = match length_type_id {
                    0 => {
                        let total_length = bits[start + 1..start + 16].iter().fold(0, fold_fn);
                        let sub_bits = bits[start + 16..start + 16 + total_length]
                            .iter()
                            .map(|c| *c)
                            .collect::<Vec<_>>();
                        (Self::decode(sub_bits), start + 1 + 15 + total_length)
                    }
                    1 => {
                        let nums = bits[start + 1..start + 12].iter().fold(0, fold_fn);
                        let mut sub_packets = Vec::new();
                        let mut length = 6 + 12;

                        for _ in 0..nums {
                            let (packet, len) = Self::decode_one(
                                &bits[length..].iter().map(|c| *c).collect::<Vec<_>>(),
                            );
                            sub_packets.push(packet);
                            length += len;
                        }

                        (sub_packets, length)
                    }
                    _ => unreachable!("unknown length type id: {}", length_type_id),
                };

                (
                    Self::Operator {
                        version,
                        type_id,
                        sub_packets,
                    },
                    length,
                )
            }
        }
    }

    fn get_version(&self) -> usize {
        match self {
            &Packet::Literal { version, .. } => version,
            &Packet::Operator {
                version,
                ref sub_packets,
                ..
            } => version + sub_packets.iter().map(Self::get_version).sum::<usize>(),
        }
    }

    fn eval(&self) -> usize {
        match self {
            &Packet::Literal { val, .. } => val,
            &Packet::Operator {
                type_id,
                ref sub_packets,
                ..
            } => match type_id {
                0 => sub_packets.iter().fold(0, |acc, p| acc + p.eval()),
                1 => sub_packets.iter().fold(1, |acc, p| acc * p.eval()),
                2 => sub_packets.iter().map(Self::eval).min().unwrap(),
                3 => sub_packets.iter().map(Self::eval).max().unwrap(),
                5 => {
                    assert_eq!(
                        sub_packets.len(),
                        2,
                        "greater than operator always has two sub packets"
                    );
                    if sub_packets[0].eval() > sub_packets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    assert_eq!(
                        sub_packets.len(),
                        2,
                        "less than operator always has two sub packets"
                    );
                    if sub_packets[0].eval() < sub_packets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    assert_eq!(
                        sub_packets.len(),
                        2,
                        "equal to operator always has two sub packets"
                    );
                    if sub_packets[0].eval() == sub_packets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!("unknown type_id: {}", type_id),
            },
        }
    }
}

type Bit = u8;

// FIXME: does boolean type make it better?
fn get_bitsmap() -> HashMap<char, [Bit; 4]> {
    HashMap::from([
        ('0', [0, 0, 0, 0]),
        ('1', [0, 0, 0, 1]),
        ('2', [0, 0, 1, 0]),
        ('3', [0, 0, 1, 1]),
        ('4', [0, 1, 0, 0]),
        ('5', [0, 1, 0, 1]),
        ('6', [0, 1, 1, 0]),
        ('7', [0, 1, 1, 1]),
        ('8', [1, 0, 0, 0]),
        ('9', [1, 0, 0, 1]),
        ('A', [1, 0, 1, 0]),
        ('B', [1, 0, 1, 1]),
        ('C', [1, 1, 0, 0]),
        ('D', [1, 1, 0, 1]),
        ('E', [1, 1, 1, 0]),
        ('F', [1, 1, 1, 1]),
    ])
}

fn parse_input(input: &Vec<String>) -> Vec<Packet> {
    let bitsmap = get_bitsmap();
    let bits: Vec<Bit> = input[0]
        .chars()
        .map(|c| bitsmap.get(&c).unwrap())
        .flatten()
        .map(|b| *b)
        .collect::<Vec<_>>();

    Packet::decode(bits)
}

pub fn part1(input: &Vec<String>) -> bool {
    let packets = parse_input(input);

    let res: usize = packets.iter().map(Packet::get_version).sum();

    res == 871
}

pub fn part2(input: &Vec<String>) -> bool {
    let packets = parse_input(input);
    assert_eq!(packets.len(), 1, "only one root packet expected");
    let res = packets[0].eval();

    res == 68703010504
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_literal() {
        let bits: Vec<Bit> = "110100101111111000101000"
            .chars()
            .map(|b| match b {
                '1' => 1,
                '0' => 0,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        let (packet, length) = Packet::decode_one(&bits);
        assert_eq!(
            packet,
            Packet::Literal {
                version: 6,
                val: 2021
            },
            ""
        );
        assert_eq!(length, 21, "");

        let bits: Vec<Bit> = "11010001010"
            .chars()
            .map(|b| match b {
                '1' => 1,
                '0' => 0,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        let (packet, length) = Packet::decode_one(&bits);
        assert_eq!(
            packet,
            Packet::Literal {
                version: 6,
                val: 10
            },
            ""
        );
        assert_eq!(length, 11, "");
    }

    #[test]
    fn parse_operator_0() {
        let bits: Vec<Bit> = "00111000000000000110111101000101001010010001001000000000"
            .chars()
            .map(|b| match b {
                '1' => 1,
                '0' => 0,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        let (packet, length) = Packet::decode_one(&bits);
        assert_eq!(
            packet,
            Packet::Operator {
                version: 1,
                type_id: 6,
                sub_packets: vec![
                    Packet::Literal {
                        version: 6,
                        val: 10
                    },
                    Packet::Literal {
                        version: 2,
                        val: 20,
                    }
                ]
            },
            ""
        );
        assert_eq!(length, 49, "");
    }

    #[test]
    fn parse_operator_1() {
        let bits: Vec<Bit> = "11101110000000001101010000001100100000100011000001100000"
            .chars()
            .map(|b| match b {
                '1' => 1,
                '0' => 0,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        let (packet, length) = Packet::decode_one(&bits);
        assert_eq!(
            packet,
            Packet::Operator {
                version: 7,
                type_id: 3,
                sub_packets: vec![
                    Packet::Literal { version: 2, val: 1 },
                    Packet::Literal { version: 4, val: 2 },
                    Packet::Literal { version: 1, val: 3 }
                ]
            },
            ""
        );
        assert_eq!(length, 51, "");
    }

    #[test]
    fn more_example_0() {
        let bitsmap = get_bitsmap();
        let bits: Vec<Bit> = "8A004A801A8002F478"
            .chars()
            .map(|c| bitsmap.get(&c).unwrap())
            .flatten()
            .map(|b| *b)
            .collect::<Vec<_>>();

        let (packet, length) = Packet::decode_one(&bits);
        assert_eq!(
            packet,
            Packet::Operator {
                version: 4,
                type_id: 2,
                sub_packets: vec![Packet::Operator {
                    version: 1,
                    type_id: 2,
                    sub_packets: vec![Packet::Operator {
                        version: 5,
                        type_id: 2,
                        sub_packets: vec![Packet::Literal {
                            version: 6,
                            val: 15,
                        }]
                    }]
                }]
            },
            ""
        );
        assert_eq!(length, 69, "");
        assert_eq!(packet.get_version(), 16, "");
    }

    #[test]
    fn more_example_1() {
        let bitsmap = get_bitsmap();
        let bits: Vec<Bit> = "620080001611562C8802118E34"
            .chars()
            .map(|c| bitsmap.get(&c).unwrap())
            .flatten()
            .map(|b| *b)
            .collect::<Vec<_>>();

        let (packet, _length) = Packet::decode_one(&bits);
        assert_eq!(packet.get_version(), 12, "");
    }

    #[test]
    fn more_example_2() {
        let bitsmap = get_bitsmap();
        let bits: Vec<Bit> = "C0015000016115A2E0802F182340"
            .chars()
            .map(|c| bitsmap.get(&c).unwrap())
            .flatten()
            .map(|b| *b)
            .collect::<Vec<_>>();

        let (packet, _length) = Packet::decode_one(&bits);
        assert_eq!(packet.get_version(), 23, "");
    }

    #[test]
    fn more_example_3() {
        let bitsmap = get_bitsmap();
        let bits: Vec<Bit> = "A0016C880162017C3686B18A3D4780"
            .chars()
            .map(|c| bitsmap.get(&c).unwrap())
            .flatten()
            .map(|b| *b)
            .collect::<Vec<_>>();

        let (packet, _length) = Packet::decode_one(&bits);
        assert_eq!(packet.get_version(), 31, "");
    }
}
