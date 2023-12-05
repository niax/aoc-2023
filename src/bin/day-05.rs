use aoc_2023::commons::io::Input;
use itertools::Itertools;
use peg::str::LineCol;
use std::error::Error;
use std::str::FromStr;

peg::parser! {
    grammar puzle_parser() for str {
        rule fileend() -> () = ("\n" / "");

        rule number() -> u32
            = n:$(['0'..='9']+) {? n.parse().or(Err("bad number")) }

        rule number_list() -> Vec<u32>
            = number() ** " "

        rule mapping() -> Mapping
            = dst_start:number() " " src_start:number() " " length:number() {
                Mapping {
                    src_range: src_start..(src_start + length),
                    dst_start
                }
            }

        rule mapping_list() -> Vec<Mapping>
            = mapping() ** "\n"

        pub rule puzzle() -> Puzzle
            = "seeds: " seeds:number_list()
            "\n\nseed-to-soil map:\n"
            seeds_to_soil:mapping_list()
            "\n\nsoil-to-fertilizer map:\n"
            soil_to_fertilizer:mapping_list()
            "\n\nfertilizer-to-water map:\n"
            fertilizer_to_water:mapping_list()
            "\n\nwater-to-light map:\n"
            water_to_light:mapping_list()
            "\n\nlight-to-temperature map:\n"
            light_to_temperature:mapping_list()
            "\n\ntemperature-to-humidity map:\n"
            temperature_to_humidity:mapping_list()
            "\n\nhumidity-to-location map:\n"
            humidity_to_location:mapping_list()
            fileend() {
                Puzzle {
                    seeds,
                    seeds_to_soil,
                    soil_to_fertilizer,
                    fertilizer_to_water,
                    water_to_light,
                    light_to_temperature,
                    temperature_to_humidity,
                    humidity_to_location,
                }
            }

    }
}

#[derive(Debug)]
pub struct Mapping {
    src_range: std::ops::Range<u32>,
    dst_start: u32,
}

fn apply_mappings(mappings: &[Mapping], i: u32) -> u32 {
    for mapping in mappings {
        if mapping.src_range.contains(&i) {
            let offset = i - mapping.src_range.start;
            return offset + mapping.dst_start;
        }
    }
    i
}

#[derive(Debug)]
pub struct Puzzle {
    seeds: Vec<u32>,
    seeds_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

impl FromStr for Puzzle {
    type Err = peg::error::ParseError<LineCol>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        puzle_parser::puzzle(s)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = puzle_parser::puzzle(Input::from_argv()?.as_str())?;

    let part1 = input
        .seeds
        .iter()
        .map(|seed| {
            let soil = apply_mappings(&input.seeds_to_soil, *seed);
            let fertilizer = apply_mappings(&input.soil_to_fertilizer, soil);
            let water = apply_mappings(&input.fertilizer_to_water, fertilizer);
            let light = apply_mappings(&input.water_to_light, water);
            let temperature = apply_mappings(&input.light_to_temperature, light);
            let humidity = apply_mappings(&input.temperature_to_humidity, temperature);
            let location = apply_mappings(&input.humidity_to_location, humidity);
            location
        })
        .min()
        .unwrap();

    println!("{}", part1);

    /*
    let mut part2 = u32::MAX;
    for mut chunk in &input.seeds.iter().chunks(2) {
        let a = *chunk.next().unwrap();
        let b = *chunk.next().unwrap();
        let chunk_ans = (a..(a + b))
            .map(|seed| {
                let soil = apply_mappings(&input.seeds_to_soil, seed);
                let fertilizer = apply_mappings(&input.soil_to_fertilizer, soil);
                let water = apply_mappings(&input.fertilizer_to_water, fertilizer);
                let light = apply_mappings(&input.water_to_light, water);
                let temperature = apply_mappings(&input.light_to_temperature, light);
                let humidity = apply_mappings(&input.temperature_to_humidity, temperature);
                let location = apply_mappings(&input.humidity_to_location, humidity);
                location
            })
            .min()
            .unwrap();
        part2 = part2.min(chunk_ans);
    }
    println!("{}", part2);
    */

    Ok(())
}
