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

public class RanNum.Window : Gtk.ApplicationWindow {

    private uint configure_id;

    public Window (Application app) {
        Object (
            application: app
        );
    }

    construct {
        int window_x, window_y;
        var rect = Gtk.Allocation ();

        Application.settings.get ("window-position", "(ii)", out window_x, out window_y);
        Application.settings.get ("window-size", "(ii)", out rect.width, out rect.height);

        if (window_x != -1 || window_y != -1) {
            move (window_x, window_y);
        }

        set_allocation (rect);

        if (Application.settings.get_boolean ("window-maximized")) {
            maximize ();
        }


        var header = new Gtk.HeaderBar ();
        header.title = "RanNum";
        // header.subtitle = "Random number generation";

        header.show_close_button = true;

        var start_button = new Gtk.Button.from_icon_name ("media-playback-start", Gtk.IconSize.LARGE_TOOLBAR);
        start_button.valign = Gtk.Align.CENTER;
        
        var menu_button = new Gtk.MenuButton ();
        menu_button.image = new Gtk.Image.from_icon_name ("open-menu", Gtk.IconSize.LARGE_TOOLBAR);
        menu_button.tooltip_text = "Menu";
        menu_button.valign = Gtk.Align.CENTER;

        var menu_popover = new Gtk.Popover (menu_button);
        menu_button.popover = menu_popover;
        var menu_grid = new RanNum.Menu ();
        menu_popover.add (menu_grid);

        header.pack_start (start_button);
        header.pack_end (menu_button);

        set_titlebar (header);

        var number_display = new RanNum.NumDisplay ();
        add (number_display);

        start_button.clicked.connect (e => {
            number_display.num_gen();
        });

        show_all ();
    }

    public override bool configure_event (Gdk.EventConfigure event) {
        if (configure_id != 0) {
            GLib.Source.remove (configure_id);
        }

        configure_id = Timeout.add (100, () => {
            configure_id = 0;

            if (is_maximized) {
                Application.settings.set_boolean ("window-maximized", true);
            } else {
                Application.settings.set_boolean ("window-maximized", false);

                Gdk.Rectangle rect;
                get_allocation (out rect);
                Application.settings.set ("window-size", "(ii)", rect.width, rect.height);

                int root_x, root_y;
                get_position (out root_x, out root_y);
                Application.settings.set ("window-position", "(ii)", root_x, root_y);
            }

            return false;
        });

        return base.configure_event (event);
    }
}