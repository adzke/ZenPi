#include "buttons.h"
#include <iostream>
#include <cstdlib> // for system()

Buttons::Buttons() : mBox(Gtk::ORIENTATION_VERTICAL),
                     leftBox(Gtk::ORIENTATION_VERTICAL),
                     rightBox(Gtk::ORIENTATION_VERTICAL)
{

    set_title("ZEN PLAYER by Ad");
    set_size_request(250, 250);
    add(mBox);

    // Create a CssProvider
    auto css_provider = Gtk::CssProvider::create();

    // Load CSS data from a string
    css_provider->load_from_data("window { background: rgb(131,58,180); background: linear-gradient(90deg, rgba(131,58,180,1) 0%, rgba(253,29,29,1) 50%, rgba(252,176,69,1) 100%); } button {background: rgb(131,58,180);} ");

    // Get the style context of the window
    auto style_context = get_style_context();

    // Add the CssProvider to the style context with a high priority
    style_context->add_provider(css_provider, GTK_STYLE_PROVIDER_PRIORITY_USER);

    leftBox.set_hexpand(true);           // expand horizontally
    leftBox.set_halign(Gtk::ALIGN_FILL); // fill horizontally
    leftBox.set_vexpand(true);           // expand vertically
    leftBox.set_valign(Gtk::ALIGN_FILL); // fill vertically

    showButton.set_hexpand(true);           // expand horizontally
    showButton.set_halign(Gtk::ALIGN_FILL); // fill horizontally
    showButton.set_vexpand(true);           // expand vertically
    showButton.set_valign(Gtk::ALIGN_FILL); // fill vertically

    hideButton.set_hexpand(true);           // expand horizontally
    hideButton.set_halign(Gtk::ALIGN_FILL); // fill horizontally
    hideButton.set_vexpand(true);           // expand vertically
    hideButton.set_valign(Gtk::ALIGN_FILL); // fill vertically

    mBox.add(leftBox);
    mBox.add(rightBox);

    showButton.set_label("Play Delta");
    hideButton.set_label("Stop Delta");
    label.set_label("This is a basic button example");
    label.set_padding(20, 20);
 
    showButton.signal_clicked().connect(sigc::mem_fun(*this, &Buttons::on_label_show));
    hideButton.signal_clicked().connect(sigc::mem_fun(*this, &Buttons::on_label_hide));

    leftBox.pack_start(showButton, Gtk::PACK_SHRINK);
    leftBox.pack_start(hideButton, Gtk::PACK_SHRINK);

    rightBox.pack_start(label, Gtk::PACK_SHRINK);

    show_all_children();
    label.hide();
}
void Buttons::on_label_show()
{	
    if (playing == true) {
   	return;	
    }    
    playing = true;
    system("mpv ~/Downloads/delta.m4a &"); // list files in current directory
}

void Buttons::on_label_hide()
{
    playing = false;
    system("pkill mpv"); // list files in current directory
    label.hide();
}

Buttons::~Buttons() {}
