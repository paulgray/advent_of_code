use std::collections::HashSet;

#[allow(dead_code)]
fn print_map(points : &HashSet<(i32, i32)>) {
    let mut min_x = i32::max_value();
    let mut max_x = i32::min_value();
    let mut min_y = i32::max_value();
    let mut max_y = i32::min_value();

    for (x, y) in points {
        if *x > max_x {
            max_x = *x;
        }
        if *x < min_x {
            min_x = *x;
        }
        if *y > max_y {
            max_y = *y;
        }
        if *y < min_y {
            min_y = *y;
        }
    }

    for x in min_x-1..max_x+1 {
        for y in min_y-1..max_y+1 {
            if points.contains(&(x, y)) {
                print!("X");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    println!();
}

fn find_closest_intersection(wire1 : &str, wire2 : &str) -> i32 {
    let w1 : Vec<&str> = wire1.split(",").collect();
    let w2 : Vec<&str> = wire2.split(",").collect();

    let mut points = HashSet::new();
    let mut matches = HashSet::new();
    
    // at first initialize the set with all wire 1 coords
    let mut x = 0;
    let mut y = 0;
    let mut x_mod;
    let mut y_mod;
    for p in w1 {
        let direction = p.get(0..1).unwrap();
        match direction {
            "R" => { x_mod = 1;  y_mod = 0 },
            "L" => { x_mod = -1; y_mod = 0 },
            "U" => { x_mod = 0;  y_mod = 1 },
            "D" => { x_mod = 0;  y_mod = -1}
            d   => panic!("Unknown direction {}", d),
        }

        let offset = p.get(1..).unwrap().parse::<i32>().unwrap();
        if x_mod == 1 {
            for i in x..(x+offset) {
                points.insert((i, y));
            }
        } else if x_mod == -1 {
            for i in (x-offset)..x+1 {
                points.insert((i, y));
            }
        }

        if y_mod == 1 {
            for i in y..(y+offset) {
                points.insert((x, i));
            }
        } else if y_mod == -1 {
            for i in (y-offset)..y+1 {
                points.insert((x, i));
            }
        }

        x = x+x_mod*offset;
        y = y+y_mod*offset;

        //print_map(&points);
    }

    //print_map(&points);

    // now we have all wire 1 coords in the set
    // let's start adding wire 2 points
    x = 0;
    y = 0;
    for p in w2 {
        let direction = p.get(0..1).unwrap();
        match direction {
            "R" => { x_mod = 1;  y_mod = 0 },
            "L" => { x_mod = -1; y_mod = 0 },
            "U" => { x_mod = 0;  y_mod = 1 },
            "D" => { x_mod = 0;  y_mod = -1}
            d   => panic!("Unknown direction: {}", d),
        }

        let offset = p.get(1..).unwrap().parse::<i32>().unwrap();
        if x_mod == 1 {
            for i in x..(x+offset) {
                if points.contains(&(i, y)) {
                    matches.insert((i, y));
                }
            }
        } else if x_mod == -1 {
            for i in (x-offset)..x+1 {
                if points.contains(&(i, y)) {
                    matches.insert((i, y));
                }
            }
        }

        if y_mod == 1 {
            for i in y..(y+offset) {
                if points.contains(&(x, i)) {
                    matches.insert((x, i));
                }
            }
        } else if y_mod == -1 {
            for i in (y-offset)..y+1 {
                if points.contains(&(x, i)) {
                    matches.insert((x, i));
                }
            }
        }

        x = x+x_mod*offset;
        y = y+y_mod*offset;
    }

    println!("Number of crosses {}", matches.len());

    let mut distances = matches.iter()
                               .map(|(px, py)| px.abs() + py.abs())
                               .collect::<Vec<i32>>();
    distances.sort();
    
    return distances[1];
}

fn main() {
    let wire1 = "R1000,D722,L887,D371,R430,D952,R168,D541,L972,D594,R377,U890,R544,U505,L629,U839,L680,D863,L315,D10,L482,U874,L291,U100,R770,D717,L749,U776,L869,D155,R250,U672,L195,D991,L556,D925,R358,U296,R647,D652,L790,D780,L865,U405,L400,D160,L460,U50,R515,D666,R306,U746,R754,U854,L332,U254,R673,U795,R560,U69,L507,U332,L328,U547,L717,U291,R626,U868,L583,D256,R371,U462,R793,U559,L571,U270,R738,U425,L231,U549,L465,U21,L647,U43,R847,U104,L699,U378,L549,D975,R13,D306,R532,D730,L566,U846,L903,D224,R448,D424,L727,D199,L626,D872,L541,D786,L304,U462,R347,U379,R29,D556,L775,D768,L284,D480,R654,D659,R818,D57,L77,U140,R619,D148,R686,D461,L910,U244,R115,D769,R968,U802,L737,U868,R399,D150,L791,U579,L856,D11,R115,U522,L443,D575,L133,U750,R437,U718,L79,D119,L97,U471,R817,U438,R157,U105,L219,U777,L965,U687,L906,D744,L983,D350,R664,D917,R431,D721,L153,U757,L665,U526,L49,U166,L59,D293,R962,D764,R538,U519,L24,U91,R11,U574,L647,U891,R44,D897,L715,U498,L624,D573,R287,U762,L613,D79,R122,U148,L849,D385,R792,D20,L512,D431,R818,U428,L10,D800,R773,D936,L594,D38,R824,D216,L220,U358,L463,U550,R968,D346,L658,U113,R813,U411,L730,D84,R479,U877,L730,D961,L839,D792,R424,U321,L105,D862,L815,D243,L521,D913,L1,D513,L269,U495,L27,U16,R904,D926,R640,U948,R346,D240,L273,U131,L296,U556,R347,D640,L261,D43,R136,U824,R126,U583,R736,U530,L734,U717,L256,U362,L86,U48,R851,U519,L610,D134,L554,D766,L179,U637,R71,D895,L21,D908,R486,D863,R31,U85,R420,D718,R466,D861,R655,D304,L701,D403,L860,D208,L595,U64,R999";
    let wire2 = "L992,D463,R10,D791,R312,D146,R865,D244,L364,D189,R35,U328,R857,D683,L660,D707,L908,D277,R356,U369,R197,D35,R625,D862,L769,U705,L728,U999,R938,U233,L595,U266,L697,U966,L536,D543,L669,D829,R910,U693,R753,D389,L647,U603,L660,D787,L138,D119,L131,D266,R268,D917,R776,U290,R920,U904,L46,D139,L341,D19,R982,U790,L981,U791,L147,U30,L246,U677,R343,D492,R398,D234,R76,D423,L709,D392,R741,U408,R878,U29,R446,U36,R806,U78,L76,D813,R584,U682,L187,U666,L340,D301,L694,U15,R800,U276,L755,U558,R366,D309,R571,U976,L286,D833,R318,U365,L864,U408,L352,D61,R284,D272,R240,D845,L206,U721,R367,D541,R628,U581,L750,D680,R695,D30,R849,U743,L214,U605,R533,U493,R803,D783,R168,U877,L61,D726,L794,D116,R717,U44,R964,U674,L291,D372,L381,D523,L644,U438,R983,D390,R520,D471,R556,D693,L919,D863,R84,D629,L264,D429,R82,U64,R835,D801,R93,U770,R441,D152,L718,D788,L797,U699,L82,U206,L40,U952,R902,U570,L759,D655,L131,D901,L470,D804,L407,U458,L922,D21,L171,U841,L237,D301,R192,D293,R984,U243,R846,U139,L413,U162,R925,D235,L115,U443,L884,D910,R335,U274,L338,U160,R125,D775,R824,D821,R531,D761,L189,U822,L602,D732,R473,U149,L128,U30,R77,D957,R811,D154,L988,D237,R425,D855,R482,D571,R134,D731,L905,U869,R916,D689,L17,U24,R353,D996,R832,U855,L76,U659,R581,D483,R821,D145,R199,D344,R487,D436,L92,U491,R365,D909,L17,D148,R307,U57,R666,U660,R195,D767,R612,D902,L594,D299,R670,D881,L583,D793,R58,U89,L99,D355,R394,D350,R920,U544,R887,U564,L238,U979,L565,D914,L95,U150,R292,U495,R506,U475,R813,D308,L797,D484,R9";

    let distance = find_closest_intersection(wire1, wire2);
    println!("Distance {}", distance);
}

#[test]
fn test() {
    assert_eq!(6, find_closest_intersection("R8,U5,L5,D3", "U7,R6,D4,L4"));
    assert_eq!(159, find_closest_intersection("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"));
    assert_eq!(135, find_closest_intersection("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"));
}