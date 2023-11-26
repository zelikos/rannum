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

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/dev/zelikos/rollit/ui/dialogs/dice-chooser.ui")]
    pub struct RollitDiceChooser {
        #[template_child]
        pub dice_tray: TemplateChild<adw::PreferencesGroup>,
        #[template_child]
        pub d6: TemplateChild<gtk::CheckButton>,
        #[template_child]
        pub d12: TemplateChild<gtk::CheckButton>,
        #[template_child]
        pub d20: TemplateChild<gtk::CheckButton>,
        #[template_child]
        pub current_dice: TemplateChild<adw::SpinRow>,
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RollitDiceChooser {
        const NAME: &'static str = "RollitDiceChooser";
        type Type = super::RollitDiceChooser;
        type ParentType = adw::Window;
        // type ParentType = adw::PreferencesWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            klass.install_action("dice.show-toast", Some("(si)"), move |dice, _, var| {
                if let Some((ref toast, i)) = var.and_then(|v| v.get::<(String, i32)>()) {
                    dice.show_toast(toast, adw::ToastPriority::__Unknown(i));
                }
            });

            // klass.install_action("dice.add-preset", None, move |dice, _, _| {
            //     dice.add_preset();
            // });

            // TODO: Move to dice_row.rs
            // klass.install_action("dice.set-dice", None, move |dice, _, _| {
            //     dice.set_dice();
            // });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RollitDiceChooser {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().bind_prefs();
        }
    }

    impl WidgetImpl for RollitDiceChooser {}
    impl WindowImpl for RollitDiceChooser {}
    impl AdwWindowImpl for RollitDiceChooser {}
    impl PreferencesWindowImpl for RollitDiceChooser {}
}

glib::wrapper! {
    pub struct RollitDiceChooser(ObjectSubclass<imp::RollitDiceChooser>)
        @extends gtk::Widget, gtk::Window, adw::Window,// adw::PreferencesWindow,
        @implements gtk::Accessible, gtk::Actionable;
}

impl RollitDiceChooser {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new()
    }

    // fn add_preset(&self) {
    //     TODO: Actually add presets
    //     let settings = utils::settings_manager();
    //     let max = settings.int("max-roll");
    //     log::debug!("{} added as a preset", max);
    //     self.activate_action(
    //         "dice.show-toast",
    //         Some(&(gettext("Preset added"), 1).to_variant()),
    //     )
    //     .unwrap();
    // }

    // TODO: Delete specified preset
    // fn del_preset(&self) {

    // }

    fn bind_prefs(&self) {
        let imp = self.imp();
        let settings = utils::settings_manager();

        settings
            .bind("max-roll", imp.max_roll.deref(), "value")
            .build();

        let current_val: i32 = imp.max_roll.value() as i32;

        match current_val {
            6 => imp.d6.set_active(true),
            12 => imp.d12.set_active(true),
            20 => imp.d20.set_active(true),
            _ => log::debug!("No presets match current value"),
        }

        imp.d6
            .connect_activate(glib::clone!(@weak self as pref => move |_| {
                pref.set_dice(6);
            }));

        imp.d12
            .connect_activate(glib::clone!(@weak self as pref => move |_| {
                pref.set_dice(12);
            }));

        imp.d20
            .connect_activate(glib::clone!(@weak self as pref => move |_| {
                pref.set_dice(20);
            }));
    }

    fn show_toast(&self, text: impl AsRef<str>, priority: adw::ToastPriority) {
        let imp = self.imp();

        let toast = adw::Toast::new(text.as_ref());
        toast.set_priority(priority);
        toast.set_timeout(1);

        imp.toast_overlay.add_toast(toast);
    }

    // TODO: Move to dice_row.rs
    fn set_dice(&self, sides: i32) {
        let settings = utils::settings_manager();
        settings.set_int("max-roll", sides).unwrap();
    }
}
