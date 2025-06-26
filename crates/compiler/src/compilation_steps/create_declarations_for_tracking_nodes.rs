use crate::prelude::*;
use yarnspinner_core::prelude::Library;
use yarnspinner_core::types::Type;

pub(crate) fn create_declarations_for_tracking_nodes(
    mut state: CompilationIntermediate,
) -> CompilationIntermediate {
    let tracking_declarations = state.tracking_nodes.iter().map(|node| {
        let name = Library::generate_unique_visited_variable_for_node(node);
        Declaration::new(name, Type::Number)
            .with_default_value(0.0)
            .with_description(format!(
                "The generated variable for tracking visits of node {node}"
            ))
    });

    // adding the generated tracking variables into the declaration list
    // this way any future variable storage system will know about them
    // if we didn't do this later stages wouldn't be able to interface with them

    state
        .known_variable_declarations
        .extend(tracking_declarations.clone());
    state
        .derived_variable_declarations
        .extend(tracking_declarations);

    state
}
