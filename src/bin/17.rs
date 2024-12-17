use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(17);

#[allow(non_camel_case_types)]
#[repr(u8)]
enum OpCode {
    adv = 0, // `A` / [combo]^2 => truncated => `A`
    bxl = 1, // `B` XOR [literal] => `B`
    bst = 2, // [combo] % 8 => B
    jnz = 3, // if `A` == 0 ? NOP : JMP to [literal]
    bxc = 4, // `B` XOR `C` (ignores operand) => B
    out = 5, // [combo] % 8 => OUTPUT
    bdv = 6, // same as adv but result is stored in `B`
    cdv = 7, // same as adv but result is stored in `C`
}
impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == OpCode::adv as u8 => Ok(OpCode::adv),
            x if x == OpCode::bxl as u8 => Ok(OpCode::bxl),
            x if x == OpCode::bst as u8 => Ok(OpCode::bst),
            x if x == OpCode::jnz as u8 => Ok(OpCode::jnz),
            x if x == OpCode::bxc as u8 => Ok(OpCode::bxc),
            x if x == OpCode::out as u8 => Ok(OpCode::out),
            x if x == OpCode::bdv as u8 => Ok(OpCode::bdv),
            x if x == OpCode::cdv as u8 => Ok(OpCode::cdv),
            _ => Err(()),
        }
    }
}

const REG_A: usize = 0;
const REG_B: usize = 1;
const REG_C: usize = 2;

pub fn part_one(input: &str) -> Option<String> {
    let re = Regex::new(r"Register A: (?<reg_a>\d+)\nRegister B: (?<reg_b>\d+)\nRegister C: (?<reg_c>\d+)\n\nProgram: (?<program>.*)").unwrap();
    let cap = re.captures(input).unwrap();

    let mut registers = vec![
        cap["reg_a"].parse::<usize>().unwrap(),
        cap["reg_b"].parse::<usize>().unwrap(),
        cap["reg_c"].parse::<usize>().unwrap(),
    ];

    let program = cap["program"].split(',').map(|c| c.parse::<u8>().unwrap()).collect_vec();

    let output = run_program(&mut registers, program);
    // println!("OUTPUT {:?}", output);

    let result = output.into_iter().map(|d| char::from_digit(d.try_into().unwrap(), 10).unwrap()).join(",");
    // println!("RESULT {result}");
    Some(result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn run_program(registers: &mut Vec<usize>, program: Vec<u8>) -> Vec<usize> {
    let mut output: Vec<usize> = vec![];

    let literal = |v: usize| v;
    let combo = |v: usize, r: Vec<usize>| {
        if v == 7 { panic!() }
        else if (0..=3).contains(&v) { v }
        else { r[v - 4] }
    };

    let mut ptr = 0;
    while ptr < program.len() {
        let op = program[ptr];
        let oper = program[ptr+1] as usize;

        match op.try_into() {
            Ok(OpCode::adv) => {
                // `A` / [combo]^2 => truncated => `A`
                let value = registers[REG_A] / 2_usize.pow(combo(oper, registers.to_vec()) as u32);
                // println!("ADV: REG_A = {value}");
                registers[REG_A] = value;
                ptr += 2;
            },
            Ok(OpCode::bxl) => {
                // `B` XOR [literal] => `B`
                let value = registers[REG_B] ^ literal(oper);
                // println!("BXL: REG_B = {value}");
                registers[REG_B] = value;
                ptr += 2;
            },
            Ok(OpCode::bst) => {
                // [combo] % 8 => B
                let value = combo(oper, registers.to_vec()) % 8;
                // println!("BST: REG_B = {value}");
                registers[REG_B] = value;
                ptr += 2;
            },
            Ok(OpCode::jnz) => {
                // if `A` == 0 ? NOP : JMP to [literal]
                if registers[REG_A] == 0 {
                    // println!("JNZ: NOP");
                    ptr += 2;
                } else {
                    // println!("JNZ: {}", literal(oper));
                    ptr = literal(oper);
                }
            },
            Ok(OpCode::bxc) => {
                // `B` XOR `C` (ignores operand) => B
                let value = registers[REG_B] ^ registers[REG_C];
                // println!("BXC: REG_B = {value}");
                registers[REG_B] = value;
                ptr += 2;
            },
            Ok(OpCode::out) => {
                // [combo] % 8 => OUTPUT
                let value = combo(oper, registers.to_vec()) % 8;
                // println!("OUT: {}", value);
                output.push(value);
                ptr += 2;
            },
            Ok(OpCode::bdv) => {
                // same as adv but result is stored in `B`
                let value = registers[REG_A] / 2_usize.pow(combo(oper, registers.to_vec()) as u32);
                // println!("BDV: REG_B = {value}");
                registers[REG_B] = value;
                ptr += 2;
            },
            Ok(OpCode::cdv) => {
                // same as adv but result is stored in `C`
                let value = registers[REG_A] / 2_usize.pow(combo(oper, registers.to_vec()) as u32);
                // println!("CDV: REG_C = {value}");
                registers[REG_C] = value;
                ptr += 2;
            },
            Err(_) => panic!()
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() {
        // If register `C` contains `9`, the program `2,6` would set register `B` to `1`.
        let mut registers = vec![0, 0, 9];
        run_program(&mut registers, vec![2,6]);
        assert_eq!(registers[1], 1);

        // If register `A` contains `10`, the program `5,0,5,1,5,4` would output `0,1,2`.
        registers = vec![10, 0, 0];
        let result = run_program(&mut registers, vec![5,0,5,1,5,4]);
        assert_eq!(result, [0,1,2]);

        // If register `A` contains `2024`, the program `0,1,5,4,3,0` would output `4,2,5,6,7,7,7,7,3,1,0` and leave `0` in register `A`.
        registers = vec![2024, 0, 0];
        let result = run_program(&mut registers, vec![0,1,5,4,3,0]);
        assert_eq!(result, [4,2,5,6,7,7,7,7,3,1,0]);
        assert_eq!(registers[0], 0);

        // If register `B` contains `29`, the program `1,7` would set register `B` to `26`.
        registers = vec![0, 29, 0];
        run_program(&mut registers, vec![1,7]);
        assert_eq!(registers[1], 26);

        // If register `B` contains `2024` and register `C` contains `43690`, the program `4,0` would set register `B` to `44354`.
        registers = vec![0, 2024, 43690];
        run_program(&mut registers, vec![4,0]);
        assert_eq!(registers[1], 44354);
    }

    #[test]
    fn test_part_one() {
        let binding = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&binding);
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
