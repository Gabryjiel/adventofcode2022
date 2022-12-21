fn main() {
    task1()
}

fn task1() {
    let input =
        std::fs::read_to_string("./src/bin/day10/input.txt").expect("Unable to read the file");

    let result = input.split('\n').fold((0, 1, 0), |acc, line| {
        let (cycles, value, sum) = acc;
        let mut signal_values_sum = sum;

        let (cycles_to_add, value_to_add) = if line == "noop" {
            (1, 0)
        } else {
            let (_command, value) = line.split_once(' ').expect("Canno split");
            let num = value.parse::<i32>().expect("Cannot parse");
            (2, num)
        };

        for i in 1..=cycles_to_add {
            if ((acc.0 + i) - 20) % 40 == 0 && cycles < 221 {
                signal_values_sum += value * (cycles + i)
            }
        }

        (
            cycles + cycles_to_add,
            value + value_to_add,
            signal_values_sum,
        )
    });

    println!("{:?}", result);
}
