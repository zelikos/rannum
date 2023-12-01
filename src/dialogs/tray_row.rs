/*  Copyright (C) 2023 Patrick Csikos (https://zelikos.dev)
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

use crate::utils;

use core::ops::Deref;

use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

use std::cell::Cell;

mod imp {
    use super::*;

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(resource = "/dev/zelikos/rollit/ui/dialogs/tray-row.ui")]
    pub struct RollitTrayRow {
        pub dice_value: Cell<u32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RollitTrayRow {
        const NAME: &'static str = "RollitTrayRow";
        type Type = super::RollitTrayRow;
        type ParentType = adw::ActionRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            klass.install_action("row.set-dice", None, move |row, _, _| {
                row.set_max_roll();
            });
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
    impl ListBoxRowImpl for RollitTrayRow {}
    impl PreferencesRowImpl for RollitTrayRow {}
    impl ActionRowImpl for RollitTrayRow {}
}

glib::wrapper! {
    pub struct RollitTrayRow(ObjectSubclass<imp::RollitTrayRow>)
        @extends gtk::Widget, gtk::ListBoxRow, adw::PreferencesRow, adw::ActionRow,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable;
}

impl RollitTrayRow {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn dice_value(&self) -> u32 {
        self.imp().dice_value.get()
    }

    pub fn set_dice_value(&self, val: u32) {
        self.imp().dice_value.set(val);
    }

    pub fn set_max_roll(&self) {
        let settings = utils::settings_manager();
        settings.set_int("max-roll", self.imp().dice_value.get() as i32);
    }
}
