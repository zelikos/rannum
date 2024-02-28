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

use crate::utils;
use crate::widgets::RollitTrayRow;

use core::ops::Deref;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gio::glib::VariantTy;
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
        #[template_child]
        pub reset_button: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RollitDiceChooser {
        const NAME: &'static str = "RollitDiceChooser";
        type Type = super::RollitDiceChooser;
        type ParentType = adw::Dialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            klass.install_action(
                "dice.show-toast",
                Some(VariantTy::new("(si)").unwrap()),
                move |dice, _, var| {
                    if let Some((ref toast, i)) = var.and_then(|v| v.get::<(String, i32)>()) {
                        dice.show_toast(toast, adw::ToastPriority::__Unknown(i));
                    }
                },
            );

            klass.install_action("dice.add-to-tray", None, move |dice, _, _| {
                dice.add_to_tray();
            });

            // klass.install_action("dice.remove-from-tray", None, move |dice, _, _| {
            //     dice.remove_from_tray();
            // });

            klass.install_action("dice.reset-tray", None, move |dice, _, _| {
                dice.show_reset_dialog();
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

        fn dispose(&self) {
            self.obj().save_tray();
        }
    }

    impl WidgetImpl for RollitDiceChooser {}
    impl AdwDialogImpl for RollitDiceChooser {}
}

glib::wrapper! {
    pub struct RollitDiceChooser(ObjectSubclass<imp::RollitDiceChooser>)
        @extends gtk::Widget, adw::Dialog,
        @implements gtk::Accessible, gtk::Actionable;
}

impl RollitDiceChooser {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn add_to_tray(&self) {
        let tray_items = self.tray_items();
        let current = self.imp().current_dice.value() as u32;
        let mut found: bool = false;

        for item in tray_items {
            if item.downcast::<RollitTrayRow>().unwrap().dice_value() == current {
                found = true;
                break;
            }
        }

        if found {
            log::debug!("{} already in tray", current);
        } else {
            log::debug!("{} added to tray", current);
            self.imp()
                .dice_tray
                .append(&RollitTrayRow::from_int(current));
        }
    }

    // fn remove_from_tray(&self) {
    //     let settings = utils::settings_manager();
    //     let mut tray_items = settings.strv("dice-tray");
    //     if let Some(current) = self.imp().dice_tray.selected_row() {
    //         let val = current
    //             .clone()
    //             .downcast::<RollitTrayRow>()
    //             .unwrap()
    //             .dice_value();

    //         let mut i: usize = 0;
    //         for dice in &tray_items {
    //             if dice.to_string() == val.to_string() {
    //                 break;
    //             } else {
    //                 i += 1;
    //             }
    //         }

    //         tray_items.remove(i);
    //         self.imp().dice_tray.remove(&current);
    //         log::debug!("{} removed from tray", val);

    //         let _ = settings.set_strv("dice-tray", tray_items);
    //     }
    // }

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

    fn save_tray(&self) {
        let tray_items = self.tray_items();
        let mut saved_tray: glib::StrV = glib::StrV::new();

        for item in tray_items {
            let val = item.downcast::<RollitTrayRow>().unwrap().dice_value();
            saved_tray.push(val.to_string().into());
        }

        let _ = utils::settings_manager().set_strv("dice-tray", saved_tray);
        log::debug!("Tray items saved");
    }

    fn tray_items(&self) -> Vec<gtk::ListBoxRow> {
        let imp = self.imp();

        imp.dice_tray
            .set_selection_mode(gtk::SelectionMode::Multiple);
        imp.dice_tray.select_all();

        let tray_items = imp.dice_tray.selected_rows();

        imp.dice_tray.unselect_all();
        imp.dice_tray.set_selection_mode(gtk::SelectionMode::Single);

        tray_items
    }

    fn show_reset_dialog(&self) {
        let dialog = adw::AlertDialog::new(
            Some(&gettext("Reset Dice Tray?")),
            Some(&gettext(
                "This will remove all added dice, and restore any original dice that were removed.",
            )),
        );

        dialog.add_response("cancel", &gettext("_Cancel"));
        dialog.add_response("reset", &gettext("_Reset"));
        dialog.set_response_appearance("reset", adw::ResponseAppearance::Destructive);
        dialog.set_default_response(Some("cancel"));

        dialog.connect_response(
            Some("reset"),
            glib::clone!(@weak self as chooser => move |_,_| {
                chooser.reset_tray();
            }),
        );

        dialog.present(self);
    }

    fn reset_tray(&self) {
        let settings = utils::settings_manager();
        settings.reset("dice-tray");
        self.imp().dice_tray.remove_all();
        self.load_tray();
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
