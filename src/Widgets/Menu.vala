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
        six_sided = new Gtk.RadioButton (new SList<Gtk.RadioButton> ());
        var six_sided_accel_label = new Granite.AccelLabel ("d6", "<Ctrl>1");

        var six_button = new Gtk.Button ();
        six_button.get_style_context ().add_class (Gtk.STYLE_CLASS_FLAT);
        six_button.add (six_sided_accel_label);
        six_button.clicked.connect ( () => {
            six_sided.clicked ();
        });

        ten_sided = new Gtk.RadioButton.from_widget (six_sided);
        var ten_sided_accel_label = new Granite.AccelLabel ("d10", "<Ctrl>2");

        var ten_button = new Gtk.Button ();
        ten_button.get_style_context ().add_class (Gtk.STYLE_CLASS_FLAT);
        ten_button.add (ten_sided_accel_label);
        ten_button.clicked.connect ( () => {
            ten_sided.clicked ();
        });

        twenty_sided = new Gtk.RadioButton.from_widget (six_sided);
        var twenty_sided_accel_label = new Granite.AccelLabel ("d20", "<Ctrl>3");

        var twenty_button = new Gtk.Button ();
        twenty_button.get_style_context ().add_class (Gtk.STYLE_CLASS_FLAT);
        twenty_button.add (twenty_sided_accel_label);
        twenty_button.clicked.connect ( () => {
            twenty_sided.clicked ();
        });

        // var six_sided_accel_label = new Gtk.Label (Granite.markup_accel_tooltip ({"<Ctrl>1"})) {
        //     halign = Gtk.Align.END,
        //     use_markup = true
        // };
        // six_sided_accel_label.get_style_context ().add_class (Granite.STYLE_CLASS_KEYCAP);
        // var ten_sided_accel_label = new Gtk.Label (Granite.markup_accel_tooltip ({"<Ctrl>2"})) {
        //     halign = Gtk.Align.END,
        //     use_markup = true
        // };
        // var twenty_sided_accel_label = new Gtk.Label (Granite.markup_accel_tooltip ({"<Ctrl>3"})) {
        //     halign = Gtk.Align.END,
        //     use_markup = true
        // };

        var presets = new Gtk.Grid () {
            column_spacing = 6,
            row_spacing = 6,
            margin = 12,
            margin_bottom = 0
        };

        presets.attach (six_sided, 0, 0);
        presets.attach (six_button, 1, 0);
        presets.attach (ten_sided, 0, 1);
        presets.attach (ten_button, 1, 1);
        presets.attach (twenty_sided, 0, 2);
        presets.attach (twenty_button, 1, 2);

        // var presets = new Gtk.Box (VERTICAL, 12);
        // presets.add (six_button);
        // presets.add (ten_sided);
        // presets.add (twenty_sided);

        custom_sided = new Gtk.RadioButton.from_widget (six_sided);

        max_entry = new Gtk.SpinButton.with_range (1, 100, 1) {
            sensitive = false
        };

        var custom_setting = new Gtk.Box (HORIZONTAL, 6) {
            margin = 12,
            margin_top = 0
        };

        custom_setting.pack_start (custom_sided);
        custom_setting.pack_end (max_entry);

        var separator = new Gtk.Separator (HORIZONTAL);

        var menu_box = new Gtk.Box (VERTICAL, 12);

        menu_box.add (presets);
        menu_box.add (separator);
        menu_box.add (custom_setting);
        menu_box.show_all ();

        load_max ();

        menu_popover = new Gtk.Popover (this);
        menu_popover.add (menu_box);
        popover = menu_popover;

        label = max_roll.to_string();
        tooltip_text = _("Dice settings");
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
