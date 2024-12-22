use std::{collections::VecDeque, hash::Hash};

use utils::{
    direction::Direction,
    hash::{FxHashMap, FxHashSet},
    Solution,
};

pub fn solve(input: &str) -> Solution {
    let codes = input.lines();
    let codes = {
        let mut arr = [""; 5];

        for (index, code) in codes.enumerate() {
            arr[index] = code;
        }

        arr
    };
    let codes = codes.map(|code| {
        let sequence = code
            .chars()
            .map(|c| match c {
                '0' => NumericKey::Digit0,
                '1' => NumericKey::Digit1,
                '2' => NumericKey::Digit2,
                '3' => NumericKey::Digit3,
                '4' => NumericKey::Digit4,
                '5' => NumericKey::Digit5,
                '6' => NumericKey::Digit6,
                '7' => NumericKey::Digit7,
                '8' => NumericKey::Digit8,
                '9' => NumericKey::Digit9,
                'A' => NumericKey::Activate,
                _ => unreachable!(),
            })
            .collect::<Vec<NumericKey>>();

        let mut numeric_value = 0;
        for c in code.chars().filter(|c| c.is_numeric()) {
            numeric_value = numeric_value * 10 + (c as u8 - b'0') as usize;
        }

        (sequence, numeric_value)
    });

    let numeric_paths = NumericKey::ALL
        .into_iter()
        .flat_map(|first| {
            NumericKey::ALL
                .into_iter()
                .map(move |second| (first, second))
        })
        .map(|(source, target)| {
            (
                (source, target),
                find_shortest_paths(
                    |key| {
                        Direction::ALL.into_iter().filter_map(move |direction| {
                            key.move_dir(direction).map(|neigbor| (neigbor, direction))
                        })
                    },
                    source,
                    target,
                )
                .into_iter()
                .map(|path| {
                    let mut new_path = Vec::new();

                    for direction in path {
                        let key = match direction {
                            Direction::East => DirectionKey::Right,
                            Direction::West => DirectionKey::Left,
                            Direction::North => DirectionKey::Up,
                            Direction::South => DirectionKey::Down,
                        };

                        new_path.push(key);
                    }

                    new_path
                })
                .collect::<Vec<_>>(),
            )
        })
        .collect::<FxHashMap<(NumericKey, NumericKey), Vec<Vec<DirectionKey>>>>();

    let direction_paths = DirectionKey::ALL
        .into_iter()
        .flat_map(|first| {
            DirectionKey::ALL
                .into_iter()
                .map(move |second| (first, second))
        })
        .map(|(source, target)| {
            (
                (source, target),
                find_shortest_paths(
                    |key| {
                        Direction::ALL.into_iter().filter_map(move |direction| {
                            key.move_dir(direction).map(|neigbor| (neigbor, direction))
                        })
                    },
                    source,
                    target,
                )
                .into_iter()
                .map(|path| {
                    let mut new_path = Vec::new();

                    for direction in path {
                        let key = match direction {
                            Direction::East => DirectionKey::Right,
                            Direction::West => DirectionKey::Left,
                            Direction::North => DirectionKey::Up,
                            Direction::South => DirectionKey::Down,
                        };

                        new_path.push(key);
                    }

                    new_path
                })
                .collect::<Vec<_>>(),
            )
        })
        .collect::<FxHashMap<(DirectionKey, DirectionKey), Vec<Vec<DirectionKey>>>>();

    let part_1: usize = codes
        .iter()
        .map(|(code, numeric)| {
            find_shortest_sequence::<2>(&numeric_paths, &direction_paths, &code) * numeric
        })
        .sum();

    let part_2: usize = codes
        .iter()
        .map(|(code, numeric)| {
            find_shortest_sequence::<25>(&numeric_paths, &direction_paths, &code) * numeric
        })
        .sum();

    (part_1, part_2).into()
}

fn find_shortest_sequence<const N: usize>(
    numeric_paths: &FxHashMap<(NumericKey, NumericKey), Vec<Vec<DirectionKey>>>,
    direction_paths: &FxHashMap<(DirectionKey, DirectionKey), Vec<Vec<DirectionKey>>>,
    sequence: &[NumericKey],
) -> usize {
    let target_sequence = {
        let mut vec = vec![NumericKey::Activate];
        vec.extend(sequence.into_iter().copied());

        vec
    };

    target_sequence
        .into_iter()
        .map_windows(|&[source, target]| {
            numeric_paths
                .get(&(source, target))
                .unwrap()
                .iter()
                .cloned()
                .map(|mut path| {
                    path.push(DirectionKey::Activate);
                    find_shortest_direction_sequence(
                        &mut FxHashMap::default(),
                        direction_paths,
                        path,
                        N - 1,
                    )
                })
                .min()
                .unwrap()
        })
        .sum::<usize>()
}

fn find_shortest_direction_sequence(
    cache: &mut FxHashMap<(Vec<DirectionKey>, usize), usize>,
    paths: &FxHashMap<(DirectionKey, DirectionKey), Vec<Vec<DirectionKey>>>,
    sequence: Vec<DirectionKey>,
    depth: usize,
) -> usize {
    let check = (sequence, depth);
    if let Some(buttons) = cache.get(&check).copied() {
        return buttons;
    }
    let sequence = check.0;

    let target_sequence = {
        let mut vec = sequence.clone();

        vec.insert(0, DirectionKey::Activate);
        vec
    };

    let result = target_sequence
        .into_iter()
        .map_windows(|&[source, target]| {
            let shortest_paths = paths.get(&(source, target)).unwrap();

            match depth {
                0 => shortest_paths[0].len() + 1,
                depth => shortest_paths
                    .iter()
                    .cloned()
                    .map(|mut path| {
                        path.push(DirectionKey::Activate);
                        find_shortest_direction_sequence(cache, paths, path, depth - 1)
                    })
                    .min()
                    .unwrap(),
            }
        })
        .sum::<usize>();

    cache.insert((sequence, depth), result);

    result
}

fn find_shortest_paths<T: Ord + Hash + Copy, I: Iterator<Item = (T, Direction)>>(
    f: fn(T) -> I,
    source: T,
    target: T,
) -> Vec<Vec<Direction>> {
    let mut queue = VecDeque::new();
    queue.push_back((source, Vec::new(), FxHashSet::default()));

    let mut paths = Vec::new();
    let mut lowest = usize::MAX;

    while let Some((node, path, mut visited)) = queue.pop_front() {
        if node == target {
            if path.len() <= lowest {
                lowest = path.len();
                paths.push(path);
            }
            continue;
        }

        if !visited.insert(node) {
            continue;
        }

        for (next, direction) in f(node) {
            let mut path = path.clone();
            path.push(direction);
            queue.push_back((next, path, visited.clone()));
        }
    }

    paths
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum NumericKey {
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    #[default]
    Activate,
}

impl NumericKey {
    const ALL: [Self; 11] = [
        Self::Digit0,
        Self::Digit1,
        Self::Digit2,
        Self::Digit3,
        Self::Digit4,
        Self::Digit5,
        Self::Digit6,
        Self::Digit7,
        Self::Digit8,
        Self::Digit9,
        Self::Activate,
    ];

    fn move_dir(self, direction: Direction) -> Option<Self> {
        let x = self.column() as isize + direction.offset().0;
        let y = self.row() as isize + direction.offset().1;

        let Ok(x) = x.try_into() else {
            return None;
        };

        let Ok(y) = y.try_into() else {
            return None;
        };

        Self::get(x, y)
    }

    fn row(self) -> usize {
        match self {
            Self::Digit7 | Self::Digit8 | Self::Digit9 => 0,
            Self::Digit4 | Self::Digit5 | Self::Digit6 => 1,
            Self::Digit1 | Self::Digit2 | Self::Digit3 => 2,
            Self::Digit0 | Self::Activate => 3,
        }
    }

    fn column(self) -> usize {
        match self {
            Self::Digit7 | Self::Digit4 | Self::Digit1 => 0,
            Self::Digit8 | Self::Digit5 | Self::Digit2 | Self::Digit0 => 1,
            Self::Digit9 | Self::Digit6 | Self::Digit3 | Self::Activate => 2,
        }
    }

    const fn get(column: usize, row: usize) -> Option<Self> {
        let position = match (column, row) {
            (0, 0) => Self::Digit7,
            (1, 0) => Self::Digit8,
            (2, 0) => Self::Digit9,

            (0, 1) => Self::Digit4,
            (1, 1) => Self::Digit5,
            (2, 1) => Self::Digit6,

            (0, 2) => Self::Digit1,
            (1, 2) => Self::Digit2,
            (2, 2) => Self::Digit3,

            (1, 3) => Self::Digit0,
            (2, 3) => Self::Activate,
            _ => return None,
        };

        Some(position)
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum DirectionKey {
    Up,
    Down,
    Left,
    Right,
    #[default]
    Activate,
}

impl DirectionKey {
    const ALL: [Self; 5] = [
        Self::Up,
        Self::Down,
        Self::Left,
        Self::Right,
        Self::Activate,
    ];

    fn move_dir(self, direction: Direction) -> Option<Self> {
        let x = self.column() as isize + direction.offset().0;
        let y = self.row() as isize + direction.offset().1;

        let Ok(x) = x.try_into() else {
            return None;
        };

        let Ok(y) = y.try_into() else {
            return None;
        };

        Self::get(x, y)
    }

    const fn row(self) -> usize {
        match self {
            Self::Up | Self::Activate => 0,
            Self::Left | Self::Down | Self::Right => 1,
        }
    }

    const fn column(self) -> usize {
        match self {
            Self::Left => 0,
            Self::Up | Self::Down => 1,
            Self::Activate | Self::Right => 2,
        }
    }

    const fn get(column: usize, row: usize) -> Option<Self> {
        let position = match (column, row) {
            (1, 0) => Self::Up,
            (1, 1) => Self::Down,
            (2, 1) => Self::Right,
            (0, 1) => Self::Left,
            (2, 0) => Self::Activate,
            _ => return None,
        };

        Some(position)
    }
}
