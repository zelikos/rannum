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
use gettextrs::gettext;
use gtk::glib;
use gtk::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/dev/zelikos/rollit/gtk/dice-settings.ui")]
    pub struct RollitDiceSettings {
        #[template_child]
        pub dice_presets: TemplateChild<adw::PreferencesGroup>,
        #[template_child]
        pub max_roll: TemplateChild<gtk::SpinButton>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RollitDiceSettings {
        const NAME: &'static str = "RollitDiceSettings";
        type Type = super::RollitDiceSettings;
        type ParentType = adw::Window;
        // type ParentType = adw::PreferencesWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            // TODO: Move to dice_row.rs
            klass.install_action("dice.set-dice", None, move |dice, _, _| {
                dice.set_dice();
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RollitDiceSettings {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().bind_spinner();
        }
    }

    impl WidgetImpl for RollitDiceSettings {}
    impl WindowImpl for RollitDiceSettings {}
    impl AdwWindowImpl for RollitDiceSettings {}
    impl PreferencesWindowImpl for RollitDiceSettings {}
}

glib::wrapper! {
    pub struct RollitDiceSettings(ObjectSubclass<imp::RollitDiceSettings>)
        @extends gtk::Widget, gtk::Window, adw::Window,// adw::PreferencesWindow,
        @implements gtk::Accessible, gtk::Actionable;
}

impl RollitDiceSettings {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn bind_spinner(&self) {
        let settings = utils::settings_manager();
        settings
            .bind("max-roll", self.imp().max_roll.deref(), "value")
            .build();
    }

    // TODO: Move to dice_row.rs; add toast overlay for dice settings window
    fn set_dice(&self) {
        self.activate_action(
            "win.show-toast",
            Some(&(gettext("Dice set"), 0).to_variant()),
        )
        .unwrap();
    }
}
