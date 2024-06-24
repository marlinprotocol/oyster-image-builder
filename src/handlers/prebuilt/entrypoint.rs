pub fn update_entrypoint(entrypoint: &mut String, entrypoint_commands: &Vec<String>) {
    if let Some(pos) = entrypoint.find("# user defined commands") {
        let insertion_point = pos + "# user defined commands".len();
        let new_commands: String = entrypoint_commands.iter()
            .map(|cmd| format!("\n{}", cmd))
            .collect();
        entrypoint.insert_str(insertion_point, &new_commands);
    }
}
