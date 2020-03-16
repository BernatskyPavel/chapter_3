use std::io;
use std::ops::Add;

fn main() {
    loop {
        println!("Please select task:");
        println!("1.Temperatures converter.");
        println!("2.Fibonacci numbers.");
        println!("3.Christmas carol.");
        println!("4.Exit.");

        let mut action_index = String::new();

        io::stdin()
            .read_line(&mut action_index)
            .expect("Failed to read line");

        let action_index: usize = match action_index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if action_index == 1 {
            let regim: usize = loop {
                println!("Please select converter regim:");
                println!("1.F -> C.");
                println!("2.C -> F.");

                let mut regim_index = String::new();

                io::stdin()
                    .read_line(&mut regim_index)
                    .expect("Failed to read line");

                let regim_index: usize = match regim_index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                break regim_index;
            };
            let temperature: f64 = loop {
                println!("Please input temperature:");
                let mut temperature = String::new();
                io::stdin()
                    .read_line(&mut temperature)
                    .expect("Failed to read line");

                let temperature: f64 = match temperature.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                break temperature;
            };
            println!(
                "Result temperature is {}.",
                temperatures_task(temperature, regim)
            );
        } else if action_index == 2 {
            let fibonacci_index: usize = loop {
                println!("Please input Fibonacci number's index:");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read line");

                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                break index;
            };
            println!(
                "Fibonacci number with index {} is {}",
                fibonacci_index,
                n_fibonacci_number(fibonacci_index)
            );
        } else if action_index == 3 {
            let song_index: usize = loop {
                println!("Please input number of days in song:");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read line");

                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                break index;
            };
            twelve_days(song_index);
        } else if action_index == 4 {
            break;
        }
    }
}

fn temperatures_task(temperature: f64, regim: usize) -> f64 {
    if regim == 1 {
        5.0 / 9.0 * (temperature - 32.0)
    } else if regim == 2 {
        9.0 / 5.0 * temperature + 32.0
    } else {
        0.0
    }
}

fn n_fibonacci_number(fibonacci_index: usize) -> usize {
    if fibonacci_index == 0 {
        0
    } else if fibonacci_index == 1 {
        1
    } else {
        let mut prev_numbers: (usize, usize) = (0, 1);
        let mut number: usize;
        let mut index: usize = 2;
        let number: usize = loop {
            number = prev_numbers.0 + prev_numbers.1;
            prev_numbers.0 = prev_numbers.1;
            prev_numbers.1 = number;
            if index == fibonacci_index {
                break number;
            };
            index = index + 1;
        };
        number
    }
}

fn twelve_days(days_number: usize) {
    if days_number > 12 || days_number == 0 {
        println!("Wrong number of days!");
        return;
    }
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let sents = [
        "Partridge in a Pear Tree\n",
        "2 Turtle Doves\n",
        "3 French Hens\n",
        "4 Calling Birds\n",
        "5 Golden Rings\n",
        "6 Geese a Laying\n",
        "7 Swans a Swimming\n",
        "8 Maids a Milking\n",
        "9 Ladies Dancing\n",
        "10 Lords a Leaping\n",
        "11 Pipers Piping\n",
        "12 Drummers Drumming\n",
    ];

    for index in 0..days_number {
        let mut i = index;
        let mut sents_str = String::new();
        if index == 0 {
            sents_str = sents_str.add("A ").add(sents[index]);
        } else {
            sents_str = loop {
                sents_str = sents_str.add(sents[i]);
                if i == 1 {
                    sents_str = sents_str.add("and a ").add(sents[i - 1]);
                    break sents_str;
                };
                i = i - 1;
            }
        }
        println!(
            "On the {} day of Christmas\nmy true love sent to me:\n{}",
            days[index], sents_str
        );
    }
}
