//! NexoPDF identity and links in the current Home, About and Help surfaces.

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use nexopdf_engine::links;
use nexopdf_ui_egui::{Dialog, NexoPDFApp};

fn harness(setup: impl FnOnce(&mut NexoPDFApp) + 'static) -> Harness<'static, NexoPDFApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = NexoPDFApp::new();
        app.set_option("language", "en").unwrap();
        setup(&mut app);
        app
    });
    h.run_steps(4);
    h
}

#[test]
fn github_command_opens_the_nexopdf_repository() {
    let mut h = harness(|_| {});
    assert!(h.state_mut().execute("help.github"));
    assert_eq!(h.state().last_opened_url.as_deref(), Some("https://github.com/junior-aguiar-eng/NexoPDF"));
}

#[test]
fn home_screen_shows_nexopdf_identity() {
    let h = harness(|_| {});
    h.get_by_label("Welcome to NexoPDF");
    assert_eq!(h.query_all_by_label("Contributors").count(), 0);
    assert_eq!(h.query_all_by_label("Join our Discord").count(), 0);
    assert!(h.query_all_by_label("NexoPDF").count() >= 1, "the NexoPDF mark is exposed to accessibility");
}

#[test]
fn about_dialog_shows_nexopdf_identity() {
    let pdf = b"%PDF-1.7\n1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj\n2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj\n3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] >> endobj\ntrailer << /Root 1 0 R >>\n%%EOF";
    // With a document open, so the home screen's own links are not on screen.
    let h = harness(move |app| {
        app.open_bytes("one.pdf", None, pdf.to_vec()).unwrap();
        app.dialog = Some(Dialog::About);
    });
    assert!(h.query_all_by_label("NexoPDF").count() >= 2, "About shows the NexoPDF mark and name");
    h.get_by_label(&format!("Version {}", env!("CARGO_PKG_VERSION")));
    assert_eq!(h.query_all_by_label("Contributors").count(), 0);
    assert_eq!(h.query_all_by_label("Join our Discord").count(), 0);
}

#[test]
fn help_commands_open_each_link() {
    for l in links::LINKS {
        let mut h = harness(|_| {});
        assert!(h.state_mut().execute(l.command), "{}", l.command);
        assert_eq!(h.state().last_opened_url.as_deref(), Some(l.url));
        assert!(nexopdf_engine::commands::command(l.command).is_some(), "the NexoPDF project command remains registered");
    }
}
