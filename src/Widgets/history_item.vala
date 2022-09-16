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
    public class HistoryItem : Adw.ActionRow {

        public HistoryItem (string roll_result) {
            title = roll_result;
            tooltip_text = "Copy to clipboard";
            activatable = true;
        }

        construct {
            add_suffix(new Gtk.Image.from_icon_name ("edit-copy-symbolic"));

            this.activated.connect (() => {
                Gdk.Clipboard clip = get_clipboard();
                clip.set_text (title);
            });
        }
    }
}
