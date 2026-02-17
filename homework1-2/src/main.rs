fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers: [i32; 10] = [3, 8, 17, 24, 9, 32, 13, 18, 7, 90];

    for number in numbers {
        if number % 3 == 0 && number % 5 == 0 {
            println!("{}: FizzBuzz", number);
        } else if number % 3 == 0 {
            println!("{}: Fizz", number);
        } else if number % 5 == 0 {
            println!("{}: Buzz", number);
        } else if is_even(number) {
            println!("{}: Even", number);
        } else {
            println!("{}: Odd", number);
        }
    }

    let mut index = 0;
    let mut sum = 0;

    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }

    println!("Sum of numbers: {}", sum);

    let mut largest = numbers[0];
    let mut i = 1;

    loop {
        if i >= numbers.len() {
            break;
        }
        if numbers[i] > largest {
            largest = numbers[i];
        }

        i += 1;
    }

    println!("Largest number: {}", largest);
}