macro_rules! closure_non_local_return_docs {
    () => {
        concat!(
            "\n\n",
            "If the closure syntax is used: ",
            "early returns (e.g.: `return`) don't return from closure scope,",
            " they return from the function that this macro is called inside of.",
            "\n\n",
        )
    };
}
pub(crate) use closure_non_local_return_docs;

macro_rules! closure_arg_pattern_limitations_docs {($($parameter_plurality:literal)?) => {concat!(
    "# Closure Argument\n",
    "\n",
    "The closure argument must be one of:\n",
    "- closure syntax with no type annotations \n",
    "(on", $(" either the parameter", $parameter_plurality, " or",)? " the return type) \n",
    "- a function variable\n",
    "- a function path\n",
    crate::docs::closure_non_local_return_docs!(),
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
            crate::docs::closure_non_local_return_docs!(),
        )
    };
}
pub(crate) use closure_arg_annotated_params_limitations_docs;
