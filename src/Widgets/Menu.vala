/*  Copyright (C) 2020-2021 Patrick Csikos (https://zelikos.github.io)
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

    public signal void close_menu ();

    private Gtk.RadioButton six_sided;
    private Gtk.RadioButton ten_sided;
    private Gtk.RadioButton twenty_sided;

    private Gtk.RadioButton custom_sided;
    private Gtk.SpinButton max_entry;

    private Gtk.Popover menu_popover;

    public int max_roll { get; private set; }

    construct {
        six_sided = new Gtk.RadioButton.with_label (new SList<Gtk.RadioButton> (), _("d6"));
        ten_sided = new Gtk.RadioButton.with_label_from_widget (six_sided, _("d10"));
        twenty_sided = new Gtk.RadioButton.with_label_from_widget (six_sided, _("d20"));

        var six_sided_accel_label = new Gtk.Label (Granite.markup_accel_tooltip ({"<Ctrl>1"})) {
            halign = Gtk.Align.END,
            use_markup = true
        };

        var ten_sided_accel_label = new Gtk.Label (Granite.markup_accel_tooltip ({"<Ctrl>2"})) {
            halign = Gtk.Align.END,
            use_markup = true
        };
        var twenty_sided_accel_label = new Gtk.Label (Granite.markup_accel_tooltip ({"<Ctrl>3"})) {
            halign = Gtk.Align.END,
            use_markup = true
        };

        var presets = new Gtk.Grid () {
            column_homogeneous = true,
            row_spacing = 12,
            margin = 12
        };

        presets.attach (six_sided, 0, 0);
        presets.attach (six_sided_accel_label, 1, 0);
        presets.attach (ten_sided, 0, 1);
        presets.attach (ten_sided_accel_label, 1, 1);
        presets.attach (twenty_sided, 0, 2);
        presets.attach (twenty_sided_accel_label, 1, 2);

        custom_sided = new Gtk.RadioButton.from_widget (six_sided);

        max_entry = new Gtk.SpinButton.with_range (1, 100, 1) {
            sensitive = false
        };

        var custom_setting = new Gtk.Grid () {
            column_spacing = 6,
            margin = 12
        };

        custom_setting.attach (custom_sided, 0, 0);
        custom_setting.attach (max_entry, 1, 0);

        var separator = new Gtk.Separator (HORIZONTAL);

        var menu_grid = new Gtk.Grid ();

        menu_popover = new Gtk.Popover (this);

        menu_grid.attach (presets, 0, 0);
        menu_grid.attach (separator, 0, 1);
        menu_grid.attach (custom_setting, 0, 2);
        menu_grid.show_all ();

        load_max ();

        menu_popover.add (menu_grid);
        popover = menu_popover;

        label = max_roll.to_string();
        tooltip_text = _("Dice Settings");
        tooltip_markup = Granite.markup_accel_tooltip ({"<Ctrl>D"}, tooltip_text);

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

        close_menu.connect ( () => {
            popover.popdown ();
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

    public void shortcut_pressed (int shortcut) {
        switch (shortcut) {
            case 1:
                six_sided.clicked();
                break;
            case 2:
                ten_sided.clicked();
                break;
            case 3:
                twenty_sided.clicked();
                break;
            case 4:
                custom_sided.clicked();
                if (menu_popover.visible) {
                    max_entry.grab_focus();
                }
                break;
        }
    }
}
