macro_rules! impl_trait_n_args {
	( $($var:ident: $typevar:ident),* ) => (
		impl<$($typevar,)* Result, F: FnOnce($($typevar),*) -> Result> FnBox<($($typevar,)*), Result> for F {
			fn call(self: Box<Self>, ($($var,)*): ($($typevar,)*)) -> Result {
				let this : Self = *self;
				this($($var),*)
			}
		}
	)
}

macro_rules! build_n_args {
	( $name:ident [$($add:tt)*]: $($var:ident: $typevar:ident),* ) => (
		impl<'a, $($typevar,)* Result> $name<'a, ($($typevar,)*), Result> {
			/// call inner function, consumes the box.
			#[inline]
			pub fn call(self $(, $var: $typevar)*) -> Result {
				FnBox::call(self.0, ($($var ,)*))
			}
		}

		impl<'a,  $($typevar,)* Result, F: 'a + FnOnce($($typevar),*) -> Result $($add)*> From<F> for $name<'a, ($($typevar,)*), Result> {
			fn from(func: F) -> Self {
				$name(Box::new(func) as Box<FnBox<($($typevar,)*), Result> $($add)* + 'a>)
			}
		}
	)
}
