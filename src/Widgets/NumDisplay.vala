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
 * Authored by Patrick Csikos <akzeldev@fastmail.com>
 */

public class Rollit.NumDisplay : Gtk.Stack {
    Gtk.Label roll_result;

    construct {
        transition_type = Gtk.StackTransitionType.SLIDE_UP;
        hexpand = true;
        margin = 12;
        vexpand = true;

        roll_result = new Gtk.Label (null);
        roll_result.get_style_context ().add_class ("result-label");

        var welcome = new Gtk.Label (null);
        welcome.get_style_context ().add_class (Granite.STYLE_CLASS_H2_LABEL);
        welcome.label = "Ready to Roll";

        add_named (welcome, "welcome");
        add_named (roll_result, "roll-result");
        visible_child_name = "welcome";
    }

    public void num_gen (int max_num) {
        const int MIN_NUM = 1;
        int rnd_num;

        if (visible_child_name != "roll-result") {
            visible_child_name = "roll-result";
        }
        // max_num + 1 so that max num is included in roll
        rnd_num = Random.int_range (MIN_NUM, (max_num + 1));
        roll_result.label = @"$rnd_num";
    }
}
