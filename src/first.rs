use crate::utils;

pub fn execute() {
    let data = utils::get_data(1);
    let split_data = data.split("\n");
    let mut elf_calories = 0;
    let mut elf_most_calories = 0;
    for s in split_data {
        match s.parse::<i32>() {
            Ok(n) => {
                elf_calories = elf_calories + n;
                if elf_calories > elf_most_calories {
                    elf_most_calories = elf_calories;
                }
            }
            _ => {
                elf_calories = 0;
            }
        }
    }
    println!("{:?}", elf_most_calories);
}
