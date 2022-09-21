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
    [GtkTemplate (ui = "/com/gitlab/zelikos/rollit/gtk/main-view.ui")]
    public class MainView : Adw.Bin {
        [GtkChild] private unowned Gtk.Revealer result_revealer;
        [GtkChild] private unowned Gtk.Stack result_stack;
        [GtkChild] private unowned Gtk.Label result_label;
        [GtkChild] private unowned Gtk.SpinButton max_roll;

        private Settings settings = new Settings ("com.gitlab.zelikos.rollit");


        construct {
            this.settings.bind ("max-roll", max_roll, "value", SettingsBindFlags.DEFAULT);
        }

        public int get_max_roll () {
            return max_roll.get_value_as_int();
        }

        public void set_result_label (string result) {
            if (result_stack.visible_child_name != "result") {
                result_stack.visible_child = result_stack.get_child_by_name ("result");
                result_label.label = result;
                result_revealer.reveal_child = true;
            } else {
                result_revealer.reveal_child = false;
                Timeout.add (result_revealer.transition_duration, () => {
                    result_label.label = result;
                    result_revealer.reveal_child = true;
                    return false;
                });
            }
        }

        public void reset_label () {
            result_stack.visible_child = result_stack.get_child_by_name ("empty");
        }
    }
}
