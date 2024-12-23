use std::{collections::hash_map::Entry, hash::Hash};

use utils::{
    hash::{FxHashMap, FxHashSet},
    Solution,
};

pub fn solve(input: &str) -> Solution {
    let part_1 = part_1(input);
    let part_2 = part_2(input);

    (part_1, part_2).into()
}

fn part_1(input: &str) -> usize {
    let connections = input.lines().map(|line| line.split_once('-').unwrap());

    let mut nodes = Vec::new();

    let mut node_indices = FxHashMap::<&str, usize>::default();
    for (node_a, node_b) in connections {
        let node_a_index = match node_indices.entry(node_a) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let index = *entry.insert(nodes.len());
                nodes.push((node_a, FxHashSet::default()));

                index
            }
        };

        let node_b_index = match node_indices.entry(node_b) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let index = *entry.insert(nodes.len());
                nodes.push((node_b, FxHashSet::default()));

                index
            }
        };

        nodes[node_a_index].1.insert(node_b_index);
        nodes[node_b_index].1.insert(node_a_index);
    }

    let mut triads = Vec::new();
    for index_a in 0..nodes.len() {
        for index_b in index_a + 1..nodes.len() {
            for index_c in index_b + 1..nodes.len() {
                if !(nodes[index_a].0.starts_with('t')
                    || nodes[index_b].0.starts_with('t')
                    || nodes[index_c].0.starts_with('t'))
                {
                    continue;
                }

                if (nodes[index_a].1.contains(&index_b) && nodes[index_a].1.contains(&index_c))
                    && (nodes[index_b].1.contains(&index_a) && nodes[index_b].1.contains(&index_c))
                    && (nodes[index_c].1.contains(&index_a) && nodes[index_c].1.contains(&index_b))
                {
                    triads.push((index_a, index_b, index_c));
                }
            }
        }
    }

    triads.len()
}

fn part_2(input: &str) -> String {
    let connections = input.lines().map(|line| line.split_once('-').unwrap());

    let mut nodes = FxHashMap::<&str, FxHashSet<&str>>::default();
    for (node_a, node_b) in connections {
        nodes.entry(node_a).or_default().insert(node_b);
        nodes.entry(node_b).or_default().insert(node_a);
    }

    let mut maximals = Vec::new();

    let node_list = {
        let mut list = FxHashSet::default();
        for &key in nodes.keys() {
            list.insert(key);
        }

        list
    };

    bron_kerbosch(
        &mut maximals,
        &nodes,
        FxHashSet::default(),
        node_list,
        FxHashSet::default(),
    );

    let mut maximal = maximals
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();

    maximal.sort();

    let mut result = String::new();
    for node in maximal {
        result.push_str(node);
        result.push(',');
    }

    result.pop().unwrap();

    result
}

fn bron_kerbosch<'input>(
    maximals: &mut Vec<FxHashSet<&'input str>>,
    neighbors: &FxHashMap<&'input str, FxHashSet<&'input str>>,
    partial: FxHashSet<&'input str>,
    mut candidates: FxHashSet<&'input str>,
    mut exclusions: FxHashSet<&'input str>,
) {
    if candidates.is_empty() && exclusions.is_empty() {
        maximals.push(partial);
        return;
    }

    while let Some(&candidate) = candidates.iter().next() {
        let mut new_partial = partial.clone();
        new_partial.insert(candidate);

        bron_kerbosch(
            maximals,
            neighbors,
            new_partial,
            intersection(candidates.clone(), &neighbor_set(neighbors, candidate)),
            intersection(exclusions.clone(), &neighbor_set(neighbors, candidate)),
        );

        candidates.remove(&candidate);
        exclusions.insert(candidate);
    }
}

fn intersection<'input, T: Hash + Eq + Copy>(
    mut set_a: FxHashSet<T>,
    set_b: &FxHashSet<T>,
) -> FxHashSet<T> {
    let mut to_remove = Vec::new();

    for value in set_a.iter() {
        if !set_b.contains(value) {
            to_remove.push(*value);
        }
    }

    for value in to_remove {
        set_a.remove(&value);
    }

    set_a
}

fn neighbor_set<'input>(
    neighbors: &FxHashMap<&'input str, FxHashSet<&'input str>>,
    node: &'input str,
) -> FxHashSet<&'input str> {
    neighbors.get(&node).unwrap().clone()
}
