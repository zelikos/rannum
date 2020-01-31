public class Application : Gtk.Application {

    public Application () {
        Object (
            application_id: "com.github.zelikos.rannum",
            flags: ApplicationFlags.FLAGS_NONE
        );
    }

    protected override void activate () {
        Gtk.ApplicationWindow window = new MyApp.Window (this);

        add_window (window);
    }
}