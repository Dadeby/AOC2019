use std::fs::File;
use std::io::BufReader;
use std::io::{self, prelude::*};

fn main() -> io::Result<()>{
    let filename = String::from("src/day1/input.txt");

    let massVec = readFile(filename);
    let mut sum = 0;
    for mass in &massVec
        {
            sum += calcFuel(*mass);
        }

    println!("Fuel needed is {}", sum);
    let mut sumWithFuel = 0;
    for mass in &massVec
        {

        let mut fuel = calcFuel(*mass);
        sumWithFuel += fuel;
        loop
            {
                fuel = calcFuel(fuel);
                if fuel <= 0
                {
                    break;
                }
                sumWithFuel += fuel;
            }
        }
    println!("Total fueld needed is {}", sumWithFuel);
    Ok(())
}

fn calcFuel(mass: i64) -> i64 {
    return (mass / 3) - 2;
}

fn readFile(filename : String) -> Vec<i64> {
    println!("Reading input from {}", filename);

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut res : Vec<i64> = Vec::new();
    for line in reader.lines()
        {
            let line = line.unwrap();
            let mass = line.trim().parse().unwrap();
            res.push(mass);
        }
    return res;
}