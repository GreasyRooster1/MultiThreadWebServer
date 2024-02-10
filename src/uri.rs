use crate::paths::DATA_PATH;

pub(crate) fn find(uri: &str) -> String {
    //this should return "data/foo.html" (or whatever else, based on the URI) and keep uri's owner the same
    DATA_PATH.to_owned().clone() + &*uri
}
pub(crate) fn extract(http_request: &str) -> &str {
    let line = http_request.lines().next().unwrap();
    // return uri (remove GET prefix and HTTP/1.1 suffix)
    line.strip_prefix("GET").unwrap().strip_suffix("HTTP/1.1").unwrap().trim()
}
pub(crate) fn extension(filename:&str) -> &str {
    // get everything after the '.' char
    &filename[filename.rfind(".").unwrap_or(filename.len())..filename.len()]
}


//tests
#[cfg(test)]
mod tests {
    use crate::uri;
    #[test]
    fn find() {
        assert_eq!(uri::find("/index.html"), "data/index.html");
        assert_eq!(uri::find("/foo/bar.html"), "data/foo/bar.html");
        assert_eq!(uri::find("/"), "data/");

        let uri :&str = "/index.html";
        assert_eq!(uri::find(uri), "data/index.html");
        assert_eq!(uri,"/index.html");

        let mut uri_mut :&str = "/foo.html";
        assert_eq!(uri::find(uri_mut), "data/foo.html");
        assert_eq!(uri_mut,"/foo.html");
        uri_mut = "/bar.html";
        assert_eq!(uri::find(uri_mut), "data/bar.html");
        assert_eq!(uri_mut,"/bar.html");
    }
    #[test]
    fn extract() {
        assert_eq!(uri::extract("GET / HTTP/1.1"), "/");
        assert_eq!(uri::extract("GET /index.html HTTP/1.1"), "/index.html");
        assert_eq!(uri::extract("GET /foo/bar.html HTTP/1.1"), "/foo/bar.html");
        assert_eq!(uri::extract("GET /GET/HTTP/1.1 HTTP/1.1"), "/GET/HTTP/1.1");
    }
    #[test]
    fn extension(){
        assert_eq!(uri::extension("index.html"),".html");
        assert_eq!(uri::extension("script.js"),".js");
        assert_eq!(uri::extension("no-extension"),"");
        assert_eq!(uri::extension("mult.aple.ext.ensio.ns.html"),".html");
    }
}