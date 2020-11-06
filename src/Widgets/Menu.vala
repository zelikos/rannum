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

public class Rollit.Menu : Gtk.MenuButton {

    public int max_roll { get; set; }

    private Gtk.Popover menu_popover;
    private Gtk.Grid menu_grid;

    private Gtk.RadioButton six_sided;
    private Gtk.RadioButton ten_sided;
    private Gtk.RadioButton twenty_sided;
    private Gtk.Box preset_box;

    private Gtk.Separator separator;
    private Gtk.RadioButton custom_sided;
    private Gtk.SpinButton max_entry;

    construct {
        six_sided = new Gtk.RadioButton.with_label (new SList<Gtk.RadioButton> (), _("d6"));
        ten_sided = new Gtk.RadioButton.with_label_from_widget (six_sided, _("d10"));
        twenty_sided = new Gtk.RadioButton.with_label_from_widget (six_sided, _("d20"));

        preset_box = new Gtk.Box (Gtk.Orientation.VERTICAL, 12) {
            margin = 12,
            margin_bottom = 6
        };

        preset_box.add (six_sided);
        preset_box.add (ten_sided);
        preset_box.add (twenty_sided);

        custom_sided = new Gtk.RadioButton.from_widget (six_sided) {
            margin_left = 12,
            margin_bottom = 6,
            margin_right = 6
        };

        max_entry = new Gtk.SpinButton.with_range (1, 100, 1) {
            margin = 12,
            margin_top = 6,
            margin_left = 0,
            sensitive = false
        };

        menu_grid = new Gtk.Grid () {
            orientation = Gtk.Orientation.VERTICAL
        };
        menu_grid.set_row_spacing (6);

        separator = new Gtk.Separator (HORIZONTAL);

        menu_popover = new Gtk.Popover (this);

        //  menu_grid.attach (six_sided, 0, 0, 2);
        //  menu_grid.attach (ten_sided, 0, 1, 2);
        //  menu_grid.attach (twenty_sided, 0, 2, 2);
        menu_grid.attach (preset_box, 0, 0, 2);
        menu_grid.attach (separator, 0, 1, 2);
        menu_grid.attach (custom_sided, 0, 2, 1);
        menu_grid.attach (max_entry, 1, 2, 1);
        menu_grid.show_all ();

        load_max ();

        menu_popover.add (menu_grid);
        popover = menu_popover;

        label = max_roll.to_string();
        get_style_context ().add_class (Gtk.STYLE_CLASS_SUGGESTED_ACTION);

        six_sided.clicked.connect ( () => {
            change_max (6, "d6");
        });

        ten_sided.clicked.connect ( () => {
            change_max (10, "d10");
        });

        twenty_sided.clicked.connect ( () => {
            change_max (20, "d20");
        });

        custom_sided.clicked.connect ( () => {
            change_max (max_entry.get_value_as_int ());
        });

        max_entry.value_changed.connect ( () => {
            change_max (max_entry.get_value_as_int ());
        });
    }

    private void load_max () {
        var custom_roll = Application.settings.get_int ("custom-roll");
        var selection = Application.settings.get_string ("last-selected");

        switch (selection) {
            case "d6":
                six_sided.active = true;
                max_roll = 6;
                break;
            case "d10":
                ten_sided.active = true;
                max_roll = 10;
                break;
            case "d20":
                twenty_sided.active = true;
                max_roll = 20;
                break;
            default:
                custom_sided.active = true;
                max_roll = custom_roll;
                max_entry.sensitive = true;
                break;
        }
        max_entry.value = custom_roll;
    }

    private void change_max (int roll, string selection = "custom") {
        max_roll = roll;
        Application.settings.set_string ("last-selected", selection);
        if (selection != "custom") {
            max_entry.sensitive = false;
        } else if (selection == "custom") {
            Application.settings.set_int ("custom-roll", roll);
            max_entry.sensitive = true;
        }
        label = max_roll.to_string();
    }
}
