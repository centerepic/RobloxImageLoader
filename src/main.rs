use serde_json;

use image::{open, GenericImageView};
use std::{fs, path::PathBuf, process, io, collections::HashMap}; // why the fuck didnt you do this

fn get_input_file() -> PathBuf {
    let files = match fs::read_dir("./input") {
        // get all files under ./input
        Ok(files) => files,
        Err(_) => {
            let _ = fs::create_dir("./input"); // create the folder if it doesent exist
            println!("Place your files inside the input directory");

            io::stdin().read_line(&mut String::new()).unwrap();
            process::exit(0)
        }
    };

    let files: Vec<PathBuf> = files // read_dir returns an iterator, so we map it and return the path from each item
        .map(|file| file.unwrap().path())
        .collect(); // convert the map into a Vec

    if files.is_empty() { // check if the dir is empty
        println!("Input directory is empty");

        io::stdin().read_line(&mut String::new()).unwrap();
        process::exit(0)
    }

    return files.get(0).unwrap().to_path_buf();
}

fn main() {
    let start = std::time::Instant::now();

    println!("Sasware PNG to RBXMX converter v1 : Now with 2% more rust.");

    let first_file = get_input_file();

    println!("Loading image: {}", first_file.display());

    let image = match open(&first_file) {
        Ok(image) => image,
        Err(e) => {
            println!("Error loading image: {}", e);
            process::exit(0)
        }
    };

    println!("Image loaded!");
    println!("Image dimensions: {:?}", image.dimensions());
    println!("Image color: {:?}", image.color());

    let mut output: HashMap<u32, HashMap<u32, (u8, u8, u8)>> = HashMap::new();

    /*
        y: {
            x: {
                r,
                g,
                b,
            }
        }
    */

    for pixel in image.pixels() {
        let (x_pos, y_pos, rgb) = pixel;

        let y = output.entry(y_pos).or_insert_with(HashMap::new);
        y.insert(x_pos, (rgb[0], rgb[1], rgb[2]));
    }

    println!("Finished processing!");
    println!("Time elapsed: {}ms", start.elapsed().as_millis());

    let json_data = serde_json::to_string(&output).unwrap(); // write the json to a string
    let output_data = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
        <roblox version=\"4\">
            <Item class=\"ModuleScript\" referent=\"RBXLuaSourceContainer\">
                <Properties>
                    <string name=\"Name\">ModuleScript</string>
                    <string name=\"Source\">
                        local module = {{}}
                        module.Image = '{}'
                        return module
                    </string>
                </Properties>
            </Item>
        </roblox>",
        &json_data
    );

    match fs::read_dir("./output") {
        Ok(files) => files,
        Err(_) => {
            let _ = fs::create_dir("./output"); // create the folder if it doesent exist
            fs::read_dir("./output").unwrap()
        }
    };

    let file_name = first_file.file_stem().unwrap().to_str().unwrap();
    fs::write(format!("./output/{}.rbxmx", file_name), output_data).expect("Unable to write file");
}
