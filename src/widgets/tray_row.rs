/*  Copyright (C) 2023-2024 Patrick Csikos (https://zelikos.dev)
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

use crate::models::RollitTrayItem;

use std::cell::RefCell;

use adw::subclass::prelude::*;
use glib::{Binding, BindingFlags};
use gtk::glib;
use gtk::prelude::*;

mod imp {
    use super::*;

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(resource = "/dev/zelikos/rollit/ui/dialogs/tray-row.ui")]
    pub struct RollitTrayRow {
        #[template_child]
        pub dice_value: TemplateChild<gtk::Label>,
        pub bindings: RefCell<Vec<Binding>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RollitTrayRow {
        const NAME: &'static str = "RollitTrayRow";
        type Type = super::RollitTrayRow;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RollitTrayRow {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for RollitTrayRow {}
    impl BinImpl for RollitTrayRow {}
}

glib::wrapper! {
    pub struct RollitTrayRow(ObjectSubclass<imp::RollitTrayRow>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl RollitTrayRow {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn bind(&self, tray_item: &RollitTrayItem) {
        let imp = self.imp();

        let dice_value = imp.dice_value.get();
        let mut bindings = imp.bindings.borrow_mut();

        let title_binding = tray_item
            .bind_property("dice-value", &dice_value, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        bindings.push(title_binding);
    }

    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
