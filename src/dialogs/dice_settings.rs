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

use adw::subclass::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/dev/zelikos/rollit/gtk/dice-settings.ui")]
    pub struct RollitDiceSettings {}

    #[glib::object_subclass]
    impl ObjectSubclass for RollitDiceSettings {
        const NAME: &'static str = "RollitDiceSettings";
        type Type = super::RollitDiceSettings;
        type ParentType = adw::Window;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RollitDiceSettings {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for RollitDiceSettings {}
    impl WindowImpl for RollitDiceSettings {}
    impl AdwWindowImpl for RollitDiceSettings {}
}

glib::wrapper! {
    pub struct RollitDiceSettings(ObjectSubclass<imp::RollitDiceSettings>)
        @extends gtk::Widget, gtk::Window, adw::Window,
        @implements gtk::Accessible, gtk::Actionable;
}

impl RollitDiceSettings {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new()
    }
}
