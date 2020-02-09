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

public class Rollit.Menu : Gtk.Grid {

    Gtk.SpinButton max_entry;

    construct {
        var style_switch = new Granite.ModeSwitch.from_icon_name (
            "display-brightness-symbolic",
            "weather-clear-night-symbolic"
        );
        style_switch.primary_icon_tooltip_text = _("Light");
        style_switch.secondary_icon_tooltip_text = _("Dark");
        style_switch.halign = Gtk.Align.CENTER;
        style_switch.margin = 12;

        var gtk_settings = Gtk.Settings.get_default ();
        style_switch.bind_property ("active", gtk_settings, "gtk_application_prefer_dark_theme");
        Application.settings.bind ("dark-style", style_switch, "active", SettingsBindFlags.DEFAULT);

        var max_label = new Gtk.Label (_("Max Roll:"));
        max_label.margin_end = 12;
        max_entry = new Gtk.SpinButton.with_range (1, 100, 1);
        max_entry.value = 6; // to reflect a standard six-sided die

        var max_setting = new Gtk.Grid ();
        max_setting.orientation = Gtk.Orientation.HORIZONTAL;
        max_setting.margin = 12;
        max_setting.add (max_label);
        max_setting.add (max_entry);

        orientation = Gtk.Orientation.VERTICAL;
        //width_request = 200;
        add (style_switch);
        add (new Gtk.Separator (HORIZONTAL));
        add (max_setting);
        show_all ();
    }

    public int get_max_value () {
        var max_value = max_entry.get_value_as_int ();
        return max_value;
    }
}
