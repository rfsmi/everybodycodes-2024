macro_rules! make_runner {
    (@expand
        [ $($crunched:tt)* ]
        $day:tt + + ,
        $($rest:tt)*
    ) => (
        crate::utils::make_runner!(@crunch $($crunched)* { $day 3 2 1 } $($rest)*);
    );
    (@expand
        [ $($crunched:tt)* ]
        $day:tt + ,
        $($rest:tt)*
    ) => (
        crate::utils::make_runner!(@crunch $($crunched)* { $day 2 1 } $($rest)*);
    );
    (@expand
        [ $($crunched:tt)* ]
        $day:tt ,
        $($rest:tt)*
    ) => (
        crate::utils::make_runner!(@crunch $($crunched)* { $day 1 } $($rest)*);
    );
    (@expand [ $($crunched:tt)* ]) => (
        crate::utils::make_runner!(@assemble $($crunched)* );
    );
    (@crunch
        { $($modules:tt)* }
        { $($variants:tt)* }
        { $($runners:tt)* }
        { $day:tt $($part:tt)+ }
        $($rest:tt)*
    ) => (
        paste::paste! { crate::utils::make_runner!(@expand [
            {
                mod [< day $day >];
                $($modules)*
            }
            {
                $([< Day $day _ $part >],)+
                $($variants)*
            }
            {
                $(
                    Task::[< Day $day _ $part >] => {
                        let name = concat!($day, " (part ", $part, ")");
                        let input = include_str!(concat!("../inputs/", $day, "-", $part, ".txt"));
                        let result = [< day $day >]::[< solve_ $part >](input);
                        (name, result.to_string())
                    },
                )+
                $($runners)*
            }
        ] $($rest)* ); }
    );
    (@assemble
        { $($modules:tt)* }
        { $($variants:tt)* }
        { $($runners:tt)* }
    ) => (
        #[derive(clap::ValueEnum, Copy, Clone, Debug)]
        enum Task { $($variants)* Latest }

        $($modules)*

        fn run(args: Args) {
            let start = std::time::Instant::now();
            let (day, result) = match args.task {
                Task::Latest | $($runners)*
            };
            let duration = start.elapsed().as_secs_f32();
            println!("Computed result for day {day} in {duration:.3} seconds: {result}");
        }
    );
    ($($days:tt)*) => {
        crate::utils::make_runner!(@expand [{} {} {}] $($days)*);
    };
}

pub(crate) use make_runner;
