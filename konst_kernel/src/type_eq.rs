use core::marker::PhantomData;

// documented in konst::polymorphism::type_eq;
pub trait HasTypeWitness<W: TypeWitnessTypeArg<Arg = Self>> {
    /// A constant of the type witness
    const WITNESS: W;

    #[doc(hidden)]
    const __PRIV_KO9Y329U2U: __Priv<Self, W>;
}

impl<T, W> HasTypeWitness<W> for T
where
    W: TypeWitnessTypeArg<Arg = T> + MakeTypeWitness,
{
    const WITNESS: W = W::MAKE;

    #[doc(hidden)]
    const __PRIV_KO9Y329U2U: __Priv<Self, W> = __Priv(PhantomData, PhantomData);
}

#[doc(hidden)]
pub struct __Priv<T: ?Sized, W>(
    PhantomData<fn() -> PhantomData<W>>,
    PhantomData<fn() -> PhantomData<T>>,
);

pub trait TypeWitnessTypeArg {
    /// The type argument of the type witness.
    ///
    /// Note: this is not necessarily one of the types
    /// that this witness type witnesses.
    type Arg;
}

pub trait MakeTypeWitness: TypeWitnessTypeArg {
    /// A constant with the type witness
    const MAKE: Self;
}

mod type_eq {
    use core::marker::PhantomData;

    pub struct TypeEq<L: ?Sized, R: ?Sized>(
        PhantomData<(
            fn(PhantomData<L>) -> PhantomData<L>,
            fn(PhantomData<R>) -> PhantomData<R>,
        )>,
    );

    impl<L: ?Sized> TypeEq<L, L> {
        /// Constructs a `TypeEq<L, L>`.
        pub const NEW: Self = TypeEq(PhantomData);
    }
}
pub use type_eq::TypeEq;

impl<L: ?Sized, R: ?Sized> Copy for TypeEq<L, R> {}

impl<L: ?Sized, R: ?Sized> Clone for TypeEq<L, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<L, R> TypeEq<L, R> {
    /// Whether `L` is the same type as `R`.
    const ARE_SAME_TYPE: Amb = {
        // hacky way to emulate a lifetime-unaware
        // `TypeId::of<L>() == TypeId::of<R>()`
        let approx_same_type = {
            core::mem::size_of::<L>() == core::mem::size_of::<R>()
                && core::mem::align_of::<L>() == core::mem::align_of::<R>()
                && core::mem::size_of::<Option<L>>() == core::mem::size_of::<Option<R>>()
                && core::mem::align_of::<Option<L>>() == core::mem::align_of::<Option<R>>()
        };

        if approx_same_type {
            Amb::Indefinite
        } else {
            Amb::No
        }
    };

    /// Hints to the compiler that a `TypeEq<L, R>`
    /// can only be constructed if `L == R`.
    #[inline(always)]
    pub const fn reachability_hint<T>(self, val: T) -> T {
        if let Amb::No = Self::ARE_SAME_TYPE {
            // safety: it's impossible to have a `TypeEq<L, R>` value,
            // where `L` and `R` are not the same type
            unsafe { core::hint::unreachable_unchecked() }
        }

        val
    }

    /// A no-op cast from `L` to `R`
    ///
    /// This method uses the fact that
    /// having a `TypeEq<L, R>` value proves that `L` and `R` are the same type.
    #[inline(always)]
    pub const fn to_right(self, from: L) -> R {
        self.reachability_hint(());

        unsafe { crate::__priv_transmute!(L, R, from) }
    }
}

enum Amb {
    // indefinitely false/true
    Indefinite,
    // definitely false
    No,
}
