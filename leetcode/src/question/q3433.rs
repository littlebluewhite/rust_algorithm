use std::cmp::Ordering;

enum Payload {
    Offline(usize),
    Message(String),
}

struct Event {
    time: i32,
    payload: Payload,
}

pub fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
    let n = number_of_users as usize;
    let mut e: Vec<Event> = events
        .into_iter()
        .map(|event| {
            let time = event[1].parse().unwrap();
            let payload = match event[0].as_str() {
                "OFFLINE" => Payload::Offline(event[2].parse().unwrap()),
                _ => Payload::Message(event[2].parse().unwrap()),
            };
            Event { time, payload }
        })
        .collect();
    e.sort_by(|a, b| {
        let t_cmp = a.time.cmp(&b.time);
        if t_cmp != Ordering::Equal {
            return t_cmp;
        }
        let pa = matches!(a.payload, Payload::Message(_));
        let pb = matches!(b.payload, Payload::Message(_));
        pa.cmp(&pb)
    });
    let mut online = vec![0i32; n];
    let mut mentions = vec![0i32; n];
    for event in e {
        match event.payload {
            Payload::Offline(id) => online[id] = event.time + 60,
            Payload::Message(m) => {
                for token in m.split_whitespace() {
                    match token {
                        "ALL" => {
                            for i in 0..n {
                                mentions[i] += 1;
                            }
                        }
                        "HERE" => {
                            for i in 0..n {
                                if online[i] <= event.time {
                                    mentions[i] += 1;
                                }
                            }
                        }
                        _ if token.starts_with("id") => {
                            if let Ok(id) = token[2..].parse::<usize>() {
                                mentions[id] += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    mentions
}
