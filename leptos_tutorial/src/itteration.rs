use leptos::prelude::*;


#[component]
pub fn ItterateStaticViews() -> impl IntoView {
    // create a list of 5 signals
    let length = 5;
    let counters = (1..=length).map(|idx| RwSignal::new(idx));

    let counter_buttons = counters
        .map(|count| {
            view! {
                <li>
                    <button
                        on:click=move |_| *count.write() += 1
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();

    view! {
        <ul>{counter_buttons}</ul>
    }
}

#[component]
pub fn ItterateDynamicList(
    initial_length: usize,
) -> impl IntoView {
    let mut next_counter_id = initial_length;

    let initial_counters = (0..initial_length)
        .map(|id| (id, ArcRwSignal::new(id + 1)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = signal(initial_counters);

    let add_counter = move |_| {
        let sig = ArcRwSignal::new(next_counter_id + 1);
        set_counters.update(move |counters| {
            counters.push((next_counter_id, sig))
        });
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                <For
                    each=move || counters.get()
                    key=|counter| counter.0
                    children=move |(id, count)| {
                        let count = RwSignal::from(count);
                        view! {
                            <li>
                                <button
                                    on:click=move |_| *count.write() += 1
                                >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters
                                            .write()
                                            .retain(|(counter_id, _)| {
                                                counter_id != &id
                                            });
                                    }
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Counter {
  id: usize,
  count: RwSignal<i32>
}


pub fn ItterateDynamicListWithEnumerate() -> impl IntoView {

    //TODO
    view! {
        <ForEnumerate
            each=move || counters.get() 
            key=|counter| counter.id    
            children={move |index: ReadSignal<usize>, counter: Counter| {
                view! {
                    <button>{move || index.get()} ". Value: " {move || counter.count.get()}</button>
                }
            }}
        />
    }
}