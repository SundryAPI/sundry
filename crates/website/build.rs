fn main() {
    // Expose env variables to the client if they start with PUBLIC_
    match dotenvy::dotenv_iter() {
        Ok(iter) => {
            for item in iter {
                if let Ok((key, val)) = item {
                    if key.starts_with("PUBLIC_") {
                        println!("cargo:rustc-env={}={}", key, val);
                    }
                }
            }
        }
        Err(e) => eprintln!("loading env variables: {e:?}"),
    }
}
