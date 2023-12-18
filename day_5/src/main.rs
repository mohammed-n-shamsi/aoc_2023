use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::{
    collections::HashMap,
    fs::{self},
    sync::{Arc, Mutex},
    thread,
};

#[derive(Debug, Clone)]
struct Range {
    dest_range: std::ops::Range<u64>,
    source_range: std::ops::Range<u64>,
}

#[derive(Debug, Clone)]
struct SeedSoilRanges {
    ranges: Vec<Range>,
}

#[derive(Debug, Clone)]
struct SoilFertilizerRanges {
    ranges: Vec<Range>,
}

#[derive(Debug, Clone)]
struct FertilizerWaterRanges {
    ranges: Vec<Range>,
}

#[derive(Debug, Clone)]
struct WaterLightRanges {
    ranges: Vec<Range>,
}

#[derive(Debug, Clone)]
struct LightTemperatureRanges {
    ranges: Vec<Range>,
}

#[derive(Debug, Clone)]
struct TemperatureHumidityRanges {
    ranges: Vec<Range>,
}

#[derive(Debug, Clone)]
struct HumidityLocationRanges {
    ranges: Vec<Range>,
}

#[derive(Parser, Debug, Clone)]
#[grammar = "input.pest"]
struct MapParser {
    seeds: Vec<u64>,
    seed_soil_map: SeedSoilRanges,
    soil_fertilizer_map: SoilFertilizerRanges,
    fertilizer_water_map: FertilizerWaterRanges,
    water_light_map: WaterLightRanges,
    light_temperature_map: LightTemperatureRanges,
    temperature_humidity_map: TemperatureHumidityRanges,
    humidity_location_map: HumidityLocationRanges,
}

fn produce_range(input_rule: Pair<'_, Rule>) -> Range {
    let mut dst_range_start = 0;
    let mut src_range_start = 0;
    let mut range_len = 0;

    for ranges in input_rule.into_inner() {
        let val = ranges.as_str().parse::<u64>().unwrap();
        match ranges.as_rule() {
            Rule::dst_range_start => dst_range_start = val,
            Rule::src_range_start => src_range_start = val,
            Rule::range_len => range_len = val,
            _ => {
                println!("Bad times")
            }
        }
    }

    Range {
        dest_range: dst_range_start..dst_range_start + range_len,
        source_range: src_range_start..src_range_start + range_len,
    }
}

fn push_ranges(input_rule: Pair<'_, Rule>, ranges: &mut Vec<Range>) {
    for token in input_rule.into_inner() {
        if token.as_rule() == Rule::range {
            ranges.push(produce_range(token));
        }
    }
}

fn parse_input(input: String) -> MapParser {
    let parse = MapParser::parse(Rule::grammar, &input).expect("Bad parse");

    let mut map = MapParser {
        seeds: Vec::new(),
        seed_soil_map: SeedSoilRanges { ranges: Vec::new() },
        soil_fertilizer_map: SoilFertilizerRanges { ranges: Vec::new() },
        fertilizer_water_map: FertilizerWaterRanges { ranges: Vec::new() },
        water_light_map: WaterLightRanges { ranges: Vec::new() },
        light_temperature_map: LightTemperatureRanges { ranges: Vec::new() },
        temperature_humidity_map: TemperatureHumidityRanges { ranges: Vec::new() },
        humidity_location_map: HumidityLocationRanges { ranges: Vec::new() },
    };

    for item in parse {
        match item.as_rule() {
            Rule::seed => map.seeds.push(item.as_str().parse::<u64>().unwrap()),
            Rule::seed_soil_map => {
                push_ranges(item, &mut map.seed_soil_map.ranges);
            }
            Rule::soil_fert_map => {
                push_ranges(item, &mut map.soil_fertilizer_map.ranges);
            }
            Rule::fert_water_map => {
                push_ranges(item, &mut map.fertilizer_water_map.ranges);
            }
            Rule::water_light_map => {
                push_ranges(item, &mut map.water_light_map.ranges);
            }
            Rule::light_temp_map => {
                push_ranges(item, &mut map.light_temperature_map.ranges);
            }
            Rule::temp_humidity_map => {
                push_ranges(item, &mut map.temperature_humidity_map.ranges);
            }
            Rule::humidity_location_map => {
                push_ranges(item, &mut map.humidity_location_map.ranges);
            }
            _ => {}
        }
    }

    map
}

fn find_dest_map(range_map: &Vec<Range>, source_val: u64) -> u64 {
    let mut dest_val = source_val;
    for range in range_map {
        if range.source_range.contains(&source_val) {
            let total_dist = source_val - range.source_range.start;
            dest_val = range.dest_range.start + total_dist;
            break;
        }
    }
    dest_val
}

fn main() {
    let input_string = fs::read_to_string("input.txt").unwrap();
    let map = parse_input(input_string);

    let mut seed_ranges = Vec::new();

    for seed_idx in 0..map.seeds.len() - 2 {
        if seed_idx % 2 == 0 {
            seed_ranges.push(map.seeds[seed_idx]..map.seeds[seed_idx] + map.seeds[seed_idx + 1])
        }
    }
    println!("seed ranges: {:?}", seed_ranges);
    let smallest_seed = Arc::new(Mutex::new(u64::max_value()));
    let smallest_value = Arc::new(Mutex::new(u64::max_value()));

    let smallest_seed_cl = Arc::clone(&smallest_seed);
    let smallest_value_cl = Arc::clone(&smallest_value);

    let mut td_handles = Vec::new();
    for seed_range in seed_ranges {
        let thread_map = map.clone();
        let seed_cl = Arc::clone(&smallest_seed);
        let value_cl = Arc::clone(&smallest_value);

        let handle = thread::spawn(move || {
            println!(
                "Starting with range {:?} on: {:?}",
                seed_range,
                thread::current().id()
            );

            let iter_range = seed_range.end - seed_range.start;
            let mut heartbeat = 0;
            for seed in seed_range {
                heartbeat += 1;
                if heartbeat == iter_range / 20 {
                    heartbeat = 0;
                    println!("Heartbeat on {:?}", thread::current().id());
                }

                let mut running_value = seed;

                running_value = find_dest_map(&thread_map.seed_soil_map.ranges, running_value);
                running_value =
                    find_dest_map(&thread_map.soil_fertilizer_map.ranges, running_value);
                running_value =
                    find_dest_map(&thread_map.fertilizer_water_map.ranges, running_value);
                running_value = find_dest_map(&thread_map.water_light_map.ranges, running_value);
                running_value =
                    find_dest_map(&thread_map.light_temperature_map.ranges, running_value);
                running_value =
                    find_dest_map(&thread_map.temperature_humidity_map.ranges, running_value);
                running_value =
                    find_dest_map(&thread_map.humidity_location_map.ranges, running_value);

                let mut seed_global = seed_cl.lock().unwrap();
                let mut value_global = value_cl.lock().unwrap();

                if running_value < *value_global {
                    *seed_global = seed;
                    *value_global = running_value;
                }
            }
            println!("Done with range on: {:?}", thread::current().id());
        });

        td_handles.push(handle);
    }

    for handle in td_handles {
        handle.join().unwrap();
    }

    let final_seed = smallest_seed_cl.lock().unwrap();
    let final_value = smallest_value_cl.lock().unwrap();

    println!("{final_seed} :: {final_value}");
}
