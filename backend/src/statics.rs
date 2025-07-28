use include_dir::{ include_dir, Dir };
use salvo::prelude::*;

static FRONTEND: Dir = include_dir!("./dist");

#[handler]
pub async fn index(req: &mut Request, res: &mut Response) {
    let path = req.uri().path();

    if path == "/" {
        res.add_header("Content-Type", "text/html", true).unwrap();
        res.write_body(FRONTEND.get_file("index.html").unwrap().contents().to_vec()).unwrap();
    } else {
        match get_file(&path[1..]) {
            Some(contents) => {
                res.add_header(
                    "Content-Type",
                    mime_guess::from_path(path).first().unwrap().as_ref(),
                    true
                ).unwrap();
                res.write_body(contents.to_vec()).unwrap();
            }
            None => {
                res.add_header("Content-Type", "text/html", true).unwrap();
                res.write_body(
                    FRONTEND.get_file("index.html").unwrap().contents().to_vec()
                ).unwrap();
            }
        }
    }
}

fn get_file(path: &str) -> Option<&'static [u8]> {
    FRONTEND.get_file(path).map(|f| f.contents())
}
