mod item;
mod list;
mod sortable_service;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use list::SongList;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct SortingChange {
    old_index: usize,
    new_index: usize,
}

impl SortingChange {
    pub fn new(old_index: usize,
               new_index: usize, ) -> Self {
        Self {
            old_index,
            new_index,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Entry {
    pub name: String
}

impl Entry {
    fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }
}

struct App {
    link: ComponentLink<Self>,
    list: Vec<Entry>,
}

/// Move the entry from one index to another one
fn move_entry(list: &mut Vec<Entry>, from: usize, to: usize) {
    let item = list.remove(from);
    list.insert(to, item);
}

enum Msg {
    SortingChange(SortingChange),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            list: vec![
                Entry::new("Entry 1"),
                Entry::new("Entry 2"),
                Entry::new("Entry 3"),
                Entry::new("Entry 4"),
                Entry::new("Entry 5"),
            ],
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SortingChange(s) => {
                move_entry(&mut self.list, s.old_index, s.new_index);
                true // Indicate that the Component should re-render
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        let on_sorting_change = self.link.callback(|e| { Msg::SortingChange(e) });
        let songs = Rc::new(self.list.clone());

        html! {
            <SongList on_sorting_change=on_sorting_change songs=songs />
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
