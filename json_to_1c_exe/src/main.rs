use clap::{value_parser, Arg, ArgAction, Command};
use json_to_1c_lib::ParseJson1c;
use std::{
    fs,
    io::{self, BufRead},
};

fn main() {
    let cmd = Command::new("cmd")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .action(ArgAction::Set)
                .value_name("FILE")
                .help("Имя файла JSON"),
        )
        .arg(
            Arg::new("random")
                .short('r')
                .long("random")
                .action(ArgAction::SetTrue)
                .help("Добавить комментарий для генерации случайных значений в yaxunit"),
        )
        .arg(
            Arg::new("show_area")
                .short('s')
                .long("showarea")
                .action(ArgAction::SetTrue)
                .help("Добавить обрамление объектов областями 1С"),
        )
        .arg(
            Arg::new("count_to_wrap_array")
                .short('w')
                .long("count_to_wrap_array")
                .action(ArgAction::Set)
                .value_parser(value_parser!(usize))
                .default_value("7")
                .help("Сворачивать массивы в цикле, при первышении длины"),
        )
        .get_matches();

    let random = cmd.get_flag("random");
    let show_area = cmd.get_flag("show_area");
    let count_to_wrap_array = *cmd.get_one::<usize>("count_to_wrap_array").unwrap();

    let mut str = String::new();

    if let Some(file_name) = cmd.get_one::<String>("file") {
        let res: Result<String, std::io::Error> = fs::read_to_string(file_name);
        str = match res {
            Ok(s) => s,
            Err(_) => panic!("Не читается файл"),
        };
    } else {
        println!("Вставьте JSON:");

        let lines = io::stdin().lock().lines();
        for line in lines {
            let last_input = line.unwrap();

            str.push_str(&last_input);

            if last_input.is_empty() {
                break;
            }
        }
    };

    let parser = ParseJson1c::new(random, show_area, count_to_wrap_array);

    let result_str = parser.parse(str.as_str());

    for str in &result_str {
        println!("\t{str}");
    }
}
