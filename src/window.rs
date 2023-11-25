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

use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::prelude::*;
use gtk::{gio, glib};

use crate::application::RollitApplication;
use crate::config::PROFILE;
use crate::dialogs::RollitDiceChooser;
use crate::utils;
use crate::widgets::{RollitHistoryPane, RollitMainView};

mod imp {
    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/dev/zelikos/rollit/ui/window.ui")]
    pub struct RollitWindow {
        #[template_child]
        pub dice_chooser_label: TemplateChild<adw::ButtonContent>,
        #[template_child]
        pub history_pane: TemplateChild<RollitHistoryPane>,
        #[template_child]
        pub main_view: TemplateChild<RollitMainView>,
        #[template_child]
        pub rollit_split_view: TemplateChild<adw::OverlaySplitView>,
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
    }

    impl Default for RollitWindow {
        fn default() -> Self {
            Self {
                dice_chooser_label: TemplateChild::default(),
                history_pane: TemplateChild::default(),
                main_view: TemplateChild::default(),
                rollit_split_view: TemplateChild::default(),
                toast_overlay: TemplateChild::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RollitWindow {
        const NAME: &'static str = "RollitWindow";
        type Type = super::RollitWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            // Set up actions
            klass.install_action("win.roll-dice", None, move |win, _, _| {
                win.roll_dice();
            });

            klass.install_action("win.clear-history", None, move |win, _, _| {
                win.clear_history();
            });

            klass.install_action("win.copy-latest", None, move |win, _, _| {
                win.imp().main_view.copy_latest();
            });

            klass.install_action("win.undo-clear", None, move |win, _, _| {
                win.undo_clear();
            });

            klass.install_action("win.toggle-history", None, move |win, _, _| {
                win.toggle_history();
            });

            klass.install_action("win.dice-chooser", None, move |win, _, _| {
                win.show_dice_chooser();
            });

            klass.install_action("win.show-toast", Some("(si)"), move |win, _, var| {
                if let Some((ref toast, i)) = var.and_then(|v| v.get::<(String, i32)>()) {
                    win.show_toast(toast, adw::ToastPriority::__Unknown(i));
                }
            });
        }
        // You must call `Widget`'s `init_template()` within `instance_init()`
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RollitWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            // Devel Profile
            if PROFILE == "Devel" {
                obj.add_css_class("devel");
            }

            obj.setup_actions();
            obj.setup_settings();
        }
    }
    impl WidgetImpl for RollitWindow {}
    impl WindowImpl for RollitWindow {}
    impl ApplicationWindowImpl for RollitWindow {}
    impl AdwApplicationWindowImpl for RollitWindow {}
}

glib::wrapper! {
    pub struct RollitWindow(ObjectSubclass<imp::RollitWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Root;
}

#[gtk::template_callbacks]
impl RollitWindow {
    pub fn new(app: &RollitApplication) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    fn setup_actions(&self) {
        self.action_set_enabled("win.clear-history", false);
        self.action_set_enabled("win.undo-clear", false);
        self.action_set_enabled("win.copy-latest", false);
    }

    fn setup_settings(&self) {
        let settings = utils::settings_manager();
        settings.bind("window-width", self, "default-width").build();
        settings
            .bind("window-height", self, "default-height")
            .build();
        settings.bind("window-maximized", self, "maximized").build();

        let val = settings.int("max-roll");
        self.imp().dice_chooser_label.set_label(&val.to_string());
    }

    fn roll_dice(&self) {
        let roll_result = self.imp().main_view.get_roll_result();
        self.imp().history_pane.add_result(roll_result);

        self.action_set_enabled("win.clear-history", true);
        self.action_set_enabled("win.undo-clear", false);
        self.action_set_enabled("win.copy-latest", true);
    }

    fn clear_history(&self) {
        let imp = self.imp();

        imp.history_pane.hide_history();
        imp.main_view.hide_label();
        self.undo_toast();

        self.action_set_enabled("win.clear-history", false);
        self.action_set_enabled("win.undo-clear", true);
        self.action_set_enabled("win.copy-latest", false);
    }

    fn undo_clear(&self) {
        let imp = self.imp();

        imp.history_pane.show_history();
        imp.main_view.show_label();

        self.action_set_enabled("win.clear-history", true);
        self.action_set_enabled("win.undo-clear", false);
        self.action_set_enabled("win.copy-latest", true);
    }

    fn toggle_history(&self) {
        let split_view = &self.imp().rollit_split_view;

        if split_view.shows_sidebar() {
            split_view.set_show_sidebar(false);
        } else {
            split_view.set_show_sidebar(true);
        }
    }

    fn show_dice_chooser(&self) {
        let dice_chooser = RollitDiceChooser::new();

        dice_chooser.set_transient_for(Some(self));

        dice_chooser.connect_destroy(glib::clone!(@weak self as win => move |_| {
            let settings = utils::settings_manager();
            let val = settings.int("max-roll");
            win.imp().dice_chooser_label.set_label(&val.to_string());
        }));

        dice_chooser.present();
    }

    fn show_toast(&self, text: impl AsRef<str>, priority: adw::ToastPriority) {
        let imp = self.imp();

        let toast = adw::Toast::new(text.as_ref());
        toast.set_priority(priority);
        toast.set_timeout(1);

        imp.toast_overlay.add_toast(toast);
    }

    fn undo_toast(&self) {
        let imp = self.imp();

        let toast = adw::Toast::new(&gettext("Results cleared"));
        toast.set_button_label(Some(&gettext("Undo")));
        toast.set_action_name(Some("win.undo-clear"));
        toast.set_priority(adw::ToastPriority::High);

        imp.toast_overlay.add_toast(toast);
    }
}
