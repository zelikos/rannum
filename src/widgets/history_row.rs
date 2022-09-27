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
use crate::i18n::*;
use crate::models::RollitHistoryItem;

use std::cell::RefCell;

use adw::subclass::prelude::*;
use glib::{Binding, BindingFlags};
use gtk::prelude::*;
use gtk::{CompositeTemplate};

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/gitlab/zelikos/rollit/gtk/history-row.ui")]
    pub struct RollitHistoryRow {
        #[template_child]
        pub roll_result: TemplateChild<gtk::Label>,
        #[template_child]
        pub max_suffix: TemplateChild<gtk::Label>,
        pub bindings: RefCell<Vec<Binding>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RollitHistoryRow {
        const NAME: &'static str = "RollitHistoryRow";
        type Type = super::RollitHistoryRow;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);

            klass.install_action("history.copy-result", None, move |history, _, _| {
                history.copy_result();
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RollitHistoryRow {
        fn constructed (&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for RollitHistoryRow {}
    impl BinImpl for RollitHistoryRow {}
}

glib::wrapper! {
    pub struct RollitHistoryRow(ObjectSubclass<imp::RollitHistoryRow>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl RollitHistoryRow {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create RollitHistoryRow")
    }

    pub fn bind(&self, result_item: &RollitHistoryItem) {
        let imp = self.imp();
        
        let roll_result = imp.roll_result.get();
        let max_val = imp.max_suffix.get();
        let mut bindings = imp.bindings.borrow_mut();

        let title_binding = result_item
            .bind_property("result", &roll_result, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        bindings.push(title_binding);

        let subtitle_binding = result_item
            .bind_property("max-val", &max_val, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        bindings.push(subtitle_binding);
    }

    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }

    fn copy_result (&self) {
        let result = self.imp().roll_result.label();
        let clipboard = self.clipboard();
        clipboard.set_text(&result);

        self.activate_action("win.show-toast", Some(&(i18n("Result copied"), 0).to_variant())).unwrap();
    }
}

