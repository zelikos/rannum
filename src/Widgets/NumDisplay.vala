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

public class Rollit.NumDisplay : Gtk.Stack {

    private Gtk.Label roll_result;

    construct {
        transition_type = Gtk.StackTransitionType.SLIDE_UP;
        hexpand = true;
        vexpand = true;

        roll_result = new Gtk.Label (null);
        roll_result.get_style_context ().add_class ("result-label");

        var welcome = new Gtk.Label (null);
        welcome.label = _("Ready to Roll");
        welcome.get_style_context ().add_class (Granite.STYLE_CLASS_H2_LABEL);

        var blank = new Gtk.Label (null);

        add_named (welcome, "welcome");
        add_named (roll_result, "roll-result");
        add_named (blank, "blank");
        visible_child_name = "welcome";
    }

    public void num_gen (int max_num) {
        const int MIN_NUM = 1;
        int rnd_num;

        visible_child_name = "blank";

        // max_num + 1 so that max num is included in roll
        rnd_num = Random.int_range (MIN_NUM, (max_num + 1));
        roll_result.label = @"$rnd_num";

        visible_child_name = "roll-result";
    }
}
