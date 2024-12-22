use utils::{hash::FxHashMap, Solution};

pub fn solve(input: &str) -> Solution {
    let mut blocks = Vec::new();
    let mut file_blocks = FxHashMap::default();
    for (index, value) in input
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .map(|ch| ch as u8 - b'0')
        .enumerate()
    {
        if index % 2 == 0 {
            for _ in 0..value {
                blocks.push(index / 2)
            }
            file_blocks.insert(index / 2, value);
        } else {
            for _ in 0..value {
                blocks.push(usize::MAX)
            }
        }
    }

    let mut part_1_blocks = blocks.clone();
    let mut start_ptr = 0;
    let mut end_ptr = part_1_blocks.len() - 1;
    while start_ptr < end_ptr {
        while part_1_blocks
            .get(start_ptr)
            .is_some_and(|&value| value != usize::MAX)
        {
            start_ptr += 1;
        }
        while part_1_blocks
            .get(end_ptr)
            .is_some_and(|&value| value == usize::MAX)
        {
            end_ptr -= 1;
        }

        if start_ptr >= end_ptr {
            break;
        }

        part_1_blocks[start_ptr] = part_1_blocks[end_ptr];
        part_1_blocks[end_ptr] = usize::MAX;
    }

    let mut part_1 = 0;
    for (index, owner) in part_1_blocks.into_iter().enumerate() {
        if owner != usize::MAX {
            part_1 += index * owner;
        }
    }

    let mut part_2_blocks = blocks;
    let mut end_ptr = part_2_blocks.len() - 1;
    let mut current_file = part_2_blocks
        .iter()
        .rev()
        .find(|&&owner| owner != usize::MAX)
        .copied()
        .unwrap();
    loop {
        while part_2_blocks
            .get(end_ptr)
            .is_some_and(|&owner| owner != current_file)
        {
            let Some(new_end_ptr) = end_ptr.checked_sub(1) else {
                break;
            };

            end_ptr = new_end_ptr;
        }

        if end_ptr == 0 {
            break;
        }

        current_file = if let Some(new_file) = current_file.checked_sub(1) {
            new_file
        } else {
            break;
        };
        let current_file = current_file + 1;

        let file_size = file_blocks.get(&part_2_blocks[end_ptr]).copied().unwrap() as usize;
        let Some(free_start) = find_free_space(&part_2_blocks, file_size) else {
            continue;
        };
        if free_start > end_ptr {
            continue;
        }

        for index in 0..file_size {
            part_2_blocks[free_start + index] = current_file;

            part_2_blocks[end_ptr - (file_size - 1 - index)] = usize::MAX;
        }
    }

    let mut part_2 = 0;
    for (index, owner) in part_2_blocks.into_iter().enumerate() {
        if owner != usize::MAX {
            part_2 += index * owner;
        }
    }

    (part_1, part_2).into()
}

pub fn find_free_space(disk: &Vec<usize>, file_size: usize) -> Option<usize> {
    let mut start_ptr = 0;
    while let Some(owner) = disk.get(start_ptr).copied() {
        if owner != usize::MAX {
            start_ptr += 1;
            continue;
        }

        let free_start = start_ptr;
        let mut free_size = 0;
        while let Some(owner) = disk.get(start_ptr).copied() {
            if owner != usize::MAX {
                break;
            }

            start_ptr += 1;
            free_size += 1;
        }

        if free_size >= file_size {
            return Some(free_start);
        }
    }

    None
}
