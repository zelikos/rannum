/*  Copyright (C) 2020-2023 Patrick Csikos (https://zelikos.dev)
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * Authored by Patrick Csikos <pcsikos@zelikos.dev>
 */

use crate::models::RollitHistoryItem;
use crate::utils;
use crate::widgets::RollitHistoryRow;

use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::prelude::*;
use gtk::{gio, glib, ListItem};
use gtk::{NoSelection, SignalListItemFactory};
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/dev/zelikos/rollit/ui/widgets/history-pane.ui")]
    pub struct RollitHistoryPane {
        #[template_child]
        pub(super) history_list: TemplateChild<gtk::ListView>,
        #[template_child]
        pub(super) history_scroll: TemplateChild<gtk::ScrolledWindow>,
        #[template_child]
        pub(super) history_stack: TemplateChild<gtk::Stack>,
        pub(super) results: RefCell<Option<gio::ListStore>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RollitHistoryPane {
        const NAME: &'static str = "RollitHistoryPane";
        type Type = super::RollitHistoryPane;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RollitHistoryPane {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.setup_results();
            obj.setup_factory();
        }
    }

    impl WidgetImpl for RollitHistoryPane {}
    impl BinImpl for RollitHistoryPane {}
}

glib::wrapper! {
    pub struct RollitHistoryPane(ObjectSubclass<imp::RollitHistoryPane>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Buildable;
}

impl RollitHistoryPane {
    pub fn add_result(&self, result: u32) {
        let settings = utils::settings_manager();
        let max = settings.int("max-roll") as u32;
        let imp = self.imp();

        let result_item = RollitHistoryItem::new(result, max);

        if imp.history_stack.visible_child_name().unwrap() == "empty" {
            self.clear_history();
            self.show_history();
        }

        // Prepend new result
        self.results().insert(0, &result_item);

        log::debug!("Result of {} added, out of a possible {}", result, max);
        log::debug!("Number of results: {}", self.results().n_items());
    }

    fn results(&self) -> gio::ListStore {
        self.imp()
            .results
            .borrow()
            .clone()
            .expect("Could not retrieve results.")
    }

    fn scroll_view(&self) {
        let vadj = self.imp().history_scroll.vadjustment();
        let history = &self.imp().history_list;

        // Only autoscroll if the view is already at the top.
        if vadj.value() == vadj.lower() {
            history.scroll_to(0, gtk::ListScrollFlags::NONE, None);
        }
    }

    fn setup_results(&self) {
        let imp = self.imp();
        let model = gio::ListStore::new::<RollitHistoryItem>();

        imp.results.replace(Some(model));

        let selection_model = NoSelection::new(Some(self.results()));
        imp.history_list.set_model(Some(&selection_model));

        imp.history_list.connect_activate(
            glib::clone!(@weak self as pane, @weak selection_model => move |_, pos| {
                let result = selection_model
                    .upcast::<gio::ListModel>()
                    .item(pos)
                    .unwrap()
                    .downcast::<RollitHistoryItem>()
                    .unwrap();

                let result: u32 = result.property("result");

                let clipboard = pane.clipboard();
                clipboard.set_text(&result.to_string());

                pane.activate_action(
                    "win.show-toast",
                    Some(&(gettext("Result copied"), 0).to_variant()),
                )
                .unwrap();
            }),
        );

        selection_model.connect_items_changed(glib::clone!(@weak self as pane => move |_,_,_,_| {
            pane.scroll_view();
        }));
    }

    fn setup_factory(&self) {
        let factory = SignalListItemFactory::new();

        // Connect empty 'RollitHistoryRow' during setup
        factory.connect_setup(move |_, list_item| {
            let result_row = RollitHistoryRow::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&result_row));
        });

        // Tell factory how to bind 'RollitHistoryRow' to 'RollitHistoryItem'
        factory.connect_bind(move |_, list_item| {
            let result_item = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<RollitHistoryItem>()
                .expect("The item must be a 'RollitHistoryItem'.");

            let result_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<RollitHistoryRow>()
                .expect("The child must be a 'RollitHistoryRow'.");

            result_row.bind(&result_item);
        });

        // Tell factory how to unbind 'RollitHistoryRow' from 'RollitHistoryItem'
        factory.connect_unbind(move |_, list_item| {
            // Get 'RollitHistoryRow' from 'ListItem'
            let result_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<RollitHistoryRow>()
                .expect("The child must be a 'RollitHistoryRow'.");

            result_row.unbind();
        });

        self.imp().history_list.set_factory(Some(&factory));
    }

    pub fn hide_history(&self) {
        let imp = self.imp();
        imp.history_stack
            .set_visible_child(&imp.history_stack.child_by_name("empty").unwrap());
    }

    pub fn show_history(&self) {
        let imp = self.imp();
        imp.history_stack
            .set_visible_child(&imp.history_stack.child_by_name("filled").unwrap());
    }

    fn clear_history(&self) {
        self.results().remove_all();
        log::debug!("History list cleared");
    }
}
