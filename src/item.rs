use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink};
use std::fmt::Debug;
use stdweb::js;
use crate::Entry;

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct SongListItemProps {
    pub song: Entry,

    pub index: usize,
}

#[allow(dead_code)]
pub struct Item {
    /// State from the parent
    props: SongListItemProps,
    /// Utility object
    link: ComponentLink<Self>,
}

impl Component for Item {
    type Message = ();
    type Properties = SongListItemProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        let new_name = &props.song.name;
        let old_name = &self.props.song.name;
        js!(console.log("%c" + @{new_name} + " vs " + @{old_name}, "color:Orange"));
        self.props = props;
        return true;
        // if self.props != props {
        //     true
        // } else {
        //     false
        // }
    }

    fn view(&self) -> VNode {
        let name = &self.props.song.name;
        let index = &self.props.index;
        let href = format!("#/song/{}", index);
        let pos = index + 1;

        html! { <a role="button" data-index=index href=href><span>{pos}</span>{name}</a> }
    }
}
