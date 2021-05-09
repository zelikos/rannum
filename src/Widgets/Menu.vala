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

    private SList<Gtk.CheckButton> dice_selection;

    private Rollit.MenuItem six_sided;
    private Rollit.MenuItem ten_sided;
    private Rollit.MenuItem twenty_sided;

    private Gtk.CheckButton custom_sided;
    private Gtk.SpinButton max_entry;

    private Gtk.Popover menu_popover;

    public int max_roll { get; private set; }

    construct {
        dice_selection = new SList<Gtk.CheckButton> ();

        six_sided = new Rollit.MenuItem ("d6", "<Ctrl>1");
        ten_sided = new Rollit.MenuItem ("d10", "<Ctrl>2");
        twenty_sided = new Rollit.MenuItem ("d20", "<ctrl>3");

        var presets = new Gtk.Box (VERTICAL, 6) {
            margin_start = 6,
            margin_end = 6,
            margin_top = 6,
            margin_bottom = 0
        };

        presets.append (six_sided);
        presets.append (ten_sided);
        presets.append (twenty_sided);

        max_entry = new Gtk.SpinButton.with_range (1, 100, 1) {
            sensitive = false
        };

        custom_sided = new Gtk.CheckButton ();
        six_sided.dice_radio.set_group (custom_sided);
        ten_sided.dice_radio.set_group (custom_sided);
        twenty_sided.dice_radio.set_group (custom_sided);

        var custom_setting = new Gtk.Box (HORIZONTAL, 6) {
            margin_start = 12,
            margin_end = 12,
            margin_bottom = 12,
            margin_top = 6
        };

        custom_setting.append (custom_sided);
        custom_setting.append (max_entry);

        var separator = new Gtk.Separator (HORIZONTAL);

        var menu_box = new Gtk.Box (VERTICAL, 6);

        menu_box.append (presets);
        menu_box.append (separator);
        menu_box.append (custom_setting);
        // menu_box.show_all ();



        load_max ();

        menu_popover = new Gtk.Popover ();
        menu_popover.set_child (menu_box);
        popover = menu_popover;

        label = max_roll.to_string();
        tooltip_text = _("Dice settings");
        // tooltip_markup = Granite.markup_accel_tooltip ({"<Ctrl>D"}, tooltip_text);
        tooltip_markup = ("Ctrl+D");

        six_sided.clicked.connect ( () => {
            change_max (6, "d6");
        });

        ten_sided.clicked.connect ( () => {
            change_max (10, "d10");
        });

        twenty_sided.clicked.connect ( () => {
            change_max (20, "d20");
        });

        custom_sided.toggled.connect ( () => {
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
                six_sided.dice_radio.active = true;
                max_roll = 6;
                break;
            case "d10":
                ten_sided.dice_radio.active = true;
                max_roll = 10;
                break;
            case "d20":
                twenty_sided.dice_radio.active = true;
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
                custom_sided.toggled();
                if (menu_popover.visible) {
                    max_entry.grab_focus();
                }
                break;
        }
    }
}
