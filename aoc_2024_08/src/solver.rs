use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct MapPoint {
    symbol: char,
    position: Position,
}

impl MapPoint {
    fn is_antenna(&self) -> bool {
        !('.'.eq_ignore_ascii_case(&self.symbol) || '#'.eq_ignore_ascii_case(&self.symbol))
    }
}

#[derive(Clone)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn calculate_mirror_point(&self, other: &Position) -> Option<Position> {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);

        let new_x = if self.x < other.x {
            self.x.checked_sub(dx)?
        } else {
            self.x.checked_add(dx)?
        };
        let new_y = if self.y < other.y {
            self.y.checked_sub(dy)?
        } else {
            self.y.checked_add(dy)?
        };

        Some(
            Position {
                x: new_x,
                y: new_y
            }
        )
    }

    fn is_below_limit(&self, limit: &Position) -> bool {
        self.x < limit.x && self.y < limit.y
    }

    fn calculate_harmonic_mirror_points(&self, other: &Position, limit: &Position) -> Vec<Position> {
        let mut last_self = self.clone();
        let mut last_other = other.clone();

        let mut harmonic_mirror_points = Vec::new();

        while last_self.is_below_limit(limit) && last_other.is_below_limit(limit) {
            match last_self.calculate_mirror_point(&last_other) {
                Some(p) if p.is_below_limit(limit) => {
                    harmonic_mirror_points.push(p.clone());
                    last_other = last_self.clone();
                    last_self = p;
                }
                _ => break,
            }
        }


        harmonic_mirror_points.push(self.clone());

        harmonic_mirror_points
    }
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let pos_string = format!("{}:{}", self.x, self.y);
        state.write(pos_string.as_bytes())
    }

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized
    {
        for d in data {
            let pos_string = format!("{}:{}", d.x, d.y);
            state.write(pos_string.as_bytes())
        }
    }
}

impl PartialEq<Self> for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }
}

impl Eq for Position {}

#[derive(Clone)]
pub struct Solver {
    map_points: Vec<MapPoint>,
    limit: Position,
    debug: bool,
}

impl Solver {
    pub fn read_antenna_data(self, include_harmonics: bool) -> Vec<Position> {
        self.map_points.iter().filter_map(|mp| {
            if mp.is_antenna() {
                Some(self.map_points.iter()
                    .filter(|maybe_matching| {
                        maybe_matching.symbol.eq(&mp.symbol) && maybe_matching.position.ne(&mp.position)
                    }).fold(Vec::new(), |antennas, matching_antenna| {
                        if include_harmonics {
                            let antinodes = mp.position.calculate_harmonic_mirror_points(&matching_antenna.position, &self.limit);

                            let mut new_antennas = antennas.clone();
                            for antinode in antinodes {
                                new_antennas.push(antinode);
                            }
                            new_antennas
                        } else {
                            match mp.position.calculate_mirror_point(&matching_antenna.position) {
                                Some(p) if p.is_below_limit(&self.limit) => {
                                    let mut new_antennas = antennas.clone();
                                    new_antennas.push(p);
                                    new_antennas
                                },
                                _ => antennas,
                            }
                        }



                    })
                )
            } else {
                None
            }
        }).flatten().collect()
    }

    fn init(input: &Vec<Vec<char>>, debug: bool) -> Solver {
        Solver {
            map_points: input.iter().enumerate().map(|(y, line)| {
                line.iter().enumerate().map(move |(x, symbol)| MapPoint {
                    symbol: symbol.clone(),
                    position: Position { x: x.clone(),y: y.clone() }
                })
            }).flatten().collect(),
            limit: Position {
                x: input.first().map(|l| l.len()).unwrap_or(0),
                y: input.len()
            },
            debug
        }
    }

    pub fn new(input: &Vec<Vec<char>>) -> Solver {
        Self::init(input, false)
    }

    pub fn new_with_debug(input: &Vec<Vec<char>>) -> Solver {
        Self::init(input, true)
    }

    pub fn calculate_distinct_antinode_positions(self, include_harmonics: bool) -> u128 {
        let mut unique_positions = HashSet::new();
        let positions = self.clone().read_antenna_data(include_harmonics);

        for position in positions {
            let _ = unique_positions.insert(position);
        }

        if self.debug {
            let mut upv = Vec::from_iter(unique_positions.iter());
            upv.sort_by(|pa, pb| {
                if pa.x.eq(&pb.x) {
                    pa.y.cmp(&pb.y)
                } else {
                    pa.x.cmp(&pb.x)
                }
            });
            upv.iter().for_each(|p| {
                println!("Antinode @ {}:{}", p.x, p.y)
            })
        }

        unique_positions.len() as u128
    }
}
