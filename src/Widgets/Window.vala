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

public class Rollit.Window : Gtk.ApplicationWindow {

    private uint configure_id;

    public Window (Application app) {
        Object (
            application: app
        );
    }

    construct {
        default_width = 320;
        default_height = 260;

        int window_x, window_y;
        Application.settings.get ("window-position", "(ii)", out window_x, out window_y);

        
        if (window_x != -1 || window_y != -1) {
            move (window_x, window_y);
        }

        var header = new Gtk.HeaderBar ();
        header.title = "Roll-It";
        //header.get_style_context ().add_class (Gtk.STYLE_CLASS_FLAT);
        header.get_style_context ().add_class ("default-decoration");
        header.show_close_button = true;
        header.decoration_layout = "close:";

        var menu_button = new Gtk.MenuButton ();
        menu_button.image = new Gtk.Image.from_icon_name ("open-menu-symbolic", Gtk.IconSize.SMALL_TOOLBAR);
        menu_button.tooltip_text = _("Settings");
        menu_button.valign = Gtk.Align.CENTER;

        var menu_popover = new Gtk.Popover (menu_button);
        menu_button.popover = menu_popover;
        var menu_grid = new Rollit.Menu ();
        menu_popover.add (menu_grid);

        header.pack_end (menu_button);

        set_titlebar (header);

        var number_display = new Rollit.NumDisplay ();

        var roll_button = new Gtk.Button.with_label (_("Roll"));
        var btn_box = new Gtk.ButtonBox (Gtk.Orientation.HORIZONTAL);
        btn_box.halign = Gtk.Align.CENTER;
        btn_box.margin = 12;
        btn_box.margin_top = 0;
        btn_box.add (roll_button);

        var main_view = new Gtk.Grid ();
        main_view.attach (number_display, 1, 1, 1, 1);
        main_view.attach (btn_box, 1, 2, 1, 1);

        add (main_view);

        roll_button.clicked.connect (e => {
            int max_roll = menu_grid.get_max_value ();
            number_display.num_gen (max_roll);
        });

        show_all ();
    }

    public override bool configure_event (Gdk.EventConfigure event) {
        if (configure_id != 0) {
            GLib.Source.remove (configure_id);
        }

        configure_id = Timeout.add (100, () => {
            configure_id = 0;

            int root_x, root_y;
            get_position (out root_x, out root_y);
            Application.settings.set ("window-position", "(ii)", root_x, root_y);

            return false;
            }
        );

        return base.configure_event (event);
    }
}
