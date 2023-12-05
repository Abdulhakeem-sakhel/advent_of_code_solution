use std::fs;

static _SEEDS: &str = "seeds:";
static _SEED_TO_SOIL: &str = "seed-to-soil map:";
static _SOIL_TO_FERTILIZER: &str = "soil-to-fertilizer map:";
static _FERTILIZER_TO_WATER: &str = "fertilizer-to-water map:";
static _WATER_TO_LIGHT: &str = "water-to-light map:";
static _LIGHT_TO_TEMPERATURE: &str = "light-to-temperature map:";
static _TEMPERATURE_TO_HUMIDITY: &str = "temperature-to-humidity map:";
static _HUMIDITY_TO_LOCATION: &str = "humidity-to-location map:";


fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("cant't find the file");
    println!("the answer = {}",get_min_location_from_seeds(contents.as_str()));
}

#[derive(Default, Debug)]
struct RangeMap {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize
}

impl RangeMap {
    fn get(&self, value: usize) -> Option<usize> {
        if value < self.source_range_start + self.range_length && value >= self.source_range_start {
            return Some(self.destination_range_start + (value - self.source_range_start))
        } else {
            None
        }
    }
}
// #[inline]
// fn build_map_for_almanac(map: &mut HashMap<usize, usize>, range_map: RangeMap) {
//     for i in 0..range_map.range_length {
//         map.insert(range_map.source_range_start + i, range_map.destination_range_start + i);
//     }

// }
fn get_seeds(soil_almanac: &str, slice_start:&str, slice_end: &str) -> Vec<usize> {
    let n = soil_almanac.find(slice_start).unwrap();
    let n2 = soil_almanac.find(slice_end).unwrap();
    //println!("{:?}", soil_almanac.get(n..n2).unwrap().trim_end_matches("\n\n").split(" ").collect::<Vec<&str>>());
    soil_almanac.get(n..n2).unwrap()
        .trim_end_matches("\n\n")
        .split(" ")
        .filter_map(|num| num.parse::<usize>().ok())
        .collect::<Vec<usize>>()
}

fn get_dest_range_map_vec(range_maps_vector: &Vec<RangeMap>, value: usize) -> usize {
    for range_map in range_maps_vector.iter() {
        match range_map.get(value) {
            Some(t) => return t,
            None => continue
        }
    }

    return value
}

fn get_range_map(soil_almanac: &str, slice_start:&str, slice_end: &str) -> Vec<RangeMap> {
    // let mut map: HashMap<usize, usize> = HashMap::new();
    
    let n = soil_almanac.find(slice_start).unwrap();
    let n2: usize;
    if slice_end.is_empty() {
        n2 = soil_almanac.len();
    } else {
        n2 = soil_almanac.find(slice_end).unwrap();
    }

    let ranges_number:Vec<&str> = soil_almanac.get(n..n2).unwrap()
        .lines()
        .filter(|a| !a.is_empty())
        .collect();

    let mut range_map_vec: Vec<RangeMap> = Vec::new();

    for i in 1..ranges_number.len() {
        let mut range_map = RangeMap::default();
        let number:Vec<usize> = ranges_number[i].split(" ")
            .map(|a| a.parse::<usize>().unwrap())
            .collect();
        assert_eq!(3, number.len());
        range_map.destination_range_start = number[0];
        range_map.source_range_start = number[1];
        range_map.range_length = number[2];
        range_map_vec.push(range_map);
    }

    return range_map_vec;
}

fn get_min_location_from_seeds(soil_almanac: &str ) -> usize{
    let seeds: Vec<usize> = get_seeds(soil_almanac, _SEEDS, _SEED_TO_SOIL);
    //println!("{:?}", seeds);
    let seed_to_soil_map:Vec<RangeMap>  = get_range_map(soil_almanac, _SEED_TO_SOIL, _SOIL_TO_FERTILIZER);
    let soil_to_fertilizer_map:Vec<RangeMap>  = get_range_map(soil_almanac, _SOIL_TO_FERTILIZER, _FERTILIZER_TO_WATER);
    let fertilizer_to_water_map:Vec<RangeMap>  = get_range_map(soil_almanac, _FERTILIZER_TO_WATER, _WATER_TO_LIGHT);
    let water_to_light_map:Vec<RangeMap>  = get_range_map(soil_almanac, _WATER_TO_LIGHT, _LIGHT_TO_TEMPERATURE);
    let light_to_temperature_map: Vec<RangeMap>  = get_range_map(soil_almanac, _LIGHT_TO_TEMPERATURE, _TEMPERATURE_TO_HUMIDITY);
    let temperature_to_humidity_map: Vec<RangeMap> = get_range_map(soil_almanac, _TEMPERATURE_TO_HUMIDITY, _HUMIDITY_TO_LOCATION);
    let humidity_to_location_map: Vec<RangeMap> = get_range_map(soil_almanac, _HUMIDITY_TO_LOCATION, "");
 
    return  seeds
        .iter()
        .map(|seed: &usize| -> usize {
            let soil: usize = get_dest_range_map_vec(&seed_to_soil_map, *seed);
            let fertilizer: usize = get_dest_range_map_vec(&soil_to_fertilizer_map, soil);
            let water: usize = get_dest_range_map_vec(&fertilizer_to_water_map, fertilizer);
            let light: usize = get_dest_range_map_vec(&water_to_light_map, water);
            let temperature: usize = get_dest_range_map_vec(&light_to_temperature_map, light);
            let humidity: usize = get_dest_range_map_vec(&temperature_to_humidity_map, temperature);
            let location: usize = get_dest_range_map_vec(&humidity_to_location_map, humidity);
            return location;
        })
        .min()
        .unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_soil() {
        let soil_almanac: &str = 
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
       assert_eq!(35, get_min_location_from_seeds(soil_almanac));
    }
}