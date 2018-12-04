extern crate regex;
extern crate chrono;
use regex::Regex;
use std::collections::HashMap;
use chrono::{NaiveDateTime, Timelike};

fn main() {
    let input = include_str!("input_sorted");
    // let input = include_str!("test");

    // [1518-05-08 00:02] Guard #2719 begins shift
    let guard_reg = Regex::new(r"\[(.*)\] Guard #(\d+) begins shift").unwrap();
    // [1518-04-12 00:57] wakes up
    let wake_reg = Regex::new(r"\[(.*)\] wakes up").unwrap();
    // [1518-11-12 00:30] falls asleep
    let sleep_reg = Regex::new(r"\[(.*)\] falls asleep").unwrap();

    let mut guard_info: HashMap<u32, HashMap<u64, u64>> = HashMap::new();

    let mut current_guard = 0;
    let mut current_sleep: Option<NaiveDateTime> = None;
    for line in input.lines() {
        if let Some(c) = guard_reg.captures(line) {
            current_guard = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
            if !guard_info.contains_key(&current_guard) {
                guard_info.insert(current_guard, HashMap::new());
            }
        } else if let Some(c) = wake_reg.captures(line) {
            let wakes = NaiveDateTime::parse_from_str(c.get(1).unwrap().as_str(), "%Y-%m-%d %H:%M").ok().unwrap();
            let sleep = current_sleep.unwrap();
            let mins = wakes.signed_duration_since(sleep).num_minutes() as u64;
            let sleep_secs = sleep.num_seconds_from_midnight() as u64;

            let mut guard_mins = guard_info.get_mut(&current_guard).unwrap();
            for min in 0..mins {
                let curr_min = ((sleep_secs + (min * 60)) / 60) % (24*60);
                let num_slept = guard_mins.get(&curr_min).unwrap_or(&0u64) + 1;
                guard_mins.insert(curr_min, num_slept);
            }

            current_sleep = None;
        } else if let Some(c) = sleep_reg.captures(line) { 
            current_sleep = NaiveDateTime::parse_from_str(c.get(1).unwrap().as_str(), "%Y-%m-%d %H:%M").ok();
        } else {
            panic!("Unmatched line {}", line);
        }
    }

    // part1, find guard who sleeps most
    let most_sleepy = guard_info.iter()
        .map(|g_info| {
            let (guard, mins) = g_info;
            let mut total = 0;
            for min in mins {
                total += min.1;
            }
            (guard, total)
        })
        .max_by_key(|x| x.1).unwrap();
    let most_sleepy_minute = guard_info.get(most_sleepy.0).unwrap().iter()
        .max_by_key(|x| x.1).unwrap().0;

    println!("Most sleepy guard: {}, slept {}", most_sleepy.0, most_sleepy.1);
    println!("Most sleepy minute for that guard {}", most_sleepy_minute);
    println!("Part1: {}", *most_sleepy.0 as u64 * most_sleepy_minute);
    

    // part2, find most frequent minute sleepy guard
    let mut freq_guard = 0;
    let mut freq_min = 0;
    let mut freq_min_times = 0;
    for (guard, mins) in guard_info.iter() {
        if mins.len() > 0 {
            let maxmin = mins.iter().max_by_key(|x| x.1).unwrap();
            if freq_min_times < *maxmin.1 {
                freq_min_times = *maxmin.1;
                freq_min = *maxmin.0;
                freq_guard = *guard;
            }
        }
    }
    println!("Guard with most frequent: {}", freq_guard);
    println!("Most frequent min: {}", freq_min);
    println!("Part2: {}", freq_guard as u64 * freq_min);
}
