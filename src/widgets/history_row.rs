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

use std::cell::RefCell;

use adw::subclass::prelude::*;
use glib::{Binding, BindingFlags};
use gtk::glib;
use gtk::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/dev/zelikos/rollit/gtk/history-row.ui")]
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
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RollitHistoryRow {
        fn constructed(&self) {
            self.parent_constructed();
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
        glib::Object::new() //.expect("Failed to create RollitHistoryRow")
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
}
