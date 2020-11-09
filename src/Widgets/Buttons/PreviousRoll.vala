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

 public class Rollit.PreviousRoll : Gtk.Button {

    private Gtk.Image copy_icon;

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
        margin = 12;
        margin_bottom = 0;

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

        add (button_layout);

        //  clicked.connect ( () => {
        //      TODO: Copy roll_amount to clipboard
        //  });
    }
 }