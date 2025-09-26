fn is_even(n: i32) -> bool {
    if n%2==0{
        true
    }
    else{
        false
    }
}

fn main() {
    let numbers: [i32; 10] = [11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let mut sum= 0;
    let mut greatest= 0;

    for i in 0..10{

        if numbers[i] % 3 == 0 && numbers[i] % 5 == 0{
            println!("FizzBuzz")
        }
        else if numbers[i] % 3==0 {
            println!("Fizz")
        }

        else if numbers[i] % 5 == 0{
                println!("Buzz")
        }

        else{
        println!("{}",is_even(numbers[i]));
        }
    }
    
    let mut x= 0;
    
    while x< 10{
        sum= sum+ numbers[x];
        x+=1;
    }

    println!("Sum= {}", sum);

    x=0;
    loop {
        if numbers [x] > greatest{
            greatest= numbers[x];
        }
        x+=1;
        
        if x==10{
            break
        }    
    }

    println!("Greatest= {}", greatest);
}