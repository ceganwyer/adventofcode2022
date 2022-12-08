use eyre::{eyre, Result};

type TreeGrid = Vec<Vec<i32>>;

#[derive(Debug)]
pub struct TreeCoordinate {
    x: usize,
    y: usize,
}

pub fn parse_grid(input: &str) -> Result<TreeGrid> {
    let mut grid = TreeGrid::new();
    for line in input.lines() {
        let chars = line.chars();
        let mut row = Vec::new();
        for char in chars {
            let tree = char.to_digit(10)
                .ok_or_else(|| eyre!("Could not parse tree"))?;
            row.push(tree as i32);
        }
        grid.push(row);
    }
    Ok(grid)
}

// Solution for part 1
pub fn find_visible_trees(grid: &TreeGrid) -> Vec<TreeCoordinate> {
    let mut visible_trees = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let loc = TreeCoordinate { x, y };
            if is_visible(&loc, &grid) {
                visible_trees.push(loc);
            }
        }
    }
    visible_trees
}

fn is_visible(loc: &TreeCoordinate, grid: &TreeGrid) -> bool {
    if loc.x == 0 || loc.y == 0 ||
        loc.x == grid[loc.y].len() - 1 || loc.y == grid.len() - 1 {
        return true;
    }
    let tree = grid[loc.y][loc.x];

    // Horizontal checks
    let row = &grid[loc.y];
    // Check left
    let vis_left = row[..loc.x].iter().all(|size| size < &tree);
    // Check right
    let vis_right = row[loc.x + 1..].iter().all(|size| size < &tree);

    // Vertical checks
    let col = get_column(&grid, loc.x);
    // Check top
    let vis_top = col[..loc.y].iter().all(|&size| size < tree);
    //Check bottom
    let vis_bottom = col[loc.y + 1..].iter().all(|&size| size < tree);

    vis_left || vis_right || vis_top || vis_bottom
}

fn get_column(grid: &TreeGrid, index: usize) -> Vec<i32> {
    grid.iter()
        .map(|r| *r.iter().nth(index).unwrap())
        .collect::<Vec<_>>()
}

// Solution for part 2
pub fn find_max_scenic_score(grid: &TreeGrid) -> Result<i32> {
    let mut scores: Vec<i32> = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let loc = TreeCoordinate { x, y };
            let score = calc_scenic_score(&loc, grid);
            scores.push(score);
        }
    }
    scores.sort();
    scores.pop().ok_or_else(|| eyre!("No scores were found!"))
}

fn calc_scenic_score(loc: &TreeCoordinate, grid: &TreeGrid) -> i32 {
    if loc.x == 0 || loc.y == 0 ||
        loc.x == grid[loc.y].len() - 1 || loc.y == grid.len() - 1 {
        return 0
    }
    let row = &grid[loc.y];
    let col = get_column(&grid, loc.x);
    let tree = grid[loc.y][loc.x];
    let horizontal_score = calc_horizontal(loc, tree, row);
    let vertical_score = calc_vertical(loc, tree, &col);

    horizontal_score * vertical_score
}

fn calc_horizontal(loc: &TreeCoordinate, tree: i32, row: &Vec<i32>) -> i32 {
    let mut blocked = false;
    let left = row[..=loc.x-1].iter().rev().fold(0, |accum, size| {
        if !blocked {
            if size < &tree {
                accum + 1
            } else {
                blocked = true;
                accum + 1
            }
        } else {
            accum
        }
    });

    blocked = false;
    let right = row[loc.x+1..].iter().fold(0, |accum, size| {
        if !blocked {
            if size < &tree {
                accum + 1
            } else {
                blocked = true;
                accum + 1
            }
        } else {
            accum
        }
    });

    left * right
}

fn calc_vertical(loc: &TreeCoordinate, tree: i32, col: &Vec<i32>) -> i32 {
    let mut blocked = false;
    let top = col[..=loc.y-1].iter().rev().fold(0, |accum , size | {
        if !blocked {
            if size < &tree {
                accum + 1
            } else {
                blocked = true;
                accum + 1
            }
        } else {
            accum
        }
    });

    blocked = false;
    let bottom = col[loc.y+1..].iter().fold(0, |accum, size| {
        if !blocked {
            if size < &tree {
                accum + 1
            } else {
                blocked = true;
                accum + 1
            }
        } else {
            accum
        }
    });
    top * bottom
}