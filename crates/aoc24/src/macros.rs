#[macro_export]
macro_rules! run_solutions {
    ($($module:ident),*) => {
        run(&[
            $( $module::solve, )*
        ])
    };
}
