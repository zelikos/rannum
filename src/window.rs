/*  Copyright (C) 2020-2022 Patrick Csikos (https://zelikos.github.io)
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
 * Authored by Patrick Csikos <zelikos@pm.me>
 */

use crate::deps::*;

use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::CompositeTemplate;

use crate::config;
use crate::utils;
use crate::widgets::{RollitMainView, RollitHistoryPane};


mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/gitlab/zelikos/rollit/gtk/window.ui")]
    pub struct RollitWindow {
        #[template_child]
        pub history_pane: TemplateChild<RollitHistoryPane>,
        #[template_child]
        pub main_view: TemplateChild<RollitMainView>,
        #[template_child]
        pub rollit_flap: TemplateChild<adw::Flap>,
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RollitWindow {
        const NAME: &'static str = "RollitWindow";
        type Type = super::RollitWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            // bind_template() is a function generated by the
            // CompositeTemplate macro to bind all children at once.
            Self::bind_template(klass);
            Self::Type::bind_template_callbacks(klass);

            // Set up actions
            klass.install_action("win.roll-dice", None, move |win, _, _| {
                win.roll_dice();
            });

            klass.install_action("win.clear-history", None, move |win, _, _| {
                win.clear_history();
            });

            klass.install_action("win.toggle-history", None, move |win, _, _| {
                win.toggle_history();
            });

            klass.install_action("win.show-toast", Some("(si)"), move |win, _, var| {
                if let Some((ref toast, i)) = var.and_then(|v| v.get::<(String, i32)>()) {
                    win.show_toast(toast, adw::ToastPriority::__Unknown(i));
                }
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RollitWindow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            if config::PROFILE == ".Devel" {
                obj.add_css_class("devel");
            }

            obj.setup_settings();

            // Set help overlay
            let builder = gtk::Builder::from_resource("/com/gitlab/zelikos/rollit/gtk/help-overlay.ui");
            let help_overlay = builder.object("help_overlay").unwrap();
            obj.set_help_overlay(Some(&help_overlay));

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
    pub fn new<A: glib::IsA<gtk::Application>>(app: &A) -> Self {
        glib::Object::new(&[("application", app)]).expect("Failed to create RollitWindow")
    }

    fn setup_settings(&self) {
        let settings = utils::settings_manager();
        settings.bind ("window-width", self, "default-width").build();
        settings.bind ("window-height", self, "default-height").build();
        settings.bind ("window-maximized", self, "maximized").build();
    }

    fn roll_dice(&self) {
        let roll_result = self.imp().main_view.get_roll_result();
        self.imp().history_pane.add_result(roll_result);
    }

    fn clear_history(&self) {
        self.imp().main_view.reset_label();
        self.imp().history_pane.clear_history();
    }

    fn toggle_history(&self) {
        let flap = &self.imp().rollit_flap;

        if flap.reveals_flap() {
            flap.set_reveal_flap(false);
        } else {
            flap.set_reveal_flap(true);
        }
    }

     fn show_toast(&self, text: impl AsRef<str>, priority: adw::ToastPriority) {
        let imp = self.imp();

        let toast = adw::Toast::new(text.as_ref());
        toast.set_priority(priority);

        imp.toast_overlay.add_toast(&toast);
    }}

