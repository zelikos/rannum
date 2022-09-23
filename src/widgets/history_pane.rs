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
use crate::widgets::RollitHistoryItem;

use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::CompositeTemplate;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/gitlab/zelikos/rollit/gtk/history-pane.ui")]
    pub struct RollitHistoryPane {
        #[template_child]
        pub(super) history_list: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub(super) history_stack: TemplateChild<gtk::Stack>,
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
    pub fn add_result (&self, result: u16) {
        let settings = utils::settings_manager();
        let max = settings.int("max-roll") as u16;
        let imp = self.imp();

        let result_item = RollitHistoryItem::new();
        result_item.add_result(result, max);

        imp.history_list.append(&result_item);

        if imp.history_stack.visible_child_name().unwrap() != "filled" {
            imp.history_stack.set_visible_child(&imp.history_stack.child_by_name("filled").unwrap());
        }
    }

    pub fn clear_history(&self) {
        let imp = self.imp();

        let mut current_item = imp.history_list.row_at_index(0);
        if current_item != None {
            while current_item != None {
                current_item = current_item;
                imp.history_list.remove (&current_item.unwrap());
                current_item = imp.history_list.row_at_index(0);
            }
        }

        imp.history_stack.set_visible_child(&imp.history_stack.child_by_name("empty").unwrap());
    }
}

