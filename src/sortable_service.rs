use yew::Callback;
use stdweb::js;
use stdweb::web::HtmlElement;
use crate::SortingChange;

/// Service to make a HtmlElement sortable using [Shopify/draggable](https://github.com/Shopify/draggable)
pub struct SortableService {}

impl SortableService {
    pub fn new() -> Self {
        Self {}
    }

    // pub fn make_sortable<E: 'static + ::stdweb::private::JsSerializeOwned + ::stdweb::web::IHtmlElement>(
    pub fn make_sortable(
        &self,
        element: HtmlElement,
        callback: Callback<SortingChange>,
    ) {
        self.register(element, callback);
    }

    //noinspection RsLiveness
    // fn register<E: 'static + ::stdweb::private::JsSerializeOwned + ::stdweb::web::IHtmlElement>(
    fn register(
        &self,
        element: HtmlElement,
        callback: Callback<SortingChange>) {
        let handler = move |old_index: i64, new_index: i64| {
            callback.emit(SortingChange::new(old_index as usize, new_index as usize));
        };
        js!(@(no_return)
            const element = @{element};
            const sortable = new Sortable.default(element, {
                draggable: "a",
                delay: 300
            });

            // sortable.on("sortable:start", (e) => console.log(e, "sortable:start"));
            // sortable.on("sortable:sort", (e) => console.log(e, "sortable:sort"));
            // sortable.on("sortable:sorted", (e) => console.log(e, "sortable:sorted"));
            sortable.on("sortable:stop", (e) => {
                setTimeout(() => {
                    const handler = @{handler};
                    handler(e.oldIndex, e.newIndex);
                }, 100);
                console.debug(e.oldIndex, e.newIndex);
            });
        );
    }
}
