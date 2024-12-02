#![feature(iter_intersperse)]

use plot::{Plot, Position};

mod plot;

fn main() {
    part1();
    part2();
}

const TEST: &str = include_str!("../../../data/2023/day21/test.txt");
const PART_1: &str = include_str!("../../../data/2023/day21/p1.txt");
const PART_2: &str = include_str!("../../../data/2023/day21/p2.txt");

fn part1() {
    let plot = parse_input(PART_1);
    println!("Part 1: {}", plot.calc_history(plot.start()).get(64).size());
}

fn part2() {
    let plot = parse_input(PART_2);
    // let plot = parse_input(TEST);

    let steps_to_ul_plot = plot.start().manhattan_distance(Position::new(-1, -1));
    let steps_to_uc_plot = plot.start().manhattan_distance(Position::new(plot.start().x(), -1));
    let steps_to_ur_plot = plot.start().manhattan_distance(Position::new(plot.width() as isize, -1));

    let steps_to_cl_plot = plot.start().manhattan_distance(Position::new(-1, plot.start().y()));
    let steps_to_cr_plot = plot.start().manhattan_distance(Position::new(plot.width() as isize, plot.start().y()));

    let steps_to_dl_plot = plot.start().manhattan_distance(Position::new(-1, plot.height() as isize));
    let steps_to_dc_plot = plot.start().manhattan_distance(Position::new(plot.start().x(), plot.height() as isize));
    let steps_to_dr_plot = plot.start().manhattan_distance(Position::new(plot.width() as isize, plot.height() as isize));


    let pos_ul = Position::new(0, 0);
    let pos_uc = Position::new(plot.start().x(), 0);
    let pos_ur = Position::new(plot.width() as isize - 1, 0);

    let pos_cl = Position::new(0, plot.start().y());
    let pos_cr = Position::new(plot.width() as isize - 1, plot.start().y());

    let pos_dl = Position::new(0, plot.height() as isize - 1);
    let pos_dc = Position::new(plot.start().x(), plot.height() as isize - 1);
    let pos_dr = Position::new(plot.width() as isize - 1, plot.height() as isize - 1);


    let history_ul = plot.calc_history(pos_dr);
    let history_uc = plot.calc_history(pos_dc);
    let history_ur = plot.calc_history(pos_dl);

    let history_cl = plot.calc_history(pos_cr);
    let history_cr = plot.calc_history(pos_cl);

    let history_dl = plot.calc_history(pos_ur);
    let history_dc = plot.calc_history(pos_uc);
    let history_dr = plot.calc_history(pos_ul);

    const STEPS: usize = 26501365;

    assert_eq!(plot.width(), plot.height());

    let mut ul_positions = 0;
    for i in 0.. {
        let steps_to_get_to_plot = i * plot.width() + steps_to_ul_plot as usize;
        if steps_to_get_to_plot > STEPS {
            break;
        }
        let steps_remaining = STEPS - steps_to_get_to_plot;
        ul_positions += history_ul.get(steps_remaining).size() * (i + 1);
    }

    let mut uc_positions = 0;
    for i in 0.. {
        let steps_to_get_to_plot = i * plot.height() + steps_to_uc_plot as usize;
        if steps_to_get_to_plot > STEPS {
            break;
        }
        let steps_remaining = STEPS - steps_to_get_to_plot;
        uc_positions += history_uc.get(steps_remaining).size();
    }

    let mut ur_positions = 0;
    for i in 0.. {
        let steps_to_get_to_plot = i * plot.width() + steps_to_ur_plot as usize;
        if steps_to_get_to_plot > STEPS {
            break;
        }
        let steps_remaining = STEPS - steps_to_get_to_plot;
        ur_positions += history_ur.get(steps_remaining).size() * (i + 1);
    }

    let mut cl_positions = 0;
    for i in 0.. {
        let steps_to_get_to_plot = i * plot.width() + steps_to_cl_plot as usize;
        if steps_to_get_to_plot > STEPS {
            break;
        }
        let steps_remaining = STEPS - steps_to_get_to_plot;
        cl_positions += history_cl.get(steps_remaining).size();
    }

    let mut cr_positions = 0;
    for i in 0.. {
        let steps_to_get_to_plot = i * plot.width() + steps_to_cr_plot as usize;
        if steps_to_get_to_plot > STEPS {
            break;
        }
        let steps_remaining = STEPS - steps_to_get_to_plot;
        cr_positions += history_cr.get(steps_remaining).size();
    }

    let mut dl_positions = 0;
    for i in 0.. {
        let steps_to_get_to_plot = i * plot.width() + steps_to_dl_plot as usize;
        if steps_to_get_to_plot > STEPS {
            break;
        }
        let steps_remaining = STEPS - steps_to_get_to_plot;
        dl_positions += history_dl.get(steps_remaining).size() * (i + 1);
    }

    let mut dc_positions = 0;
    for i in 0.. {
        let steps_to_get_to_plot = i * plot.height() + steps_to_dc_plot as usize;
        if steps_to_get_to_plot > STEPS {
            break;
        }
        let steps_remaining = STEPS - steps_to_get_to_plot;
        dc_positions += history_dc.get(steps_remaining).size();
    }

    let mut dr_positions = 0;
    for i in 0.. {
        let steps_to_get_to_plot = i * plot.width() + steps_to_dr_plot as usize;
        if steps_to_get_to_plot > STEPS {
            break;
        }
        let steps_remaining = STEPS - steps_to_get_to_plot;
        dr_positions += history_dr.get(steps_remaining).size() * (i + 1);
    }

    let mm_positions = plot.calc_history(plot.start()).get(STEPS).size();

    let total_positions = 0
        + ul_positions + uc_positions + ur_positions
        + cl_positions + mm_positions + cr_positions
        + dl_positions + dc_positions + dr_positions;


    println!("Part 2: {}", total_positions);
}


fn parse_input(input: &'static str) -> Plot {
    input.parse().unwrap()
}
