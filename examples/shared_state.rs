use ferrum_runtime::prelude::*;

fn main() {
    let name: FeStr = FeStr::from("Adam");
    print(&name);

    let name1: FeShared<FeStr> = FeShared::new(name);
    print(&name1);

    let mut name2: FeShared<FeStr> = FeShared::share(&name1);

    let mut name3: FeShared<FeStr> = FeShared::share(&name1);

    print(&name1);

    *name2 = FeStr::from("Foo");
    print(&name1);

    *name3 = FeStr::from("Bar");
    print(name1);
}

