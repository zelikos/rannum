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

    private Gtk.Grid button_layout;
    private Gtk.Image copy_icon;

    public Gtk.Label roll_amount { get; set; }

    construct {
        button_layout = new Gtk.Grid () {
            column_homogeneous = true,
            column_spacing = 24,
            margin = 12
        };

        roll_amount = new Gtk.Label ("0");

        copy_icon = new Gtk.Image.from_icon_name ("edit-copy-symbolic", Gtk.IconSize.MENU);

        button_layout.attach (roll_amount, 0, 0);
        button_layout.attach (copy_icon, 1, 0);

        add (button_layout);

        //  clicked.connect ( () => {

        //  });
    }
 }