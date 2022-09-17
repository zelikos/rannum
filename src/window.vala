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
    [GtkTemplate (ui = "/com/gitlab/zelikos/rollit/window.ui")]
    public class Window : Gtk.ApplicationWindow {
        [GtkChild] private unowned Gtk.Label result_label;
        [GtkChild] private unowned Gtk.SpinButton max_roll;
        [GtkChild] private unowned Gtk.ListBox history_list;
        [GtkChild] private unowned Gtk.Stack history_stack;
        [GtkChild] private unowned Adw.ToastOverlay toast_overlay;

        private Adw.Toast result_toast = new Adw.Toast (_("Result copied"));

        private Settings settings = new Settings ("com.gitlab.zelikos.rollit");

        private ActionEntry[] actions = {
            { "roll", on_roll_action },
            { "clear", on_clear_action },
            { "add-toast", add_toast },
        };

        public Window (Adw.Application app) {
            Object (application: app);
        }

        construct {
            var action_group = new SimpleActionGroup ();
            action_group.add_action_entries (actions, this);

            insert_action_group ("dice", action_group);

            this.settings.bind ("window-width", this, "default-width", SettingsBindFlags.DEFAULT);
            this.settings.bind ("window-height", this, "default-height", SettingsBindFlags.DEFAULT);
            this.settings.bind ("window-maximized", this, "maximized", SettingsBindFlags.DEFAULT);

            this.settings.bind ("max-roll", max_roll, "value", SettingsBindFlags.DEFAULT);
        }

        private void on_roll_action () {
            const int MIN_NUM = 1;
            int max_num;
            string rnd_num;

            max_num = max_roll.get_value_as_int();

            rnd_num = (Random.int_range (MIN_NUM, (max_num + 1))).to_string();

            result_label.label = rnd_num;

            var roll_result = new Rollit.HistoryItem(rnd_num);
            roll_result.subtitle = (_("Out of ") + max_num.to_string());

            history_list.append(roll_result);

            if (history_stack.visible_child_name != "filled") {
                history_stack.visible_child = history_stack.get_child_by_name ("filled");
            }
        }

        private void on_clear_action () {
            Gtk.ListBoxRow? current_item = history_list.get_row_at_index (0);
            while (current_item != null) {
                history_list.remove (current_item);
                current_item = history_list.get_row_at_index (0);
            }

            history_stack.visible_child = history_stack.get_child_by_name ("empty");
        }

        private void add_toast () {
            toast_overlay.add_toast (result_toast);
        }
    }
}
