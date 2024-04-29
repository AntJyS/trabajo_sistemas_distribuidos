use std::env;
use std::io::Write;
use std::net::TcpStream;

fn compute_segment(start: usize, num_terms: usize) -> Vec<f64> {
    (start..start + num_terms)
        .map(|k| {
            let term = (-1.0f64).powi(k as i32) / (2.0 * k as f64 + 1.0);
            term
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let start_term: usize = args[1].parse().unwrap();
    let terms_per_client: usize = args[2].parse().unwrap();

    let segment = compute_segment(start_term, terms_per_client);
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    for &term in &segment {
        let buffer = (4.0 * term).to_be_bytes();
        stream.write_all(&buffer).unwrap();
    }
}

