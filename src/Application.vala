/*  Copyright (C) 2020 Patrick Csikos (https://zelikos.github.io)
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
 * Authored by Patrick Csikos <akzeldev@fastmail.com>
 */

public class Application : Gtk.Application {

    public static GLib.Settings settings;
    static construct {
        settings = new GLib.Settings ("com.github.zelikos.rannum");
    }

    public Application () {
        Object (
            application_id: "com.github.zelikos.rannum",
            flags: ApplicationFlags.FLAGS_NONE
        );
    }

    protected override void activate () {
        var gtk_settings = Gtk.Settings.get_default ();
        gtk_settings.gtk_application_prefer_dark_theme = settings.get_boolean ("dark-style");

        var provider = new Gtk.CssProvider ();
        provider.load_from_resource ("/com/github/zelikos/rannum/styles/global.css");
        Gtk.StyleContext.add_provider_for_screen (
            Gdk.Screen.get_default (),
            provider,
            Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
        );

        var window = new Rollit.Window (this);

        add_window (window);
    }
}
