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
use crate::i18n::*;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::clone;
use gtk_macros::*;

use crate::config;
use crate::window::RollitWindow;


mod imp {
    use super::*;

    // Holds state and widgets
    #[derive(Debug, Default)]
    pub struct RollitApplication {}

    // Basics for GObject
    #[glib::object_subclass]
    impl ObjectSubclass for RollitApplication {
        const NAME: &'static str = "RollitApplication";
        type Type = super::RollitApplication;
        type ParentType = adw::Application;
    }

    // Overrides GObject vfuncs
    impl ObjectImpl for RollitApplication {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            obj.setup_gactions();
        }
    }

    // Overrides GApplication vfuncs
    impl ApplicationImpl for RollitApplication {
        fn activate(&self, application: &Self::Type) {
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = RollitWindow::new(application);
                window.upcast()
            };

            window.present();
        }
    }

    impl GtkApplicationImpl for RollitApplication {}
    impl AdwApplicationImpl for RollitApplication {}
}

glib::wrapper! {
    pub struct RollitApplication(ObjectSubclass<imp::RollitApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

#[allow(clippy::new_without_default)]
impl RollitApplication {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &config::APP_ID.to_string()),
            ("flags", &gio::ApplicationFlags::FLAGS_NONE),
            ("resource-base-path", &"/com/gitlab/zelikos/rollit/".to_string()),
        ])
        .expect("Failed to create RollitApplication")
    }

    fn setup_gactions(&self) {
        // action! is a macro from gtk_macros
        // that creates a GSimpleAction with a callback.
        // clone! is a macro from glib-rs that allows
        // you to easily handle references in callbacks
        // without refcycles or leaks.
        //
        // When you don't want the callback to keep the
        // Object alive, pass as @weak. Otherwise, pass
        // as @strong. Most of the time you will want
        // to use @weak.
        action!(
            self,
            "about",
            clone!(@weak self as app => move |_, _| {
                app.show_about();
            })
        );

        action!(
            self,
            "quit",
            clone!(@weak self as app => move |_, _| {
                app.quit();
            })
        );

        self.set_accels_for_action("app.quit", &["<Primary>Q"]);
        self.set_accels_for_action("win.roll-dice", &["<Primary>R"]);
        self.set_accels_for_action("win.clear-history", &["<Primary>L"]);
    }

    fn show_about(&self) {
        // Builders are a pattern that allow you to create
        // an object and set all relevant properties very
        // easily in a way that's idiomatic to Rust.
        let about = adw::AboutWindow::builder()
            .application_name(&i18n("Roll-It"))
            .application_icon(config::APP_ID)
            .version(config::VERSION)
            .developer_name(&i18n("Patrick \"zelikos\" Csikos"))
            .website("https://gitlab.com/zelikos/rollit")
            .issue_url("https://gitlab.com/zelikos/rollit/issues")
            .developers(vec![String::from(
                "Patrick Csikos <zelikos@pm.me>",
            )])
            .copyright(&i18n("Copyright © 2020-2022 Patrick Csikos"))
            .license_type(gtk::License::Gpl30)
            .build();

        if let Some(window) = self.active_window() {
            about.set_transient_for(Some(&window));
        }

        about.show();
    }
}
