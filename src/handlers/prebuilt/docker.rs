use crate::BaseImage;

pub fn update_dockerfile(image_dockerfile: &mut String, base_image: &BaseImage) {
    // Split the Dockerfile content into lines
    let mut lines: Vec<String> = image_dockerfile.lines().map(|line| line.to_string()).collect();

    // Update the 2nd and 5th lines with the new commands
    lines[1] = format!("{} {}", "FROM", base_image.name);

    // Insert multiple RUN commands starting from the 5th line
    let run_commands: Vec<String> = base_image.commands.iter().map(|cmd| format!("{} {}", "RUN", cmd)).collect();
    lines.splice(4..5, run_commands);

    // Join the lines back into a single stringZ
    *image_dockerfile = lines.join("\n");
}