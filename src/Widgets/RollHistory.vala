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

public class Rollit.RollHistory : Gtk.Grid {

    private GLib.List<PreviousRoll> previous_rolls_list;
    private Gtk.ScrolledWindow scroll_box;
    private Gtk.ListBox previous_rolls_box;

    public Gtk.Button clear_button;

    construct {
        previous_rolls_box = new Gtk.ListBox () {
            hexpand = true,
            vexpand = true
        };

        scroll_box = new Gtk.ScrolledWindow (null, null);
        scroll_box.hscrollbar_policy = NEVER;
        scroll_box.add (previous_rolls_box);

        var clear_text = new Gtk.Label (_("Clear"));
        var clear_icon = new Gtk.Image.from_icon_name ("edit-clear-all-symbolic", Gtk.IconSize.SMALL_TOOLBAR);

        var bottom_row = new Gtk.Box (HORIZONTAL, 32);
        bottom_row.pack_start (clear_text);
        bottom_row.pack_end (clear_icon);
        bottom_row.margin = 6;

        clear_button = new Gtk.Button () {
            tooltip_markup = Granite.markup_accel_tooltip ({"<Alt>C"}, _("Clear History"))
        };
        clear_button.get_style_context ().add_class (Gtk.STYLE_CLASS_FLAT);
        clear_button.add (bottom_row);

        attach (scroll_box, 0, 0);
        attach (clear_button, 0, 1);

        clear_button.clicked.connect (() => {
            clear_rolls ();
        });

        show_all ();
    }

    private void clear_rolls () {
        foreach (PreviousRoll item in previous_rolls_list) {
            item.destroy ();
        }
    }

    public void add_roll (int roll) {
        var new_roll = new Rollit.PreviousRoll.with_num (roll);

        previous_rolls_list.append (new_roll);
        previous_rolls_box.prepend (new_roll);
        previous_rolls_box.show_all ();
    }
}
