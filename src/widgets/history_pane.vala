/*  Copyright (C) 2020-2022 Patrick Csikos (https://zelikos.github.io)
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

namespace Rollit {
    [GtkTemplate (ui = "/com/gitlab/zelikos/rollit/gtk/history-pane.ui")]
    public class HistoryPane : Adw.Bin {
        [GtkChild] private unowned Gtk.Stack history_stack;
        [GtkChild] private unowned Gtk.ListBox history_list;

        public void add_result (Rollit.HistoryItem roll_result) {
            history_list.append(roll_result);
            if (history_stack.visible_child_name != "filled") {
                history_stack.visible_child = history_stack.get_child_by_name ("filled");
            }
        }

        public void clear_history () {
            Gtk.ListBoxRow? current_item = history_list.get_row_at_index (0);
            while (current_item != null) {
                history_list.remove (current_item);
                current_item = history_list.get_row_at_index (0);
            }

            history_stack.visible_child = history_stack.get_child_by_name ("empty");
        }
    }
}
