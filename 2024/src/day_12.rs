pub fn solve(_input: &str) -> (usize, usize) {
    /*
    let mut visited = HashSet::default();
    let grid = Grid::new_by_char(input, |c| c);

    fn claim(
        p: (isize, isize),
        group: char,
        grid: &Grid<char>,
        visited: &mut HashSet<i(isize, isize)>,
        perimeter: &mut i64,
        area: &mut i64,
    ) {
        if visited.contains(&p) || *grid.get_signed(p.0, p.1).unwrap() != group {
            return;
        }

        visited.insert(p);
        *area += 1;

        for (x_offset, y_offset) in CARDINAL_DIRECTIONS
            .into_iter()
            .map(|direction| direction.offset())
        {
            if grid.get_signed(p.0, p.1).is_none_or(|&val| val != group) {
                *perimeter += 1;
            }

            claim((column + x_offset, row + y_offset), group, grid, visited, perimeter, area);
        }

        *perimeter += p
            .neighbors4()
            .filter(|n| grid.get(*n) != Some(&group))
            .count() as i64;

        for n in grid.neighbors4(p) {
            claim(n, group, grid, visited, perimeter, area);
        }
    }

    grid.points()
        .map(|p| {
            let mut area = 0;
            let mut perimeter = 0;
            claim(p, grid[p], &grid, &mut visited, &mut perimeter, &mut area);
            area * perimeter
        })
        .sum()

    */

    (0, 0)
}

/*
fn part2(input: &str) -> i64 {
    let mut visited = HashSet::default();
    let grid = Grid::new_by_char(input, |c| c);

    fn claim(
        p: Vec2,
        group: char,
        grid: &Grid<char>,
        visited: &mut HashSet<Vec2>,
        sides: &mut i64,
        area: &mut i64,
    ) {
        if visited.contains(&p) || grid[p] != group {
            return;
        }

        visited.insert(p);
        *area += 1;
        *sides += Direction::ALL
            .into_iter()
            // If the grid has a member of the same group in the direction we
            // are checking then this isn't an edge at all let alone a unique side.
            .filter(|d| grid.get(p + d.vector()) != Some(&group))
            // If this is an edge we want to count each edge only once. We check
            // if this is the left-most square on this edge. This is the case if
            // either there is no square in the same group to the left, or there
            // is such a square but it has another square above it so the edge
            // still ends here.
            .filter(|d| {
                grid.get(p + d.turn_left().vector()) != Some(&group)
                    || grid.get(p + d.vector() + d.turn_left().vector()) == Some(&group)
            })
            .count() as i64;

        visited.insert(p);

        for n in grid.neighbors4(p) {
            claim(n, group, grid, visited, sides, area);
        }
    }

    grid.points()
        .map(|p| {
            let mut area = 0;
            let mut perimeter = 0;
            claim(p, grid[p], &grid, &mut visited, &mut perimeter, &mut area);
            area * perimeter
        })
        .sum()
}
*/

/*
pub fn solve(input: &str) -> (usize, usize) {
    let data = input
        .as_bytes()
        .split(|&byte| byte == b'\n')
        .flat_map(|line| line.iter().map(|&byte| byte))
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let width = input
        .as_bytes()
        .iter()
        .position(|&byte| byte == b'\n')
        .unwrap();
    let height = data.len() / width;
    let grid = Grid::new(data, width, height);

    let mut marked = Grid::new(
        vec![false; width * height].into_boxed_slice(),
        width,
        height,
    );

    for row in 0..grid.height() {
        for column in 0..grid.width() {
            print!("{}", *grid.get(column,  row).unwrap() as char);
        }
        println!();
    }

    let mut part_1 = 0;
    for row in 0..grid.height() {
        for column in 0..grid.width() {
            let value = *grid.get(column, row).unwrap();
            println!("Region {}:", value as char);

            let (perimeter, area) = mark_region(&grid, &mut marked, (column as isize, row as isize), value);
            part_1 += perimeter * area;
            println!("{perimeter} * {area}");
        }
    }

    (part_1, 0)
}

fn calculate_region(
    grid: &Grid<u8>,
    marked: &mut Grid<bool>,
    position: (isize, isize),
    value: u8
) -> (usize, usize) {
    if marked.get_signed(position.0, position.1).is_some_and(|&visited| visited) {
        return (0, 0);
    }

    let mut area = 1;
    let mut perimeter = 0;
    for (x_offset, y_offset) in CARDINAL_DIRECTIONS
        .into_iter()
        .map(|direction| direction.offset())
    {
        let column = position.0 + x_offset;
        let row = position.1 + y_offset;

        if grid.get_signed(column, row).is_none_or(|&val| value != val) {
            perimeter +=  1;
        } else if grid.get_signed(column, row).
    }

    (perimeter, area)
}

fn mark_region(
    grid: &Grid<u8>,
    marked: &mut Grid<bool>,
    position: (isize, isize),
    value: u8,
) -> (usize, usize) {
    if grid
        .get_signed(position.0, position.1)
        .is_none_or(|&val| val != value)
    {
        return (1, 0);
    }

    let marker = marked.get_signed_mut(position.0, position.1).unwrap();
    if *marker {
        return (0, 0);
    }

    *marker = true;

    let mut area = 1;
    let mut perimeter = 0;
    for (x_offset, y_offset) in CARDINAL_DIRECTIONS
        .into_iter()
        .map(|direction| direction.offset())
    {
        let column = position.0 + x_offset;
        let row = position.1 + y_offset;

        let (additional_area, additional_perimeter) =
            mark_region(grid, marked, (column, row), value);
        area += additional_area;
        perimeter += additional_perimeter;
    }

    (perimeter, area)
}
*/
