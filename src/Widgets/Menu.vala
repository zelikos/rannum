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

    public int max_roll { get; set; }
    private Gtk.RadioButton six_sided;
    private Gtk.RadioButton ten_sided;
    private Gtk.RadioButton twenty_sided;
    private Gtk.RadioButton custom_sided;
    private Gtk.SpinButton max_entry;

    construct {
        six_sided = new Gtk.RadioButton.with_label (new SList<Gtk.RadioButton> (), _("d6"));
        ten_sided = new Gtk.RadioButton.with_label_from_widget (six_sided, _("d10"));
        twenty_sided = new Gtk.RadioButton.with_label_from_widget (six_sided, _("d20"));
        custom_sided = new Gtk.RadioButton.with_label_from_widget (six_sided, _("Custom"));
        six_sided.margin = ten_sided.margin = twenty_sided.margin = custom_sided.margin = 6;

        max_entry = new Gtk.SpinButton.with_range (1, 100, 1) {
            margin = 6,
            margin_top = 0
        };

        orientation = Gtk.Orientation.VERTICAL;
        margin = 6;

        add (six_sided);
        add (ten_sided);
        add (twenty_sided);
        add (new Gtk.Separator (HORIZONTAL));
        add (custom_sided);
        add (max_entry);

        load_max ();

        six_sided.clicked.connect ( () => {
            save_max (6);
        });

        ten_sided.clicked.connect ( () => {
            save_max (10);
        });

        twenty_sided.clicked.connect ( () => {
            save_max (20);
        });

        custom_sided.clicked.connect ( () => {
            save_max (max_entry.get_value_as_int (), true);
        });

        max_entry.value_changed.connect ( () => {
            save_max (max_entry.get_value_as_int (), true);
        });

        show_all ();
    }

    private void load_max () {
        max_roll = Application.settings.get_int ("max-roll");
        max_entry.sensitive = false;

        switch (max_roll) {
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
                max_entry.value = max_roll;
                max_entry.sensitive = true;
                break;
        }
    }

    private void save_max (int roll, bool custom = false) {
        max_roll = roll;
        Application.settings.set_int ("max-roll", max_roll);
        if (!custom) {
            max_entry.sensitive = false;
        } else if (custom) {
            max_entry.sensitive = true;
        }
    }
}
