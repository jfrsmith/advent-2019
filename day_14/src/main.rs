use std::collections::HashMap;

#[derive(Debug)]
struct Reaction {
    num_output: u64,
    inputs: Vec<(String, u64)>,
}

type Reactions = HashMap<String, Reaction>;

fn parse(input: &'static str) -> Reactions {
    let mut reactions = Reactions::new();

    for l in input.lines() {
        let split_reaction = l.trim().split("=>").map(|s| s.trim()).collect::<Vec<&str>>();
        assert!(split_reaction.len() == 2);
        let inputs_str = split_reaction[0].split(",").map(|s| s.trim()).collect::<Vec<&str>>();
        let output_str = split_reaction[1];

        let split_quantities = |s: &str| -> (String, u64) {
            let split = s.split_whitespace().collect::<Vec<&str>>();
            assert!(split.len() == 2);
            (split[1].to_owned(), split[0].parse::<u64>().unwrap())
        };

        let inputs = inputs_str.iter().map(|i| split_quantities(i)).collect::<Vec<(String, u64)>>();
        let (output_chem, num_output) = split_quantities(output_str);

        reactions.insert(output_chem, Reaction { num_output, inputs });
    }

    reactions
}

fn run_reaction(reactions: &Reactions, chemical: &str, amount: u64, leftover: &mut HashMap<String, u64>) -> u64 {
    println!("Running reaction to create {} x {}...", amount, chemical);

    let reaction = reactions.get(chemical).expect(&format!("Failed to find reaction for chemical '{}'", chemical));
    let runs_required = ((amount as f32) / (reaction.num_output as f32)).ceil() as u64;
    let output = runs_required * reaction.num_output;
    println!("      Creating {} x {} requires us to run reaction {} times which will create {} x {}", amount, chemical, runs_required, output, chemical);

    if reaction.inputs[0].0 == "ORE" {
        println!("      Creating {} x {} requires {} x ORE!", output, chemical, (runs_required * reaction.inputs[0].1));
        *leftover.entry("ORE".to_owned()).or_insert(0) += runs_required * reaction.inputs[0].1;
    } else {
        for r in reaction.inputs.iter() {
            let mut amount_required = r.1 * runs_required;
            println!("      Creating {} x {} requires: {} x ({} x {})", output, chemical, runs_required, r.1, r.0);

            if let Some(waste) = leftover.remove(&r.0) {
                println!("              We have {} x {} in the waste pile that we can use for this!", waste, r.0);  
                if waste > amount_required {
                    leftover.insert(r.0.to_owned(), waste - amount_required);
                    amount_required = 0;
                } else {
                    amount_required -= waste;
                }
            }

            let num_created = run_reaction(reactions, &r.0, amount_required, leftover);

            if num_created > amount_required {
                println!("Last reaction tree created {} leftover {}", num_created - amount_required, r.0);
                println!("*****************************************");
                *leftover.entry(r.0.to_owned()).or_insert(0) += num_created - amount_required;
            }
        }
    }

    println!("      Created {} x {}!", output, chemical);
    println!("*****************************************");
    output
}

fn get_ore_for_fuel(reactions: &Reactions) -> u64 {
    let mut leftover_map = HashMap::new();
    run_reaction(&reactions, "FUEL", 1, &mut leftover_map);
    *leftover_map.get("ORE").expect("Failed to find any ORE in the leftover pile")
}

fn main() {
    println!("Part 1 => {}", get_ore_for_fuel(&parse(include_str!("../input/day_14.txt"))));
}

#[test]
fn part_1_complete() {
    assert_eq!(get_ore_for_fuel(&parse(include_str!("../input/day_14.txt"))), 202617);
}

#[test]
fn leftover() {
    let input = "10 ORE => 5 A
    2 A => 3 B
    7 B => 1 C
    7 B, 1 C => 1 FUEL";

    assert_eq!(get_ore_for_fuel(&parse(&input)), 20);
}

#[test]
fn test_a() {
    let input = "10 ORE => 10 A
    1 ORE => 1 B
    7 A, 1 B => 1 C
    7 A, 1 C => 1 D
    7 A, 1 D => 1 E
    7 A, 1 E => 1 FUEL";

    assert_eq!(get_ore_for_fuel(&parse(&input)), 31);
}

#[test]
fn test_b() {
    let input = "9 ORE => 2 A
    8 ORE => 3 B
    7 ORE => 5 C
    3 A, 4 B => 1 AB
    5 B, 7 C => 1 BC
    4 C, 1 A => 1 CA
    2 AB, 3 BC, 4 CA => 1 FUEL";

    assert_eq!(get_ore_for_fuel(&parse(&input)), 165);
}

#[test]
fn test_c() {
    let input = "157 ORE => 5 NZVS
    165 ORE => 6 DCFZ
    44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
    12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
    179 ORE => 7 PSHF
    177 ORE => 5 HKGWZ
    7 DCFZ, 7 PSHF => 2 XJWVT
    165 ORE => 2 GPVTF
    3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    assert_eq!(get_ore_for_fuel(&parse(&input)), 13312);
}

#[test]
fn test_d() {
    let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
    17 NVRVD, 3 JNWZP => 8 VPVL
    53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
    22 VJHF, 37 MNCFX => 5 FWMGM
    139 ORE => 4 NVRVD
    144 ORE => 7 JNWZP
    5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
    5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
    145 ORE => 6 MNCFX
    1 NVRVD => 8 CXFTF
    1 VJHF, 6 MNCFX => 4 RFSQX
    176 ORE => 6 VJHF";

    assert_eq!(get_ore_for_fuel(&parse(&input)), 180697);
}

#[test]
fn test_e() {
    let input = "171 ORE => 8 CNZTR
    7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
    114 ORE => 4 BHXH
    14 VRPVC => 6 BMBT
    6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
    6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
    15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
    13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
    5 BMBT => 4 WPTQ
    189 ORE => 9 KTJDG
    1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
    12 VRPVC, 27 CNZTR => 2 XDBXC
    15 KTJDG, 12 BHXH => 5 XCVML
    3 BHXH, 2 VRPVC => 7 MZWV
    121 ORE => 7 VRPVC
    7 XCVML => 6 RJRHP
    5 BHXH, 4 VRPVC => 5 LTCX";

    assert_eq!(get_ore_for_fuel(&parse(&input)), 2210736);
}