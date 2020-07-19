use std::env;
use std::fs;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let modulo: &usize = &args[2].parse().unwrap();
    let iterations: usize = args[3].parse().unwrap();

    for n in 1..=iterations {
        let mut count = 1;

        let mut files: Vec<_> = fs::read_dir(&path)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        files.sort_by_key(|dir| dir.path());

        for (i, file) in files.iter().enumerate(){   
            if (((i+1 % modulo) + modulo) % modulo) == 0 {
                fs::remove_file(file.path());
            } else {
                let mut rng = rand::thread_rng();
                fs::rename(file.path(), file.path().with_file_name(&format!("{}", rng.gen::<f64>())[2..]).as_path());
                count += 1;
            }
        }

    }

    let mut files: Vec<_> = fs::read_dir(&path)
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    files.sort_by_key(|dir| dir.path());

    let mut count = 1;
    for (i, file) in files.iter().enumerate() {   
        fs::rename(file.path(), file.path().with_file_name(&format!("image {}", count)).as_path());
        count += 1;
    }
    
}

