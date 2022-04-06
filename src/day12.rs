use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use anyhow::{anyhow, bail};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Cave {
    Start,
    Small((u8, u8)),
    Big((u8, u8)),
    End,
}

impl Debug for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "start"),
            Self::End => write!(f, "end"),
            Self::Big((a, b)) => write!(f, "{}{}", *a as char, *b as char),
            Self::Small((a, b)) => write!(f, "{}{}", *a as char, *b as char),
        }
    }
}

impl FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            big if big.chars().all(|char| char.is_ascii_uppercase()) => {
                let big = big.as_bytes();
                if big.len() == 2 {
                    Ok(Self::Big((big[0], big[1])))
                } else {
                    bail!("Invalid input")
                }
            }
            small if small.chars().all(|char| char.is_ascii_lowercase()) => {
                let small = small.as_bytes();
                if small.len() == 2 {
                    Ok(Self::Small((small[0], small[1])))
                } else {
                    bail!("Invalid input")
                }
            }
            _ => bail!("Invalid input"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CaveGraph {
    adjacencies: HashMap<Cave, Vec<Cave>>,
}

impl CaveGraph {
    pub fn find_all_paths(&self) -> Vec<Vec<Cave>> {
        let mut paths: Vec<Vec<Cave>> = Vec::new();
        let path: Vec<Cave> = Vec::new();

        self.traverse_path(Cave::Start, path, &mut paths);

        paths
    }

    pub fn find_all_paths_alt(&self) -> Vec<Vec<Cave>> {
        let mut paths: Vec<Vec<Cave>> = Vec::new();
        let path: Vec<Cave> = Vec::new();

        self.traverse_path_alt(Cave::Start, path, &mut paths, false);

        paths
    }

    fn traverse_path(&self, current: Cave, mut path: Vec<Cave>, paths: &mut Vec<Vec<Cave>>) {
        path.push(current);

        if current == Cave::End {
            paths.push(path);
            return;
        }

        // Every cave other than End is connected, therefore we can unwrap
        for &cave in self.adjacencies.get(&current).unwrap() {
            match cave {
                Cave::Big(_) => {
                    self.traverse_path(cave, path.clone(), paths);
                }
                Cave::Small(_) => {
                    if path.contains(&cave) {
                        continue;
                    } else {
                        self.traverse_path(cave, path.clone(), paths);
                    }
                }
                Cave::End => {
                    self.traverse_path(cave, path.clone(), paths);
                }
                _ => unreachable!(),
            }
        }
    }

    fn traverse_path_alt(
        &self,
        current: Cave,
        mut path: Vec<Cave>,
        paths: &mut Vec<Vec<Cave>>,
        small_twice: bool,
    ) {
        path.push(current);

        if current == Cave::End {
            paths.push(path);
            return;
        }

        // Every cave other than End is connected, therefore we can unwrap
        for &cave in self.adjacencies.get(&current).unwrap() {
            match cave {
                Cave::Big(_) => {
                    self.traverse_path_alt(cave, path.clone(), paths, small_twice);
                }
                Cave::Small(_) => {
                    if path.contains(&cave) {
                        if small_twice {
                            continue;
                        } else {
                            self.traverse_path_alt(cave, path.clone(), paths, true);
                        }
                    } else {
                        self.traverse_path_alt(cave, path.clone(), paths, small_twice);
                    }
                }
                Cave::End => {
                    self.traverse_path_alt(cave, path.clone(), paths, small_twice);
                }
                _ => unreachable!(),
            }
        }
    }
}

impl FromStr for CaveGraph {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut adjacencies: HashMap<Cave, Vec<Cave>> = HashMap::new();

        for line in s.lines() {
            let mut spliterator = line.split('-');
            let left: Cave = spliterator
                .next()
                .ok_or_else(|| anyhow!("Invalid input"))?
                .parse()?;
            let right: Cave = spliterator
                .next()
                .ok_or_else(|| anyhow!("Invalid input"))?
                .parse()?;

            let left_adjacencies = adjacencies.entry(left).or_default();

            // We need to populate the adjacencies for left and right as traversal of the graph is
            // possible in both directions, however, we can't traverse back to start or away from
            // end.
            if left != Cave::End && right != Cave::Start {
                left_adjacencies.push(right);
            }

            let right_adjacencies = adjacencies.entry(right).or_default();
            if right != Cave::End && left != Cave::Start {
                right_adjacencies.push(left);
            }
        }

        Ok(CaveGraph { adjacencies })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_a() {
        let cave_graph: CaveGraph = "\
start-AA
start-bb
AA-cc
AA-bb
bb-dd
AA-end
bb-end"
            .parse()
            .unwrap();

        let all_paths = cave_graph.find_all_paths();
        assert_eq!(all_paths.len(), 10);
    }

    #[test]
    fn part_1_b() {
        let cave_graph: CaveGraph = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
            .parse()
            .unwrap();

        let all_paths = cave_graph.find_all_paths();
        assert_eq!(dbg!(all_paths).len(), 19);
    }

    #[test]
    fn part_2() {
        let cave_graph: CaveGraph = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
            .parse()
            .unwrap();

        let all_paths = cave_graph.find_all_paths_alt();
        assert_eq!(dbg!(all_paths).len(), 103);
    }
}
