use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client(mut stream: TcpStream, terms_per_client: usize, results: Arc<Mutex<Vec<f64>>>) {
    let mut buffer = vec![0u8; terms_per_client * 8];
    stream.read_exact(&mut buffer).unwrap();

    let client_result: Vec<f64> = buffer
        .chunks_exact(8)
        .map(|bytes| f64::from_be_bytes(bytes.try_into().unwrap()))
        .collect();

    let mut data = results.lock().unwrap();
    data.extend(client_result);
}

fn main() {
    let num_clients: usize = 3;
    let terms_per_client: usize = 100000;

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let results = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];

    for _ in 0..num_clients {
        let results = Arc::clone(&results);
        if let Ok((stream, _)) = listener.accept() {
            let handle = thread::spawn(move || {
                handle_client(stream, terms_per_client, results);
            });
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_results = results.lock().unwrap();
    let pi_estimate: f64 = 4.0 * final_results.iter().sum::<f64>();
    println!("Estimated Pi: {}", pi_estimate);
}
