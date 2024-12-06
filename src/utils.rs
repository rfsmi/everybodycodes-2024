macro_rules! make_runner {
    (@expand
        [ $($crunched:tt)* ]
        $quest:tt + + ,
        $($rest:tt)*
    ) => (
        crate::utils::make_runner!(@crunch $($crunched)* { $quest 3 2 1 } $($rest)*);
    );
    (@expand
        [ $($crunched:tt)* ]
        $quest:tt + ,
        $($rest:tt)*
    ) => (
        crate::utils::make_runner!(@crunch $($crunched)* { $quest 2 1 } $($rest)*);
    );
    (@expand
        [ $($crunched:tt)* ]
        $quest:tt ,
        $($rest:tt)*
    ) => (
        crate::utils::make_runner!(@crunch $($crunched)* { $quest 1 } $($rest)*);
    );
    (@expand [ $($crunched:tt)* ]) => (
        crate::utils::make_runner!(@assemble $($crunched)* );
    );
    (@crunch
        { $($modules:tt)* }
        { $($variants:tt)* }
        { $($runners:tt)* }
        { $quest:tt $($part:tt)+ }
        $($rest:tt)*
    ) => (
        paste::paste! { crate::utils::make_runner!(@expand [
            {
                mod [< quest $quest >];
                $($modules)*
            }
            {
                $([< Quest $quest _ $part >],)+
                $($variants)*
            }
            {
                $(
                    Task::[< Quest $quest _ $part >] => {
                        let name = concat!($quest, " (part ", $part, ")");
                        let input = include_str!(concat!("../inputs/", $quest, "-", $part, ".txt"));
                        let result = [< quest $quest >]::[< solve_ $part >](input);
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
            let (quest, result) = match args.task {
                Task::Latest | $($runners)*
            };
            let duration = start.elapsed().as_secs_f32();
            println!("Computed result for quest {quest} in {duration:.3} seconds: {result}");
        }
    );
    ($($quests:tt)*) => {
        crate::utils::make_runner!(@expand [{} {} {}] $($quests)*);
    };
}

pub(crate) use make_runner;
