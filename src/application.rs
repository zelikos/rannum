/*  Copyright (C) 2020-2023 Patrick Csikos (https://zelikos.github.io)
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

use log::{debug, info};

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::config::{APP_ID, PKGDATADIR, PROFILE, VERSION};
use crate::window::RollitWindow;

mod imp {
    use super::*;
    use glib::WeakRef;
    use once_cell::sync::OnceCell;

    // Holds state and widgets
    #[derive(Debug, Default)]
    pub struct RollitApplication {
        pub window: OnceCell<WeakRef<RollitWindow>>,
    }

    // Basics for GObject
    #[glib::object_subclass]
    impl ObjectSubclass for RollitApplication {
        const NAME: &'static str = "RollitApplication";
        type Type = super::RollitApplication;
        type ParentType = adw::Application;
    }

    // Overrides GObject vfuncs
    impl ObjectImpl for RollitApplication {}

    // Overrides GApplication vfuncs
    impl ApplicationImpl for RollitApplication {
        fn activate(&self) {
            debug!("GtkApplication<RollitApplication>::activate");
            self.parent_activate();
            let app = self.obj();

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.present();
                return;
            }

            let window = RollitWindow::new(&app);
            self.window
                .set(window.downgrade())
                .expect("Window already set.");

            app.main_window().present();
        }

        fn startup(&self) {
            debug!("GtkApplication<RollitApplication>::startup");
            self.parent_startup();
            let app = self.obj();

            // Set icons for shell
            gtk::Window::set_default_icon_name(APP_ID);

            app.setup_gactions();
            app.setup_accels();
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
    fn main_window(&self) -> RollitWindow {
        self.imp().window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        // Quit
        let action_quit = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| {
                // This is needed to trigger the delete event and saving the window state
                app.main_window().close();
                app.quit();
            })
            .build();

        // About
        let action_about = gio::ActionEntry::builder("about")
            .activate(|app: &Self, _, _| {
                app.show_about();
            })
            .build();
        self.add_action_entries([action_quit, action_about]);
    }

    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<Primary>Q"]);
        self.set_accels_for_action("win.roll-dice", &["<Primary>R"]);
        self.set_accels_for_action("win.clear-history", &["<Primary>L"]);
        self.set_accels_for_action("win.undo-clear", &["<Primary>Z"]);
        self.set_accels_for_action("win.toggle-history", &["<Primary>H"]);
        self.set_accels_for_action("win.copy-latest", &["<Primary>C"]);
    }

    fn show_about(&self) {
        let builder = gtk::Builder::from_resource("/dev/zelikos/rollit/gtk/about.ui");

        let about: adw::AboutWindow = builder.object("about_window").unwrap();
        about.set_application_icon(APP_ID);
        about.set_version(VERSION);

        if let Some(window) = self.active_window() {
            about.set_transient_for(Some(&window));
        }

        about.present();
    }

    pub fn run(&self) -> glib::ExitCode {
        info!("Roll-It ({})", APP_ID);
        info!("Version: {} ({})", VERSION, PROFILE);
        info!("Datadir: {}", PKGDATADIR);

        ApplicationExtManual::run(self)
    }
}

impl Default for RollitApplication {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", APP_ID)
            .property("resource-base-path", "/dev/zelikos/rollit/")
            .build()
    }
}
