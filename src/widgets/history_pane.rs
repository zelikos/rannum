/*  Copyright (C) 2020-2022 Patrick Csikos (https://zelikos.github.io)
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
 * Authored by Patrick Csikos <zelikos@pm.me>
 */

use crate::deps::*;
use crate::utils;
use crate::models::RollitHistoryItem;
use crate::widgets::RollitHistoryRow;

use std::cell::RefCell;

use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{CompositeTemplate, NoSelection, SignalListItemFactory};

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/dev/zelikos/rollit/gtk/history-pane.ui")]
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
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RollitHistoryPane {
        fn constructed (&self, obj: &Self::Type) {
            self.parent_constructed(obj);

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
    pub fn add_result (&self, result: u32) {
        let settings = utils::settings_manager();
        let max = settings.int("max-roll") as u32;
        let imp = self.imp();

        let result_item = RollitHistoryItem::new(result, max);

        if imp.history_stack.visible_child_name().unwrap() == "empty" {
            self.clear_history();
            self.show_history();
        }

        // Append new result
        self.results().append(&result_item);

        // Prepend new result
        // self.results().insert(0, &result_item);

        let vadj = imp.history_scroll.vadjustment();
        vadj.set_value(vadj.upper());

        log::debug!("Result of {} added, out of a possible {}", result, max);
    }

    fn results(&self) -> gio::ListStore {
        self.imp()
            .results
            .borrow()
            .clone()
            .expect("Could not retrieve results.")
    }

    fn setup_results(&self) {
        let imp = self.imp();
        let model = gio::ListStore::new(RollitHistoryItem::static_type());

        imp.results.replace(Some(model));

        let selection_model = NoSelection::new(Some(&self.results()));
        imp.history_list.set_model(Some(&selection_model));
    }

    fn setup_factory(&self) {
        let factory = SignalListItemFactory::new();

        // Connect empty 'RollitHistoryRow' during setup
        factory.connect_setup(move |_, list_item| {
            let result_row = RollitHistoryRow::new();
            list_item.set_child(Some(&result_row));
        });

        // Tell factory how to bind 'RollitHistoryRow' to 'RollitHistoryItem'
        factory.connect_bind(move |_, list_item| {
            let result_item = list_item
                .item()
                .expect("The item must exist.")
                .downcast::<RollitHistoryItem>()
                .expect("The item must be a 'RollitHistoryItem'.");

            let result_row = list_item
                .child()
                .expect("The child must exist.")
                .downcast::<RollitHistoryRow>()
                .expect("The child must be a 'RollitHistoryRow'.");

            result_row.bind(&result_item);
        });

        // Tell factory how to unbind 'RollitHistoryRow' from 'RollitHistoryItem'
        factory.connect_unbind(move |_, list_item| {
            // Get 'RollitHistoryRow' from 'ListItem'
            let result_row = list_item
                .child()
                .expect("The child must exist.")
                .downcast::<RollitHistoryRow>()
                .expect("The child must be a 'RollitHistoryRow'.");

            result_row.unbind();
        });

        self.imp().history_list.set_factory(Some(&factory));
    }

    pub fn hide_history(&self) {
        let imp = self.imp();
        imp.history_stack.set_visible_child(&imp.history_stack.child_by_name("empty").unwrap());
    }

    pub fn show_history(&self) {
        let imp = self.imp();
        imp.history_stack.set_visible_child(&imp.history_stack.child_by_name("filled").unwrap());
    }

    fn clear_history(&self) {
        self.results().remove_all();
        log::debug!("History list cleared");
    }
}

