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

public class RanNum.NumDisplay : Gtk.Label {

    construct {
        margin = 20;
        get_style_context ().add_class ("h1");
        label = "";
    }

    public void num_gen () {
        int min_num = 1, max_num = 100, rnd_num;

        rnd_num = Random.int_range (min_num, max_num);
        this.set_label (@"$rnd_num");
    }
}