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

use gettextrs::*;
use gtk::gio::{self, prelude::*};

mod application;
mod config;
mod i18n;
mod utils;
mod widgets;
mod window;

mod deps {
    pub use gtk::{gdk, gdk_pixbuf, gio, glib, graphene};
}

use application::RollitApplication;

fn main() {
    pretty_env_logger::init();

    // Translations
    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain("rollit", config::LOCALEDIR).unwrap();
    textdomain("rollit").unwrap();

    // Load resources
    let resources = gio::Resource::load(config::PKGDATADIR.to_owned() + "/rollit.gresource")
        .expect("Could not load resources");
    gio::resources_register(&resources);

    // Create Application
    let app = RollitApplication::new();

    // Run application.
    std::process::exit(app.run());
}
