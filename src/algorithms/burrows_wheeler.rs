#[allow(non_snake_case)]
pub mod BurrowsWheeler {

    pub fn encode(input: &[u8]) -> (usize, Vec<u8>) {
        let lenght = input.len();
        let mut table = vec![Vec::<u8>::with_capacity(lenght); lenght];

        let mut current_row: Vec<u8> = input.iter().map(|&c| c).collect();
        for i in 0..lenght {
            // adding current_row to table
            table[i] = current_row.clone();

            // updating current_row
            let first_value = current_row[0];
            for i in 0..lenght - 1 {
                current_row[i] = current_row[i + 1];
            }

            // updating last value
            let last_index = lenght - 1;
            current_row[last_index] = first_value;
        }

        // sort lexicographically per default
        table.sort();

        let index = table.iter().position(|row| row == input).unwrap();

        let transformed: Vec<u8> = table.iter().map(|row| row[lenght - 1]).collect();

        (index, transformed)
    }

    // TODO move to front algo

    use std::sync::{Arc, Mutex};

    use rayon::prelude::*;

    /// same functionality but works in parallel
    pub fn encode_par(input: &[u8]) -> (usize, Vec<u8>) {
        let lenght = input.len();

        // per-row mutexes to reduce contention
        let table: Vec<Arc<Mutex<Vec<u8>>>> = (0..lenght)
            .map(|_| Arc::new(Mutex::new(input.to_vec())))
            .collect();
        let table = Arc::new(table);

        // NOTE: could possibly use num_cpus::get_physical();
        // let optimal_threads = num_cpus::get_physical();
        let optimal_threads = num_cpus::get();
        rayon::ThreadPoolBuilder::new()
            .num_threads(optimal_threads) // Set optimal for your CPU
            .build_global()
            .unwrap();

        // updating rows in parallel
        (0..lenght).into_par_iter().for_each(|i| {
            let mut row = table[i].lock().unwrap();
            row.rotate_left(i);
        });

        let mut table: Vec<Vec<u8>> = table
            .iter()
            .map(|row| row.lock().unwrap().clone())
            .collect();

        table.sort();

        let index = table.iter().position(|row| row == input).unwrap();

        let transformed: Vec<u8> = table.iter().map(|row| row[lenght - 1]).collect();

        (index, transformed)
    }

    // TODO make parallel too?
    pub fn decode(index: usize, input: &[u8]) -> Vec<u8> {
        let lenght = input.len();

        // initiating table
        let mut table: Vec<Vec<u8>> = input
            .iter()
            .map(|&c| {
                let mut row = vec![0; lenght];
                row[lenght - 1] = c;
                row
            })
            .collect();

        // rebuilding original table
        for j in (0..lenght - 1).rev() {
            table.sort();

            for i in 0..lenght {
                table[i][j] = input[i];
            }
        }

        table.sort();

        table[index].clone()
    }

    /// DEBUG function
    #[allow(dead_code)]
    fn print_table(table: &Vec<Vec<u8>>) {
        for row in table {
            print!("[");
            for c in row {
                print!("'{}',", *c as char);
            }
            println!("]");
        }
    }
}

#[cfg(test)]
mod tests {

    use std::time::Instant;

    use super::*;

    #[test]
    fn burrows_wheeler_encode() {
        let text: Vec<u8> = "BANANA".bytes().collect();
        let (index, transformed) = BurrowsWheeler::encode(&text);

        let result: Vec<u8> = "NNBAAA".bytes().collect();
        assert_eq!((3, result), (index, transformed));
    }

    #[test]
    fn burrows_wheeler_encode_sync() {
        let text: Vec<u8> = "BANANA".bytes().collect();
        let (index, transformed) = BurrowsWheeler::encode_par(&text);

        let result: Vec<u8> = "NNBAAA".bytes().collect();
        assert_eq!((3, result), (index, transformed));
    }

    #[test]
    fn bwt_encode_benchmark() {
        // comment to test
        return;

        #[allow(unreachable_code)]
        let mut text: Vec<u8> = "BANANA".bytes().collect();

        // DEBUG
        // let mut bloat: Vec<u8> = "BANANA".bytes().collect();
        // for _ in 0..1000 {
        //     text.extend_from_slice(&bloat);
        // }

        let start = Instant::now();
        let _ = BurrowsWheeler::encode(&text);
        let duration_seq = start.elapsed();
        println!("Time elapsed for sequential encoding: {:?}", duration_seq);

        let start = Instant::now();
        let _ = BurrowsWheeler::encode_par(&text);
        let duration_seq = start.elapsed();
        println!("Time elapsed for parallel: {:?}", duration_seq);
    }

    #[test]
    fn burrows_wheeler_decode() {
        let text: Vec<u8> = "NNBAAA".bytes().collect();
        let transformed = BurrowsWheeler::decode(3, &text);

        let result: Vec<u8> = "BANANA".bytes().collect();
        assert_eq!(result, transformed);
    }
}
