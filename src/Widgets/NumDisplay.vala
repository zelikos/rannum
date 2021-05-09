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

public class Rollit.NumDisplay : Gtk.Widget {

    private Gtk.Label roll_result;
    private Gtk.Stack num_display_stack;

    static construct {
        set_layout_manager_type (typeof (Gtk.BinLayout));
    }

    construct {
        hexpand = true;
        vexpand = true;

        num_display_stack = new Gtk.Stack ();

        num_display_stack.transition_type = Gtk.StackTransitionType.SLIDE_UP;
        num_display_stack.transition_duration = 200;
        // num_display_stack.hexpand = true;
        // num_display_stack.vexpand = true;

        roll_result = new Gtk.Label (null);
        roll_result.get_style_context ().add_class ("result-label");

        var welcome = new Gtk.Label (null);
        welcome.label = _("Ready to Roll");
        // welcome.get_style_context ().add_class (Granite.STYLE_CLASS_H2_LABEL);

        var blank = new Gtk.Label (null);

        num_display_stack.add_named (welcome, "welcome");
        num_display_stack.add_named (roll_result, "roll-result");
        num_display_stack.add_named (blank, "blank");
        num_display_stack.visible_child_name = "welcome";

        num_display_stack.set_parent (this);
    }

    protected override void dispose () {
        num_display_stack.unparent ();
}

    public int num_gen (int max_num) {
        const int MIN_NUM = 1;
        int rnd_num;

        num_display_stack.visible_child_name = "blank";

        // max_num + 1 so that max num is included in roll
        rnd_num = Random.int_range (MIN_NUM, (max_num + 1));
        roll_result.label = @"$rnd_num";

        num_display_stack.visible_child_name = "roll-result";

        return rnd_num;
    }
}
