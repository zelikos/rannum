public class MyApp.Window : Gtk.ApplicationWindow {
    
    public Window (Application app) {
        Object (
            application: app
        );
    }

    construct {
        title = "RanNum";
        window_position = Gtk.WindowPosition.CENTER;
        set_default_size (800, 600);
        
        show_all ();
    }
}