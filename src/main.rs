mod archive_tools;

use archive_tools::tests::*;

fn main() {
    // Testing
    // test_types();
    // test_forms();
    // test_archive();

    // Writing
    let path = "./archives/test_archive.smn";
    test_archive_write(path);
    //test_archive_read(path);
    test_write_form("./archives/test_archive.smn");
    //test_perf_write_x_forms(&path, 5000);
    //test_form_delete(path);
    
}
