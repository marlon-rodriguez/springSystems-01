fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        return 0;
    }
    else if guess < secret {
        return -1;
    }
    else{
        return 1;
    }
}

fn main(){
    let secret = 150;
    let mut guess = 297;
    let mut attempts = 0;

    loop {
        let result = check_guess(guess, secret);
        attempts += 1;

        if result == 0 {
            println!("The guess {} is correct!", guess);
            break;
        }
        else if result == -1 {
            println!("The guess {} is too low.", guess);
            guess = guess + (secret - guess) / 2 + 1;
        }
        else {
            println!("The guess {} is too high.", guess);
            guess = guess + (secret - guess) / 2 - 1;
        }
    }

    println!("Amount of guesses: {}", attempts);
}