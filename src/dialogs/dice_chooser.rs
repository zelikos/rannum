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
use crate::utils;
use crate::widgets::RollitTrayRow;

use core::ops::Deref;

use adw::subclass::prelude::*;
use gio::glib::VariantTy;
use gtk::prelude::*;
use gtk::{gio, glib, ListItem};
use gtk::{SignalListItemFactory, SingleSelection};
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/dev/zelikos/rollit/ui/dialogs/dice-chooser.ui")]
    pub struct RollitDiceChooser {
        #[template_child]
        pub dice_tray: TemplateChild<gtk::ListView>,
        #[template_child]
        pub current_dice: TemplateChild<adw::SpinRow>,
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
        pub tray_items: RefCell<Option<gio::ListStore>>,
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
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RollitDiceChooser {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.setup_tray();
            obj.setup_factory();

            obj.bind_prefs();
            obj.load_tray();
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
        let settings = utils::settings_manager();
        let current = self.imp().current_dice.value() as u32;

        if self.check_duplicate(&current) {
            log::debug!("{} already in tray", current);
        } else {
            let tray_item = RollitTrayItem::new(current);
            self.tray_items().append(&tray_item);
            log::debug!("{} added to tray", current);
        }
    }

    fn check_duplicate(&self, current: &u32) -> bool {
        true
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

    fn setup_tray(&self) {
        let imp = self.imp();
        let model = gio::ListStore::new::<RollitTrayItem>();

        imp.tray_items.replace(Some(model));

        let selection_model = SingleSelection::new(Some(self.tray_items()));
        imp.dice_tray.set_model(Some(&selection_model));

        imp.dice_tray.connect_activate(
            glib::clone!(@weak self as tray, @weak selection_model => move |_, pos| {
                let dice = selection_model
                    .upcast::<gio::ListModel>()
                    .item(pos)
                    .unwrap()
                    .downcast::<RollitTrayItem>()
                    .unwrap();

                let val: i32 = dice.property("dice-value");

                let settings = utils::settings_manager();
                let _ = settings.set_int("max-roll", val);
            }),
        );
    }

    fn setup_factory(&self) {
        let factory = SignalListItemFactory::new();

        // Connect empty 'RollitTrayRow' during setup
        factory.connect_setup(move |_, list_item| {
            let tray_row = RollitTrayRow::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&tray_row));
        });

        // Tell factory how to bind 'RollitTrayRow' to 'RollitTrayItem'
        factory.connect_bind(move |_, list_item| {
            let tray_item = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<RollitTrayItem>()
                .expect("The item must be a 'RollitTrayItem'.");

            let tray_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<RollitTrayRow>()
                .expect("The child must be a 'RollitTrayRow'.");

            tray_row.bind(&tray_item);
        });

        // Tell factory how to unbind 'RollitTrayRow' from 'RollitTrayItem'
        factory.connect_unbind(move |_, list_item| {
            // Get 'RollitTrayRow' from 'ListItem'
            let tray_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<RollitTrayRow>()
                .expect("The child must be a 'RollitTrayRow'.");

            tray_row.unbind();
        });

        self.imp().dice_tray.set_factory(Some(&factory));
    }

    fn tray_items(&self) -> gio::ListStore {
        self.imp()
            .tray_items
            .borrow()
            .clone()
            .expect("Could not retrieve tray items.")
    }

    fn load_tray(&self) {
        let settings = utils::settings_manager();
        let saved_items: glib::StrV = settings.strv("dice-tray");

        log::debug!("Tray contents:");
        for dice in &saved_items {
            let dice_val = match dice.parse::<u32>() {
                Ok(val) => val,
                Err(e) => {
                    log::debug!("{}", e);
                    0
                }
            };

            let row = RollitTrayItem::new(dice_val);

            self.tray_items().append(&row);
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
