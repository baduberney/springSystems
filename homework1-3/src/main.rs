fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret_number: i32 = 7;
    let guess: i32 = 7;
    let mut attempts = 0;

    loop {
        attempts += 1;

        let result = check_guess(guess, secret_number);

        if result == 0 {
            println!("Correct. The number was {}.", secret_number);
            break;
        } else if result == 1 {
            println!("{} is too high.", guess);
            break;
        } else {
            println!("{} is too low.", guess);
            break;
        }
    }

    println!("It took {} guesses.", attempts);
}