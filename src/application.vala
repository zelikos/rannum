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
    public class Application : Adw.Application {
        public Application () {
            Object (
                application_id: "com.gitlab.zelikos.rollit",
                flags: ApplicationFlags.FLAGS_NONE
            );
        }

        construct {
            ActionEntry[] action_entries = {
                { "about", this.on_about_action },
                // { "preferences", this.on_preferences_action },
                { "quit", this.quit }
            };
            this.add_action_entries (action_entries, this);
            this.set_accels_for_action ("app.quit", {"<primary>q"});
        }

        public override void activate () {
            base.activate ();
            var win = this.active_window;
            if (win == null) {
                win = new Rollit.Window (this);
            }
            win.present ();
        }

        private void on_about_action () {
            string[] authors = { "Patrick Csikos <zelikos@pm.me>" };

            string translators = "translator-credits";

            var about = new Adw.AboutWindow () {
                application_name = "Roll-It",
                application_icon = "com.gitlab.zelikos.rollit",
                version = "3.0.0",
                comments = "Roll the dice",
                copyright = "Copyright © 2020-2022 Patrick Csikos",
                // license_type = ,
                developer_name = "Patrick Csikos",
                developers = authors,
                translator_credits = translators,
                website = "https://gitlab.com/zelikos/rollit",
                issue_url = "https://gitlab.com/zelikos/rollit/issues"
            };

            about.set_transient_for (this.get_active_window());
            about.show();

            // Gtk.show_about_dialog (this.active_window,
            //                        "program-name", "Roll-It",
            //                        "authors", authors,
            //                        "version", "3.0.0",
            //                        "title", "About Roll-It");
        }

        // private void on_preferences_action () {
        //     message ("app.preferences action activated");
        // }
    }
}
