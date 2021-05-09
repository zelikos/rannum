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
 * Authored by Patrick Csikos <zelikos@pm.me>
 */

public class Rollit.Application : Gtk.Application {

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
        // var granite_settings = Granite.Settings.get_default ();

        gtk_settings.gtk_application_prefer_dark_theme = settings.get_boolean ("dark-style");

        // gtk_settings.gtk_application_prefer_dark_theme = granite_settings.prefers_color_scheme == Granite.Settings.ColorScheme.DARK;

        // granite_settings.notify["prefers-color-scheme"].connect (() => {
        //     gtk_settings.gtk_application_prefer_dark_theme = granite_settings.prefers_color_scheme == Granite.Settings.ColorScheme.DARK;
        // });

        var provider = new Gtk.CssProvider ();
        provider.load_from_resource ("/com/github/zelikos/rannum/styles/global.css");
        Gtk.StyleContext.add_provider_for_screen (
            Gdk.Screen.get_default (),
            provider,
            Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
        );

        var window = new Rollit.MainWindow (this);

        add_window (window);
    }

    public static int main (string[] args) {
        var app = new Application ();

        return app.run (args);
    }
}
