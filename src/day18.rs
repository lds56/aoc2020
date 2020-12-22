
// use super::util;
// use std::error;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Vec<String> {// Result<Vec<String>, Box<dyn error::Error>> {

    input
        .lines()
        .map(|line| {
            line.replace(' ', "").to_string()
        })
        .collect()
}

enum CalcItem {
    Op(char),
    Num(i64),
}

fn calc(expr: &[CalcItem]) -> i64 {

    expr
        .iter()
        .fold((-1i64, &CalcItem::Op('?')), |(val, op), ch| {

            match ch {
                CalcItem::Op('+') | CalcItem::Op('*') => (val, ch),
                CalcItem::Num(num) =>
                    (if val < 0 { *num } else {
                        match op {
                            CalcItem::Op('+') => val + num,
                            CalcItem::Op('*') => val * num,
                            _ => val,
                        }
                    }, ch),
                _ => (val, ch),
            }

        }).0

}

fn calc2(expr: &[CalcItem]) -> i64 {

    expr
        .iter()
        .fold((Vec::<i64>::new(), false), |(mut v, f), ch| {
            match ch {
                CalcItem::Op('+') => (v, true),
                CalcItem::Op('*') => (v, f),
                CalcItem::Num(n)  => if f {
                    if let Some(last) = v.last_mut() {
                        *last += *n;
                    }
                    (v, false)
                } else { v.push(*n); (v, false) }
                _ => (v, f),
            }
        })
        .0
        .iter()
        .product()
}

#[aoc(day18, part1)]
fn solvor_part1(lines: &[String]) -> i64 {

    lines
        .iter()
        .map(|line| {

            let mut calc_stack: Vec<CalcItem> = vec![];

            for ch in line.chars() {

                if ch != ')' {

                    if let Some(num) = ch.to_digit(10) {
                        calc_stack.push(CalcItem::Num(num as i64));
                    } else {
                        calc_stack.push(CalcItem::Op(ch));
                    }

                } else {

                    for i in (0..calc_stack.len()).rev() {
                        if let CalcItem::Op('(') = calc_stack[i] {
                            let tmp = calc(&calc_stack[i..]);
                            calc_stack.truncate(i);
                            calc_stack.push(CalcItem::Num(tmp));
                            break;
                        }
                    }
                }
            }

            calc(&calc_stack)

        })
        .sum()

}


#[aoc(day18, part2)]
fn solvor_part2(lines: &[String]) -> i64 {

    lines
        .iter()
        .map(|line| {

            let mut calc_stack: Vec<CalcItem> = vec![];

            for ch in line.chars() {

                if ch != ')' {

                    if let Some(num) = ch.to_digit(10) {
                        calc_stack.push(CalcItem::Num(num as i64));
                    } else {
                        calc_stack.push(CalcItem::Op(ch));
                    }

                } else {

                    for i in (0..calc_stack.len()).rev() {
                        if let CalcItem::Op('(') = calc_stack[i] {
                            let tmp = calc2(&calc_stack[i..]);
                            calc_stack.truncate(i);
                            calc_stack.push(CalcItem::Num(tmp));
                            break;
                        }
                    }
                }
            }

            calc2(&calc_stack)

        })
        .sum()

}
