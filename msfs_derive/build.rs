fn main() {
    // let wasm = std::env::var("TARGET").unwrap().starts_with("wasm32-");
    let msfs_sdk = match msfs_sdk::calculate_msfs_sdk_path() {
        Ok(path) => path,
        Err(error) => {
            println!("MSFS SDK folder not found.");
            println!("Set the path using environment variable MSFS_SDK or check the folder C:\\MSFS SDK exists.");
            println!("SDK can be downloaded through the developer menu in MSFS.");
            panic!("{:?}", error);
        }
    };
    println!("Found MSFS SDK: {:?}", msfs_sdk);

    let sample_path = std::path::PathBuf::from(msfs_sdk.as_str()).join("Samples");
    match std::fs::read_dir(sample_path) {
        Ok(_) => (),
        Err(error) => {
            println!("Samples path not found.");
            println!("Install SDK Samples through the developer menu in MSFS.");
            panic!("{:?}", error);
        }
    };

    let simvar_sample_path = std::path::PathBuf::from(msfs_sdk.as_str())
        .join("Samples")
        .join("SimvarWatcher")
        .join("Simvars");
    let read_dir = match std::fs::read_dir(simvar_sample_path) {
        Ok(dir) => dir,
        Err(error) => {
            println!("Samples/SimvarWatcher/Simvars path not found.");
            panic!("{:?}", error);
        }
    };

    let data = read_dir
        .map(|f| f.unwrap())
        .flat_map(|f| {
            let source = std::fs::read_to_string(f.path()).unwrap();
            source
                .lines()
                .filter(|l| !l.is_empty())
                .map(|l| {
                    let parts = l.split(',').collect::<Vec<&str>>();
                    println!("parts {:?}", parts);
                    (parts[0].to_owned(), parts[1].to_owned())
                })
                .collect::<Vec<(String, String)>>()
        })
        .map(|s| {
            format!(
                "simvars.insert({:?}.to_string(), {:?}.to_string());",
                s.0, s.1
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let source = format!(
        "
    use std::collections::HashMap;

    pub fn get_simvars() -> HashMap<String, String> {{
        let mut simvars = HashMap::new();
        {}
        simvars
    }}
    ",
        data
    );

    //println!("DATA:");
    //print!("{}", data);

    //panic!("Bye");
    std::fs::write(
        std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs"),
        source,
    )
    .unwrap();
}
