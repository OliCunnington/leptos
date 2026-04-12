use leptos::prelude::*;

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}


#[component]
pub fn ItterateOverComplexData() -> impl IntoView {
    let (data, set_data) = signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
            // nested signal
            // value: RwSignal::new(10),
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        },
    ]);
    view! {
        <button on:click=move |_| {
            // nested signal
            // for row in &*data.read() {
            //    row.value.update(|value| *value *= 2);
            // }
            set_data.update(|data| {
                for row in data {
                    row.value *= 2;
                }
            });
            leptos::logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        <For
            each=move || data.get()
            // key=|state| state.key.clone() // does not update/re-draw when value is changed
                                             // as key is unchanged
            key=|state| (state.key.clone(), state.value) // least efficient solution
                                             // entire row is discarded and re-drawn
                                             // deriving PartialEq, Eq, and Hash on DatabaseEntry would
                                             // allow: key=|state| state.clone()
            let(child)
        >
            <p>{child.value}</p>
        </For>
    }
}

// nested signal
#[derive(Debug, Clone)]
struct NestedSignalDatabaseEntry {
    key: String,
    value: RwSignal<i32>,
}

#[component]
pub fn NestedSignalItterateOverComplexData() -> impl IntoView {
    let (data, set_data) = signal(vec![
        NestedSignalDatabaseEntry {
            key: "foo".to_string(),
            value: RwSignal::new(10),
        },
        NestedSignalDatabaseEntry {
            key: "bar".to_string(),
            value: RwSignal::new(20),
        },
        NestedSignalDatabaseEntry {
            key: "baz".to_string(),
            value: RwSignal::new(15),
        },
    ]);
    view! {
        <button on:click=move |_| {
            for row in &*data.read() {
               row.value.update(|value| *value *= 2);
            }
            
            leptos::logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        <For
            each=move || data.get()
            key=|state| state.key.clone()
            let(child)
        >
            <p>{child.value}</p>
        </For>
    }
}


//memoized slices...
#[component]
pub fn MemoizedSlicesItterateOverComplexData() -> impl IntoView {
    let (data, set_data) = signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        },
    ]);
    view! {
        <button on:click=move |_| {
            set_data.update(|data| {
                for row in data {
                    row.value *= 2;
                }
            });
            leptos::logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        <ForEnumerate
            each=move || data.get()
            key=|state| state.key.clone()
            children=move |index, _| {
                let value = Memo::new(move |_| {
                    data.with(|data| data.get(index.get()).map(|d| d.value).unwrap_or(0))
                });
                view! {
                    <p>{value}</p>
                }
            }
        />
    }
}

//stores... new(ish)
#[derive(Store, Debug, Clone)]
pub struct Data {
    #[store(key: String = |row| row.key.clone())]
    rows: Vec<StoresDatabaseEntry>,
}

#[derive(Store, Debug, Clone)]
struct StoresDatabaseEntry {
    key: String,
    value: i32,
}

#[component]
pub fn StoresItterateOverComplexData() -> impl IntoView {
    let data = Store::new(Data {
        rows: vec![
            StoresDatabaseEntry {
                key: "foo".to_string(),
                value: 10,
            },
            StoresDatabaseEntry {
                key: "bar".to_string(),
                value: 20,
            },
            StoresDatabaseEntry {
                key: "baz".to_string(),
                value: 15,
            },
        ],
    });

    view! {
        <button on:click=move |_| {
            use reactive_stores::StoreFieldIterator;

            for row in data.rows().iter_unkeyed() {
                *row.value().write() *= 2;
            }
            leptos::logging::log!("{:?}", data.get());
        }>
            "Update Values"
        </button>
        <For
            each=move || data.rows()
            key=|row| row.read().key.clone()
            children=|child| {
                let value = child.value();
                view! { <p>{move || value.get()}</p> }
            }
        />
    }
}