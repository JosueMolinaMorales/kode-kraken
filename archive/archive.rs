/**
 * This file stores the olds mutation code. To be removed once it is no longer needed
 */
fn mutate(config: MutationCommandConfig, ouptut_directory: String) -> Result<(), CliError> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_kotlin::language()).unwrap();

    let mut file_mutations: Vec<FileMutations> = vec![];
    let directory = Path::new(config.path.as_str())
        .read_dir()
        .map_err(|_| CliError { kind: ErrorKind::Io, message: "Could not read directory".into()})?;

    // Check to see if the output directory exists if not create it
    let output_dir = Path::new(ouptut_directory.as_str());
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).unwrap();
    }

    for entry in directory {
        let entry = entry.unwrap();
        let path = entry.path();
        // Refactoring for a directory will be needed
        if !path.is_file() {
            continue;
        }

        let file_name = path.file_name().unwrap().to_str().unwrap();
        if !file_name.ends_with(".kt") {
            continue;
        }
        // prepend mutation to file name
        let file_name = format!("mutation_{file_name}",);
        let file = fs::read_to_string(path.clone()).expect("File Not Found!");
        let parsed = parser.parse(&file, None).unwrap();
        let root_node = parsed.root_node();
        let mut cursor = parsed.walk();

        let mut mutations_made = Vec::new();
        println!("File: {}", output_dir.join(&file_name).display());
        // Write file to mutation folder
        fs::write(output_dir.join(&file_name), file.clone()).unwrap();
        search_children(
            root_node, 
            &mut cursor, 
            " ", 
            false, 
            &mut mutations_made,
            file.to_string(),
            format!("{}", output_dir.join(&file_name).display())
        );
        file_mutations.push(FileMutations {
            mutations: mutations_made,
        });
    }

    println!("File Mutations: {file_mutations:#?}",);
    Ok(())
}

fn search_children(
    root: tree_sitter::Node, 
    cursor: &mut tree_sitter::TreeCursor, 
    prefix: &str,
    parent_was_comp_exp: bool,
    mutations_made: &mut Vec<Mutation>,
    kt_file: String,
    output_file: String,
) {
    root
        .children(&mut cursor.clone())
        .for_each(|node| {
            let node_type = KotlinTypes::new(node.kind()).expect("Failed to convert to KotlinType");
            if parent_was_comp_exp && node_type == KotlinTypes::NonNamedType("==".to_string()) {
                // TODO: Inserting mutants will need to be updated
                //       to account for the fact that the start and end
                //       bytes will change as we insert new mutants
                let new_op = "!=".as_bytes();
                let mut mutated_file: Vec<u8> = kt_file.as_bytes().to_vec();
                for (i, b) in mutated_file.iter_mut().skip(node.start_byte()).enumerate() {
                    if i >= (node.end_byte() - node.start_byte()) {
                        break;
                    }
                    *b = new_op[i];
                }
                fs::write(&output_file, mutated_file).unwrap();
                mutations_made.push(Mutation::new(
                    node.start_byte(), 
                    node.end_byte(), 
                    "!=".to_string(), 
                    "==".to_string(),
                    node.start_position().row + 1,
                    MutationOperators::RelationalOperator
                ));
            }
            println!("{}({} {} - {})", prefix, node.kind(), node.start_position(), node.end_position());
            let kt_file = fs::read_to_string(&output_file).unwrap();

            search_children(
                node, 
                &mut cursor.clone(), 
                &format!("    {prefix}"),
                node_type == KotlinTypes::ComparisonExpression || node_type == KotlinTypes::EqualityExpression,
                mutations_made,
                kt_file,
                output_file.clone()
            )
        })
}