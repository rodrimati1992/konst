/// Trait for all the types that can be iterated over with ranges.
///
/// This trait is sealed and can only be implemented by `konst`
pub use konst_kernel::step_kk::Step;

/// [Type witness](crate::docs::type_witnesses)
/// for all the types that can be iterated over with ranges.
pub use konst_kernel::step_kk::StepWitness;
