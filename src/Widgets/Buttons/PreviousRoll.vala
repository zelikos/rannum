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

public class Rollit.PreviousRoll : Gtk.ListBoxRow {

    public signal void copied ();

    private Gtk.Image copy_icon;
    private uint timeout_id;

    public string roll_label { get; construct set; }
    public Gtk.Label roll_amount { get; set; }

    public PreviousRoll (int roll) {
        Object (
            roll_label: roll.to_string()
        );
    }

    construct {
        roll_amount = new Gtk.Label (roll_label);

        copy_icon = new Gtk.Image.from_icon_name ("edit-copy-symbolic");//, Gtk.IconSize.SMALL_TOOLBAR);

        var button_layout = new Gtk.Box (HORIZONTAL, 12);
        button_layout.append (roll_amount);
        button_layout.append (copy_icon);

        var copied_label = new Gtk.Label (_("Copied"));

        var stack = new Gtk.Stack () {
            transition_duration = 200, // Granite.TRANSITION_DURATION_OPEN,
            transition_type = Gtk.StackTransitionType.CROSSFADE
        };

        stack.add_named (button_layout, "button-box");
        stack.add_named (copied_label, "copied");
        stack.visible_child_name = "button-box";

        var button = new Gtk.Button () {
            margin_start = 6,
            margin_end = 6,
            margin_top = 6,
            margin_bottom = 6,
            tooltip_text = _("Copy result to clipboard")
        };

        button.set_child (stack);

        set_child (button);

        button.clicked.connect ( () => {
            uint duration = 1000;
            // copy_to_clipboard (roll_label);

            stack.visible_child_name = "copied";
            timeout_id = GLib.Timeout.add (duration, () => {
                stack.visible_child_name = "button-box";
                timeout_id = 0;
                return false;
            });
        });

        activate.connect ( () => {
            button.clicked ();
        });
    }

    // private void copy_to_clipboard (string roll) {
    //     var cb = Gtk.Clipboard.get (Gdk.Atom.NONE);
    //     cb.set_text (roll, -1);
    //     copied ();
    // }
 }
