/*  Copyright (C) 2021 Patrick Csikos (https://zelikos.github.io)
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

public class Rollit.MenuItem : Gtk.Button {

    public Gtk.RadioButton dice_radio { get; private set; }
    public string dice_label { get; construct set; }
    public string dice_accel { get; construct set; }

    public MenuItem (string label, string accel) {
        Object (
            dice_label: label,
            dice_accel: accel
        );
    }

    construct {
        get_style_context ().add_class (Gtk.STYLE_CLASS_FLAT);

        dice_radio = new Gtk.RadioButton (null);
        var accel_label = new Granite.AccelLabel (dice_label, dice_accel);

        var box = new Gtk.Box (HORIZONTAL, 6);
        box.pack_start (dice_radio);
        box.pack_end (accel_label);

        add (box);

        clicked.connect ( () => {
           dice_radio.clicked ();
        });
    }
}
