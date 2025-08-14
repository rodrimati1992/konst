macro_rules! closure_arg_pattern_limitations_docs {($($parameter_plurality:literal)?) => {concat!(
    "# Closure Argument\n",
    "\n",
    "The closure argument must be one of:\n",
    "- closure syntax with no type annotations \n",
    "(on", $(" either the parameter", $parameter_plurality, " or",)? " the return type) \n",
    "- a function variable\n",
    "- a function path\n",
)}}
pub(crate) use closure_arg_pattern_limitations_docs;

macro_rules! closure_arg_annotated_params_options_docs {
    ($parameter_plurality:literal) => {concat!(
        "- closure syntax where each parameter has a pattern. There can't be type annotations \n",
        "(on either parameter", $parameter_plurality, " or return types) \n",
        "- closure syntax where each parameter has a single-token parameter pattern",
        ". Parameter and return types can be annotated. \n",
        "- a function variable\n",
        "- a function path\n",
    )}
}
pub(crate) use closure_arg_annotated_params_options_docs;

macro_rules! closure_arg_annotated_params_limitations_docs {
    ($parameter_plurality:literal) => {
        concat!(
            "# Closure Argument\n",
            "\n",
            "The closure argument must be one of:\n",
            crate::docs::closure_arg_annotated_params_options_docs!($parameter_plurality),
        )
    };
}
pub(crate) use closure_arg_annotated_params_limitations_docs;
