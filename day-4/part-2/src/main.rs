use std::fs;

fn get_file_content(file_path: &String) -> String {
    println!("Loading input file: {}", file_path);
    fs::read_to_string(file_path).expect("Cannot load file")
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Scratchcard {
    id: usize,
    winning_numbers: Vec<u32>,
    our_numbers: Vec<u32>,
    copies: u32,
}

// Using a two dimensional array allows us to handle multiple scratchcard copies
type ScratchcardList = Vec<Scratchcard>;

fn parse_scratchcard(line: &str) -> Scratchcard {
    let mut split = line.split("|");
    let mut first_block = split.next().unwrap().split(":");

    let mut scratchcard: Scratchcard = Scratchcard {
        id: first_block
            .next()
            .unwrap()
            .replace("Card", "")
            .replace(" ", "")
            .parse()
            .expect("Error parsing scratchcard id"),
        winning_numbers: vec![],
        our_numbers: Vec::new(),
        copies: 1,
    };

    if let Some(winning_numbers) = first_block.next() {
        for winning_number in winning_numbers.split(" ") {
            let temp = winning_number.parse().unwrap_or(0);
            if temp != 0 {
                scratchcard.winning_numbers.push(temp);
            }
        }
    }

    if let Some(our_numbers) = split.next() {
        for our_number in our_numbers.split(" ") {
            let temp = our_number.parse().unwrap_or(0);
            if temp != 0 {
                scratchcard.our_numbers.push(temp);
            }
        }
    }

    return scratchcard;
}

#[cfg(test)]
mod tests_parse_scratchcard {
    use super::*;

    #[test]
    fn parse_scratchcard_01() {
        let scratchcard = parse_scratchcard(&String::from(
"Card   123: 73 92 13 35 18 96 37 72 76 39 | 82 14 66 57 25 98 49 28  3 95 81 85 31 30 16 79  7 12 55 19 97 45  9 58  2"
        ));

        assert_eq!(scratchcard.id, 123);
        assert_eq!(
            scratchcard.winning_numbers,
            vec![73, 92, 13, 35, 18, 96, 37, 72, 76, 39]
        );
        assert_eq!(
            scratchcard.our_numbers,
            vec![
                82, 14, 66, 57, 25, 98, 49, 28, 3, 95, 81, 85, 31, 30, 16, 79, 7, 12, 55, 19, 97,
                45, 9, 58, 2
            ]
        );
    }
}

fn get_winning_numbers_count(scratchcard: &Scratchcard) -> u32 {
    return scratchcard
        .our_numbers
        .iter()
        .map(|n| scratchcard.winning_numbers.iter().find(|w| w == &n))
        .filter(|found| match found {
            None => false,
            Some(_) => true,
        })
        .count() as u32;
}

#[cfg(test)]
mod tests_get_winning_numbers_count {
    use super::*;

    #[test]
    fn get_winning_numbers_count_regular() {
        assert_eq!(
            get_winning_numbers_count(&Scratchcard {
                id: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                our_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
                copies: 1
            }),
            4
        );
    }

    #[test]
    fn get_winning_numbers_count_empty_our_numbers() {
        assert_eq!(
            get_winning_numbers_count(&Scratchcard {
                id: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                our_numbers: vec![],
                copies: 1
            }),
            0
        );
    }

    #[test]
    fn get_winning_numbers_count_empty_winning_numbers() {
        assert_eq!(
            get_winning_numbers_count(&Scratchcard {
                id: 1,
                winning_numbers: vec![],
                our_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
                copies: 1
            }),
            0
        );
    }

    #[test]
    fn get_winning_numbers_count_double_winning_number_83() {
        assert_eq!(
            get_winning_numbers_count(&Scratchcard {
                id: 1,
                winning_numbers: vec![41, 48, 83, 86, 17, 83],
                our_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53, 83],
                copies: 1
            }),
            5
        );
    }
}

fn insert_copies(card_id: usize, scratchcard_list: &ScratchcardList) -> ScratchcardList {
    let mut out_list = scratchcard_list.clone();

    let how_many_to_add = get_winning_numbers_count(&scratchcard_list.get(card_id).unwrap());
    // println!("Adding {} after card {}", how_many_to_add, card_id);

    for i in 1..=how_many_to_add as usize {
        let further_card = scratchcard_list.get(card_id + i);
        match further_card {
            Some(_) => out_list[card_id + i].copies += 1,
            _ => {}
        };
    }
    return out_list;
}

#[cfg(test)]
mod tests_insert_copies {
    use super::*;

    #[test]
    fn insert_copies_not_enough_scratchcards() {
        let mut scratchcard_list: ScratchcardList = vec![];
        scratchcard_list.push(Scratchcard {
            id: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            our_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            copies: 1,
        });
        insert_copies(0, &scratchcard_list);
        assert_eq!(
            scratchcard_list
                .iter()
                .fold(0, |acc, card| acc + card.copies),
            1
        );
    }

    #[test]
    fn insert_copies_add_only_one_cause_not_enough_to_add() {
        let mut scratchcard_list: ScratchcardList = vec![];
        scratchcard_list.push(Scratchcard {
            id: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            our_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            copies: 1,
        });

        scratchcard_list.push(Scratchcard {
            id: 2,
            winning_numbers: vec![73, 92, 13, 35, 18, 96, 37, 72, 76, 39],
            our_numbers: vec![
                82, 14, 66, 57, 25, 98, 49, 28, 3, 95, 81, 85, 31, 30, 16, 79, 7, 12, 55, 19, 97,
                45, 9, 58, 2,
            ],
            copies: 1,
        });
        let updated_list = insert_copies(0, &scratchcard_list);
        assert_eq!(
            updated_list.iter().fold(0, |acc, card| acc + card.copies),
            3
        );
    }

    #[test]
    fn insert_copies_add_enough() {
        let mut scratchcard_list: ScratchcardList = vec![];
        scratchcard_list.push(Scratchcard {
            id: 0,
            winning_numbers: vec![41, 48, 83, 86, 17],
            our_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            copies: 1,
        });

        for i in 0..10 {
            scratchcard_list.push(Scratchcard {
                id: i,
                winning_numbers: vec![],
                our_numbers: vec![],
                copies: 1,
            });
        }
        let updated_list = insert_copies(0, &scratchcard_list);
        assert_eq!(
            updated_list.iter().fold(0, |acc, card| acc + card.copies),
            15
        );
    }
}

fn main() {
    let scratchcard_list: ScratchcardList = {
        // Initializing with an empty scratchcard at position 0 to avoid
        // a noobish off-by-one error
        let mut temp: ScratchcardList = vec![Scratchcard {
            id: 0,
            winning_numbers: vec![],
            our_numbers: vec![],
            copies: 0,
        }];
        for line in get_file_content(&String::from("assets/input")).lines() {
            let new_scratchcard = parse_scratchcard(line);
            temp.push(new_scratchcard);
        }
        temp
    };

    let mut card_id = 0;
    let mut temp_list: ScratchcardList = scratchcard_list.clone();
    while let Some(card) = temp_list.clone().get(card_id) {
        println!(
            "Handling card {}, which has {} copies",
            card.id, card.copies
        );
        for _ in 1..=card.copies {
            temp_list = insert_copies(card_id, &temp_list);
        }
        card_id += 1;
    }

    println!(
        "Scratchcard copies count: {}",
        temp_list.iter().fold(0, |acc, card| acc + card.copies)
    );
    println!("Finished !");
}
