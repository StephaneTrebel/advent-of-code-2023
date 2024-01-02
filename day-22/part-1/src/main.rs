use std::{collections::HashMap, fs};

use regex::Regex;

fn get_file_content(file_path: &str) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Coords {
    x: usize,
    y: usize,
    z: usize,
}
type Block = (Coords, Coords);

// A block will be identified by its index in the vector
type Blocks = HashMap<usize, Block>;

// A 3D Grid that indicates if there is a block there, and which one (as referred
// by its index)
type Grid = HashMap<Coords, usize>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Content {
    blocks: Blocks,
    grid: Grid,
    max_x: usize,
    max_y: usize,
    max_z: usize,
}

fn parse_content(lines: &str) -> Content {
    let re_block = Regex::new(r"(?<start_x>[0-9]+),(?<start_y>[0-9]+),(?<start_z>[0-9]+)\~(?<end_x>[0-9]+),(?<end_y>[0-9]+),(?<end_z>[0-9]+)").unwrap();

    let mut blocks: Blocks = Blocks::new();
    let mut grid: Grid = Grid::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    lines
        .split("\n")
        .filter(|line| !line.is_empty())
        .enumerate()
        .for_each(|(index, line)| {
            let Some(caps) = re_block.captures(line) else {
                panic!("Invalid block: {}", line);
            };
            let start_x = caps["start_x"].parse().unwrap();
            let start_y = caps["start_y"].parse().unwrap();
            let start_z = caps["start_z"].parse().unwrap();
            let end_x = caps["end_x"].parse().unwrap();
            let end_y = caps["end_y"].parse().unwrap();
            let end_z = caps["end_z"].parse().unwrap();

            if end_x < start_x || end_y < start_y || end_z < start_z {
                panic!("End should always be greater than start: {}", line);
            }

            if end_x > max_x {
                max_x = end_x;
            }
            if end_y > max_y {
                max_y = end_y;
            }
            if end_z > max_z {
                max_z = end_z;
            }

            let block: Block = (
                Coords {
                    x: start_x,
                    y: start_y,
                    z: start_z,
                },
                Coords {
                    x: end_x,
                    y: end_y,
                    z: end_z,
                },
            );
            let block_id = index + 1;
            blocks.insert(block_id, block.clone());

            if start_z == end_z {
                if start_x == end_x {
                    for y in start_y..=end_y {
                        grid.insert(
                            Coords {
                                x: start_x,
                                y,
                                z: start_z,
                            },
                            block_id,
                        );
                    }
                } else {
                    for x in start_x..=end_x {
                        grid.insert(
                            Coords {
                                x,
                                y: start_y,
                                z: start_z,
                            },
                            block_id,
                        );
                    }
                }
            } else {
                for z in start_z..=end_z {
                    grid.insert(
                        Coords {
                            x: start_x,
                            y: start_y,
                            z,
                        },
                        block_id,
                    );
                }
            }
        });

    Content {
        blocks,
        grid,
        max_x,
        max_y,
        max_z,
    }
}

#[cfg(test)]
mod tests_parse_content {
    use super::*;

    #[test]
    fn parse_content_01() {
        let content = parse_content(
            &"\
1,0,1~1,2,1
",
        );
        assert_eq!(content.blocks.len(), 1);
    }
}

fn settle(content: &Content) -> Content {
    let mut settled_grid: Grid = content.grid.clone();
    let mut new_content: Content = content.clone();

    // Going for the bottom to top
    for z in 1..=content.max_z {
        // Browsing every possible block part on the Z plane
        for y in 0..=content.max_y {
            for x in 0..=content.max_x {
                // If we find a block part, retrieve its id
                if let Some(block_id) = settled_grid.clone().get(&Coords { x, y, z }) {
                    // For every part of the block, push down as much as possible
                    let (start, end) = content.blocks.get(block_id).unwrap().clone();

                    // Vertical block: We have to move every block part accross Z planes
                    if start.z != end.z {
                        let mut lowest_possible_z = start.z;
                        // Iterate for every part of this block
                        for tmp_z in (1..start.z).rev() {
                            if let Some(_) = settled_grid.get(&Coords { x, y, z: tmp_z }) {
                                break;
                            }
                            lowest_possible_z = tmp_z;
                        }
                        // If block can be vertically moved
                        if lowest_possible_z != start.z {
                            // For every block part
                            for (index, block_z) in (start.z..=end.z).enumerate() {
                                // Remove it from its original place and move it to new, lower place
                                settled_grid.insert(
                                    Coords {
                                        x,
                                        y,
                                        z: lowest_possible_z + index,
                                    },
                                    *block_id,
                                );
                                settled_grid.remove(&Coords { x, y, z: block_z });
                            }
                            // Update the block reference starting/ending positions
                            new_content.blocks.insert(
                                *block_id,
                                (
                                    Coords {
                                        x: start.x,
                                        y: start.y,
                                        z: lowest_possible_z,
                                    },
                                    Coords {
                                        x: end.x,
                                        y: end.y,
                                        z: lowest_possible_z + end.z - start.z,
                                    },
                                ),
                            );
                        }
                    }
                    // Horizontal block
                    else {
                        // Either alongside X
                        if start.x != end.x {
                            // Iterate for every part of this block
                            // to find the lowest possible Z to move the block parts
                            let mut lowest_possible_z = 1;
                            for block_x in start.x..=end.x {
                                let mut out_z = z;
                                for tmp_z in (1..z).rev() {
                                    if let Some(_) = settled_grid.get(&Coords {
                                        x: block_x,
                                        y,
                                        z: tmp_z,
                                    }) {
                                        break;
                                    }
                                    out_z = tmp_z;
                                }
                                if out_z > lowest_possible_z {
                                    lowest_possible_z = out_z;
                                }
                            }
                            if lowest_possible_z != z {
                                for block_x in start.x..=end.x {
                                    settled_grid.insert(
                                        Coords {
                                            x: block_x,
                                            y,
                                            z: lowest_possible_z,
                                        },
                                        *block_id,
                                    );
                                    settled_grid.remove(&Coords { x: block_x, y, z });
                                }
                                new_content.blocks.insert(
                                    *block_id,
                                    (
                                        Coords {
                                            x: start.x,
                                            y: start.y,
                                            z: lowest_possible_z,
                                        },
                                        Coords {
                                            x: end.x,
                                            y: end.y,
                                            z: lowest_possible_z,
                                        },
                                    ),
                                );
                            }
                        }
                        // Or alongside Y
                        else {
                            // Iterate for every part of this block
                            // to find the lowest possible Z to move the block parts
                            let mut lowest_possible_z = 1;
                            for block_y in start.y..=end.y {
                                let mut out_z = z;
                                for tmp_z in (1..z).rev() {
                                    if let Some(_) = settled_grid.get(&Coords {
                                        x,
                                        y: block_y,
                                        z: tmp_z,
                                    }) {
                                        break;
                                    }
                                    out_z = tmp_z;
                                }
                                if out_z > lowest_possible_z {
                                    lowest_possible_z = out_z;
                                }
                            }
                            if lowest_possible_z != z {
                                for block_y in start.y..=end.y {
                                    settled_grid.insert(
                                        Coords {
                                            x,
                                            y: block_y,
                                            z: lowest_possible_z,
                                        },
                                        *block_id,
                                    );
                                    settled_grid.remove(&Coords { x, y: block_y, z });
                                }
                                new_content.blocks.insert(
                                    *block_id,
                                    (
                                        Coords {
                                            x: start.x,
                                            y: start.y,
                                            z: lowest_possible_z,
                                        },
                                        Coords {
                                            x: end.x,
                                            y: end.y,
                                            z: lowest_possible_z,
                                        },
                                    ),
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    new_content.grid = settled_grid.clone();
    new_content
}

#[cfg(test)]
mod tests_settle {
    use super::*;

    #[test]
    fn settle_vertical_block() {
        let content = parse_content(
            &"\
1,0,30~1,0,32
",
        );
        let new_content = settle(&content);
        assert_eq!(
            new_content.grid,
            HashMap::from_iter(vec![
                (Coords { x: 1, y: 0, z: 1 }, 1),
                (Coords { x: 1, y: 0, z: 2 }, 1),
                (Coords { x: 1, y: 0, z: 3 }, 1)
            ])
        );
    }

    #[test]
    fn settle_horizontal_block_x_axis() {
        let content = parse_content(
            &"\
1,0,30~3,0,30
",
        );
        let new_content = settle(&content);
        assert_eq!(
            new_content.grid,
            HashMap::from_iter(vec![
                (Coords { x: 1, y: 0, z: 1 }, 1),
                (Coords { x: 2, y: 0, z: 1 }, 1),
                (Coords { x: 3, y: 0, z: 1 }, 1)
            ])
        );
    }

    #[test]
    fn settle_horizontal_block_y_axis() {
        let content = parse_content(
            &"\
7,3,30~7,5,30
",
        );
        let new_content = settle(&content);
        assert_eq!(
            new_content.grid,
            HashMap::from_iter(vec![
                (Coords { x: 7, y: 3, z: 1 }, 1),
                (Coords { x: 7, y: 4, z: 1 }, 1),
                (Coords { x: 7, y: 5, z: 1 }, 1)
            ])
        );
    }

    #[test]
    fn settle_vertical_block_on_another_block() {
        let content = parse_content(
            &"\
1,0,30~1,0,32
1,0,2~1,0,2
",
        );
        let new_content = settle(&content);
        assert_eq!(
            new_content.grid,
            HashMap::from_iter(vec![
                (Coords { x: 1, y: 0, z: 1 }, 2),
                (Coords { x: 1, y: 0, z: 2 }, 1),
                (Coords { x: 1, y: 0, z: 3 }, 1),
                (Coords { x: 1, y: 0, z: 4 }, 1),
            ])
        );
    }

    #[test]
    fn settle_horizontal_block_on_another_block() {
        let content = parse_content(
            &"\
1,0,5~3,0,5
2,0,1~2,2,1
",
        );
        let new_content = settle(&content);
        assert_eq!(
            new_content.grid,
            HashMap::from_iter(vec![
                (Coords { x: 2, y: 0, z: 1 }, 2),
                (Coords { x: 2, y: 1, z: 1 }, 2),
                (Coords { x: 2, y: 2, z: 1 }, 2),
                (Coords { x: 1, y: 0, z: 2 }, 1),
                (Coords { x: 2, y: 0, z: 2 }, 1),
                (Coords { x: 3, y: 0, z: 2 }, 1),
            ])
        );
    }

    #[test]
    fn settle_test_remove_disintegrable_block() {
        let content = parse_content(
            &"\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
",
        );
        let tmp_content = &remove_block(&settle(&content), &2);
        // Removing a disintegratable block should not change the setlled grid
        assert_eq!(settle(&tmp_content).grid, tmp_content.grid);
    }

    #[test]
    fn settle_test_input() {
        let content = parse_content(
            &"\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
",
        );
        let new_content = settle(&content);
        assert_eq!(
            new_content.grid,
            HashMap::from_iter(vec![
                (Coords { x: 1, y: 0, z: 1 }, 1), // A
                (Coords { x: 1, y: 1, z: 1 }, 1), // A
                (Coords { x: 1, y: 2, z: 1 }, 1), // A
                (Coords { x: 0, y: 0, z: 2 }, 2), // B
                (Coords { x: 1, y: 0, z: 2 }, 2), // B
                (Coords { x: 2, y: 0, z: 2 }, 2), // B
                (Coords { x: 0, y: 2, z: 2 }, 3), // C
                (Coords { x: 1, y: 2, z: 2 }, 3), // C
                (Coords { x: 2, y: 2, z: 2 }, 3), // C
                (Coords { x: 0, y: 0, z: 3 }, 4), // D
                (Coords { x: 0, y: 1, z: 3 }, 4), // D
                (Coords { x: 0, y: 2, z: 3 }, 4), // D
                (Coords { x: 2, y: 0, z: 3 }, 5), // E
                (Coords { x: 2, y: 1, z: 3 }, 5), // E
                (Coords { x: 2, y: 2, z: 3 }, 5), // E
                (Coords { x: 0, y: 1, z: 4 }, 6), // F
                (Coords { x: 1, y: 1, z: 4 }, 6), // F
                (Coords { x: 2, y: 1, z: 4 }, 6), // F
                (Coords { x: 1, y: 1, z: 5 }, 7), // G
                (Coords { x: 1, y: 1, z: 6 }, 7), // G
            ])
        );
    }

    #[test]
    fn settle_test_idempotency() {
        let content = parse_content(
            &"\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
",
        );
        let new_content = settle(&content);
        let new_content2 = settle(&new_content);
        assert_eq!(new_content.grid, new_content2.grid);
    }
}

fn remove_block(content: &Content, block_id_to_remove: &usize) -> Content {
    let mut new_content = content.clone();

    for z in 1..=content.max_z {
        // Browsing every possible block part on the Z plane
        for y in 0..=content.max_y {
            for x in 0..=content.max_x {
                if let Some(block_id) = content.grid.get(&Coords { x, y, z }) {
                    if block_id == block_id_to_remove {
                        new_content.grid.remove(&Coords { x, y, z });
                    }
                }
            }
        }
    }
    new_content.blocks.remove(block_id_to_remove);

    new_content
}

#[cfg(test)]
mod tests_remove_block {
    use super::*;

    #[test]
    fn remove_block_01() {
        let content = parse_content(
            &"\
1,0,5~3,0,5
2,0,1~2,2,1
",
        );
        let new_content = remove_block(&content, &1);
        assert_eq!(
            new_content.grid,
            HashMap::from_iter(vec![
                (Coords { x: 2, y: 0, z: 1 }, 2),
                (Coords { x: 2, y: 1, z: 1 }, 2),
                (Coords { x: 2, y: 2, z: 1 }, 2),
            ])
        );
        assert_eq!(
            new_content.blocks,
            HashMap::from_iter(vec![(
                2,
                (Coords { x: 2, y: 0, z: 1 }, Coords { x: 2, y: 2, z: 1 })
            )])
        );
    }
}

fn count_disintegratable(content: &Content) -> usize {
    let mut count = 0;
    let mut index = 0;

    // For each block in content.blocks
    for block_id in content.blocks.keys() {
        println!("{}: Testing block {}", index, block_id);
        // If we remove this block from the original grid
        let tmp_content = remove_block(&content.clone(), block_id);

        // And we settle this updated grid
        let settled = settle(&tmp_content);

        // Then if the settled grid is the same as the updated grid
        if settled.grid == tmp_content.grid {
            println!("Block {} can be disintegrated", block_id);
            // Then the block count as disintegratable
            count += 1;
        }
        index += 1
    }

    count
}

#[cfg(test)]
mod tests_count_disintegratable {
    use super::*;

    #[test]
    fn count_disintegratable_01() {
        let content = parse_content(
            &"\
1,0,5~3,0,5
2,0,1~2,2,1
",
        );
        let new_content = settle(&content);
        assert_eq!(count_disintegratable(&new_content), 1);
    }

    #[test]
    fn count_disintegratable_02() {
        assert_eq!(
            count_disintegratable(&settle(&parse_content(
                &"\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
",
            ))),
            5
        );
    }
}

#[allow(dead_code)]
fn display_content_grid_slice(content: &Content, z_level: usize) {
    for y in 0..=content.max_y {
        for x in 0..=content.max_x {
            print!(
                "{:0>2} ",
                match content.grid.get(&Coords { x, y, z: z_level }) {
                    Some(id) => id.to_string(),
                    None => "..".to_string(),
                }
            );
        }
        println!();
    }
}

fn main() {
    let content = &parse_content(&get_file_content("assets/input"));

    println!("Part 1: {}", count_disintegratable(&settle(&content)));
}
