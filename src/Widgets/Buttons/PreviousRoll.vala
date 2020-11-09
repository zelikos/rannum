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

 public class Rollit.PreviousRoll : Gtk.ListBoxRow {

    //  public signal void copied ();

    private Gtk.Image copy_icon;
    private uint timeout_id;

    public string roll_label { get; construct set; }
    public Gtk.Label roll_amount { get; set; }

    public PreviousRoll () {
        Object (
            roll_label: "0"
        );
    }

    public PreviousRoll.with_num (int roll) {
        Object (
            roll_label: roll.to_string()
        );
    }

    construct {
        roll_amount = new Gtk.Label (roll_label) {
            halign = START,
            valign = CENTER
        };

        copy_icon = new Gtk.Image.from_icon_name ("edit-copy-symbolic", Gtk.IconSize.SMALL_TOOLBAR) {
            halign = END,
            valign = CENTER
        };

        var button_layout = new Gtk.Box (HORIZONTAL, 32);
        button_layout.pack_start (roll_amount);
        button_layout.pack_end (copy_icon);

        var copied_label = new Gtk.Label (_("Copied"));

        var stack = new Gtk.Stack () {
            transition_duration = 200,
            transition_type = Gtk.StackTransitionType.CROSSFADE
        };

        stack.add_named (button_layout, "button-box");
        stack.add_named (copied_label, "copied");
        stack.visible_child_name = "button-box";

        var button = new Gtk.Button () {
            margin = 6
        };

        button.add (stack);

        add (button);

        button.clicked.connect ( () => {
            var cb = Gtk.Clipboard.get (Gdk.Atom.NONE);
            cb.set_text (roll_amount.label, -1);

            uint duration = 1000;

            stack.visible_child_name = "copied";
            timeout_id = GLib.Timeout.add (duration, () => {
                stack.visible_child_name = "button-box";
                timeout_id = 0;
                return false;
            });
        });

        activate.connect ( () => {
            button.clicked();
        });
    }
 }