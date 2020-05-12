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
        var six_sided = new Gtk.RadioButton.with_label (new SList<Gtk.RadioButton> (), _("d6"));
        var ten_sided = new Gtk.RadioButton.with_label (six_sided.get_group (), _("d10"));
        var twenty_sided = new Gtk.RadioButton.with_label (six_sided.get_group (), _("d20"));
        var custom_sided = new Gtk.RadioButton.with_label (six_sided.get_group (), _("Custom"));
        six_sided.margin = ten_sided.margin = twenty_sided.margin = custom_sided.margin = 6;
        
        /*
        var max_label = new Gtk.Label (_("Max Roll:"));
        max_label.margin_end = 12;*/
        max_entry = new Gtk.SpinButton.with_range (1, 100, 1);
        max_entry.margin = 6;
        max_entry.margin_top = 0;

        // Read last value from settings (and save in settings when changed
        max_entry.value = Application.settings.get_int("max-roll");
        max_entry.value_changed.connect( () => {
            Application.settings.set_int("max-roll", max_entry.get_value_as_int());
        });

        var max_setting = new Gtk.Grid ();
        max_setting.orientation = Gtk.Orientation.VERTICAL;
        max_setting.margin = 6;
        //max_setting.add (max_label);
        max_setting.add (six_sided);
        max_setting.add (ten_sided);
        max_setting.add (twenty_sided);
        max_setting.add (custom_sided);
        max_setting.add (max_entry);


        orientation = Gtk.Orientation.VERTICAL;
        // add (style_switch);
        add (new Gtk.Separator (HORIZONTAL));
        add (max_setting);
        show_all ();
    }

    public int get_max_value () {
        var max_value = max_entry.get_value_as_int ();
        return max_value;
    }
}
