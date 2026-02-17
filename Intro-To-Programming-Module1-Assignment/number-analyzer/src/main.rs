
fn is_even(num:i32) -> bool {
    return num % 2 == 0;
}

fn main(){
    let arr : [i32; 10] = [12, 45, 67, 89, 23, 56, 78, 90, 34, 29];
    for i in 0..arr.len() {
        if is_even(arr[i]){
            println!("{} is even", arr[i]);
        }
        else{
            println!("{} is odd", arr[i]);
        }

        if arr[i] % 3 == 0 && arr[i] % 5 == 0{
            println!("FizzBuzz");
        }
        else if arr[i] % 3 == 0{
            println!("Fizz");
        }
        else if arr[i] % 5 == 0{
            println!("Buzz");
        }
    }

    println!("\n");
    let mut sum = 0;
    let mut i = 0;
    while i < arr.len(){
        sum += arr[i];
        i += 1;
    }
    println!("Sum = {}", sum);
    println!("\n");
    
    let mut largest = arr[0];
    let mut j = 1;

    loop {
        if arr[j] > largest {
            largest = arr[j];
        }
        j += 1;
        if j >= arr.len() {
            break;
        }
    }

    println!("Largest number = {}", largest);
}