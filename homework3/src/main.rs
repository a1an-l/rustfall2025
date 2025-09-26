fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret{
        return 0;
    }

    else if guess > secret{ 
        return 1;
    }

    else{
        return -1;
    }
}

fn main() {
    let secret= 10;
    let mut guess =1;
    let mut guesses =0;
    loop{
        
        guesses+=1;
        if check_guess(guess, secret)== -1{
            println!("Too low.");
        }
        else if check_guess(guess, secret)== 1{
            println!("Too high.");
        }

        else{
            println!("Correct");
            break;
        }
        if guess < secret{
        guess+=2;
        }
        else {
            guess-=1;
        }
    }
    println!("Guesses= {}", guesses);

}
