use image::open;
use image::GenericImageView;
fn main() {

    let start = std::time::Instant::now();

    println!("Sasware PNG to RBXMX converter v1 : Now with 2% more rust.");

    let first_file = std::fs::read_dir("./input")
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();

    let image = open(&first_file);

    println!("Loading image: {}", first_file.display());

    match image {
        Ok(image) => {
            println!("Image loaded!");
            println!("Image dimensions: {:?}", image.dimensions());
            println!("Image color: {:?}", image.color());
            
            let mut json_data: String = String::with_capacity(16 + image.dimensions().0 as usize * image.dimensions().1 as usize * 3);
            let mut current_row: String = String::new();

            let mut x = 0;
            let mut y = 0;

            json_data.push_str("{\"0\": {");

            for pixel in image.pixels() {
                x += 1;

                current_row.push_str(
                    format!(
                        "\"{}\": [{},{},{}]",
                        x,
                        pixel.2[0],
                        pixel.2[1],
                        pixel.2[2]
                    ).as_str()
                );

                if x != image.dimensions().0 {
                    current_row.push_str(", ");
                }
                
                if x == image.dimensions().0 {

                    json_data.push_str(current_row.as_str());
                    json_data.push_str("}");

                    if y != image.dimensions().1 - 1 {
                        json_data.push_str(format!(",\"{}\": {{", y + 1).as_str());
                    }

                    current_row.clear();
                    
                    x = 0;
                    y += 1;
                }

                if y == image.dimensions().1 {
                    json_data.push_str("}");
                }
            }
            println!("Finished processing!");
            println!("Time elapsed: {}ms", start.elapsed().as_millis());
            std::fs::write(
                format!("./output/{}.rbxmx", first_file.file_stem().unwrap().to_str().unwrap()), format!(
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
</roblox>"
            ,&json_data))
            .expect("Unable to write file");
        }
        Err(e) => {
            println!("Error loading image: {}", e);
        }
    }
}