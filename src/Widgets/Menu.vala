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

public class Rollit.Menu : Gtk.Grid {

    construct {
        var six_sided = new Gtk.RadioButton.with_label (new SList<Gtk.RadioButton> (), _("d6"));
        var ten_sided = new Gtk.RadioButton.with_label (six_sided.get_group (), _("d10"));
        var twenty_sided = new Gtk.RadioButton.with_label (six_sided.get_group (), _("d20"));
        var custom_sided = new Gtk.RadioButton.with_label (six_sided.get_group (), _("Custom"));
        six_sided.margin = ten_sided.margin = twenty_sided.margin = custom_sided.margin = 6;
        

        var max_entry = new Gtk.SpinButton.with_range (1, 100, 1);
        max_entry.margin = 6;
        max_entry.margin_top = 0;

        // Read last value from settings
        int last_value = Application.settings.get_int ("max-roll");
        
        switch (last_value) {
            case 6:
                six_sided.active = true;
                break;
            case 10:
                ten_sided.active = true;
                break;
            case 20:
                twenty_sided.active = true;
                break;
            default:
                custom_sided.active = true;
                max_entry.value = last_value;
                break;
        }

        orientation = Gtk.Orientation.VERTICAL;
        margin = 6;
        add (six_sided);
        add (ten_sided);
        add (twenty_sided);
        add (new Gtk.Separator (HORIZONTAL));
        add (custom_sided);
        add (max_entry);
        show_all ();

        
        six_sided.clicked.connect ( () => {
            Application.settings.set_int ("max-roll", 6);
        });
        
        ten_sided.clicked.connect ( () => {
            Application.settings.set_int ("max-roll", 10);
        });
        
        twenty_sided.clicked.connect ( () => {
            Application.settings.set_int ("max-roll", 20);
        });
        
        custom_sided.clicked.connect ( () => {
            Application.settings.set_int ("max-roll", max_entry.get_value_as_int ());
        });
        
        max_entry.value_changed.connect ( () => {
            Application.settings.set_int ("max-roll", max_entry.get_value_as_int ());
            custom_sided.active = true;
        });
    }
}
