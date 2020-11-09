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

public class Rollit.RollHistory : Gtk.Grid { // TODO: Change to appropriate type of widget

    private SList<Rollit.PreviousRoll> previous_rolls;
    private Gtk.Box previous_rolls_box;
    private Gtk.Button clear_button;
    //  private Gtk.Box average_roll;

    private const uint MAX_PREVIOUS_ROLLS = 10;

    construct {
        row_spacing = 12;
        margin = 12;

        //  previous_rolls = new SList<Rollit.PreviousRoll> ();
        var previous_roll = new Rollit.PreviousRoll ();

        previous_rolls_box = new Gtk.Box (VERTICAL, 12) {
            hexpand = true,
            vexpand = true
        };

        previous_rolls_box.add (previous_roll);

        clear_button = new Gtk.Button.with_label (_("Clear"));
        clear_button.get_style_context ().add_class ("destructive-action");

        var separator = new Gtk.Separator (HORIZONTAL);

        attach (previous_rolls_box, 0, 0);
        //  attach (separator, 0, 1);
        attach (clear_button, 0, 1);

        show_all ();
    }

    private void clear_rolls () {
        //  previous_rolls.remove_all();
    }

    //  private void set_average () {

    //  }

    public void add_roll (int roll) {

    }
}