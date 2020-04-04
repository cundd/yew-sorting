pub use super::item::Item;
use log::info;
use std::rc::Rc;
use yew::prelude::*;
use stdweb::web::HtmlElement;
use crate::sortable_service::SortableService;
use crate::{SortingChange, Entry};

#[derive(Properties, PartialEq, Clone)]
pub struct SongListProps {
    pub songs: Rc<Vec<Entry>>,
    pub on_sorting_change: Callback<SortingChange>,
}

pub enum Msg {
    SetlistChangeSorting(SortingChange),
}

pub struct SongList {
    props: SongListProps,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    sortable_service: SortableService,
}

impl Component for SongList {
    type Message = Msg;
    type Properties = SongListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            node_ref: NodeRef::default(),
            sortable_service: SortableService::new(),
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        if let Some(element) = self.node_ref.cast::<HtmlElement>() {
            self.sortable_service.make_sortable(element, self.link.callback(|e| Msg::SetlistChangeSorting(e)));
        }

        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetlistChangeSorting(e) => {
                info!("Handle Setlist sorting change: Move {} to {}", e.old_index, e.new_index);
                self.props.on_sorting_change.emit(e);

                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        return true;
        // if self.props != props {
        //     true
        // } else {
        //     // false
        //     true
        // }
    }

    fn view(&self) -> Html {
        let songs = &self.props.songs;
        let render = |(index, song): (usize, &Entry)| {
            html! { <Item index=index song=song.clone() /> }
        };

        info!(
            "Redraw song list {:?}",
            songs.iter().map(|s| s.name.clone()).collect::<Vec<String>>()
        );

        (html! {
            <div ref=self.node_ref.clone()>
                {for songs.iter().enumerate().map(render)}
            </div>
        }) as Html
    }
}
