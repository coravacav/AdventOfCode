use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(input));
    // println!("part2: {}", part2(input));
}

fn is_possible(input: &str) -> (usize, usize, usize, bool) {
    let grab_color_regex = Regex::new(r"(\d+) (blue|red|green)").unwrap();

    let mut x = grab_color_regex
        .captures_iter(input)
        .map(|cap| (cap[1].to_owned(), cap[2].to_owned()))
        .map(|(count, color)| match color.as_str() {
            "red" => (
                "red",
                count.parse::<usize>().unwrap(),
                count.parse::<usize>().unwrap() <= MAX_RED,
            ),
            "green" => (
                "green",
                count.parse::<usize>().unwrap(),
                count.parse::<usize>().unwrap() <= MAX_GREEN,
            ),
            "blue" => (
                "blue",
                count.parse::<usize>().unwrap(),
                count.parse::<usize>().unwrap() <= MAX_BLUE,
            ),
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    dbg!(&x);

    (
        *x.iter()
            .find(|(color, _, _x)| *color == "red")
            .map(|(_, x, _)| x)
            .unwrap_or(&1),
        *x.iter()
            .find(|(color, _, _x)| *color == "green")
            .map(|(_, x, _)| x)
            .unwrap_or(&1),
        *x.iter()
            .find(|(color, _, _x)| *color == "blue")
            .map(|(_, x, _)| x)
            .unwrap_or(&1),
        x.iter().all(|(_s, _, x)| *x),
    )
}

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|game| {
            let mut split = game.split(":");
            let _game = split.next().unwrap();
            // // select only numbers and parse
            // let game = game
            //     .chars()
            //     .filter(|c| c.is_numeric())
            //     .collect::<String>()
            //     .parse()
            //     .unwrap();

            let rem = split.next().unwrap();

            // dbg!(rem);

            let res = rem.split(";").map(|input| (is_possible(input)));

            let max_red = res.clone().max_by_key(|(red, _, _, _)| *red).unwrap().0;
            let max_green = res.clone().max_by_key(|(_, green, _, _)| *green).unwrap().1;
            let max_blue = res.clone().max_by_key(|(_, _, blue, _)| *blue).unwrap().2;

            dbg!(max_red, max_green, max_blue);

            let possible = res.clone().all(|(_, _, _, possible)| possible);

            max_red * max_green * max_blue
        })
        .sum()
}

#[test]
fn test_part1() {
    let test = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    assert_eq!(part1(test), 2286);
}
