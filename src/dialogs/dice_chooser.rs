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

use crate::dialogs::RollitTrayRow;
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
        pub dice_tray: TemplateChild<gtk::ListBox>,
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

            klass.install_action("dice.add-to-tray", None, move |dice, _, _| {
                dice.add_to_tray();
            });

            klass.install_action("dice.remove-from-tray", None, move |dice, _, _| {
                dice.remove_from_tray();
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RollitDiceChooser {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().bind_prefs();
            self.obj().load_tray();
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

    fn add_to_tray(&self) {
        let settings = utils::settings_manager();
        let mut tray_items = settings.strv("dice-tray");
        let current = self.imp().current_dice.value() as u32;

        if tray_items.contains(current.to_string()) {
            log::debug!("{} already in tray", current);
        } else {
            tray_items.push(current.to_string().into());
            settings.set_strv("dice-tray", tray_items);
            self.imp()
                .dice_tray
                .append(&RollitTrayRow::from_int(current));
            log::debug!("{} added to tray", current);
        }
    }

    fn remove_from_tray(&self) {
        let settings = utils::settings_manager();
        let mut tray_items = settings.strv("dice-tray");
        let current = self.imp().dice_tray.selected_row().unwrap();
        let val = current
            .clone()
            .downcast::<RollitTrayRow>()
            .unwrap()
            .dice_value();

        let mut i: usize = 0;
        for dice in &tray_items {
            if dice.to_string() == val.to_string() {
                break;
            } else {
                i += 1;
            }
        }

        tray_items.remove(i);
        self.imp().dice_tray.remove(&current);
        log::debug!("{} removed from tray", val);

        settings.set_strv("dice-tray", tray_items);
    }

    fn load_tray(&self) {
        let settings = utils::settings_manager();
        let tray_items: glib::StrV = settings.strv("dice-tray");

        log::debug!("Tray contents:");
        for dice in &tray_items {
            let dice_val = match dice.parse::<u32>() {
                Ok(val) => val,
                Err(e) => {
                    log::debug!("{}", e);
                    0
                }
            };

            let row = RollitTrayRow::from_int(dice_val);

            self.imp().dice_tray.append(&row);
            log::debug!("{}-sided dice", dice_val);
        }
    }

    fn bind_prefs(&self) {
        let imp = self.imp();
        let settings = utils::settings_manager();

        settings
            .bind("max-roll", imp.current_dice.deref(), "value")
            .build();
    }

    fn show_toast(&self, text: impl AsRef<str>, priority: adw::ToastPriority) {
        let imp = self.imp();

        let toast = adw::Toast::new(text.as_ref());
        toast.set_priority(priority);
        toast.set_timeout(1);

        imp.toast_overlay.add_toast(toast);
    }
}
