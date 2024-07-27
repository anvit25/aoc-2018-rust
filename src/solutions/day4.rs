use chrono::{NaiveDateTime, Timelike};
use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
enum Action {
    BeginShift(u32),
    FallAsleep,
    WakeUp,
}
#[derive(Debug)]
struct Record {
    time: NaiveDateTime,
    action: Action,
}

impl Record {
    fn from_string(input: &str) -> Record {
        let time = NaiveDateTime::parse_from_str(&input[..18], "[%Y-%m-%d %H:%M]").unwrap();
        let action = match &input[19..24] {
            "Guard" => {
                let id = input[26..].split_whitespace().next().unwrap().parse::<u32>().unwrap();
                Action::BeginShift(id)
            },
            "falls" => Action::FallAsleep,
            "wakes" => Action::WakeUp,
            _ => panic!("Unknown action")
        };
        Record { time, action }
    }
}

fn read_input() -> Vec<Record> {
    let input = fs::read_to_string("inputs/4.txt").unwrap();
    let mut records: Vec<Record> = input.lines().map(Record::from_string).collect();
    records.sort_by(|a, b| a.time.cmp(&b.time));
    records
}

fn make_sleep_schedule(records: &Vec<Record>) -> HashMap<u32, [u32; 60]> {
    let mut sleep_schedule: HashMap<u32, [u32; 60]> = HashMap::new();
    let mut current_guard = 0;
    let mut sleep_start = 0;

    for record in records {
        match record.action {
            Action::BeginShift(id) => current_guard = id,
            Action::FallAsleep => {
                sleep_start = record.time.minute();
            },
            Action::WakeUp => {
                let sleep = sleep_schedule.entry(current_guard).or_insert([0; 60]);
                for i in sleep_start..record.time.minute() {
                    sleep[i as usize] += 1;
                }
            }
        }
    }
    sleep_schedule
}

pub fn day4a() -> u32 {
    let records = read_input();
    let sleep_schedule = make_sleep_schedule(&records);
    let sleepiest_guard = sleep_schedule.iter()
                                    .max_by_key(|x| x.1.iter().sum::<u32>())
                                    .unwrap().0;
    let sleepiest_minute = sleep_schedule.get(sleepiest_guard).unwrap()
                                        .iter().enumerate().max_by_key(|x| x.1)
                                        .unwrap().0 as u32;
    sleepiest_guard * sleepiest_minute
}

pub fn day4b() -> u32 {
    let records = read_input();
    let sleep_schedule = make_sleep_schedule(&records);
    let (guard, minute, _) = sleep_schedule.iter()
                            .map(|(guard, schedule)| {
                                let (minute, &count) = schedule.iter().enumerate().max_by_key(|x| x.1).unwrap();
                                (guard, minute as u32, count)
                            })
                            .max_by_key(|x| x.2).unwrap();
    guard * minute
}

