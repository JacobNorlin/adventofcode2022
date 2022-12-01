use std::fs;

pub fn main() {
    let content = fs::read_to_string("input/day1.txt").expect("Unable to find input");

    let elf_calories = content.split("\n\n").map(|elf| {
        let elf_cals = elf.split("\n").map(|cal_str| cal_str.parse::<i32>().unwrap());
        let cal_sum = elf_cals.fold(0, |s, cal| s + cal);
        return cal_sum;
    }).collect::<Vec<i32>>();



    let part1 = get_sum_of_top_cals(&elf_calories, 1);
    let part2 = get_sum_of_top_cals(&elf_calories, 3);


    println!("Day 1 part 1: {}", part1);
    println!("Day 1 part2: {}", part2);
}

fn get_sum_of_top_cals(list: &Vec<i32>, num: usize) -> i32{
    let top_elms = list.into_iter().fold(vec![-1; num], |mut acc, cal_sum| {
        for i in 0 .. acc.len() {
            if *cal_sum > acc[i] {
                acc[i] = *cal_sum;
                break;
            }
        }
        return acc;
    });

    return top_elms.into_iter().fold(0, |acc: i32,v: i32| acc + v);
}