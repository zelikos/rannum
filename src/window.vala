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
    [GtkTemplate (ui = "/com/gitlab/zelikos/rollit/gtk/window.ui")]
    public class Window : Adw.ApplicationWindow {
        [GtkChild] private unowned Rollit.MainView main_view;
        [GtkChild] private unowned Rollit.HistoryPane history_pane;
        [GtkChild] private unowned Adw.ToastOverlay toast_overlay;

        private Adw.Toast result_toast;

        private Settings settings = new Settings ("com.gitlab.zelikos.rollit");

        private ActionEntry[] actions = {
            { "roll", on_roll_action },
            { "clear", on_clear_action },
        };

        public Window (Adw.Application app) {
            Object (application: app);
        }

        construct {
            var action_group = new SimpleActionGroup ();
            action_group.add_action_entries (actions, this);

            insert_action_group ("dice", action_group);

            result_toast = new Adw.Toast (_("Result copied"));
            result_toast.timeout = 2;

            this.settings.bind ("window-width", this, "default-width", SettingsBindFlags.DEFAULT);
            this.settings.bind ("window-height", this, "default-height", SettingsBindFlags.DEFAULT);
            this.settings.bind ("window-maximized", this, "maximized", SettingsBindFlags.DEFAULT);
        }

        private void on_roll_action () {
            const int MIN_NUM = 1;
            int max_num;
            string rnd_num;

            max_num = main_view.get_max_roll();
            rnd_num = (Random.int_range (MIN_NUM, (max_num + 1))).to_string();

            main_view.set_result_label(rnd_num.to_string());

            var roll_result = new Rollit.HistoryItem(this, rnd_num, max_num.to_string());

            history_pane.add_result(roll_result);
        }

        private void on_clear_action () {
            history_pane.clear_history ();
            main_view.reset_label();
        }

        public void add_toast () {
            toast_overlay.add_toast (result_toast);
        }
    }
}
